//! Workspace Session Engine — runtime orchestration projection (Phase 6).
//!
//! Consumes Workspace Intelligence (and thereby existing cognition projections)
//! into one coherent working-session snapshot. Owns nothing. Never plans,
//! prepares, restores, launches, or executes. Distinct from Continuity
//! (`session_anchor`) and from Intelligence (understanding envelope).

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::WorkspaceId;
use crate::workspace_readiness::ReadinessStatus;

/// Session-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceSessionError {
    #[error("session requires a workspace id")]
    MissingWorkspace,

    #[error("session cannot execute, prepare, restore, or authorize")]
    CannotExecute,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Kind of member referenced in the session (projection pointer only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionMemberKind {
    Project,
    Task,
    Purpose,
    Application,
    Decision,
    Recommendation,
    Adaptation,
    ContinuityFacet,
    Pattern,
    Composition,
}

impl SessionMemberKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Project => "project",
            Self::Task => "task",
            Self::Purpose => "purpose",
            Self::Application => "application",
            Self::Decision => "decision",
            Self::Recommendation => "recommendation",
            Self::Adaptation => "adaptation",
            Self::ContinuityFacet => "continuity_facet",
            Self::Pattern => "pattern",
            Self::Composition => "composition",
        }
    }
}

/// One session participant — always a pointer into an existing projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionMember {
    pub id: String,
    pub kind: SessionMemberKind,
    pub label: String,
    /// Why is this here?
    pub why: String,
    /// Which projection produced it?
    pub source_projection: String,
    pub source_ref: String,
    pub authority_effect: String,
}

impl SessionMember {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
}

/// Current focus projected from WorkflowContext + Continuity + Purpose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionFocus {
    pub project_label: Option<String>,
    pub task_label: Option<String>,
    pub purpose_label: String,
    pub continuity_focus: Option<String>,
    pub composition_label: Option<String>,
    pub why: String,
    pub source_projection: String,
    pub authority_effect: String,
}

/// Timeline entry from Activity Graph / Continuity progress (pointer only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionTimelineItem {
    pub id: String,
    pub summary: String,
    pub timestamp: String,
    pub why: String,
    pub source_projection: String,
    pub source_ref: String,
}

/// Decision pointer from Decision Queue (Queue remains independent).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionDecisionRef {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub priority_line: String,
    pub why: String,
    pub source_projection: String,
    pub source_ref: String,
}

/// Recommendation pointer from Recommendation Engine (Engine remains independent).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionRecommendationRef {
    pub id: String,
    pub title: String,
    pub reason: String,
    pub why: String,
    pub source_projection: String,
    pub source_ref: String,
}

/// Readiness slice (Readiness Model remains independent SoT).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionReadinessView {
    pub overall_status: ReadinessStatus,
    pub status_line: String,
    pub gap_line: String,
    pub gap_count: usize,
    pub why: String,
    pub source_projection: String,
}

/// Risk / blocker pointer from Attention, Continuity, or Readiness gaps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionRisk {
    pub id: String,
    pub title: String,
    pub explanation: String,
    pub why: String,
    pub source_projection: String,
    pub source_ref: String,
}

/// Session health — distinguishes kernel lifecycle health from readiness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionHealth {
    pub kernel_health: String,
    pub readiness_status: ReadinessStatus,
    pub note: String,
    pub why: String,
    pub source_projection: String,
}

/// Interrupted work pointer from Continuity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionInterruption {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub why: String,
    pub source_projection: String,
    pub source_ref: String,
}

/// Momentum signals from Task Graph / Evolution / Activity (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionMomentum {
    pub progress_line: String,
    pub evolution_line: String,
    pub activity_count: usize,
    pub open_task_count: usize,
    pub why: String,
    pub source_projection: String,
}

/// Human-readable session summary lines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionSummary {
    pub headline: String,
    pub doing_line: String,
    pub matters_line: String,
    pub blocked_line: String,
    pub ready_line: String,
    pub changed_line: String,
    pub narrative: String,
}

/// Full Session Engine snapshot — runtime orchestration only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSessionState {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub session_summary: SessionSummary,
    pub members: Vec<SessionMember>,
    pub focus: SessionFocus,
    pub timeline: Vec<SessionTimelineItem>,
    pub decisions: Vec<SessionDecisionRef>,
    pub recommendations: Vec<SessionRecommendationRef>,
    pub readiness: SessionReadinessView,
    pub risks: Vec<SessionRisk>,
    pub health: SessionHealth,
    pub interruptions: Vec<SessionInterruption>,
    pub momentum: SessionMomentum,
    pub member_count: usize,
    pub decision_count: usize,
    pub recommendation_count: usize,
    pub risk_count: usize,
    pub interruption_count: usize,
    /// Intelligence snapshot id/time this session was projected from.
    pub intelligence_generated_at: String,
    pub explanation: String,
    pub evidence: Vec<String>,
    pub summary: String,
    pub authority_effect: String,
}

impl WorkspaceSessionState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn summary_projection(&self, limit: usize) -> WorkspaceSessionSummary {
        WorkspaceSessionSummary {
            workspace_id: self.workspace_id.clone(),
            workspace_name: self.workspace_name.clone(),
            generated_at: self.generated_at.clone(),
            label: self.label.clone(),
            session_summary: self.session_summary.clone(),
            focus: self.focus.clone(),
            readiness: self.readiness.clone(),
            health: self.health.clone(),
            momentum: self.momentum.clone(),
            member_count: self.member_count,
            decision_count: self.decision_count,
            recommendation_count: self.recommendation_count,
            risk_count: self.risk_count,
            interruption_count: self.interruption_count,
            top_decisions: self.decisions.iter().take(limit).cloned().collect(),
            top_recommendations: self.recommendations.iter().take(limit).cloned().collect(),
            top_risks: self.risks.iter().take(limit).cloned().collect(),
            intelligence_generated_at: self.intelligence_generated_at.clone(),
            explanation: self.explanation.clone(),
            summary: self.summary.clone(),
            authority_effect: self.authority_effect.clone(),
        }
    }
}

/// Compact projection for Operator / Assistant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSessionSummary {
    pub workspace_id: String,
    pub workspace_name: String,
    pub generated_at: String,
    pub label: String,
    pub session_summary: SessionSummary,
    pub focus: SessionFocus,
    pub readiness: SessionReadinessView,
    pub health: SessionHealth,
    pub momentum: SessionMomentum,
    pub member_count: usize,
    pub decision_count: usize,
    pub recommendation_count: usize,
    pub risk_count: usize,
    pub interruption_count: usize,
    pub top_decisions: Vec<SessionDecisionRef>,
    pub top_recommendations: Vec<SessionRecommendationRef>,
    pub top_risks: Vec<SessionRisk>,
    pub intelligence_generated_at: String,
    pub explanation: String,
    pub summary: String,
    pub authority_effect: String,
}

impl Default for WorkspaceSessionSummary {
    fn default() -> Self {
        Self {
            workspace_id: String::new(),
            workspace_name: String::new(),
            generated_at: String::new(),
            label: String::new(),
            session_summary: SessionSummary {
                headline: String::new(),
                doing_line: String::new(),
                matters_line: String::new(),
                blocked_line: String::new(),
                ready_line: String::new(),
                changed_line: String::new(),
                narrative: String::new(),
            },
            focus: SessionFocus {
                project_label: None,
                task_label: None,
                purpose_label: String::new(),
                continuity_focus: None,
                composition_label: None,
                why: String::new(),
                source_projection: String::new(),
                authority_effect: WorkspaceSessionState::AUTHORITY_EFFECT_NONE.into(),
            },
            readiness: SessionReadinessView {
                overall_status: ReadinessStatus::Ready,
                status_line: String::new(),
                gap_line: String::new(),
                gap_count: 0,
                why: String::new(),
                source_projection: String::new(),
            },
            health: SessionHealth {
                kernel_health: String::new(),
                readiness_status: ReadinessStatus::Ready,
                note: String::new(),
                why: String::new(),
                source_projection: String::new(),
            },
            momentum: SessionMomentum {
                progress_line: String::new(),
                evolution_line: String::new(),
                activity_count: 0,
                open_task_count: 0,
                why: String::new(),
                source_projection: String::new(),
            },
            member_count: 0,
            decision_count: 0,
            recommendation_count: 0,
            risk_count: 0,
            interruption_count: 0,
            top_decisions: Vec::new(),
            top_recommendations: Vec::new(),
            top_risks: Vec::new(),
            intelligence_generated_at: String::new(),
            explanation: String::new(),
            summary: String::new(),
            authority_effect: WorkspaceSessionState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Side-by-side comparison of two session snapshots (informational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSessionComparison {
    pub left_workspace_id: String,
    pub right_workspace_id: String,
    pub differences: Vec<String>,
    pub authority_effect: String,
}

impl WorkspaceSessionComparison {
    pub fn compare(left: &WorkspaceSessionState, right: &WorkspaceSessionState) -> Self {
        let mut differences = Vec::new();
        if left.focus.project_label != right.focus.project_label {
            differences.push("Active project focus differs.".into());
        }
        if left.focus.task_label != right.focus.task_label {
            differences.push("Active task focus differs.".into());
        }
        if left.readiness.overall_status != right.readiness.overall_status {
            differences.push(format!(
                "Readiness: {} → {}",
                left.readiness.overall_status.as_str(),
                right.readiness.overall_status.as_str()
            ));
        }
        if left.decision_count != right.decision_count {
            differences.push(format!(
                "Decision count: {} → {}",
                left.decision_count, right.decision_count
            ));
        }
        if left.risk_count != right.risk_count {
            differences.push(format!(
                "Risk count: {} → {}",
                left.risk_count, right.risk_count
            ));
        }
        if left.intelligence_generated_at != right.intelligence_generated_at {
            differences.push("Projected from different Intelligence snapshots.".into());
        }
        if differences.is_empty() {
            differences.push("No material session differences.".into());
        }
        Self {
            left_workspace_id: left.workspace_id.clone(),
            right_workspace_id: right.workspace_id.clone(),
            differences,
            authority_effect: WorkspaceSessionState::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

pub fn build_session_summary(label: &str, decision_count: usize, risk_count: usize) -> String {
    format!(
        "Session for \"{label}\" — {decision_count} decision(s), {risk_count} risk(s). \
         Runtime projection only; owns no source data; never executes or prepares."
    )
}

pub fn session_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

pub fn validate_session_workspace_id(
    workspace_id: impl Into<String>,
) -> Result<WorkspaceId, WorkspaceSessionError> {
    WorkspaceId::new(workspace_id).map_err(Into::into)
}
