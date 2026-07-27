//! Runtime validation for architectural resilience invariants.
//!
//! This module enforces safety-critical boundary guarantees that were previously
//! expressed as debug-only assertions. These invariants prevent:
//! - Recommendation Engine mutation from Decision Engine
//! - Invalid Decision Engine lifecycle transitions
//! - Authorization boundary violations

use workspace_domain::{
    DecisionCandidate, DecisionCandidateEvaluationOriginContract,
    DecisionCandidateEvaluationResolution, DecisionCandidateLifecycleIntegration,
    DecisionCandidateProgressionAcknowledgement, DecisionCandidateProgressionRequest,
    DecisionCandidateRanking, DecisionCandidateScore, DecisionCandidateSelection,
    DecisionEngineCandidateCreation, DecisionEngineIntakeCandidate,
    DecisionEngineIntakeDisposition, DecisionEngineIntakeEvaluation, DecisionEngineIntakeReceipt,
    DecisionEngineState, RecommendationDecisionBoundary, RecommendationDecisionConfirmation,
    RecommendationDecisionContext, RecommendationDecisionEngineAcceptance,
    RecommendationDecisionHandoffRequest, RecommendationDecisionIntakeAdapterPreparation,
    RecommendationDecisionIntakeCompatibility, RecommendationDecisionIntakeInspection,
    RecommendationDecisionIntakePackageSeal, RecommendationDecisionIntakeProceedDenial,
    RecommendationDecisionIntakeRequest, RecommendationDecisionReadiness,
    RecommendationExplanationView,
};
use crate::error::{KernelError, Result};

fn map_marker<E: std::fmt::Display>(label: &str, result: std::result::Result<(), E>) -> Result<()> {
    result.map_err(|e| KernelError::IntegrityViolation {
        message: format!("integrity violation: {label}: {e}"),
    })
}

fn require(cond: bool, message: impl Into<String>) -> Result<()> {
    if cond {
        Ok(())
    } else {
        Err(KernelError::IntegrityViolation {
            message: message.into(),
        })
    }
}

fn require_attempt_denied<T, E>(
    result: std::result::Result<T, E>,
    message: impl Into<String>,
) -> Result<()> {
    require(result.is_err(), message)
}

/// Validates that intake receipts are observational-only and do not create
/// mutation opportunities or execution hooks back into the Recommendation Engine.
pub fn validate_intake_receipts_observational(
    receipts: &[DecisionEngineIntakeReceipt],
) -> Result<()> {
    for receipt in receipts {
        // Validate no outbound hooks that could mutate RE
        if receipt.decision_engine_object_id.is_some() {
            return Err(KernelError::IntegrityViolation {
                message:
                    "integrity violation: intake receipt must not own decision_engine_object_id"
                        .to_string(),
            });
        }
        if receipt.handoff_command.is_some() {
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: intake receipt must not carry handoff_command"
                    .to_string(),
            });
        }
        // Validate observational marker
        if let Err(e) = receipt.assert_observational_only() {
            return Err(KernelError::IntegrityViolation {
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
            return Err(KernelError::IntegrityViolation {
                message:
                    "integrity violation: intake candidate must not be marked as decision_candidate"
                        .to_string(),
            });
        }
        if candidate.handoff_command.is_some() {
            return Err(KernelError::IntegrityViolation {
                message:
                    "integrity violation: intake candidate must not carry handoff_command"
                        .to_string(),
            });
        }
        // Validate intake-only marker
        if let Err(e) = candidate.assert_intake_only() {
            return Err(KernelError::IntegrityViolation {
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
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: intake evaluation must not create_decision_candidate"
                    .to_string(),
            });
        }
        if evaluation.handoff_command.is_some() {
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: intake evaluation must not carry handoff_command"
                    .to_string(),
            });
        }
        // Validate evaluation-only marker
        if let Err(e) = evaluation.assert_evaluation_only() {
            return Err(KernelError::IntegrityViolation {
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
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: candidate creation must not create_decision_score"
                    .to_string(),
            });
        }
        if creation.planner_invoked {
            return Err(KernelError::IntegrityViolation {
                message:
                    "integrity violation: candidate creation must not invoke planner directly"
                        .to_string(),
            });
        }
        if creation.handoff_command.is_some() {
            return Err(KernelError::IntegrityViolation {
                message:
                    "integrity violation: candidate creation must not carry handoff_command"
                        .to_string(),
            });
        }
        // Validate creation boundary marker
        if let Err(e) = creation.assert_creation_boundary() {
            return Err(KernelError::IntegrityViolation {
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
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: candidate selection must not invoke planner"
                    .to_string(),
            });
        }
        if selection.mutates_candidate_outcome {
            return Err(KernelError::IntegrityViolation {
                message:
                    "integrity violation: candidate selection must not mutate candidate outcome"
                        .to_string(),
            });
        }
        if selection.handoff_command.is_some() {
            return Err(KernelError::IntegrityViolation {
                message:
                    "integrity violation: candidate selection must not carry handoff_command"
                        .to_string(),
            });
        }
        if selection.mutates_recommendation_engine {
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: candidate selection must not mutate recommendation engine"
                    .to_string(),
            });
        }
        // Validate selection-only marker
        if let Err(e) = selection.assert_selection_only() {
            return Err(KernelError::IntegrityViolation {
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
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: progression request must not invoke planner"
                    .to_string(),
            });
        }
        if request.mutates_candidate_outcome {
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: progression request must not mutate outcome"
                    .to_string(),
            });
        }
        if request.handoff_command.is_some() {
            return Err(KernelError::IntegrityViolation {
                message:
                    "integrity violation: progression request must not carry handoff_command"
                        .to_string(),
            });
        }
        if request.mutates_recommendation_engine {
            return Err(KernelError::IntegrityViolation {
                message:
                    "integrity violation: progression request must not mutate recommendation engine"
                        .to_string(),
            });
        }
        // Validate request-only marker
        if let Err(e) = request.assert_request_only() {
            return Err(KernelError::IntegrityViolation {
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
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: candidate score must not have ranking_applied"
                    .to_string(),
            });
        }
        if score.selects_candidate {
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: candidate score must not select_candidate"
                    .to_string(),
            });
        }
        if score.planner_invoked {
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: candidate score must not invoke planner"
                    .to_string(),
            });
        }
        if score.handoff_command.is_some() {
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: candidate score must not carry handoff_command"
                    .to_string(),
            });
        }
        // Validate score-only marker
        if let Err(e) = score.assert_score_only() {
            return Err(KernelError::IntegrityViolation {
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
            return Err(KernelError::IntegrityViolation {
                message:
                    "integrity violation: candidate ranking must not auto-select candidate"
                        .to_string(),
            });
        }
        if ranking.selected_candidate_id.is_some() {
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: candidate ranking must not populate selected_candidate_id"
                    .to_string(),
            });
        }
        if ranking.planner_invoked {
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: candidate ranking must not invoke planner"
                    .to_string(),
            });
        }
        if ranking.handoff_command.is_some() {
            return Err(KernelError::IntegrityViolation {
                message: "integrity violation: candidate ranking must not carry handoff_command"
                    .to_string(),
            });
        }
        // Validate ranking-only marker
        if let Err(e) = ranking.assert_ranking_only() {
            return Err(KernelError::IntegrityViolation {
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
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: recommendation decision context must not have handoff_performed"
                .to_string(),
        });
    }
    if context.decision_engine_object_id.is_some() {
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: recommendation decision context must not own decision_engine_object_id"
                .to_string(),
        });
    }
    if context.may_create_intent() {
        return Err(KernelError::IntegrityViolation {
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
        return Err(KernelError::IntegrityViolation {
            message: format!(
                "integrity violation: recommendation decision readiness authority_effect must be 'none', got '{}'",
                readiness.authority_effect
            ),
        });
    }
    if readiness.may_create_decision_commands() {
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: recommendation decision readiness must not be able to create_decision_commands"
                .to_string(),
        });
    }
    if readiness.may_invoke_gateway() {
        return Err(KernelError::IntegrityViolation {
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
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: recommendation decision boundary must not create_intent"
                .to_string(),
        });
    }
    if boundary.grants_execution_authority {
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: recommendation decision boundary must not grant_execution_authority"
                .to_string(),
        });
    }
    if let Err(e) = boundary.assert_rejection_guards() {
        return Err(KernelError::IntegrityViolation {
            message: format!(
                "integrity violation: recommendation decision boundary rejection guards failed: {}",
                e
            ),
        });
    }
    if boundary.attempt_handoff().is_ok() {
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: recommendation decision boundary must not allow handoff"
                .to_string(),
        });
    }
    if boundary.attempt_create_intent().is_ok() {
        return Err(KernelError::IntegrityViolation {
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
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: recommendation decision confirmation must not be auto-confirmed"
                .to_string(),
        });
    }
    if let Err(e) = confirmation.assert_non_authoritative() {
        return Err(KernelError::IntegrityViolation {
            message: format!(
                "integrity violation: recommendation decision confirmation non-authoritative check failed: {}",
                e
            ),
        });
    }
    Ok(())
}


/// Validates a single intake receipt observational marker (write-path helper).
pub fn validate_intake_receipt_observational(receipt: &DecisionEngineIntakeReceipt) -> Result<()> {
    validate_intake_receipts_observational(std::slice::from_ref(receipt))
}

/// Validates a single intake candidate phase marker (write-path helper).
pub fn validate_intake_candidate_phase(candidate: &DecisionEngineIntakeCandidate) -> Result<()> {
    validate_intake_candidates_phase(std::slice::from_ref(candidate))
}

/// Validates a single intake evaluation phase marker (write-path helper).
pub fn validate_intake_evaluation_phase(evaluation: &DecisionEngineIntakeEvaluation) -> Result<()> {
    validate_intake_evaluations_phase(std::slice::from_ref(evaluation))
}

/// Validates a single candidate creation boundary (write-path helper).
pub fn validate_candidate_creation_bounded(
    creation: &DecisionEngineCandidateCreation,
) -> Result<()> {
    validate_candidate_creations_bounded(std::slice::from_ref(creation))
}

/// Validates a single candidate selection boundary (write-path helper).
pub fn validate_candidate_selection_bounded(selection: &DecisionCandidateSelection) -> Result<()> {
    validate_candidate_selections_bounded(std::slice::from_ref(selection))
}

/// Validates a single progression request boundary (write-path helper).
pub fn validate_progression_request_bounded(
    request: &DecisionCandidateProgressionRequest,
) -> Result<()> {
    validate_progression_requests_bounded(std::slice::from_ref(request))
}

pub fn validate_evaluation_resolution_only(
    resolution: &DecisionCandidateEvaluationResolution,
) -> Result<()> {
    map_marker("evaluation resolution marker failed", resolution.assert_resolution_only())
}

pub fn validate_candidate_score_only(score: &DecisionCandidateScore) -> Result<()> {
    if score.ranking_applied {
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: candidate score must not have ranking_applied".into(),
        });
    }
    if score.selects_candidate {
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: candidate score must not select_candidate".into(),
        });
    }
    if score.planner_invoked {
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: candidate score must not invoke planner".into(),
        });
    }
    if score.handoff_command.is_some() {
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: candidate score must not carry handoff_command".into(),
        });
    }
    map_marker("candidate score marker failed", score.assert_score_only())
}

pub fn validate_candidate_ranking_only(ranking: &DecisionCandidateRanking) -> Result<()> {
    if ranking.selects_candidate {
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: candidate ranking must not auto-select candidate".into(),
        });
    }
    if ranking.selected_candidate_id.is_some() {
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: candidate ranking must not populate selected_candidate_id"
                .into(),
        });
    }
    if ranking.planner_invoked {
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: candidate ranking must not invoke planner".into(),
        });
    }
    if ranking.handoff_command.is_some() {
        return Err(KernelError::IntegrityViolation {
            message: "integrity violation: candidate ranking must not carry handoff_command".into(),
        });
    }
    map_marker("candidate ranking marker failed", ranking.assert_ranking_only())
}

pub fn validate_progression_acknowledgement_only(
    acknowledgement: &DecisionCandidateProgressionAcknowledgement,
) -> Result<()> {
    map_marker(
        "progression acknowledgement marker failed",
        acknowledgement.assert_acknowledgement_only(),
    )
}

pub fn validate_evaluation_origin_contract_only(
    contract: &DecisionCandidateEvaluationOriginContract,
) -> Result<()> {
    map_marker(
        "evaluation origin contract marker failed",
        contract.assert_evaluation_contract_only(),
    )
}

pub fn validate_intake_disposition_only(
    disposition: &DecisionEngineIntakeDisposition,
) -> Result<()> {
    map_marker("intake disposition marker failed", disposition.assert_disposition_only())
}

pub fn validate_lifecycle_integration_only(
    integration: &DecisionCandidateLifecycleIntegration,
) -> Result<()> {
    map_marker("lifecycle integration marker failed", integration.assert_integration_only())
}

pub fn validate_provenance_retained(
    before: &DecisionCandidate,
    after: &DecisionCandidate,
) -> Result<()> {
    map_marker(
        "lifecycle provenance retention failed",
        DecisionCandidateLifecycleIntegration::assert_provenance_retained(before, after),
    )
}

pub fn validate_candidate_score_unchanged(
    before: &DecisionCandidate,
    after: &DecisionCandidate,
) -> Result<()> {
    require(
        before.score == after.score,
        "integrity violation: decision candidate score must remain unchanged",
    )
}

pub fn validate_candidate_score_outcome_unchanged(
    before: &DecisionCandidate,
    after: &DecisionCandidate,
) -> Result<()> {
    validate_candidate_score_unchanged(before, after)?;
    require(
        before.outcome == after.outcome,
        "integrity violation: decision candidate outcome must remain unchanged",
    )
}

pub fn validate_new_intake_candidate_bootstrap(candidate: &DecisionCandidate) -> Result<()> {
    require(
        candidate.score.total == 0,
        "integrity violation: newly created decision candidate must be unscored",
    )?;
    require(
        candidate.handoff_command.is_empty(),
        "integrity violation: newly created decision candidate must not carry handoff_command",
    )
}

/// Post-confirm confirmation stays non-authoritative (may be confirmed, never executes).
pub fn validate_confirmation_post_action(
    confirmation: &RecommendationDecisionConfirmation,
) -> Result<()> {
    map_marker(
        "confirmation non-authoritative check failed",
        confirmation.assert_non_authoritative(),
    )?;
    require(
        !confirmation.handoff_performed,
        "integrity violation: confirmation must not have handoff_performed",
    )?;
    require_attempt_denied(
        confirmation.attempt_create_intent(),
        "integrity violation: confirmation must not allow intent creation",
    )?;
    require_attempt_denied(
        confirmation.attempt_create_decision_engine_object(),
        "integrity violation: confirmation must not allow decision engine object creation",
    )?;
    require_attempt_denied(
        confirmation.attempt_handoff(),
        "integrity violation: confirmation must not allow handoff",
    )
}

pub fn validate_intake_request_non_authoritative(
    intake: &RecommendationDecisionIntakeRequest,
) -> Result<()> {
    map_marker(
        "intake request non-authoritative check failed",
        intake.assert_non_authoritative(),
    )?;
    require(
        intake.decision_engine_object_id.is_none(),
        "integrity violation: intake request must not own decision_engine_object_id",
    )?;
    require(
        !intake.handoff_performed,
        "integrity violation: intake request must not have handoff_performed",
    )?;
    require_attempt_denied(
        intake.attempt_create_decision_engine_object(),
        "integrity violation: intake request must not allow decision engine object creation",
    )?;
    require_attempt_denied(
        intake.attempt_handoff(),
        "integrity violation: intake request must not allow handoff",
    )
}

pub fn validate_intake_inspection_boundary(
    inspection: &RecommendationDecisionIntakeInspection,
) -> Result<()> {
    map_marker(
        "intake inspection handoff guard failed",
        inspection.assert_inspection_is_not_handoff(),
    )?;
    require_attempt_denied(
        inspection.attempt_handoff(),
        "integrity violation: intake inspection must not allow handoff",
    )?;
    require_attempt_denied(
        inspection.attempt_create_decision_engine_object(),
        "integrity violation: intake inspection must not allow decision engine object creation",
    )?;
    require(
        !inspection.may_invoke_gateway(),
        "integrity violation: intake inspection must not invoke gateway",
    )?;
    require(
        !inspection.may_create_decision_engine_object(),
        "integrity violation: intake inspection must not create decision engine object",
    )
}

pub fn validate_intake_compatibility_boundary(
    compatibility: &RecommendationDecisionIntakeCompatibility,
) -> Result<()> {
    map_marker(
        "intake compatibility non-transfer check failed",
        compatibility.assert_non_transfer(),
    )?;
    require(
        !compatibility.transfer_authorized,
        "integrity violation: intake compatibility must not authorize transfer",
    )?;
    require(
        !compatibility.may_migrate,
        "integrity violation: intake compatibility must not allow migration",
    )?;
    require_attempt_denied(
        compatibility.attempt_handoff(),
        "integrity violation: intake compatibility must not allow handoff",
    )?;
    require_attempt_denied(
        compatibility.attempt_transfer(),
        "integrity violation: intake compatibility must not allow transfer",
    )?;
    require_attempt_denied(
        compatibility.attempt_migrate(),
        "integrity violation: intake compatibility must not allow migrate",
    )?;
    require(
        !compatibility.may_invoke_gateway(),
        "integrity violation: intake compatibility must not invoke gateway",
    )
}

pub fn validate_intake_proceed_denial_boundary(
    denial: &RecommendationDecisionIntakeProceedDenial,
) -> Result<()> {
    map_marker(
        "intake proceed denial permission guard failed",
        denial.assert_compatible_is_not_permission(),
    )?;
    require(
        !denial.proceed_authorized,
        "integrity violation: intake proceed denial must not authorize proceed",
    )?;
    require(
        !denial.consume_authorized,
        "integrity violation: intake proceed denial must not authorize consume",
    )?;
    require(
        !denial.adapter_invokable,
        "integrity violation: intake proceed denial must not allow adapter invocation",
    )?;
    require_attempt_denied(
        denial.attempt_authorize_proceed(),
        "integrity violation: intake proceed denial must not authorize proceed attempt",
    )?;
    require_attempt_denied(
        denial.attempt_invoke_adapter(),
        "integrity violation: intake proceed denial must not invoke adapter",
    )?;
    require_attempt_denied(
        denial.attempt_consume(),
        "integrity violation: intake proceed denial must not allow consume",
    )?;
    require(
        !denial.may_invoke_gateway(),
        "integrity violation: intake proceed denial must not invoke gateway",
    )
}

pub fn validate_intake_package_seal_boundary(
    seal: &RecommendationDecisionIntakePackageSeal,
    intake: Option<&RecommendationDecisionIntakeRequest>,
) -> Result<()> {
    map_marker(
        "intake package seal handoff guard failed",
        seal.assert_seal_is_not_handoff(),
    )?;
    if let Some(intake) = intake {
        map_marker(
            "intake package seal mismatch",
            seal.assert_matches_intake(intake),
        )?;
    }
    require_attempt_denied(
        seal.attempt_mutate_after_seal(),
        "integrity violation: intake package seal must not allow mutation after seal",
    )?;
    require(
        !seal.proceed_authorized,
        "integrity violation: intake package seal must not authorize proceed",
    )?;
    require(
        !seal.adapter_invokable,
        "integrity violation: intake package seal must not allow adapter invocation",
    )?;
    require_attempt_denied(
        seal.attempt_invoke_adapter(),
        "integrity violation: intake package seal must not invoke adapter",
    )
}

pub fn validate_adapter_preparation_boundary(
    preparation: &RecommendationDecisionIntakeAdapterPreparation,
    expect_active: Option<bool>,
) -> Result<()> {
    map_marker(
        "adapter preparation invocation guard failed",
        preparation.assert_preparation_is_not_invocation(),
    )?;
    if let Some(expect_active) = expect_active {
        require(
            preparation.is_active_preparation() == expect_active,
            format!(
                "integrity violation: adapter preparation active state mismatch (expected {expect_active})"
            ),
        )?;
        if expect_active {
            require(
                !preparation.mapping_performed,
                "integrity violation: adapter preparation must not mark mapping_performed",
            )?;
        }
    }
    require(
        !preparation.adapter_invoked,
        "integrity violation: adapter preparation must not mark adapter_invoked",
    )?;
    require_attempt_denied(
        preparation.attempt_invoke_adapter(),
        "integrity violation: adapter preparation must not invoke adapter",
    )?;
    require_attempt_denied(
        preparation.attempt_create_decision_engine_object(),
        "integrity violation: adapter preparation must not create decision engine object",
    )?;
    require(
        !preparation.may_invoke_gateway(),
        "integrity violation: adapter preparation must not invoke gateway",
    )
}

pub fn validate_handoff_request_boundary(
    request: &RecommendationDecisionHandoffRequest,
    expect_active: Option<bool>,
) -> Result<()> {
    map_marker(
        "handoff request performance guard failed",
        request.assert_request_is_not_performed_handoff(),
    )?;
    if let Some(expect_active) = expect_active {
        require(
            request.is_active_request() == expect_active,
            format!(
                "integrity violation: handoff request active state mismatch (expected {expect_active})"
            ),
        )?;
        if expect_active {
            require(
                request.handoff_requested,
                "integrity violation: active handoff request must set handoff_requested",
            )?;
        } else {
            require(
                !request.handoff_requested,
                "integrity violation: revoked handoff request must clear handoff_requested",
            )?;
        }
    }
    require(
        !request.handoff_performed,
        "integrity violation: handoff request must not have handoff_performed",
    )?;
    require(
        request.decision_engine_object_id.is_none(),
        "integrity violation: handoff request must not own decision_engine_object_id",
    )?;
    require_attempt_denied(
        request.attempt_perform_handoff(),
        "integrity violation: handoff request must not perform handoff",
    )?;
    require_attempt_denied(
        request.attempt_create_decision_engine_object(),
        "integrity violation: handoff request must not create decision engine object",
    )?;
    require(
        !request.may_invoke_gateway(),
        "integrity violation: handoff request must not invoke gateway",
    )
}

pub fn validate_engine_acceptance_boundary(
    acceptance: &RecommendationDecisionEngineAcceptance,
    expect_awaiting: Option<bool>,
    expect_accepted: Option<bool>,
) -> Result<()> {
    map_marker(
        "engine acceptance ownership transfer guard failed",
        acceptance.assert_acceptance_is_not_ownership_transfer(),
    )?;
    if let Some(expect_awaiting) = expect_awaiting {
        require(
            acceptance.is_awaiting() == expect_awaiting,
            format!(
                "integrity violation: engine acceptance awaiting state mismatch (expected {expect_awaiting})"
            ),
        )?;
    }
    if let Some(expect_accepted) = expect_accepted {
        require(
            acceptance.is_accepted_for_future() == expect_accepted,
            format!(
                "integrity violation: engine acceptance accepted state mismatch (expected {expect_accepted})"
            ),
        )?;
    }
    require(
        !acceptance.ownership_transferred,
        "integrity violation: engine acceptance must not transfer ownership",
    )?;
    require(
        acceptance.decision_engine_object_id.is_none(),
        "integrity violation: engine acceptance must not own decision_engine_object_id",
    )?;
    require_attempt_denied(
        acceptance.attempt_transfer_ownership(),
        "integrity violation: engine acceptance must not transfer ownership",
    )?;
    require_attempt_denied(
        acceptance.attempt_create_decision_engine_object(),
        "integrity violation: engine acceptance must not create decision engine object",
    )?;
    require(
        !acceptance.may_invoke_gateway(),
        "integrity violation: engine acceptance must not invoke gateway",
    )
}

pub fn validate_explanation_view_non_authoritative(
    view: &RecommendationExplanationView,
) -> Result<()> {
    require(
        view.authority_effect == RecommendationExplanationView::AUTHORITY_EFFECT_NONE,
        "integrity violation: explanation view authority_effect must be none",
    )
}

pub fn validate_accept_emits_no_intake(intake_is_none: bool) -> Result<()> {
    require(
        intake_is_none,
        "integrity violation: accept path must not emit decision intake",
    )
}

pub fn validate_decline_clears_intake_pipeline(
    intake_is_none: bool,
    inspection_is_none: bool,
    compatibility_is_none: bool,
    denial_is_none: bool,
    seal_is_none: bool,
    preparation_is_none: bool,
    handoff_is_none: bool,
    acceptance_is_none: bool,
) -> Result<()> {
    require(
        intake_is_none
            && inspection_is_none
            && compatibility_is_none
            && denial_is_none
            && seal_is_none
            && preparation_is_none
            && handoff_is_none
            && acceptance_is_none,
        "integrity violation: decline path must clear intake pipeline surfaces",
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_path_rejects_unexpected_intake() {
        let err = validate_accept_emits_no_intake(false).unwrap_err();
        match err {
            KernelError::IntegrityViolation { message } => {
                assert!(message.contains("accept path must not emit decision intake"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn decline_pipeline_requires_all_surfaces_cleared() {
        assert!(validate_decline_clears_intake_pipeline(
            true, true, true, true, true, true, true, true
        )
        .is_ok());
        assert!(validate_decline_clears_intake_pipeline(
            false, true, true, true, true, true, true, true
        )
        .is_err());
    }
}
