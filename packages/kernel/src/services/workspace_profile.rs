//! Workspace Environment Profiles (Phase 6).
//!
//! Durable user-owned descriptions of preferred workspace setups.
//! Comparison is informational only — never launches, restores, or executes.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use serde_json::json;
use uuid::Uuid;
use workspace_database::{Database, WorkspaceProfileRepository};
use workspace_domain::{
    build_profile_summary, profile_now_rfc3339, validate_profile_workspace_id, ActorContext,
    IntentContext, ProfileSummaryLines, WorkspaceIntelligenceState, WorkspaceProfile,
    WorkspaceProfileAlignment, WorkspaceProfileComparison, WorkspaceProfileDifference,
    WorkspaceProfileEvidence, WorkspaceProfileId, WorkspaceProfileMember,
    WorkspaceProfileMemberInput, WorkspaceProfileMemberType, WorkspaceProfileState,
    WorkspaceProfileStateComparison, WorkspaceProfileStatus, WorkspaceProfileSummary,
    WorkspaceProfileValidation,
};

use crate::error::{KernelError, Result};
use crate::services::AuditService;

pub(crate) struct WorkspaceProfileService;

impl WorkspaceProfileService {
    pub(crate) fn create(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        members: Vec<WorkspaceProfileMemberInput>,
    ) -> Result<WorkspaceProfile> {
        let workspace_id =
            validate_profile_workspace_id(workspace_id).map_err(KernelError::from)?;
        let name = name.into().trim().to_string();
        if name.is_empty() {
            return Err(KernelError::from(
                workspace_domain::WorkspaceProfileError::Invalid("name required".into()),
            ));
        }
        let now = profile_now_rfc3339();
        let profile_id = WorkspaceProfileId::generate();
        let profile_members = build_members(profile_id.as_str(), &members)?;
        let profile = WorkspaceProfile {
            id: profile_id,
            workspace_id,
            name,
            description: description.into(),
            status: WorkspaceProfileStatus::Active,
            members: profile_members,
            created_at: now.clone(),
            updated_at: now,
            authority_effect: WorkspaceProfile::AUTHORITY_EFFECT_NONE.into(),
        };
        {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            let repo = WorkspaceProfileRepository::new(&guard);
            repo.upsert_profile(&profile)?;
            repo.replace_members(&profile.id, &profile.members, &profile.updated_at)?;
        }
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.profile.created",
            true,
            json!({
                "workspace_id": profile.workspace_id.as_str(),
                "profile_id": profile.id.as_str(),
                "name": profile.name,
                "member_count": profile.members.len(),
                "authority_effect": "none",
            })
            .to_string(),
        )?;
        Ok(profile)
    }

    pub(crate) fn update(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        profile_id: impl Into<String>,
        name: Option<String>,
        description: Option<String>,
        status: Option<WorkspaceProfileStatus>,
        members: Option<Vec<WorkspaceProfileMemberInput>>,
    ) -> Result<WorkspaceProfile> {
        let profile_id = WorkspaceProfileId::new(profile_id.into()).map_err(KernelError::Domain)?;
        let mut profile = {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            WorkspaceProfileRepository::new(&guard)
                .get(&profile_id)?
                .ok_or_else(|| {
                    KernelError::from(workspace_domain::WorkspaceProfileError::NotFound(
                        profile_id.to_string(),
                    ))
                })?
        };
        if let Some(name) = name {
            let name = name.trim().to_string();
            if name.is_empty() {
                return Err(KernelError::from(
                    workspace_domain::WorkspaceProfileError::Invalid("name required".into()),
                ));
            }
            profile.name = name;
        }
        if let Some(description) = description {
            profile.description = description;
        }
        if let Some(status) = status {
            profile.status = status;
        }
        if let Some(members) = members {
            profile.members = build_members(profile.id.as_str(), &members)?;
        }
        profile.updated_at = profile_now_rfc3339();
        profile.authority_effect = WorkspaceProfile::AUTHORITY_EFFECT_NONE.into();
        {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            let repo = WorkspaceProfileRepository::new(&guard);
            repo.upsert_profile(&profile)?;
            repo.replace_members(&profile.id, &profile.members, &profile.updated_at)?;
        }
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.profile.updated",
            true,
            json!({
                "workspace_id": profile.workspace_id.as_str(),
                "profile_id": profile.id.as_str(),
                "name": profile.name,
                "status": profile.status.as_str(),
                "member_count": profile.members.len(),
                "authority_effect": "none",
            })
            .to_string(),
        )?;
        Ok(profile)
    }

    pub(crate) fn get(
        db: &Arc<Mutex<Database>>,
        profile_id: impl Into<String>,
    ) -> Result<WorkspaceProfile> {
        let profile_id = WorkspaceProfileId::new(profile_id.into()).map_err(KernelError::Domain)?;
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        WorkspaceProfileRepository::new(&guard)
            .get(&profile_id)?
            .ok_or_else(|| {
                KernelError::from(workspace_domain::WorkspaceProfileError::NotFound(
                    profile_id.to_string(),
                ))
            })
    }

    pub(crate) fn list(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
        limit: usize,
    ) -> Result<Vec<WorkspaceProfile>> {
        let workspace_id =
            validate_profile_workspace_id(workspace_id).map_err(KernelError::from)?;
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        Ok(WorkspaceProfileRepository::new(&guard).list_by_workspace(&workspace_id, limit)?)
    }

    /// Generate profile read model + comparisons against current Intelligence (informational).
    pub(crate) fn generate_state(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intelligence: &WorkspaceIntelligenceState,
    ) -> Result<WorkspaceProfileState> {
        let workspace_id = validate_profile_workspace_id(intelligence.workspace_id.clone())
            .map_err(KernelError::from)?;
        let profiles = {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            WorkspaceProfileRepository::new(&guard).list_by_workspace(&workspace_id, 100)?
        };

        let current = CurrentSnapshot::from_intelligence(intelligence);
        let mut comparisons: Vec<WorkspaceProfileComparison> = profiles
            .iter()
            .filter(|p| p.status == WorkspaceProfileStatus::Active)
            .map(|p| compare_profile(p, &current))
            .collect();
        comparisons.sort_by(|a, b| {
            alignment_rank(a.alignment)
                .cmp(&alignment_rank(b.alignment))
                .then_with(|| a.profile_name.cmp(&b.profile_name))
                .then_with(|| a.profile_id.cmp(&b.profile_id))
        });

        let active_count = profiles
            .iter()
            .filter(|p| p.status == WorkspaceProfileStatus::Active)
            .count();
        let best = comparisons.first();
        let best_alignment = best.map(|c| c.alignment);
        let best_profile_name = best.map(|c| c.profile_name.clone());

        let label = intelligence.workspace_name.clone();
        let profile_summary = ProfileSummaryLines {
            headline: if profiles.is_empty() {
                format!("No workspace profiles defined for \"{label}\"")
            } else {
                format!(
                    "{} profile(s) for \"{label}\"",
                    profiles.len()
                )
            },
            alignment_line: best
                .map(|c| {
                    format!(
                        "Best alignment: {} ({})",
                        c.profile_name,
                        c.alignment.as_str()
                    )
                })
                .unwrap_or_else(|| "No active profile to compare".into()),
            missing_line: best
                .map(|c| {
                    if c.missing_count == 0 {
                        "No missing members".into()
                    } else {
                        format!("{} missing member(s)", c.missing_count)
                    }
                })
                .unwrap_or_else(|| "—".into()),
            narrative: "Workspace Profiles describe preferred setups. Comparison is \
                 informational — never launches apps, restores layouts, or grants authority."
                .into(),
        };

        let explanation = format!(
            "Workspace Profiles for \"{label}\" are user-owned durable setups. \
             References Application/Layout/Project/Task/WorkContext/Preference without \
             owning them. Comparison authority_effect=none."
        );
        let mut evidence = vec![
            format!("profiles:count={}", profiles.len()),
            format!("profiles:active={}", active_count),
            format!(
                "current:apps={}",
                current.application_ids.len() + current.application_names.len()
            ),
            format!(
                "current:project={:?}",
                current.project_id.as_ref().map(|s| s.as_str())
            ),
            format!(
                "current:task={:?}",
                current.task_id.as_ref().map(|s| s.as_str())
            ),
        ];
        evidence.sort();
        evidence.dedup();

        let state = WorkspaceProfileState {
            workspace_id: intelligence.workspace_id.clone(),
            workspace_name: intelligence.workspace_name.clone(),
            generated_at: profile_now_rfc3339(),
            label,
            profile_summary,
            profile_count: profiles.len(),
            active_count,
            best_alignment,
            best_profile_name,
            profiles,
            comparisons,
            explanation,
            evidence,
            summary: build_profile_summary(&intelligence.workspace_name, active_count),
            authority_effect: WorkspaceProfileState::AUTHORITY_EFFECT_NONE.into(),
        };

        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.profile.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "profile_count": state.profile_count,
                "authority_effect": "none",
            })
            .to_string(),
        )?;
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.profile.compared",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "comparison_count": state.comparisons.len(),
                "best_alignment": state.best_alignment.map(|a| a.as_str()),
                "authority_effect": "none",
            })
            .to_string(),
        )?;
        Ok(state)
    }

    /// Compare a single profile against current Intelligence snapshot.
    pub(crate) fn compare_one(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intelligence: &WorkspaceIntelligenceState,
        profile_id: impl Into<String>,
    ) -> Result<WorkspaceProfileComparison> {
        let profile = Self::get(db, profile_id)?;
        if profile.workspace_id.as_str() != intelligence.workspace_id {
            return Err(KernelError::from(
                workspace_domain::WorkspaceProfileError::Invalid(
                    "profile workspace mismatch".into(),
                ),
            ));
        }
        let current = CurrentSnapshot::from_intelligence(intelligence);
        let comparison = compare_profile(&profile, &current);
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.profile.compared",
            true,
            json!({
                "workspace_id": comparison.workspace_id,
                "profile_id": comparison.profile_id,
                "alignment": comparison.alignment.as_str(),
                "missing_count": comparison.missing_count,
                "authority_effect": "none",
            })
            .to_string(),
        )?;
        Ok(comparison)
    }

    pub(crate) fn compare_states(
        left: &WorkspaceProfileState,
        right: &WorkspaceProfileState,
    ) -> WorkspaceProfileStateComparison {
        WorkspaceProfileStateComparison::compare(left, right)
    }

    pub(crate) fn validate_state(state: &WorkspaceProfileState) -> WorkspaceProfileValidation {
        WorkspaceProfileValidation::validate_state(state)
    }

    pub(crate) fn validate_and_audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceProfileState,
    ) -> Result<WorkspaceProfileValidation> {
        let report = Self::validate_state(state);
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.profile.compared",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "validated": report.valid,
                "authority_effect": "none",
            })
            .to_string(),
        )?;
        Ok(report)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceProfileState,
        limit: usize,
    ) -> WorkspaceProfileSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceProfileError::CannotExecute,
        ))
    }
}

struct CurrentSnapshot {
    application_ids: HashSet<String>,
    application_names: HashSet<String>,
    layout_ids: HashSet<String>,
    project_id: Option<String>,
    task_id: Option<String>,
    work_context_ids: HashSet<String>,
    preference_ids: HashSet<String>,
    project_name: Option<String>,
    task_title: Option<String>,
}

impl CurrentSnapshot {
    fn from_intelligence(intel: &WorkspaceIntelligenceState) -> Self {
        let mut application_ids = HashSet::new();
        let mut application_names = HashSet::new();
        for app in &intel.current_applications {
            application_ids.insert(app.id.clone());
            application_names.insert(normalize(&app.name));
        }
        for app in &intel.environment.top_applications {
            application_ids.insert(app.application_id.clone());
            application_names.insert(normalize(&app.name));
        }
        let layout_ids = HashSet::new();
        // Environment may expose layout associations via gaps/groups — keep empty if none.
        let project_id = intel
            .current_project
            .as_ref()
            .map(|p| p.id.to_string())
            .or_else(|| {
                intel
                    .workflow_context
                    .active_project_id
                    .as_ref()
                    .map(|id| id.to_string())
            });
        let task_id = intel
            .current_task
            .as_ref()
            .map(|t| t.id.to_string())
            .or_else(|| {
                intel
                    .workflow_context
                    .active_task_id
                    .as_ref()
                    .map(|id| id.to_string())
            });
        let mut work_context_ids = HashSet::new();
        for ctx in &intel.work_context.top_contexts {
            work_context_ids.insert(ctx.id.clone());
            work_context_ids.insert(normalize(&ctx.name));
        }
        let mut preference_ids = HashSet::new();
        for pref in &intel.preference_highlights {
            preference_ids.insert(pref.id.clone());
            preference_ids.insert(normalize(&pref.label));
        }
        Self {
            application_ids,
            application_names,
            layout_ids,
            project_id,
            task_id,
            work_context_ids,
            preference_ids,
            project_name: intel.current_project.as_ref().map(|p| p.name.clone()),
            task_title: intel.current_task.as_ref().map(|t| t.title.clone()),
        }
    }

    fn has_application(&self, reference: &str) -> bool {
        self.application_ids.contains(reference)
            || self.application_names.contains(&normalize(reference))
    }

    fn has_layout(&self, reference: &str) -> bool {
        self.layout_ids.contains(reference)
    }

    fn has_project(&self, reference: &str) -> bool {
        self.project_id.as_deref() == Some(reference)
            || self
                .project_name
                .as_ref()
                .map(|n| normalize(n) == normalize(reference))
                .unwrap_or(false)
    }

    fn has_task(&self, reference: &str) -> bool {
        self.task_id.as_deref() == Some(reference)
            || self
                .task_title
                .as_ref()
                .map(|t| normalize(t) == normalize(reference))
                .unwrap_or(false)
    }

    fn has_work_context(&self, reference: &str) -> bool {
        self.work_context_ids.contains(reference)
            || self.work_context_ids.contains(&normalize(reference))
    }

    fn has_preference(&self, reference: &str) -> bool {
        self.preference_ids.contains(reference)
            || self.preference_ids.contains(&normalize(reference))
    }
}

fn compare_profile(
    profile: &WorkspaceProfile,
    current: &CurrentSnapshot,
) -> WorkspaceProfileComparison {
    let mut matching = Vec::new();
    let mut differences = Vec::new();
    let mut missing = Vec::new();

    for member in &profile.members {
        let present = match member.member_type {
            WorkspaceProfileMemberType::Application => {
                current.has_application(&member.reference_id)
            }
            WorkspaceProfileMemberType::Layout => current.has_layout(&member.reference_id),
            WorkspaceProfileMemberType::Project => current.has_project(&member.reference_id),
            WorkspaceProfileMemberType::Task => current.has_task(&member.reference_id),
            WorkspaceProfileMemberType::WorkContext => {
                current.has_work_context(&member.reference_id)
            }
            WorkspaceProfileMemberType::Preference => {
                current.has_preference(&member.reference_id)
            }
        };
        if present {
            matching.push(WorkspaceProfileEvidence {
                label: if member.label.is_empty() {
                    member.reference_id.clone()
                } else {
                    member.label.clone()
                },
                member_type: member.member_type.as_str().into(),
                reference_id: member.reference_id.clone(),
                why: format!(
                    "Current workspace matches profile member ({})",
                    member.relationship.as_str()
                ),
            });
        } else {
            missing.push(member.clone());
            let observed = match member.member_type {
                WorkspaceProfileMemberType::Project => current
                    .project_name
                    .clone()
                    .or_else(|| current.project_id.clone())
                    .unwrap_or_else(|| "none".into()),
                WorkspaceProfileMemberType::Task => current
                    .task_title
                    .clone()
                    .or_else(|| current.task_id.clone())
                    .unwrap_or_else(|| "none".into()),
                _ => "not present".into(),
            };
            differences.push(WorkspaceProfileDifference {
                kind: "missing".into(),
                member_type: member.member_type.as_str().into(),
                reference_id: member.reference_id.clone(),
                expected: if member.label.is_empty() {
                    member.reference_id.clone()
                } else {
                    member.label.clone()
                },
                observed,
                why: format!(
                    "Profile expects {} \"{}\" but current workspace does not match",
                    member.member_type.as_str(),
                    member.reference_id
                ),
            });
        }
    }

    // Focus difference when project expected differs from current project id.
    if let Some(expected_project) = profile
        .members
        .iter()
        .find(|m| m.member_type == WorkspaceProfileMemberType::Project)
    {
        if !current.has_project(&expected_project.reference_id) {
            if let Some(current_project) = &current.project_id {
                if !differences.iter().any(|d| {
                    d.member_type == "project" && d.reference_id == expected_project.reference_id
                }) {
                    differences.push(WorkspaceProfileDifference {
                        kind: "different".into(),
                        member_type: "project".into(),
                        reference_id: expected_project.reference_id.clone(),
                        expected: expected_project.reference_id.clone(),
                        observed: current_project.clone(),
                        why: "Project focus differs from profile expectation".into(),
                    });
                }
            }
        }
    }

    let matched_count = matching.len();
    let missing_count = missing.len();
    let alignment = if profile.members.is_empty() {
        WorkspaceProfileAlignment::Empty
    } else if missing_count == 0 {
        WorkspaceProfileAlignment::Aligned
    } else if matched_count == 0 {
        WorkspaceProfileAlignment::Divergent
    } else {
        WorkspaceProfileAlignment::Partial
    };

    WorkspaceProfileComparison {
        profile_id: profile.id.to_string(),
        profile_name: profile.name.clone(),
        workspace_id: profile.workspace_id.to_string(),
        alignment,
        matching_evidence: matching,
        differences,
        missing_members: missing,
        matched_count,
        missing_count,
        explanation: format!(
            "Compared profile \"{}\" against current workspace — informational only; \
             never launches, restores, or executes.",
            profile.name
        ),
        authority_effect: WorkspaceProfileComparison::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn build_members(
    profile_id: &str,
    inputs: &[WorkspaceProfileMemberInput],
) -> Result<Vec<WorkspaceProfileMember>> {
    let mut members = Vec::new();
    for input in inputs {
        if input.reference_id.trim().is_empty() {
            return Err(KernelError::from(
                workspace_domain::WorkspaceProfileError::Invalid(
                    "member reference_id required".into(),
                ),
            ));
        }
        let evidence = if input.evidence.trim().is_empty() {
            format!(
                "User included {} \"{}\" in this profile",
                input.member_type.as_str(),
                input.reference_id
            )
        } else {
            input.evidence.clone()
        };
        members.push(WorkspaceProfileMember {
            id: format!("member:{}", Uuid::new_v4()),
            profile_id: profile_id.into(),
            member_type: input.member_type,
            reference_id: input.reference_id.trim().to_string(),
            relationship: input.relationship,
            evidence,
            label: input.label.clone(),
            authority_effect: WorkspaceProfileMember::AUTHORITY_EFFECT_NONE.into(),
        });
    }
    members.sort_by(|a, b| {
        a.member_type
            .as_str()
            .cmp(b.member_type.as_str())
            .then_with(|| a.reference_id.cmp(&b.reference_id))
    });
    Ok(members)
}

fn normalize(value: &str) -> String {
    value.trim().to_lowercase()
}

fn alignment_rank(alignment: WorkspaceProfileAlignment) -> u8 {
    match alignment {
        WorkspaceProfileAlignment::Aligned => 0,
        WorkspaceProfileAlignment::Partial => 1,
        WorkspaceProfileAlignment::Divergent => 2,
        WorkspaceProfileAlignment::Empty => 3,
    }
}