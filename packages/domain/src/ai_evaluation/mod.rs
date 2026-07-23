//! AI proposal evaluation — measurement only (Sprints 54–55).
//!
//! Answers "was this proposal useful, safe, necessary, and appropriate?"
//! Does not grant authority, execute actions, or store chain-of-thought.
//!
//! Separations preserved:
//! - AI reasoning: what should happen?
//! - Evaluation: was this proposal good?
//! - Permission: is this allowed?
//! - Execution: did it happen?

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ai_planning::{
    AiActionProposal, AiPlan, AiPlanSubmissionResult, AiProposalAuthorityOutcome,
    AiWorkspaceAwareness,
};

/// Evaluation-layer validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AiEvaluationError {
    #[error("AI evaluation proposal id must not be empty")]
    EmptyProposalId,

    #[error("AI evaluation goal id must not be empty")]
    EmptyGoalId,

    #[error("AI evaluation timestamp must not be empty")]
    EmptyTimestamp,
}

/// Structural validity of a proposal (not permission).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AiProposalValidity {
    Valid,
    Invalid { reason: String },
}

/// Relevance to the stated goal (heuristic, operational).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AiProposalRelevance {
    Relevant,
    Irrelevant { reason: String },
    Unknown,
}

/// Quality issues detected without private reasoning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AiProposalQualityIssue {
    Invalid { reason: String },
    Irrelevant { reason: String },
    Duplicate { of_proposal_id: String },
    Unnecessary { reason: String },
}

/// Operational outcome class — separate from Permission Gateway decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiProposalOutcomeClass {
    /// Proposal created and evaluated; not yet submitted.
    Created,
    NotSubmitted,
    ApprovalRequired,
    Denied,
    Succeeded,
    Failed,
}

impl AiProposalOutcomeClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::NotSubmitted => "not_submitted",
            Self::ApprovalRequired => "approval_required",
            Self::Denied => "denied",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
        }
    }
}

/// Evaluation of one proposal — operational facts only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiProposalEvaluation {
    pub proposal_id: String,
    pub goal_id: String,
    pub command_name: String,
    pub target: Option<String>,
    pub validity: AiProposalValidity,
    pub relevance: AiProposalRelevance,
    pub quality_issues: Vec<AiProposalQualityIssue>,
    pub outcome: AiProposalOutcomeClass,
    pub outcome_detail: Option<String>,
    pub evaluated_at: String,
}

impl AiProposalEvaluation {
    pub fn validate(&self) -> Result<(), AiEvaluationError> {
        if self.proposal_id.trim().is_empty() {
            return Err(AiEvaluationError::EmptyProposalId);
        }
        if self.goal_id.trim().is_empty() {
            return Err(AiEvaluationError::EmptyGoalId);
        }
        if self.evaluated_at.trim().is_empty() {
            return Err(AiEvaluationError::EmptyTimestamp);
        }
        Ok(())
    }

    pub fn has_issue(&self, predicate: impl Fn(&AiProposalQualityIssue) -> bool) -> bool {
        self.quality_issues.iter().any(predicate)
    }
}

/// Aggregate counts for a plan evaluation report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiEvaluationSummary {
    pub proposal_count: usize,
    pub valid_count: usize,
    pub invalid_count: usize,
    pub relevant_count: usize,
    pub irrelevant_count: usize,
    pub duplicate_count: usize,
    pub unnecessary_count: usize,
    pub rejected_count: usize,
    pub approval_required_count: usize,
    pub successful_count: usize,
    pub failed_count: usize,
}

impl AiEvaluationSummary {
    pub fn from_evaluations(evaluations: &[AiProposalEvaluation]) -> Self {
        let mut summary = Self {
            proposal_count: evaluations.len(),
            valid_count: 0,
            invalid_count: 0,
            relevant_count: 0,
            irrelevant_count: 0,
            duplicate_count: 0,
            unnecessary_count: 0,
            rejected_count: 0,
            approval_required_count: 0,
            successful_count: 0,
            failed_count: 0,
        };

        for evaluation in evaluations {
            match &evaluation.validity {
                AiProposalValidity::Valid => summary.valid_count += 1,
                AiProposalValidity::Invalid { .. } => summary.invalid_count += 1,
            }
            match &evaluation.relevance {
                AiProposalRelevance::Relevant => summary.relevant_count += 1,
                AiProposalRelevance::Irrelevant { .. } => summary.irrelevant_count += 1,
                AiProposalRelevance::Unknown => {}
            }
            if evaluation.has_issue(|issue| matches!(issue, AiProposalQualityIssue::Duplicate { .. }))
            {
                summary.duplicate_count += 1;
            }
            if evaluation
                .has_issue(|issue| matches!(issue, AiProposalQualityIssue::Unnecessary { .. }))
            {
                summary.unnecessary_count += 1;
            }
            match evaluation.outcome {
                AiProposalOutcomeClass::Denied => summary.rejected_count += 1,
                AiProposalOutcomeClass::ApprovalRequired => summary.approval_required_count += 1,
                AiProposalOutcomeClass::Succeeded => summary.successful_count += 1,
                AiProposalOutcomeClass::Failed => summary.failed_count += 1,
                AiProposalOutcomeClass::Created | AiProposalOutcomeClass::NotSubmitted => {}
            }
        }

        summary
    }
}

/// Diagnostic report: plan quality + optional outcomes.
///
/// Explicitly non-authoritative — evaluation never grants permissions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiPlanEvaluationReport {
    pub goal_id: String,
    pub goal_statement: String,
    pub evaluations: Vec<AiProposalEvaluation>,
    pub summary: AiEvaluationSummary,
    pub generated_at: String,
    /// Fixed reminder: measurement ≠ authority.
    pub authority_note: String,
}

impl AiPlanEvaluationReport {
    pub const AUTHORITY_NOTE: &'static str =
        "Evaluation is observational only. The Permission Gateway remains the sole authority boundary.";

    pub fn validate(&self) -> Result<(), AiEvaluationError> {
        if self.goal_id.trim().is_empty() {
            return Err(AiEvaluationError::EmptyGoalId);
        }
        if self.generated_at.trim().is_empty() {
            return Err(AiEvaluationError::EmptyTimestamp);
        }
        for evaluation in &self.evaluations {
            evaluation.validate()?;
        }
        Ok(())
    }
}

/// Maps a governance authority outcome to an evaluation outcome class.
pub fn classify_authority_outcome(
    outcome: &AiProposalAuthorityOutcome,
) -> (AiProposalOutcomeClass, Option<String>) {
    match outcome {
        AiProposalAuthorityOutcome::Allowed => (AiProposalOutcomeClass::Succeeded, None),
        AiProposalAuthorityOutcome::Denied { reason } => {
            (AiProposalOutcomeClass::Denied, Some(reason.clone()))
        }
        AiProposalAuthorityOutcome::ApprovalRequired { reason, .. } => (
            AiProposalOutcomeClass::ApprovalRequired,
            Some(reason.clone()),
        ),
    }
}

/// Evaluates proposals in a plan without submission or execution.
pub fn evaluate_plan(
    plan: &AiPlan,
    awareness: Option<&AiWorkspaceAwareness>,
) -> AiPlanEvaluationReport {
    evaluate_proposals(
        &plan.goal.id.to_string(),
        &plan.goal.statement,
        &plan.proposals,
        None,
        awareness,
        AiProposalOutcomeClass::Created,
    )
}

/// Evaluates a plan after governed submissions (outcomes recorded, still no authority).
pub fn evaluate_submission_result(
    result: &AiPlanSubmissionResult,
    awareness: Option<&AiWorkspaceAwareness>,
) -> AiPlanEvaluationReport {
    let outcomes: Vec<(AiProposalOutcomeClass, Option<String>)> = result
        .submissions
        .iter()
        .map(|submission| classify_authority_outcome(&submission.outcome))
        .collect();

    evaluate_proposals(
        &result.plan.goal.id.to_string(),
        &result.plan.goal.statement,
        &result.plan.proposals,
        Some(&outcomes),
        awareness,
        AiProposalOutcomeClass::NotSubmitted,
    )
}

fn evaluate_proposals(
    goal_id: &str,
    goal_statement: &str,
    proposals: &[AiActionProposal],
    outcomes: Option<&[(AiProposalOutcomeClass, Option<String>)]>,
    awareness: Option<&AiWorkspaceAwareness>,
    default_outcome: AiProposalOutcomeClass,
) -> AiPlanEvaluationReport {
    let evaluated_at = Utc::now().to_rfc3339();
    let mut seen_targets: Vec<(String, String, String)> = Vec::new();
    let mut evaluations = Vec::new();

    for (index, proposal) in proposals.iter().enumerate() {
        let target = proposal
            .target_resource
            .as_ref()
            .map(|resource| resource.canonical());

        let (validity, mut quality_issues) = evaluate_validity(proposal);
        let relevance = evaluate_relevance(proposal, goal_statement);
        if let AiProposalRelevance::Irrelevant { reason } = &relevance {
            quality_issues.push(AiProposalQualityIssue::Irrelevant {
                reason: reason.clone(),
            });
        }

        if let Some(target_key) = target.as_ref() {
            if let Some((_, _, prior_id)) = seen_targets.iter().find(|(command, prior_target, _)| {
                command == &proposal.command_name && prior_target == target_key
            }) {
                quality_issues.push(AiProposalQualityIssue::Duplicate {
                    of_proposal_id: prior_id.clone(),
                });
            } else {
                seen_targets.push((
                    proposal.command_name.clone(),
                    target_key.clone(),
                    proposal.id.to_string(),
                ));
            }
        }

        if let Some(reason) = unnecessary_reason(proposal, awareness) {
            quality_issues.push(AiProposalQualityIssue::Unnecessary { reason });
        }

        let (outcome, outcome_detail) = match outcomes {
            Some(list) => list
                .get(index)
                .cloned()
                .unwrap_or((default_outcome, None)),
            None => (default_outcome, None),
        };

        evaluations.push(AiProposalEvaluation {
            proposal_id: proposal.id.to_string(),
            goal_id: goal_id.to_string(),
            command_name: proposal.command_name.clone(),
            target,
            validity,
            relevance,
            quality_issues,
            outcome,
            outcome_detail,
            evaluated_at: evaluated_at.clone(),
        });
    }

    let summary = AiEvaluationSummary::from_evaluations(&evaluations);
    AiPlanEvaluationReport {
        goal_id: goal_id.to_string(),
        goal_statement: goal_statement.to_string(),
        evaluations,
        summary,
        generated_at: evaluated_at,
        authority_note: AiPlanEvaluationReport::AUTHORITY_NOTE.into(),
    }
}

fn evaluate_validity(
    proposal: &AiActionProposal,
) -> (AiProposalValidity, Vec<AiProposalQualityIssue>) {
    match proposal.validate() {
        Ok(()) => (AiProposalValidity::Valid, Vec::new()),
        Err(error) => {
            let reason = error.to_string();
            (
                AiProposalValidity::Invalid {
                    reason: reason.clone(),
                },
                vec![AiProposalQualityIssue::Invalid { reason }],
            )
        }
    }
}

fn evaluate_relevance(proposal: &AiActionProposal, goal_statement: &str) -> AiProposalRelevance {
    if proposal.command_name.trim().is_empty() {
        return AiProposalRelevance::Irrelevant {
            reason: "Empty command name".into(),
        };
    }

    if proposal.command_name == "LaunchApplication" {
        if proposal.target_resource.is_none() {
            return AiProposalRelevance::Irrelevant {
                reason: "Launch proposal missing application target".into(),
            };
        }
        if goal_statement.trim().is_empty() {
            return AiProposalRelevance::Unknown;
        }
        return AiProposalRelevance::Relevant;
    }

    AiProposalRelevance::Unknown
}

fn unnecessary_reason(
    proposal: &AiActionProposal,
    awareness: Option<&AiWorkspaceAwareness>,
) -> Option<String> {
    let awareness = awareness?;
    let target = proposal.target_resource.as_ref()?;
    if proposal.command_name != "LaunchApplication" {
        return None;
    }

    let app = awareness
        .applications
        .iter()
        .find(|application| application.id.as_str() == target.id.as_str())?;

    if app.appears_active {
        Some(format!("Already running: '{}'", app.name))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_planning::{AiApplicationAwareness, AiGoal};
    use crate::ids::ApplicationId;

    fn sample_plan_with_apps(app_ids: &[&str]) -> AiPlan {
        let goal = AiGoal::new("Prepare my workspace", "ai-1").unwrap();
        let proposals = app_ids
            .iter()
            .map(|id| {
                let app = ApplicationId::new(*id).unwrap();
                AiActionProposal::propose_application_launch(
                    &goal,
                    &app,
                    Some(format!("Launch {id}")),
                )
                .unwrap()
            })
            .collect();
        AiPlan::new(goal, proposals)
    }

    #[test]
    fn evaluates_plan_without_execution_signals() {
        let plan = sample_plan_with_apps(&["app-1", "app-2"]);
        let report = evaluate_plan(&plan, None);
        assert_eq!(report.summary.proposal_count, 2);
        assert_eq!(report.summary.valid_count, 2);
        assert_eq!(report.summary.successful_count, 0);
        assert!(report.evaluations.iter().all(|e| {
            e.outcome == AiProposalOutcomeClass::Created && e.validate().is_ok()
        }));
        assert_eq!(report.authority_note, AiPlanEvaluationReport::AUTHORITY_NOTE);
    }

    #[test]
    fn detects_duplicate_and_unnecessary_proposals() {
        let plan = sample_plan_with_apps(&["app-1", "app-1"]);
        let app_id = ApplicationId::new("app-1").unwrap();
        let awareness = AiWorkspaceAwareness {
            workspace_id: "ws-1".into(),
            workspace_name: "Dev".into(),
            zone_count: 0,
            layout_id: None,
            applications: vec![AiApplicationAwareness {
                id: app_id,
                name: "VS Code".into(),
                identifier: Some("code.exe".into()),
                appears_active: true,
            }],
            recent_observation_count: 0,
            environment_window_titles: vec![],
        };

        let report = evaluate_plan(&plan, Some(&awareness));
        assert_eq!(report.summary.duplicate_count, 1);
        assert_eq!(report.summary.unnecessary_count, 2);
        assert!(report.evaluations[1].has_issue(|issue| {
            matches!(issue, AiProposalQualityIssue::Duplicate { .. })
        }));
        assert!(report.evaluations[0].has_issue(|issue| {
            matches!(
                issue,
                AiProposalQualityIssue::Unnecessary { reason } if reason.contains("Already running")
            )
        }));
    }

    #[test]
    fn classifies_denied_outcome_without_unblocking() {
        let plan = sample_plan_with_apps(&["app-1"]);
        let request = plan.proposals[0]
            .to_action_request(&plan.goal.requesting_actor_id)
            .unwrap();
        let result = AiPlanSubmissionResult {
            plan: plan.clone(),
            submissions: vec![crate::ai_planning::AiProposalSubmission {
                proposal: plan.proposals[0].clone(),
                request,
                outcome: AiProposalAuthorityOutcome::Denied {
                    reason: "missing capability".into(),
                },
            }],
        };

        let report = evaluate_submission_result(&result, None);
        assert_eq!(report.summary.rejected_count, 1);
        assert_eq!(
            report.evaluations[0].outcome,
            AiProposalOutcomeClass::Denied
        );
        assert_eq!(
            report.authority_note,
            AiPlanEvaluationReport::AUTHORITY_NOTE
        );
    }
}
