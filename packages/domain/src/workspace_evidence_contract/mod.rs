//! Programme IV shared observational evidence contract helpers (Batch 10).
//!
//! This module does **not** own any evidence engine, snapshot, or persistence.
//! Engine modules remain the authority for their DTOs and classification logic.
//!
//! Owns only:
//! - shared authority/actionability vocabulary
//! - deterministic digest helper
//! - baseline forbidden-phrase checks for observational narratives
//!
//! Never owns: retrieval, classification semantics, lifecycle, execution, permissions.

/// Observational artefacts never carry authority.
pub const AUTHORITY_EFFECT_NONE: &str = "none";

/// Terminal history statuses shared across Programme IV evidence engines.
pub fn is_terminal_history_status(status: &str) -> bool {
    matches!(status, "superseded" | "archived")
}

/// Shared non-actionable check for observational authority fields.
pub fn is_non_actionable_authority(authority_effect: &str, actionable: bool) -> bool {
    !actionable && authority_effect == AUTHORITY_EFFECT_NONE
}

/// Deterministic FNV-1a style digest used by Programme IV evidence IDs.
pub fn stable_digest(input: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}

/// Baseline phrases forbidden in observational evidence narratives.
/// Engines may add engine-specific extras via [`reject_forbidden_phrases_with`].
pub const BASELINE_FORBIDDEN_PHRASES: &[&str] = &[
    "recommend",
    "should do",
    "best choice",
    "optimal",
    "execute now",
    "approve",
    "dispatch",
    "automate",
    "is correct",
    "is true",
    "confident that",
    "therefore decide",
];

/// Returns `Err(phrase)` when `text` contains a forbidden phrase (case-insensitive).
pub fn reject_forbidden_phrases_with<'a>(
    text: &str,
    extra: &[&'a str],
) -> Result<(), &'a str> {
    let lower = text.to_ascii_lowercase();
    for phrase in BASELINE_FORBIDDEN_PHRASES.iter().chain(extra.iter()) {
        if lower.contains(phrase) {
            return Err(*phrase);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_is_deterministic() {
        assert_eq!(stable_digest("a|b"), stable_digest("a|b"));
        assert_ne!(stable_digest("a|b"), stable_digest("a|c"));
    }

    #[test]
    fn authority_helpers() {
        assert!(is_non_actionable_authority(AUTHORITY_EFFECT_NONE, false));
        assert!(!is_non_actionable_authority(AUTHORITY_EFFECT_NONE, true));
        assert!(is_terminal_history_status("superseded"));
        assert!(!is_terminal_history_status("current"));
    }

    #[test]
    fn baseline_forbidden_phrases_reject_recommendations() {
        assert!(reject_forbidden_phrases_with("please recommend next", &[]).is_err());
        assert!(reject_forbidden_phrases_with("observational only", &[]).is_ok());
        assert!(reject_forbidden_phrases_with("do not determine truth", &["determine truth"]).is_err());
    }
}
