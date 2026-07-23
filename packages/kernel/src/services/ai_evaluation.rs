//! AI proposal evaluation service — observation and measurement only (Sprints 54–55).
//!
//! Never calls ProcessLauncher, approval mutators, grant APIs, or Permission Gateway
//! decision paths. Evaluation cannot become authority.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    evaluate_plan, evaluate_submission_result, ActorContext, AiPlan, AiPlanEvaluationReport,
    AiPlanSubmissionResult, AiProposalEvaluation, AiProposalOutcomeClass, AiWorkspaceAwareness,
    IntentContext,
};

use crate::error::{KernelError, Result};
use crate::services::AuditService;

const MAX_AUDIT_SCAN: usize = 500;

/// Read-only evaluation of AI plans and proposal outcomes.
pub(crate) struct AiEvaluationService;

impl AiEvaluationService {
    /// Evaluates a plan's proposal quality without submission or execution.
    pub(crate) fn evaluate_plan(
        plan: &AiPlan,
        awareness: Option<&AiWorkspaceAwareness>,
    ) -> Result<AiPlanEvaluationReport> {
        let report = evaluate_plan(plan, awareness);
        report
            .validate()
            .map_err(|error| KernelError::AiEvaluationValidation {
                message: error.to_string(),
            })?;
        Ok(report)
    }

    /// Evaluates a plan after governed submissions (records outcomes only).
    pub(crate) fn evaluate_submission_result(
        result: &AiPlanSubmissionResult,
        awareness: Option<&AiWorkspaceAwareness>,
    ) -> Result<AiPlanEvaluationReport> {
        let report = evaluate_submission_result(result, awareness);
        report
            .validate()
            .map_err(|error| KernelError::AiEvaluationValidation {
                message: error.to_string(),
            })?;
        Ok(report)
    }

    /// Audits per-proposal evaluation (operational facts — no chain-of-thought).
    pub(crate) fn audit_proposal_evaluated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        evaluation: &AiProposalEvaluation,
    ) -> Result<()> {
        let issues: Vec<String> = evaluation
            .quality_issues
            .iter()
            .map(|issue| match issue {
                workspace_domain::AiProposalQualityIssue::Invalid { reason } => {
                    format!("invalid:{reason}")
                }
                workspace_domain::AiProposalQualityIssue::Irrelevant { reason } => {
                    format!("irrelevant:{reason}")
                }
                workspace_domain::AiProposalQualityIssue::Duplicate { of_proposal_id } => {
                    format!("duplicate:{of_proposal_id}")
                }
                workspace_domain::AiProposalQualityIssue::Unnecessary { reason } => {
                    format!("unnecessary:{reason}")
                }
            })
            .collect();

        let metadata = json!({
            "proposal_id": evaluation.proposal_id,
            "goal_id": evaluation.goal_id,
            "command": evaluation.command_name,
            "target": evaluation.target,
            "validity": evaluation.validity,
            "relevance": evaluation.relevance,
            "quality_issues": issues,
            "outcome": evaluation.outcome.as_str(),
            "outcome_detail": evaluation.outcome_detail,
        })
        .to_string();

        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.planning.proposal_evaluated",
            true,
            metadata,
        )
    }

    /// Audits a classified operational outcome for a proposal.
    pub(crate) fn audit_outcome_recorded(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        evaluation: &AiProposalEvaluation,
    ) -> Result<()> {
        if matches!(
            evaluation.outcome,
            AiProposalOutcomeClass::Created | AiProposalOutcomeClass::NotSubmitted
        ) {
            return Ok(());
        }

        let metadata = json!({
            "proposal_id": evaluation.proposal_id,
            "goal_id": evaluation.goal_id,
            "command": evaluation.command_name,
            "target": evaluation.target,
            "outcome": evaluation.outcome.as_str(),
            "outcome_detail": evaluation.outcome_detail,
        })
        .to_string();

        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.planning.outcome_recorded",
            true,
            metadata,
        )
    }

    /// Persists evaluation + outcome audits for a report (still non-authoritative).
    pub(crate) fn audit_report(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        report: &AiPlanEvaluationReport,
    ) -> Result<()> {
        for evaluation in &report.evaluations {
            Self::audit_proposal_evaluated(db, actor, evaluation)?;
            Self::audit_outcome_recorded(db, actor, evaluation)?;
        }
        Ok(())
    }

    /// Derived proposal evaluation history from operational audits.
    pub(crate) fn list_recent_evaluations(
        db: &Arc<Mutex<Database>>,
        limit: usize,
    ) -> Result<Vec<AiProposalEvaluation>> {
        let scan = limit.saturating_mul(4).clamp(limit.max(1), MAX_AUDIT_SCAN);
        let events = AuditService::list_recent(db, scan)?;
        let mut records = Vec::new();

        for event in events {
            if event.event_type != "ai.planning.proposal_evaluated" {
                continue;
            }
            let Some(metadata) = event.metadata.as_deref() else {
                continue;
            };
            let Ok(value) = serde_json::from_str::<serde_json::Value>(metadata) else {
                continue;
            };

            let Some(proposal_id) = value.get("proposal_id").and_then(|v| v.as_str()) else {
                continue;
            };
            let Some(goal_id) = value.get("goal_id").and_then(|v| v.as_str()) else {
                continue;
            };
            let command_name = value
                .get("command")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let target = value
                .get("target")
                .and_then(|v| v.as_str())
                .map(str::to_string);
            let outcome = value
                .get("outcome")
                .and_then(|v| v.as_str())
                .and_then(parse_outcome)
                .unwrap_or(AiProposalOutcomeClass::Created);
            let outcome_detail = value
                .get("outcome_detail")
                .and_then(|v| v.as_str())
                .map(str::to_string);

            let validity = serde_json::from_value(
                value
                    .get("validity")
                    .cloned()
                    .unwrap_or(json!({"kind":"valid"})),
            )
            .unwrap_or(workspace_domain::AiProposalValidity::Valid);
            let relevance = serde_json::from_value(
                value
                    .get("relevance")
                    .cloned()
                    .unwrap_or(json!({"kind":"unknown"})),
            )
            .unwrap_or(workspace_domain::AiProposalRelevance::Unknown);

            let quality_issues = value
                .get("quality_issues")
                .and_then(|v| v.as_array())
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|item| item.as_str())
                        .filter_map(parse_issue_tag)
                        .collect()
                })
                .unwrap_or_default();

            let evaluation = AiProposalEvaluation {
                proposal_id: proposal_id.to_string(),
                goal_id: goal_id.to_string(),
                command_name,
                target,
                validity,
                relevance,
                quality_issues,
                outcome,
                outcome_detail,
                evaluated_at: event.timestamp,
            };

            evaluation
                .validate()
                .map_err(|error| KernelError::AiEvaluationValidation {
                    message: error.to_string(),
                })?;
            records.push(evaluation);
            if records.len() >= limit {
                break;
            }
        }

        Ok(records)
    }
}

fn parse_outcome(value: &str) -> Option<AiProposalOutcomeClass> {
    match value {
        "created" => Some(AiProposalOutcomeClass::Created),
        "not_submitted" => Some(AiProposalOutcomeClass::NotSubmitted),
        "approval_required" => Some(AiProposalOutcomeClass::ApprovalRequired),
        "denied" => Some(AiProposalOutcomeClass::Denied),
        "succeeded" => Some(AiProposalOutcomeClass::Succeeded),
        "failed" => Some(AiProposalOutcomeClass::Failed),
        _ => None,
    }
}

fn parse_issue_tag(tag: &str) -> Option<workspace_domain::AiProposalQualityIssue> {
    if let Some(reason) = tag.strip_prefix("invalid:") {
        return Some(workspace_domain::AiProposalQualityIssue::Invalid {
            reason: reason.to_string(),
        });
    }
    if let Some(reason) = tag.strip_prefix("irrelevant:") {
        return Some(workspace_domain::AiProposalQualityIssue::Irrelevant {
            reason: reason.to_string(),
        });
    }
    if let Some(of_proposal_id) = tag.strip_prefix("duplicate:") {
        return Some(workspace_domain::AiProposalQualityIssue::Duplicate {
            of_proposal_id: of_proposal_id.to_string(),
        });
    }
    if let Some(reason) = tag.strip_prefix("unnecessary:") {
        return Some(workspace_domain::AiProposalQualityIssue::Unnecessary {
            reason: reason.to_string(),
        });
    }
    None
}
