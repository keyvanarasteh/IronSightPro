//! Heuristic engine — aggregates security signals into a threat score.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::signals::{Signal, SignalCategory};

/// Threat level classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ThreatLevel {
    Clean,      // 0–10
    Low,        // 11–30
    Medium,     // 31–50
    High,       // 51–75
    Critical,   // 76–100
}

impl ThreatLevel {
    /// STEP 5: Use round() instead of truncation.
    pub fn from_score(score: f64) -> Self {
        match score.round() as u32 {
            0..=10 => Self::Clean,
            11..=30 => Self::Low,
            31..=50 => Self::Medium,
            51..=75 => Self::High,
            _ => Self::Critical,
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Clean => "✅",
            Self::Low => "🟡",
            Self::Medium => "🟠",
            Self::High => "🔴",
            Self::Critical => "🚨",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// STEP 2: HeuristicConfig — configurable weights and thresholds
// ─────────────────────────────────────────────────────────────────────────────

/// Per-category weight multipliers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryWeights {
    pub static_analysis: f64,
    pub path_analysis: f64,
    pub network_behavior: f64,
    pub memory_anomaly: f64,
    pub process_behavior: f64,
}

impl Default for CategoryWeights {
    fn default() -> Self {
        Self {
            static_analysis: 1.0,
            path_analysis: 1.0,
            network_behavior: 1.2,
            memory_anomaly: 1.5,
            process_behavior: 1.0,
        }
    }
}

impl CategoryWeights {
    pub fn multiplier_for(&self, category: &SignalCategory) -> f64 {
        match category {
            SignalCategory::StaticAnalysis => self.static_analysis,
            SignalCategory::PathAnalysis => self.path_analysis,
            SignalCategory::NetworkBehavior => self.network_behavior,
            SignalCategory::MemoryAnomaly => self.memory_anomaly,
            SignalCategory::ProcessBehavior => self.process_behavior,
        }
    }
}

/// Full heuristic engine configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeuristicConfig {
    pub max_score: f64,
    pub category_weights: CategoryWeights,
}

impl Default for HeuristicConfig {
    fn default() -> Self {
        Self {
            max_score: 100.0,
            category_weights: CategoryWeights::default(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ThreatAssessment — STEP 6: confidence score, STEP 7: HashMap categories
// ─────────────────────────────────────────────────────────────────────────────

/// Complete threat assessment for a process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAssessment {
    pub pid: u32,
    pub name: String,
    pub score: f64,
    pub level: ThreatLevel,
    pub confidence: f64,
    pub signals: Vec<Signal>,
    pub category_scores: HashMap<SignalCategory, f64>,
    pub recommended_action: RecommendedAction,
}

/// Recommended automated response based on threat level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecommendedAction {
    /// No action needed.
    None,
    /// Log and monitor more closely.
    Monitor,
    /// Alert security team.
    Alert,
    /// Suspend the process (SIGSTOP).
    Suspend,
    /// Suspend, dump memory, then kill.
    SuspendDumpKill,
}

impl RecommendedAction {
    pub fn from_level(level: ThreatLevel) -> Self {
        match level {
            ThreatLevel::Clean => Self::None,
            ThreatLevel::Low => Self::Monitor,
            ThreatLevel::Medium => Self::Alert,
            ThreatLevel::High => Self::Suspend,
            ThreatLevel::Critical => Self::SuspendDumpKill,
        }
    }
}

/// Legacy compat type alias.
pub type CategoryScores = HashMap<SignalCategory, f64>;

// ─────────────────────────────────────────────────────────────────────────────
// HeuristicEngine
// ─────────────────────────────────────────────────────────────────────────────

/// The heuristic engine that computes threat assessments.
pub struct HeuristicEngine {
    config: HeuristicConfig,
}

impl Default for HeuristicEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl HeuristicEngine {
    pub fn new() -> Self {
        HeuristicEngine {
            config: HeuristicConfig::default(),
        }
    }

    /// Create with custom configuration (STEP 2).
    pub fn with_config(config: HeuristicConfig) -> Self {
        HeuristicEngine { config }
    }

    /// Compute a threat assessment from collected signals.
    /// STEP 4: deduplicates signals by name (keeps highest weight).
    /// STEP 6: computes confidence score.
    pub fn assess(&self, pid: u32, name: &str, signals: Vec<Signal>) -> ThreatAssessment {
        // ── STEP 4: Deduplicate signals by name (keep highest weight) ────
        let mut seen: HashMap<String, Signal> = HashMap::new();
        for signal in signals {
            seen.entry(signal.name.clone())
                .and_modify(|existing| {
                    if signal.weight > existing.weight {
                        *existing = signal.clone();
                    }
                })
                .or_insert(signal);
        }
        let deduped: Vec<Signal> = seen.into_values().collect();

        // ── STEP 7: HashMap-based category scores ────────────────────────
        let mut cat_scores: HashMap<SignalCategory, f64> = HashMap::new();

        for signal in &deduped {
            let multiplier = self.config.category_weights.multiplier_for(&signal.category);
            *cat_scores.entry(signal.category).or_insert(0.0) += signal.weight * multiplier;
        }

        let raw_score: f64 = cat_scores.values().sum();

        // Clamp to max_score
        let score = raw_score.min(self.config.max_score);
        let level = ThreatLevel::from_score(score);
        let recommended_action = RecommendedAction::from_level(level);

        // ── STEP 6: Confidence score ─────────────────────────────────────
        // Higher confidence with more signals from diverse categories.
        let signal_count = deduped.len() as f64;
        let category_diversity = cat_scores.len() as f64;
        let confidence = (signal_count * 0.15 + category_diversity * 0.2).min(1.0);

        ThreatAssessment {
            pid,
            name: name.to_string(),
            score,
            level,
            confidence,
            signals: deduped,
            category_scores: cat_scores,
            recommended_action,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signals;

    #[test]
    fn clean_process() {
        let engine = HeuristicEngine::new();
        let assessment = engine.assess(1, "init", vec![]);
        assert_eq!(assessment.level, ThreatLevel::Clean);
        assert_eq!(assessment.recommended_action, RecommendedAction::None);
        assert!((assessment.score - 0.0).abs() < 0.01);
        assert!((assessment.confidence - 0.0).abs() < 0.01);
    }

    #[test]
    fn low_threat_unsigned() {
        let engine = HeuristicEngine::new();
        let signals = vec![signals::unsigned_binary()]; // weight 10
        let assessment = engine.assess(100, "some_app", signals);
        assert_eq!(assessment.level, ThreatLevel::Clean); // 10×1.0 = 10 → Clean
        assert_eq!(assessment.score, 10.0);
    }

    #[test]
    fn medium_threat() {
        let engine = HeuristicEngine::new();
        let signals = vec![
            signals::suspicious_path("/tmp/evil", "Running from /tmp/"),
            signals::unsigned_binary(),
            signals::cpu_spike(95.0),
        ];
        let assessment = engine.assess(42, "evil_proc", signals);
        // With default weights: 20×1.0 + 10×1.0 + 10×1.0 = 40
        assert_eq!(assessment.level, ThreatLevel::Medium);
        assert_eq!(assessment.recommended_action, RecommendedAction::Alert);
    }

    #[test]
    fn high_threat() {
        let engine = HeuristicEngine::new();
        let signals = vec![
            signals::high_entropy(7.8),                  // 30 × 1.0 = 30
            signals::suspicious_port(4444, "Metasploit"), // 25 × 1.2 = 30
        ];
        // 30 + 30 = 60
        let assessment = engine.assess(666, "payload", signals);
        assert_eq!(assessment.level, ThreatLevel::High);
        assert_eq!(assessment.recommended_action, RecommendedAction::Suspend);
    }

    #[test]
    fn critical_threat() {
        let engine = HeuristicEngine::new();
        let signals = vec![
            signals::high_entropy(7.9),                  // 30
            signals::suspicious_port(4444, "Metasploit"), // 25
            signals::wx_violation(2),                    // 25
            signals::fileless_process(),                 // 25
        ];
        let assessment = engine.assess(999, "rootkit", signals);
        assert_eq!(assessment.level, ThreatLevel::Critical);
        assert_eq!(
            assessment.recommended_action,
            RecommendedAction::SuspendDumpKill
        );
        assert!(assessment.score <= 100.0);
    }

    #[test]
    fn category_scores_are_hashmap() {
        let engine = HeuristicEngine::new();
        let signals = vec![
            signals::unsigned_binary(),
            signals::suspicious_port(4444, "MSF"),
        ];
        let assessment = engine.assess(1, "test", signals);
        assert!(assessment.category_scores.contains_key(&SignalCategory::StaticAnalysis));
        assert!(assessment.category_scores.contains_key(&SignalCategory::NetworkBehavior));
    }

    #[test]
    fn threat_level_ordering() {
        assert!(ThreatLevel::Clean < ThreatLevel::Low);
        assert!(ThreatLevel::Low < ThreatLevel::Medium);
        assert!(ThreatLevel::Medium < ThreatLevel::High);
        assert!(ThreatLevel::High < ThreatLevel::Critical);
    }

    // ── STEP 4: dedup tests ──────────────────────────────────────────────

    #[test]
    fn duplicate_signals_deduped() {
        let engine = HeuristicEngine::new();
        let signals = vec![
            signals::unsigned_binary(), // 10
            signals::unsigned_binary(), // 10 again — should be deduped
        ];
        let assessment = engine.assess(1, "test", signals);
        assert_eq!(assessment.signals.len(), 1);
        assert!(assessment.score <= 10.0 * 1.0 + 0.01); // Only counted once
    }

    #[test]
    fn dedup_keeps_highest_weight() {
        let engine = HeuristicEngine::new();
        let signals = vec![
            Signal::new("HIGH_ENTROPY", SignalCategory::StaticAnalysis, 15.0, "low"),
            Signal::new("HIGH_ENTROPY", SignalCategory::StaticAnalysis, 30.0, "high"),
        ];
        let assessment = engine.assess(1, "test", signals);
        assert_eq!(assessment.signals.len(), 1);
        assert!(assessment.signals[0].weight >= 30.0 - 0.01);
    }

    // ── STEP 5: rounding ─────────────────────────────────────────────────

    #[test]
    fn score_10_9_rounds_to_low() {
        let level = ThreatLevel::from_score(10.9);
        assert_eq!(level, ThreatLevel::Low); // rounds to 11 → Low
    }

    #[test]
    fn score_10_4_stays_clean() {
        let level = ThreatLevel::from_score(10.4);
        assert_eq!(level, ThreatLevel::Clean); // rounds to 10 → Clean
    }

    // ── STEP 6: confidence ───────────────────────────────────────────────

    #[test]
    fn single_signal_low_confidence() {
        let engine = HeuristicEngine::new();
        let assessment = engine.assess(1, "test", vec![signals::unsigned_binary()]);
        assert!(assessment.confidence < 0.5);
    }

    #[test]
    fn many_diverse_signals_high_confidence() {
        let engine = HeuristicEngine::new();
        let signals = vec![
            signals::unsigned_binary(),
            signals::suspicious_path("/tmp/x", "tmp"),
            signals::suspicious_port(4444, "MSF"),
            signals::wx_violation(1),
            signals::cpu_spike(99.0),
        ];
        let assessment = engine.assess(1, "test", signals);
        assert!(assessment.confidence > 0.7);
    }

    // ── STEP 2: configurable weights ─────────────────────────────────────

    #[test]
    fn custom_weights_affect_scoring() {
        let config = HeuristicConfig {
            max_score: 100.0,
            category_weights: CategoryWeights {
                memory_anomaly: 3.0, // 3× multiplier
                ..Default::default()
            },
        };
        let engine = HeuristicEngine::with_config(config);
        let signals = vec![signals::wx_violation(1)]; // weight 25, category MemoryAnomaly
        let assessment = engine.assess(1, "test", signals);
        // 25 × 3.0 = 75
        assert!((assessment.score - 75.0).abs() < 0.01);
    }
}
