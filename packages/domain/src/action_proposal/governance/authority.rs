//! Shared governance authority markers (Sprint 168 simplification).
//!
//! Governance never grants execution authority. Types may still expose their own
//! `AUTHORITY_EFFECT_NONE` associated constants for API stability; those values
//! must equal [`GOVERNANCE_AUTHORITY_EFFECT_NONE`].

/// Canonical authority effect for all governance artifacts.
pub const GOVERNANCE_AUTHORITY_EFFECT_NONE: &str = "none";

/// True when a governance `authority_effect` field is the safe none marker.
pub fn governance_authority_is_none(authority_effect: &str) -> bool {
    authority_effect == GOVERNANCE_AUTHORITY_EFFECT_NONE
}
