//! Time-decay scoring — reduces threat scores over time if no new signals appear.
//!
//! When a process generates a spike (e.g., compiler build), the score should
//! decay back to zero if no new events occur. Persistent threats (backdoors)
//! keep re-triggering, so their score stays high.
//!
//! Formula: `decayed = score × e^(−λt)` where t = seconds since last signal.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

/// Configuration for the decay engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayConfig {
    /// Half-life in seconds — how long until a score halves.
    /// Default: 120s (2 minutes).
    pub half_life_secs: f64,
    /// Minimum score threshold — below this, treat as clean (0).
    pub floor: f64,
    /// Maximum history entries per PID.
    pub max_history: usize,
}

impl Default for DecayConfig {
    fn default() -> Self {
        Self {
            half_life_secs: 120.0,
            floor: 1.0,
            max_history: 100,
        }
    }
}

impl DecayConfig {
    /// Compute the decay rate λ from half-life.
    /// λ = ln(2) / half_life
    pub fn lambda(&self) -> f64 {
        std::f64::consts::LN_2 / self.half_life_secs
    }
}

/// A single score observation.
#[derive(Debug, Clone)]
struct ScoreEntry {
    score: f64,
    timestamp: Instant,
}

/// Time-decay engine — tracks scores per PID and applies exponential decay.
#[derive(Debug)]
pub struct DecayEngine {
    config: DecayConfig,
    /// PID → history of scores.
    history: HashMap<u32, Vec<ScoreEntry>>,
    /// PID → peak score (highest ever seen).
    peaks: HashMap<u32, f64>,
}

/// Snapshot of a decayed score for external consumption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayedScore {
    pub pid: u32,
    pub raw_score: f64,
    pub decayed_score: f64,
    pub peak_score: f64,
    pub age_secs: f64,
    pub is_active: bool,
}

impl DecayEngine {
    pub fn new() -> Self {
        Self::with_config(DecayConfig::default())
    }

    pub fn with_config(config: DecayConfig) -> Self {
        DecayEngine {
            config,
            history: HashMap::new(),
            peaks: HashMap::new(),
        }
    }

    /// Record a new score observation for a PID.
    pub fn record(&mut self, pid: u32, score: f64) {
        let entry = ScoreEntry {
            score,
            timestamp: Instant::now(),
        };

        let entries = self.history.entry(pid).or_default();
        entries.push(entry);

        // Trim old entries
        if entries.len() > self.config.max_history {
            entries.drain(..entries.len() - self.config.max_history);
        }

        // Update peak
        let peak = self.peaks.entry(pid).or_insert(0.0);
        if score > *peak {
            *peak = score;
        }
    }

    /// Get the current decayed score for a PID.
    pub fn get_score(&self, pid: u32) -> Option<DecayedScore> {
        let entries = self.history.get(&pid)?;
        let last = entries.last()?;
        let peak = self.peaks.get(&pid).copied().unwrap_or(0.0);

        let elapsed = last.timestamp.elapsed();
        let decayed = apply_decay(last.score, elapsed, self.config.lambda());

        let final_score = if decayed < self.config.floor {
            0.0
        } else {
            decayed
        };

        Some(DecayedScore {
            pid,
            raw_score: last.score,
            decayed_score: final_score,
            peak_score: peak,
            age_secs: elapsed.as_secs_f64(),
            is_active: final_score > self.config.floor,
        })
    }

    /// Get decayed scores for all tracked PIDs.
    pub fn all_scores(&self) -> Vec<DecayedScore> {
        self.history
            .keys()
            .filter_map(|pid| self.get_score(*pid))
            .collect()
    }

    /// Remove a PID from tracking (process exited).
    pub fn remove(&mut self, pid: u32) {
        self.history.remove(&pid);
        self.peaks.remove(&pid);
    }

    /// Get the number of active (non-decayed) threats.
    pub fn active_count(&self) -> usize {
        self.all_scores().iter().filter(|s| s.is_active).count()
    }

    /// Calculate the system-wide risk index.
    /// R = Σ(score_i × impact_i) / n
    pub fn risk_index(&self) -> f64 {
        let scores = self.all_scores();
        if scores.is_empty() {
            return 0.0;
        }
        let sum: f64 = scores.iter().map(|s| s.decayed_score).sum();
        sum / scores.len() as f64
    }

    /// Compute decayed score from a raw score and explicit elapsed time.
    /// Useful for testing without waiting.
    pub fn compute_decay_at(&self, score: f64, elapsed: Duration) -> f64 {
        let decayed = apply_decay(score, elapsed, self.config.lambda());
        if decayed < self.config.floor {
            0.0
        } else {
            decayed
        }
    }
}

impl Default for DecayEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Exponential decay: score × e^(−λt)
fn apply_decay(score: f64, elapsed: Duration, lambda: f64) -> f64 {
    let t = elapsed.as_secs_f64();
    score * (-lambda * t).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decay_config_defaults() {
        let cfg = DecayConfig::default();
        assert!((cfg.half_life_secs - 120.0).abs() < 0.01);
        assert!((cfg.floor - 1.0).abs() < 0.01);
    }

    #[test]
    fn lambda_from_half_life() {
        let cfg = DecayConfig {
            half_life_secs: 120.0,
            ..Default::default()
        };
        let lambda = cfg.lambda();
        // λ = ln(2)/120 ≈ 0.00578
        assert!((lambda - 0.00578).abs() < 0.001);
    }

    #[test]
    fn score_halves_after_half_life() {
        let engine = DecayEngine::new();
        let half_life = Duration::from_secs(120);
        let decayed = engine.compute_decay_at(100.0, half_life);
        // Should be ~50 after one half-life
        assert!((decayed - 50.0).abs() < 1.0, "Got {decayed}, expected ~50");
    }

    #[test]
    fn score_quarters_after_two_half_lives() {
        let engine = DecayEngine::new();
        let two_half = Duration::from_secs(240);
        let decayed = engine.compute_decay_at(100.0, two_half);
        // Should be ~25 after two half-lives
        assert!((decayed - 25.0).abs() < 1.0, "Got {decayed}, expected ~25");
    }

    #[test]
    fn score_drops_to_zero_below_floor() {
        let config = DecayConfig {
            half_life_secs: 10.0, // Fast decay
            floor: 1.0,
            ..Default::default()
        };
        let engine = DecayEngine::with_config(config);
        // After 100 seconds with 10s half-life: 100 × 2^(-10) ≈ 0.098
        let decayed = engine.compute_decay_at(100.0, Duration::from_secs(100));
        assert!((decayed - 0.0).abs() < 0.01, "Got {decayed}, expected 0");
    }

    #[test]
    fn record_and_retrieve() {
        let mut engine = DecayEngine::new();
        engine.record(42, 75.0);

        let score = engine.get_score(42).unwrap();
        assert_eq!(score.pid, 42);
        assert!((score.raw_score - 75.0).abs() < 0.01);
        // Just recorded, so decayed ≈ raw
        assert!((score.decayed_score - 75.0).abs() < 1.0);
        assert!(score.is_active);
    }

    #[test]
    fn peak_tracking() {
        let mut engine = DecayEngine::new();
        engine.record(1, 30.0);
        engine.record(1, 80.0);
        engine.record(1, 50.0);

        let score = engine.get_score(1).unwrap();
        assert!((score.peak_score - 80.0).abs() < 0.01);
    }

    #[test]
    fn remove_pid() {
        let mut engine = DecayEngine::new();
        engine.record(99, 50.0);
        assert!(engine.get_score(99).is_some());

        engine.remove(99);
        assert!(engine.get_score(99).is_none());
    }

    #[test]
    fn risk_index_empty() {
        let engine = DecayEngine::new();
        assert!((engine.risk_index() - 0.0).abs() < 0.01);
    }

    #[test]
    fn risk_index_with_scores() {
        let mut engine = DecayEngine::new();
        engine.record(1, 60.0);
        engine.record(2, 40.0);
        // Average ≈ (60 + 40) / 2 = 50 (freshly recorded, minimal decay)
        let ri = engine.risk_index();
        assert!((ri - 50.0).abs() < 2.0, "Got {ri}, expected ~50");
    }

    #[test]
    fn active_count() {
        let config = DecayConfig {
            half_life_secs: 120.0,
            floor: 1.0,
            ..Default::default()
        };
        let mut engine = DecayEngine::with_config(config);
        engine.record(1, 80.0);
        engine.record(2, 50.0);
        engine.record(3, 0.5); // Below floor → not active

        assert_eq!(engine.active_count(), 2);
    }
}
