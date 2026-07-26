//! Canonical Experience explanation catalog (Sprint 131).
//!
//! Single source of truth: `packages/kernel/resources/explanation-catalog.json`.
//! Rust resolver and the UI-generated catalog consume this file — do not duplicate
//! wording in resolver code.

use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
struct CatalogEntry {
    title: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct PrefixRule {
    prefix: String,
    suffixes: HashMap<String, CatalogEntry>,
}

#[derive(Debug, Deserialize)]
struct ExplanationCatalog {
    version: u32,
    exact: HashMap<String, CatalogEntry>,
    prefix_rules: Vec<PrefixRule>,
    signal_titles: HashMap<String, String>,
}

static CATALOG: OnceLock<ExplanationCatalog> = OnceLock::new();

fn catalog() -> &'static ExplanationCatalog {
    CATALOG.get_or_init(|| {
        serde_json::from_str(include_str!("../../resources/explanation-catalog.json"))
            .expect("explanation-catalog.json must parse")
    })
}

/// Lookup curated title + description for an explanation key.
pub(crate) fn lookup_explanation_key(key: &str) -> Option<(&'static str, &'static str)> {
    let cat = catalog();
    if let Some(entry) = cat.exact.get(key) {
        return Some((leak_str(&entry.title), leak_str(&entry.description)));
    }
    for rule in &cat.prefix_rules {
        if let Some(rest) = key.strip_prefix(rule.prefix.as_str()) {
            if let Some(entry) = rule.suffixes.get(rest) {
                return Some((leak_str(&entry.title), leak_str(&entry.description)));
            }
            // composition:* style wildcard inside suffix map
            for (suffix_key, entry) in &rule.suffixes {
                if let Some(wildcard) = suffix_key.strip_suffix(":*") {
                    if rest.starts_with(wildcard) {
                        return Some((leak_str(&entry.title), leak_str(&entry.description)));
                    }
                }
            }
            if let Some(entry) = rule.suffixes.get("*") {
                return Some((leak_str(&entry.title), leak_str(&entry.description)));
            }
        }
    }
    None
}

/// Signal-based fallback title when the key is unknown.
pub(crate) fn fallback_title_for_signal(signal: &str) -> String {
    catalog()
        .signal_titles
        .get(signal)
        .cloned()
        .unwrap_or_else(|| format!("Attention signal '{signal}' needs attention"))
}

pub(crate) fn catalog_version() -> u32 {
    catalog().version
}

pub(crate) fn exact_key_count() -> usize {
    catalog().exact.len()
}

/// Leak catalog strings to `'static` — catalog is process-lifetime immutable.
fn leak_str(value: &str) -> &'static str {
    Box::leak(value.to_owned().into_boxed_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_loads_and_has_expected_version() {
        assert_eq!(catalog_version(), 1);
        assert!(exact_key_count() >= 16);
    }

    #[test]
    fn every_exact_catalog_key_resolves() {
        let cat = catalog();
        for key in cat.exact.keys() {
            let resolved = lookup_explanation_key(key);
            assert!(resolved.is_some(), "exact key {key} must resolve");
        }
    }

    #[test]
    fn prefix_rules_resolve_known_samples() {
        assert!(lookup_explanation_key("task.base.blocked").is_some());
        assert!(lookup_explanation_key("task.priority.high").is_some());
        assert!(lookup_explanation_key("purpose.obstacle.blocked_task").is_some());
        assert!(lookup_explanation_key("purpose.obstacle.composition:gap").is_some());
        assert!(lookup_explanation_key("composition.gap.missing_application").is_some());
        assert!(lookup_explanation_key("environment.gap.disconnected_work").is_some());
    }

    #[test]
    fn unknown_key_returns_none_from_catalog_lookup() {
        assert!(lookup_explanation_key("future.signal.unknown").is_none());
    }
}
