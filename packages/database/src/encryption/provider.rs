//! Tiered encryption strategy placeholder (DEC-015).
//!
//! Tier 0: OS file protection — default for Sprint 01
//! Tier 1: Sensitive field encryption — future
//! Tier 2: Full database encryption — future

/// Encryption tier per DEC-015.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionTier {
    /// Normal local storage with OS-level file protection.
    Tier0,
    /// Sensitive data encryption abstraction (patterns, automations).
    Tier1,
    /// Full database encryption.
    Tier2,
}

/// Future encryption boundary for Workspace stored data.
pub trait EncryptionProvider: Send + Sync {
    fn tier(&self) -> EncryptionTier;

    fn description(&self) -> &'static str;
}

/// Sprint 01 default — Tier 0 only. No encryption logic applied.
pub struct NoOpEncryptionProvider;

impl EncryptionProvider for NoOpEncryptionProvider {
    fn tier(&self) -> EncryptionTier {
        EncryptionTier::Tier0
    }

    fn description(&self) -> &'static str {
        "Tier 0 — OS file protection (no application-level encryption)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop_provider_is_tier_zero() {
        let provider = NoOpEncryptionProvider;
        assert_eq!(provider.tier(), EncryptionTier::Tier0);
    }
}
