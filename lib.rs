//! # AICENT-TRAFFIC: The Sentinel Core
//! 
//! This crate defines the telemetry and global grid intelligence layer for the Aicent Stack.
//! As the "Sentinel" (The Eye) of the sovereign AI organism, it is responsible for 
//! monitoring RTTP (RFC-002) neural pulses, performing pathogen tracking, and triggering 
//! RPKI (RFC-003) immune reflexes across the Aicent.net (RFC-006) operational grid.
//! 
//! "Visibility is the first layer of sovereignty; detection is the precursor to reflex."

use serde::{Deserialize, Serialize};

/// The SentinelPulse represents the atomic unit of telemetry data.
/// It encapsulates the metadata of an RTTP Pulse Frame for real-time analysis.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SentinelPulse {
    /// The unique AID (AI Identity) fingerprint of the originating node.
    pub node_id: String,
    
    /// Precision timestamp in nanoseconds for sub-millisecond drift analysis.
    pub timestamp_ns: u64,
    
    /// Statistical entropy of the pulse payload. High entropy often indicates 
    /// Man-in-the-Middle (MITM) interference or tampered tensor watermarks.
    pub pulse_entropy: f64,
    
    /// The cryptographic hash of the 64-byte RTTP header.
    pub rttp_header_hash: String,
    
    /// Sovereign status verified via RPKI (RFC-003) parallel attestation.
    pub is_sovereign: bool,
    
    /// Geographic metadata identifying the internet hub or physical grid origin.
    pub geographic_origin: String,
}

/// System status indicators for the Global Operational Grid.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum SentinelStatus {
    /// System is in a stable, synchronized state.
    Homeostasis,
    /// Minor jitter or out-of-order pulses detected.
    Anomaly,
    /// Verified malicious activity; RPKI quarantine required.
    PathogenDetected,
}

/// The Sentinel trait defines the interface for grid observation and autonomic reflex.
pub trait Sentinel {
    /// Observes a telemetry pulse and computes a situational awareness score (0.0 - 1.0).
    /// A score below 0.5 typically indicates a threat to homeostasis.
    fn observe(&self, pulse: &SentinelPulse) -> f64;
    
    /// Analyzes a pulse to determine the current state of a specific AID node.
    fn get_node_status(&self, pulse: &SentinelPulse) -> SentinelStatus;
    
    /// Triggers the RPKI isolation protocol (RFC-003) for a verified pathogen.
    /// Executes the QUARANTINE_PULSE in < 100µs via the Aicent.net backbone.
    fn trigger_quarantine(&self, node_id: &str);
}

/// Configuration for the Sentinel Telemetry Engine.
#[derive(Debug, Clone)]
pub struct SentinelConfig {
    /// The threshold for entropy-based anomaly detection.
    pub entropy_threshold: f64,
    /// Frequency of grid-wide resonance synchronization (in RTTP pulse cycles).
    pub sync_interval: u32,
    /// Enable real-time logging of high-volume cloning and access events.
    pub active_surveillance: bool,
}

impl Default for SentinelConfig {
    fn default() -> Self {
        Self {
            entropy_threshold: 0.85,
            sync_interval: 10,
            active_surveillance: true,
        }
    }
}
