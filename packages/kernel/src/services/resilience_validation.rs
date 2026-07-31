//! Runtime validation for architectural resilience invariants.
//!
//! This module enforces safety-critical boundary guarantees that were previously
//! expressed as debug-only assertions. These invariants prevent:
//! - Recommendation Engine mutation from Decision Engine
//! - Invalid Decision Engine lifecycle transitions
//! - Authorization boundary violations

use workspace_domain::{
    DecisionEngineIntakeReceipt, DecisionEngineIntakeCandidate, DecisionEngineIntakeEvaluation,
    DecisionEngineCandidateCreation, DecisionCandidateSelection,
    DecisionCandidateProgressionRequest, DecisionCandidateRanking, DecisionEngineState,
    DecisionCandidate,
    RecommendationDecisionContext, RecommendationDecisionReadiness, RecommendationDecisionBoundary,
    RecommendationDecisionConfirmation,
};
use crate::error::{KernelError, Result};

/// Validates that intake receipts are observational-only and do not create
/// mutation opportunities or execution hooks back into the Recommendation Engine.
pub fn validate_intake_receipts_observational(
    receipts: &[DecisionEngineIntakeReceipt],
) -> Result<()> {
    for receipt in receipts {
        // Validate no outbound hooks that could mutate RE
        if receipt.decision_engine_object_id.is_some() {
            return Err(KernelError::ProjectionValidation {
                message:
                    "integrity violation: intake receipt must not own decision_engine_object_id"
                        .to_string(),
            });
        }
        if receipt.handoff_command.is_some() {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: intake receipt must not carry handoff_command"
                    .to_string(),
            });
        }
        // Validate observational marker
        if let Err(e) = receipt.assert_observational_only() {
            return Err(KernelError::ProjectionValidation {
                message: format!(
                    "integrity violation: intake receipt observational marker failed: {}",
                    e
                ),
            });
        }
    }
    Ok(())
}

/// Validates that intake candidates are in intake phase and cannot trigger
/// creation of decision candidates or execution paths.
pub fn validate_intake_candidates_phase(
    candidates: &[DecisionEngineIntakeCandidate],
) -> Result<()> {
    for candidate in candidates {
        if candidate.is_decision_candidate {
            return Err(KernelError::ProjectionValidation {
                message:
                    "integrity violation: intake candidate must not be marked as decision_candidate"
                        .to_string(),
            });
        }
        if candidate.handoff_command.is_some() {
            return Err(KernelError::ProjectionValidation {
                message:
                    "integrity violation: intake candidate must not carry handoff_command"
                        .to_string(),
            });
        }
        // Validate intake-only marker
        if let Err(e) = candidate.assert_intake_only() {
            return Err(KernelError::ProjectionValidation {
                message: format!(
                    "integrity violation: intake candidate phase marker failed: {}",
                    e
                ),
            });
        }
    }
    Ok(())
}

/// Validates that intake evaluations do not create unauthorized decision candidate
/// entries or bypass evaluation phase.
pub fn validate_intake_evaluations_phase(
    evaluations: &[DecisionEngineIntakeEvaluation],
) -> Result<()> {
    for evaluation in evaluations {
        if evaluation.creates_decision_candidate {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: intake evaluation must not create_decision_candidate"
                    .to_string(),
            });
        }
        if evaluation.handoff_command.is_some() {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: intake evaluation must not carry handoff_command"
                    .to_string(),
            });
        }
        // Validate evaluation-only marker
        if let Err(e) = evaluation.assert_evaluation_only() {
            return Err(KernelError::ProjectionValidation {
                message: format!(
                    "integrity violation: intake evaluation phase marker failed: {}",
                    e
                ),
            });
        }
    }
    Ok(())
}

/// Validates that candidate creations are DE-owned and do not invoke planner,
/// transfer execution authority, or bypass authorization gates.
pub fn validate_candidate_creations_bounded(
    creations: &[DecisionEngineCandidateCreation],
) -> Result<()> {
    for creation in creations {
        if creation.creates_decision_score {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: candidate creation must not create_decision_score"
                    .to_string(),
            });
        }
        if creation.planner_invoked {
            return Err(KernelError::ProjectionValidation {
                message:
                    "integrity violation: candidate creation must not invoke planner directly"
                        .to_string(),
            });
        }
        if creation.handoff_command.is_some() {
            return Err(KernelError::ProjectionValidation {
                message:
                    "integrity violation: candidate creation must not carry handoff_command"
                        .to_string(),
            });
        }
        // Validate creation boundary marker
        if let Err(e) = creation.assert_creation_boundary() {
            return Err(KernelError::ProjectionValidation {
                message: format!(
                    "integrity violation: candidate creation boundary marker failed: {}",
                    e
                ),
            });
        }
    }
    Ok(())
}

/// Validates that candidate selections preserve immutability of scores/outcomes
/// and do not mutate the Recommendation Engine or invoke planner.
pub fn validate_candidate_selections_bounded(
    selections: &[DecisionCandidateSelection],
) -> Result<()> {
    for selection in selections {
        if selection.planner_invoked {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: candidate selection must not invoke planner"
                    .to_string(),
            });
        }
        if selection.mutates_candidate_outcome {
            return Err(KernelError::ProjectionValidation {
                message:
                    "integrity violation: candidate selection must not mutate candidate outcome"
                        .to_string(),
            });
        }
        if selection.handoff_command.is_some() {
            return Err(KernelError::ProjectionValidation {
                message:
                    "integrity violation: candidate selection must not carry handoff_command"
                        .to_string(),
            });
        }
        if selection.mutates_recommendation_engine {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: candidate selection must not mutate recommendation engine"
                    .to_string(),
            });
        }
        // Validate selection-only marker
        if let Err(e) = selection.assert_selection_only() {
            return Err(KernelError::ProjectionValidation {
                message: format!(
                    "integrity violation: candidate selection marker failed: {}",
                    e
                ),
            });
        }
    }
    Ok(())
}

/// Validates that progression requests preserve candidate immutability
/// and do not invoke planner or mutate the Recommendation Engine.
pub fn validate_progression_requests_bounded(
    requests: &[DecisionCandidateProgressionRequest],
) -> Result<()> {
    for request in requests {
        if request.planner_invoked {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: progression request must not invoke planner"
                    .to_string(),
            });
        }
        if request.mutates_candidate_outcome {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: progression request must not mutate outcome"
                    .to_string(),
            });
        }
        if request.handoff_command.is_some() {
            return Err(KernelError::ProjectionValidation {
                message:
                    "integrity violation: progression request must not carry handoff_command"
                        .to_string(),
            });
        }
        if request.mutates_recommendation_engine {
            return Err(KernelError::ProjectionValidation {
                message:
                    "integrity violation: progression request must not mutate recommendation engine"
                        .to_string(),
            });
        }
        // Validate request-only marker
        if let Err(e) = request.assert_request_only() {
            return Err(KernelError::ProjectionValidation {
                message: format!(
                    "integrity violation: progression request marker failed: {}",
                    e
                ),
            });
        }
    }
    Ok(())
}

/// Validates that Decision Engine state aggregation preserves all critical invariants:
/// - No mutation of Recommendation Engine overlays
/// - No invalid lifecycle transitions
/// - No planner invocation from decision synthesis alone
pub fn validate_decision_engine_state_integrity(state: &DecisionEngineState) -> Result<()> {
    // Validate all aggregated sub-components
    validate_intake_receipts_observational(&state.intake_receipts)?;
    validate_intake_candidates_phase(&state.intake_candidates)?;
    validate_intake_evaluations_phase(&state.intake_evaluations)?;
    validate_candidate_creations_bounded(&state.candidate_creations)?;
    validate_candidate_selections_bounded(&state.candidate_selections)?;
    validate_progression_requests_bounded(&state.progression_requests)?;

    // Validate scoring integrity: scores should not auto-rank or select
    for score in &state.candidate_scores {
        if score.ranking_applied {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: candidate score must not have ranking_applied"
                    .to_string(),
            });
        }
        if score.selects_candidate {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: candidate score must not select_candidate"
                    .to_string(),
            });
        }
        if score.planner_invoked {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: candidate score must not invoke planner"
                    .to_string(),
            });
        }
        if score.handoff_command.is_some() {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: candidate score must not carry handoff_command"
                    .to_string(),
            });
        }
        // Validate score-only marker
        if let Err(e) = score.assert_score_only() {
            return Err(KernelError::ProjectionValidation {
                message: format!(
                    "integrity violation: candidate score marker failed: {}",
                    e
                ),
            });
        }
    }

    // Validate ranking integrity: ranking does not auto-select without explicit user action
    if let Some(ranking) = &state.candidate_ranking {
        if ranking.selects_candidate {
            return Err(KernelError::ProjectionValidation {
                message:
                    "integrity violation: candidate ranking must not auto-select candidate"
                        .to_string(),
            });
        }
        if ranking.selected_candidate_id.is_some() {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: candidate ranking must not populate selected_candidate_id"
                    .to_string(),
            });
        }
        if ranking.planner_invoked {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: candidate ranking must not invoke planner"
                    .to_string(),
            });
        }
        if ranking.handoff_command.is_some() {
            return Err(KernelError::ProjectionValidation {
                message: "integrity violation: candidate ranking must not carry handoff_command"
                    .to_string(),
            });
        }
        // Validate ranking-only marker
        if let Err(e) = ranking.assert_ranking_only() {
            return Err(KernelError::ProjectionValidation {
                message: format!(
                    "integrity violation: candidate ranking marker failed: {}",
                    e
                ),
            });
        }
    }

    Ok(())
}

/// Validates that Recommendation Engine decision contexts maintain proper boundaries:
/// - No unintended handoff to Decision Engine
/// - No object ownership transfer
/// - Cannot create intent
pub fn validate_decision_context_boundary(context: &RecommendationDecisionContext) -> Result<()> {
    if context.handoff_performed {
        return Err(KernelError::ProjectionValidation {
            message: "integrity violation: recommendation decision context must not have handoff_performed"
                .to_string(),
        });
    }
    if context.decision_engine_object_id.is_some() {
        return Err(KernelError::ProjectionValidation {
            message: "integrity violation: recommendation decision context must not own decision_engine_object_id"
                .to_string(),
        });
    }
    if context.may_create_intent() {
        return Err(KernelError::ProjectionValidation {
            message: "integrity violation: recommendation decision context must not be able to create_intent"
                .to_string(),
        });
    }
    Ok(())
}

/// Validates that Recommendation Engine decision readiness enforces non-execution:
/// - Authority effect must be "none"
/// - Cannot create decision commands
/// - Cannot invoke execution gateway
pub fn validate_decision_readiness_boundary(readiness: &RecommendationDecisionReadiness) -> Result<()> {
    if readiness.authority_effect != "none" {
        return Err(KernelError::ProjectionValidation {
            message: format!(
                "integrity violation: recommendation decision readiness authority_effect must be 'none', got '{}'",
                readiness.authority_effect
            ),
        });
    }
    if readiness.may_create_decision_commands() {
        return Err(KernelError::ProjectionValidation {
            message: "integrity violation: recommendation decision readiness must not be able to create_decision_commands"
                .to_string(),
        });
    }
    if readiness.may_invoke_gateway() {
        return Err(KernelError::ProjectionValidation {
            message: "integrity violation: recommendation decision readiness must not be able to invoke_gateway"
                .to_string(),
        });
    }
    Ok(())
}

/// Validates that Recommendation Engine decision boundaries preserve constraints:
/// - Does not create intent
/// - Does not grant execution authority
/// - Rejection guards are properly applied
pub fn validate_decision_boundary_constraints(boundary: &RecommendationDecisionBoundary) -> Result<()> {
    if boundary.creates_intent {
        return Err(KernelError::ProjectionValidation {
            message: "integrity violation: recommendation decision boundary must not create_intent"
                .to_string(),
        });
    }
    if boundary.grants_execution_authority {
        return Err(KernelError::ProjectionValidation {
            message: "integrity violation: recommendation decision boundary must not grant_execution_authority"
                .to_string(),
        });
    }
    if let Err(e) = boundary.assert_rejection_guards() {
        return Err(KernelError::ProjectionValidation {
            message: format!(
                "integrity violation: recommendation decision boundary rejection guards failed: {}",
                e
            ),
        });
    }
    if boundary.attempt_handoff().is_ok() {
        return Err(KernelError::ProjectionValidation {
            message: "integrity violation: recommendation decision boundary must not allow handoff"
                .to_string(),
        });
    }
    if boundary.attempt_create_intent().is_ok() {
        return Err(KernelError::ProjectionValidation {
            message: "integrity violation: recommendation decision boundary must not allow intent creation"
                .to_string(),
        });
    }
    Ok(())
}

/// Validates that Recommendation Engine decision confirmations are non-authoritative:
/// - Confirmation state must not be CONFIRMED
/// - Is non-authoritative (advisory only)
pub fn validate_decision_confirmation_non_authoritative(
    confirmation: &RecommendationDecisionConfirmation,
) -> Result<()> {
    if confirmation.confirmation_state == "confirmed" {
        return Err(KernelError::ProjectionValidation {
            message: "integrity violation: recommendation decision confirmation must not be auto-confirmed"
                .to_string(),
        });
    }
    if let Err(e) = confirmation.assert_non_authoritative() {
        return Err(KernelError::ProjectionValidation {
            message: format!(
                "integrity violation: recommendation decision confirmation non-authoritative check failed: {}",
                e
            ),
        });
    }
    Ok(())
}
