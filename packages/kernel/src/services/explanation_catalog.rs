//! Canonical Experience explanation catalog (Sprint 131–132).
//!
//! Single source of truth: `packages/kernel/resources/explanation-catalog.json`.
//! Resolution order is catalog-owned — see `resolution.order` in JSON.

use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Deserialize, PartialEq)]
struct CatalogEntry {
    title: String,
    description: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PrefixPattern {
    starts_with: String,
    title: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct PrefixRule {
    prefix: String,
    #[serde(default)]
    suffixes: HashMap<String, CatalogEntry>,
    #[serde(default)]
    patterns: Vec<PrefixPattern>,
    fallback: Option<CatalogEntry>,
}

#[derive(Debug, Deserialize)]
struct UnknownResolution {
    description_template: String,
    signal_title_fallback_template: String,
}

#[derive(Debug, Deserialize)]
struct ResolutionContract {
    order: Vec<String>,
    unknown: UnknownResolution,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct ContractFixture {
    pub key: String,
    pub signal: String,
    pub source: String,
    pub weight: i32,
    pub known: bool,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
struct ExplanationCatalog {
    version: u32,
    resolution: ResolutionContract,
    exact: HashMap<String, CatalogEntry>,
    prefix_rules: Vec<PrefixRule>,
    signal_titles: HashMap<String, String>,
    #[serde(default)]
    contract_fixtures: Vec<ContractFixture>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LookupTier {
    Exact,
    PrefixSuffix,
    PrefixPattern,
    PrefixFallback,
}

static CATALOG: OnceLock<ExplanationCatalog> = OnceLock::new();

fn catalog() -> &'static ExplanationCatalog {
    CATALOG.get_or_init(|| {
        serde_json::from_str(include_str!("../../resources/explanation-catalog.json"))
            .expect("explanation-catalog.json must parse")
    })
}

/// Lookup curated title + description for an explanation key.
///
/// Order (catalog-owned): exact → prefix suffix → prefix pattern → prefix fallback.
pub(crate) fn lookup_explanation_key(key: &str) -> Option<(&'static str, &'static str)> {
    lookup_explanation_key_with_tier(key).map(|(title, desc, _)| (title, desc))
}

pub(crate) fn lookup_explanation_key_with_tier(
    key: &str,
) -> Option<(&'static str, &'static str, LookupTier)> {
    let cat = catalog();
    if let Some(entry) = cat.exact.get(key) {
        return Some((
            leak_str(&entry.title),
            leak_str(&entry.description),
            LookupTier::Exact,
        ));
    }
    for rule in &cat.prefix_rules {
        let Some(rest) = key.strip_prefix(rule.prefix.as_str()) else {
            continue;
        };
        if let Some(entry) = rule.suffixes.get(rest) {
            return Some((
                leak_str(&entry.title),
                leak_str(&entry.description),
                LookupTier::PrefixSuffix,
            ));
        }
        for pattern in &rule.patterns {
            if rest.starts_with(pattern.starts_with.as_str()) {
                return Some((
                    leak_str(&pattern.title),
                    leak_str(&pattern.description),
                    LookupTier::PrefixPattern,
                ));
            }
        }
        if let Some(entry) = &rule.fallback {
            return Some((
                leak_str(&entry.title),
                leak_str(&entry.description),
                LookupTier::PrefixFallback,
            ));
        }
    }
    None
}

pub(crate) fn unknown_description(
    explanation_key: &str,
    signal: &str,
    source: &str,
    weight: i32,
) -> String {
    catalog()
        .resolution
        .unknown
        .description_template
        .replace("{explanation_key}", explanation_key)
        .replace("{signal}", signal)
        .replace("{source}", source)
        .replace("{weight}", &weight.to_string())
}

/// Signal-based fallback title when the key is unknown.
pub(crate) fn fallback_title_for_signal(signal: &str) -> String {
    catalog()
        .signal_titles
        .get(signal)
        .cloned()
        .unwrap_or_else(|| {
            catalog()
                .resolution
                .unknown
                .signal_title_fallback_template
                .replace("{signal}", signal)
        })
}

pub(crate) fn catalog_version() -> u32 {
    catalog().version
}

pub(crate) fn exact_key_count() -> usize {
    catalog().exact.len()
}

pub(crate) fn contract_fixtures() -> &'static [ContractFixture] {
    &catalog().contract_fixtures
}

pub(crate) fn resolution_order() -> &'static [String] {
    &catalog().resolution.order
}

/// Leak catalog strings to `'static` — catalog is process-lifetime immutable.
fn leak_str(value: &str) -> &'static str {
    Box::leak(value.to_owned().into_boxed_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn parse_generated_catalog_ts(source: &str) -> ExplanationCatalog {
        let marker = "export default ";
        let start = source
            .find(marker)
            .expect("generated catalog must contain export default");
        let json_start = start + marker.len();
        let json_end = source
            .rfind(" as const")
            .expect("generated catalog must end with as const");
        serde_json::from_str(&source[json_start..json_end])
            .expect("generated catalog JSON must parse")
    }

    #[test]
    fn catalog_loads_and_has_expected_version() {
        assert_eq!(catalog_version(), 2);
        assert!(exact_key_count() >= 16);
        assert_eq!(
            resolution_order(),
            &[
                "exact".to_string(),
                "prefix_suffix".to_string(),
                "prefix_pattern".to_string(),
                "prefix_fallback".to_string(),
                "unknown".to_string(),
            ]
        );
    }

    #[test]
    fn generated_ts_catalog_matches_json_source() {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        let json: ExplanationCatalog = serde_json::from_str(include_str!(
            "../../resources/explanation-catalog.json"
        ))
        .unwrap();
        let ts_path = manifest.join("../../app/src/generated/explanationCatalog.ts");
        let ts = fs::read_to_string(ts_path).expect("generated catalog ts must exist");
        let from_ts = parse_generated_catalog_ts(&ts);
        assert_eq!(json.version, from_ts.version);
        assert_eq!(json.exact, from_ts.exact);
        assert_eq!(json.prefix_rules.len(), from_ts.prefix_rules.len());
        assert_eq!(json.contract_fixtures, from_ts.contract_fixtures);
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
    fn prefix_resolution_follows_explicit_order() {
        let (_, _, tier) = lookup_explanation_key_with_tier("task.base.blocked").unwrap();
        assert_eq!(tier, LookupTier::PrefixSuffix);

        let (_, _, tier) =
            lookup_explanation_key_with_tier("purpose.obstacle.composition:gap").unwrap();
        assert_eq!(tier, LookupTier::PrefixPattern);

        let (_, _, tier) = lookup_explanation_key_with_tier("task.base.unlisted_status").unwrap();
        assert_eq!(tier, LookupTier::PrefixFallback);

        let (_, _, tier) =
            lookup_explanation_key_with_tier("purpose.obstacle.unlisted_kind").unwrap();
        assert_eq!(tier, LookupTier::PrefixFallback);
    }

    #[test]
    fn pattern_match_beats_generic_prefix_fallback() {
        let (title, _, tier) =
            lookup_explanation_key_with_tier("purpose.obstacle.composition:missing_application")
                .unwrap();
        assert_eq!(tier, LookupTier::PrefixPattern);
        assert_eq!(title, "Purpose blocked by composition");
    }

    #[test]
    fn unknown_key_returns_none_from_catalog_lookup() {
        assert!(lookup_explanation_key("future.signal.unknown").is_none());
    }

    #[test]
    fn contract_fixtures_resolve_through_catalog() {
        for fixture in contract_fixtures() {
            let resolved = lookup_explanation_key(&fixture.key);
            if fixture.known {
                let (title, description) = resolved.unwrap_or_else(|| {
                    panic!("fixture {} should resolve via catalog", fixture.key)
                });
                assert_eq!(title, fixture.title, "fixture {}", fixture.key);
                assert_eq!(description, fixture.description, "fixture {}", fixture.key);
            } else {
                assert!(resolved.is_none(), "fixture {}", fixture.key);
                assert_eq!(
                    fallback_title_for_signal(&fixture.signal),
                    fixture.title,
                    "fixture {}",
                    fixture.key
                );
                assert_eq!(
                    unknown_description(
                        &fixture.key,
                        &fixture.signal,
                        &fixture.source,
                        fixture.weight
                    ),
                    fixture.description,
                    "fixture {}",
                    fixture.key
                );
            }
        }
    }
}
