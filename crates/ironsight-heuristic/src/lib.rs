//! # IronSight Heuristic
//!
//! Threat scoring engine — aggregates signals from security, network,
//! and memory analysis into a normalized threat score with action recommendations.
//! Includes time-decay for reducing false positives from transient spikes.

pub mod decay;
pub mod engine;
pub mod signals;

pub use decay::{DecayConfig, DecayEngine, DecayedScore};
pub use engine::{
    CategoryScores, CategoryWeights, HeuristicConfig, HeuristicEngine,
    RecommendedAction, ThreatAssessment, ThreatLevel,
};
pub use signals::{Signal, SignalCategory};

