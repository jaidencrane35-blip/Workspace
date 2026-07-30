//! Generic desktop window grouping — fact-driven, criterion-based.
//!
//! Groups windows from observation facts only. Criteria are data (process,
//! monitor, arrangement membership, matched application) — never product
//! taxonomies (project/task/browser/media). Read-only; no OS control.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Extensible grouping criterion. Add variants without rewriting consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesktopGroupCriterion {
    ProcessId,
    MonitorIndex,
    ArrangementMembership,
    MatchedApplication,
}

impl DesktopGroupCriterion {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProcessId => "process_id",
            Self::MonitorIndex => "monitor_index",
            Self::ArrangementMembership => "arrangement_membership",
            Self::MatchedApplication => "matched_application",
        }
    }
}

/// Flat membership fact fed into the grouping engine (no UI/card semantics).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopGroupMemberFact {
    pub member_id: String,
    pub process_id: i32,
    pub process_name: Option<String>,
    pub monitor_index: Option<i32>,
    pub arrangement_ids: Vec<String>,
    pub matched_application_id: Option<String>,
    pub matched_application_name: Option<String>,
}

/// One projected group — reusable by Stage, Environment, Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopWindowGroup {
    pub id: String,
    pub criterion: String,
    pub fact_key: String,
    pub label: String,
    pub member_ids: Vec<String>,
    /// Structural observation count supporting this group (behaviour may raise it).
    pub evidence_count: i32,
    /// `structural` | `emerging` | `recurring` | `strong`
    pub confidence: String,
    pub authority_effect: String,
}

impl DesktopWindowGroup {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const CONFIDENCE_STRUCTURAL: &'static str = "structural";
    pub const CONFIDENCE_EMERGING: &'static str = "emerging";
    pub const CONFIDENCE_RECURRING: &'static str = "recurring";
    pub const CONFIDENCE_STRONG: &'static str = "strong";

    pub fn confidence_for_evidence(evidence_count: i32) -> &'static str {
        match evidence_count {
            0..=1 => Self::CONFIDENCE_STRUCTURAL,
            2..=3 => Self::CONFIDENCE_EMERGING,
            4..=6 => Self::CONFIDENCE_RECURRING,
            _ => Self::CONFIDENCE_STRONG,
        }
    }
}

/// Group members by the given criteria. Deterministic id/order. Empty criteria → empty.
///
/// Process / matched-application / arrangement groups emit when ≥1 member.
/// Monitor groups emit only when ≥2 members (co-location signal).
pub fn group_desktop_members(
    members: &[DesktopGroupMemberFact],
    criteria: &[DesktopGroupCriterion],
) -> Vec<DesktopWindowGroup> {
    let mut groups = Vec::new();
    for criterion in criteria {
        match criterion {
            DesktopGroupCriterion::ProcessId => {
                groups.extend(group_by_process(members));
            }
            DesktopGroupCriterion::MonitorIndex => {
                groups.extend(group_by_monitor(members));
            }
            DesktopGroupCriterion::ArrangementMembership => {
                groups.extend(group_by_arrangement(members));
            }
            DesktopGroupCriterion::MatchedApplication => {
                groups.extend(group_by_matched_application(members));
            }
        }
    }
    groups.sort_by(|a, b| a.id.cmp(&b.id));
    groups
}

fn group_by_process(members: &[DesktopGroupMemberFact]) -> Vec<DesktopWindowGroup> {
    let mut buckets: BTreeMap<i32, Vec<&DesktopGroupMemberFact>> = BTreeMap::new();
    for member in members {
        buckets.entry(member.process_id).or_default().push(member);
    }
    buckets
        .into_iter()
        .filter(|(_, rows)| rows.len() >= 2)
        .map(|(pid, rows)| {
            let label = rows
                .iter()
                .find_map(|row| row.process_name.as_ref())
                .cloned()
                .unwrap_or_else(|| format!("pid:{pid}"));
            DesktopWindowGroup {
                id: format!("group:process_id:{pid}"),
                criterion: DesktopGroupCriterion::ProcessId.as_str().into(),
                fact_key: pid.to_string(),
                label,
                member_ids: rows.iter().map(|row| row.member_id.clone()).collect(),
                evidence_count: 1,
                confidence: DesktopWindowGroup::CONFIDENCE_STRUCTURAL.into(),
                authority_effect: DesktopWindowGroup::AUTHORITY_EFFECT_NONE.into(),
            }
        })
        .collect()
}

fn group_by_monitor(members: &[DesktopGroupMemberFact]) -> Vec<DesktopWindowGroup> {
    let mut buckets: BTreeMap<i32, Vec<&DesktopGroupMemberFact>> = BTreeMap::new();
    for member in members {
        if let Some(index) = member.monitor_index {
            buckets.entry(index).or_default().push(member);
        }
    }
    buckets
        .into_iter()
        .filter(|(_, rows)| rows.len() >= 2)
        .map(|(index, rows)| DesktopWindowGroup {
            id: format!("group:monitor_index:{index}"),
            criterion: DesktopGroupCriterion::MonitorIndex.as_str().into(),
            fact_key: index.to_string(),
            label: format!("monitor:{index}"),
            member_ids: rows.iter().map(|row| row.member_id.clone()).collect(),
            evidence_count: 1,
            confidence: DesktopWindowGroup::CONFIDENCE_STRUCTURAL.into(),
            authority_effect: DesktopWindowGroup::AUTHORITY_EFFECT_NONE.into(),
        })
        .collect()
}

fn group_by_arrangement(members: &[DesktopGroupMemberFact]) -> Vec<DesktopWindowGroup> {
    let mut buckets: BTreeMap<String, Vec<&DesktopGroupMemberFact>> = BTreeMap::new();
    for member in members {
        for arrangement_id in &member.arrangement_ids {
            let key = arrangement_id.trim();
            if key.is_empty() {
                continue;
            }
            buckets.entry(key.to_string()).or_default().push(member);
        }
    }
    buckets
        .into_iter()
        .filter(|(_, rows)| !rows.is_empty())
        .map(|(arrangement_id, rows)| DesktopWindowGroup {
            id: format!("group:arrangement_membership:{arrangement_id}"),
            criterion: DesktopGroupCriterion::ArrangementMembership.as_str().into(),
            fact_key: arrangement_id.clone(),
            label: arrangement_id,
            member_ids: rows.iter().map(|row| row.member_id.clone()).collect(),
            evidence_count: 1,
            confidence: DesktopWindowGroup::CONFIDENCE_STRUCTURAL.into(),
            authority_effect: DesktopWindowGroup::AUTHORITY_EFFECT_NONE.into(),
        })
        .collect()
}

fn group_by_matched_application(members: &[DesktopGroupMemberFact]) -> Vec<DesktopWindowGroup> {
    let mut buckets: BTreeMap<String, Vec<&DesktopGroupMemberFact>> = BTreeMap::new();
    for member in members {
        if let Some(app_id) = member
            .matched_application_id
            .as_ref()
            .map(|id| id.trim().to_string())
            .filter(|id| !id.is_empty())
        {
            buckets.entry(app_id).or_default().push(member);
        }
    }
    buckets
        .into_iter()
        .filter(|(_, rows)| !rows.is_empty())
        .map(|(app_id, rows)| {
            let label = rows
                .iter()
                .find_map(|row| row.matched_application_name.as_ref())
                .cloned()
                .unwrap_or_else(|| app_id.clone());
            DesktopWindowGroup {
                id: format!("group:matched_application:{app_id}"),
                criterion: DesktopGroupCriterion::MatchedApplication.as_str().into(),
                fact_key: app_id,
                label,
                member_ids: rows.iter().map(|row| row.member_id.clone()).collect(),
                evidence_count: 1,
                confidence: DesktopWindowGroup::CONFIDENCE_STRUCTURAL.into(),
                authority_effect: DesktopWindowGroup::AUTHORITY_EFFECT_NONE.into(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn member(
        id: &str,
        process_id: i32,
        process_name: Option<&str>,
        monitor_index: Option<i32>,
    ) -> DesktopGroupMemberFact {
        DesktopGroupMemberFact {
            member_id: id.into(),
            process_id,
            process_name: process_name.map(str::to_string),
            monitor_index,
            arrangement_ids: Vec::new(),
            matched_application_id: None,
            matched_application_name: None,
        }
    }

    #[test]
    fn groups_by_process_and_monitor_without_taxonomies() {
        let members = vec![
            member("a", 10, Some("code.exe"), Some(0)),
            member("b", 10, Some("code.exe"), Some(1)),
            member("c", 20, Some("chrome.exe"), Some(0)),
            member("d", 30, Some("solo.exe"), Some(2)),
        ];
        let groups = group_desktop_members(
            &members,
            &[
                DesktopGroupCriterion::ProcessId,
                DesktopGroupCriterion::MonitorIndex,
            ],
        );
        let process = groups
            .iter()
            .find(|group| group.id == "group:process_id:10")
            .expect("process group");
        assert_eq!(process.member_ids, vec!["a", "b"]);
        assert_eq!(process.label, "code.exe");
        assert!(!groups.iter().any(|group| group.id == "group:process_id:30"));

        let monitor = groups
            .iter()
            .find(|group| group.id == "group:monitor_index:0")
            .expect("monitor group");
        assert_eq!(monitor.member_ids, vec!["a", "c"]);
    }

    #[test]
    fn groups_by_arrangement_membership_fact() {
        let mut a = member("a", 1, None, None);
        a.arrangement_ids = vec!["arr-focus".into()];
        let mut b = member("b", 2, None, None);
        b.arrangement_ids = vec!["arr-focus".into()];
        let groups = group_desktop_members(
            &[a, b],
            &[DesktopGroupCriterion::ArrangementMembership],
        );
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].fact_key, "arr-focus");
        assert_eq!(groups[0].member_ids, vec!["a", "b"]);
    }

    #[test]
    fn matched_application_criterion_is_generic() {
        let mut a = member("a", 1, None, None);
        a.matched_application_id = Some("app-1".into());
        a.matched_application_name = Some("Editor".into());
        let mut b = member("b", 1, None, None);
        b.matched_application_id = Some("app-1".into());
        let groups =
            group_desktop_members(&[a, b], &[DesktopGroupCriterion::MatchedApplication]);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].label, "Editor");
        assert_eq!(groups[0].criterion, "matched_application");
    }
}
