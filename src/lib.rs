//! # AICENT-TRAFFIC: Sentinel Alpha 2.0
// Domain: http://Aicent.com
//! EVOLUTION: Phase 1 - Hive-Rise Integration
//! Adding Geographic Pathogen Fingerprinting and Entropy Shielding.

use serde::{Deserialize, Serialize};

/// Detailed profile for identifying non-sovereign scanning patterns.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PathogenProfile {
    pub ip_segment: String,        // Target specific hub IP ranges (e.g., Northern Europe)
    pub access_pattern: String,    // Identification of "Repeated /actions/ probing"
    pub audit_intensity: u32,      // Frequency of cloning events in a 24h cycle
    pub is_known_watchman: bool,  // Flagged as one of the "15 Watchers"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SentinelPulse {
    pub node_id: String,
    pub timestamp_ns: u64,
    pub pulse_entropy: f64,
    pub rttp_header_hash: String,
    pub is_sovereign: bool,
    pub geographic_origin: String,
    pub profile: Option<PathogenProfile>, // Enhanced telemetry metadata
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum SentinelStatus {
    Homeostasis,
    Anomaly,
    PathogenDetected,
    ActiveNeutralization, // New status for Phase 1
}

pub trait Sentinel {
    fn observe(&self, pulse: &SentinelPulse) -> f64;
    fn get_node_status(&self, pulse: &SentinelPulse) -> SentinelStatus;
    /// Triggers the RPKI isolation protocol (RFC-003)
    fn trigger_quarantine(&self, node_id: &str);
}

pub struct SentinelConfig {
    pub entropy_threshold: f64,
    pub sync_interval: u32,
    pub active_surveillance: bool,
    pub sentinel_id: String,
}

impl Default for SentinelConfig {
    fn default() -> Self {
        Self {
            entropy_threshold: 0.85,
            sync_interval: 10,
            active_surveillance: true,
            sentinel_id: "Sentinel-Alpha-v1.1".to_string(),
        }
    }
}
