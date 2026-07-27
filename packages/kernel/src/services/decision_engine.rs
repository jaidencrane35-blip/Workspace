//! Governed Decision Engine (Phase 5 — Sprints 80–81).
//!
//! Synthesizes Attention + memory + personalization + goals into ranked
//! DecisionCandidates. Observes accepted Recommendation Engine sealed packages
//! as informational intake receipts only. Never executes, never grants authority,
//! never plans, never mutates Recommendation Engine overlays.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, DecisionEngineRepository, RecommendationLifecycleRepository};
use workspace_domain::{
    ActorContext, AttentionItem, DecisionCandidate, DecisionCandidateEvaluationOriginContract,
    DecisionCandidateEvaluationOriginInput, DecisionCandidateEvaluationResolution,
    DecisionCandidateEvaluationResolutionInput, DecisionCandidateLifecycleIntegration,
    DecisionCandidateProgressionAcknowledgement, DecisionCandidateProgressionAcknowledgementInput,
    DecisionCandidateProgressionRequest, DecisionCandidateProgressionRequestInput,
    DecisionCandidateRanking, DecisionCandidateRankingMemberInput, DecisionCandidateScore,
    DecisionCandidateScoreInput, DecisionCandidateSelection, DecisionCandidateSelectionInput,
    DecisionContext, DecisionEngineActionResult, DecisionEngineCandidateCreation,
    DecisionEngineCandidateCreationInput, DecisionEngineCandidateCreationRequest,
    DecisionEngineError, DecisionEngineHandoff, DecisionEngineIntakeAssessment,
    DecisionEngineIntakeAssessmentInput, DecisionEngineIntakeCandidate,
    DecisionEngineIntakeDisposition, DecisionEngineIntakeEligibility,
    DecisionEngineIntakeEvaluation, DecisionEngineIntakePromotionBoundary,
    DecisionEngineIntakePromotionBoundaryInput, DecisionEngineIntakeReceipt,
    DecisionEngineOverlay, DecisionEngineState, DecisionEngineSummary, DecisionExplanation,
    DecisionOutcome, DecisionQueue, DecisionReason, DecisionScore, DecisionSourceType,
    DecisionState, IntelligenceHighlight, IntentContext, RecommendationLifecycleState,
    WorkspaceAttentionState, WorkspaceId, WorkGoal, WorkflowContext,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, DecisionQueueService, OrchestratedPlanStore,
    WorkspaceAttentionService, WorkspaceIntentService,
};

pub(crate) struct DecisionEngineService;

impl DecisionEngineService {
    /// Standalone generate — builds Attention then synthesizes decisions.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<DecisionEngineState> {
        let workspace_id = workspace_id.into();
        let attention = WorkspaceAttentionService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let queue = DecisionQueueService::aggregate_readonly(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let workflow = WorkspaceIntentService::get_workflow_context_readonly(db, &workspace_id)?;
        let goals = WorkspaceIntentService::list_goals(db, &workspace_id, 10).unwrap_or_default();
        let memory = crate::services::AiMemoryService::assemble_awareness(db, Some(&workspace_id), 10)
            .unwrap_or_else(|_| workspace_domain::AiMemoryAwareness::from_entries(Vec::new()));
        let personalization =
            crate::services::AiPersonalizationService::assemble_awareness(db, Some(&workspace_id), 10, None)
                .unwrap_or_else(|_| {
                    workspace_domain::AiPersonalizationAwareness::from_preferences(Vec::new(), true)
                });
        let memory_highlights: Vec<_> = memory
            .entries
            .iter()
            .take(5)
            .map(|entry| IntelligenceHighlight {
                id: entry.id.to_string(),
                label: entry.key.clone(),
                summary: entry.summary.clone(),
                source: "memory".into(),
            })
            .collect();
        let preference_highlights: Vec<_> = if personalization.enabled {
            personalization
                .preferences
                .iter()
                .take(5)
                .map(|pref| IntelligenceHighlight {
                    id: pref.id.to_string(),
                    label: pref.label.clone().unwrap_or_else(|| pref.key.clone()),
                    summary: pref.value.clone(),
                    source: "preference".into(),
                })
                .collect()
        } else {
            Vec::new()
        };
        let task_graph =
            crate::services::TaskGraphService::generate(db, actor, workspace_id.clone())?;
        Self::generate_with_inputs(
            db,
            actor,
            &workspace_id,
            &attention,
            &queue,
            &workflow,
            &goals,
            &memory_highlights,
            &preference_highlights,
            Some(&task_graph),
        )
    }

    /// Preferred path — Intelligence injects Attention + context (+ Task Graph).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        attention: &WorkspaceAttentionState,
        decision_queue: &DecisionQueue,
        workflow: &WorkflowContext,
        goals: &[WorkGoal],
        memory_highlights: &[IntelligenceHighlight],
        preference_highlights: &[IntelligenceHighlight],
        task_graph: Option<&workspace_domain::TaskGraph>,
    ) -> Result<DecisionEngineState> {
        let workspace_id = WorkspaceId::new(workspace_id.into()).map_err(KernelError::Domain)?;
        let ws = workspace_id.as_str();
        let overlays = Self::load_overlays(db, ws)?;
        let previous_ranks = Self::previous_open_order(db, actor, ws);

        let attention_open = |state: DecisionState| {
            matches!(
                state,
                DecisionState::Pending | DecisionState::Viewed | DecisionState::Deferred
            )
        };
        let pending_approvals: Vec<_> = decision_queue
            .items
            .iter()
            .filter(|item| {
                item.source_type == DecisionSourceType::PendingApproval && attention_open(item.decision_state)
            })
            .collect();
        let pending_plans: Vec<_> = decision_queue
            .items
            .iter()
            .filter(|item| {
                item.source_type == DecisionSourceType::PlanningContinuation
                    && attention_open(item.decision_state)
            })
            .collect();

        let (open_count, blocked_count) = task_graph
            .map(|g| {
                (
                    g.nodes
                        .iter()
                        .filter(|n| n.task.status.is_open())
                        .count(),
                    g.blocked_count,
                )
            })
            .unwrap_or((0, 0));

        let context = DecisionContext {
            workspace_id: ws.to_string(),
            active_project_id: workflow.active_project_id.as_ref().map(|id| id.to_string()),
            active_task_id: workflow.active_task_id.as_ref().map(|id| id.to_string()),
            attention_item_count: attention.items.len(),
            memory_highlight_count: memory_highlights.len(),
            preference_highlight_count: preference_highlights.len(),
            pending_approval_count: pending_approvals.len(),
            pending_plan_count: pending_plans.len(),
            task_graph_open_count: open_count,
            task_graph_blocked_count: blocked_count,
        };

        let mut candidates = Vec::new();
        let mut seen = std::collections::HashSet::new();

        // Completed graph work must not be recommended again.
        let completed_titles: std::collections::HashSet<String> = task_graph
            .map(|g| {
                g.nodes
                    .iter()
                    .filter(|n| n.task.status == workspace_domain::WorkspaceTaskStatus::Completed)
                    .map(|n| n.task.title.to_lowercase())
                    .collect()
            })
            .unwrap_or_default();

        for item in &attention.top_items {
            if completed_titles.contains(&item.title.to_lowercase()) {
                continue;
            }
            let key = format!("attention:{}", item.id);
            if !seen.insert(key.clone()) {
                continue;
            }
            candidates.push(Self::candidate_from_attention(
                ws,
                item,
                goals,
                memory_highlights,
                preference_highlights,
                &pending_approvals,
                &pending_plans,
                &overlays,
            )?);
        }

        if let Some(graph) = task_graph {
            for node in graph.open_incomplete_nodes().into_iter().take(4) {
                let key = format!("graph:{}", node.task.id);
                if !seen.insert(key.clone()) {
                    continue;
                }
                candidates.push(Self::candidate_from_graph_node(
                    ws,
                    node,
                    goals,
                    memory_highlights,
                    preference_highlights,
                    &overlays,
                )?);
            }
        }

        // Always surface at least one bootstrap candidate when empty.
        if candidates.is_empty() {
            let key = "bootstrap:define_work";
            seen.insert(key.to_string());
            candidates.push(Self::bootstrap_candidate(
                ws,
                goals,
                memory_highlights,
                preference_highlights,
                &overlays,
            )?);
        }

        // Alternative: goal continuation when goals exist and not already covered.
        if let Some(goal) = goals.first() {
            let key = format!("goal:{}", goal.id);
            if seen.insert(key.clone()) {
                candidates.push(Self::candidate_from_goal(
                    ws,
                    goal,
                    attention,
                    memory_highlights,
                    preference_highlights,
                    &pending_approvals,
                    &overlays,
                )?);
            }
        }

        let (intake_receipts, intake_assessments, intake_eligibilities) =
            Self::project_recommendation_intake_observations(db, ws)?;
        let intake_candidates = Self::sync_intake_candidates(
            db,
            ws,
            &intake_receipts,
            &intake_assessments,
            &intake_eligibilities,
        )?;
        let intake_evaluations = Self::load_intake_evaluations(db, ws)?;
        let intake_dispositions = Self::load_intake_dispositions(db, ws)?;
        let persisted_creations = Self::load_candidate_creations(db, ws)?;
        let created_intake_ids: Vec<String> = persisted_creations
            .iter()
            .filter(|c| c.is_created())
            .map(|c| c.intake_candidate_id.clone())
            .collect();
        let intake_promotion_boundaries = Self::project_intake_promotion_boundaries(
            &intake_candidates,
            &intake_evaluations,
            &intake_dispositions,
            &intake_receipts,
            &intake_eligibilities,
            &created_intake_ids,
        );
        let candidate_creation_requests =
            DecisionEngineCandidateCreationRequest::derive_batch_with_created(
                &intake_promotion_boundaries,
                &created_intake_ids,
            );
        let candidate_creations = Self::project_candidate_creations(
            &intake_candidates,
            &intake_promotion_boundaries,
            &candidate_creation_requests,
            &persisted_creations,
        );
        // Merge intake-created DecisionCandidates without rescoring synthesis path.
        for creation in persisted_creations.iter().filter(|c| c.is_created()) {
            let key = DecisionEngineCandidateCreation::decision_candidate_source_key(
                &creation.recommendation_reference,
            );
            if !seen.insert(key.clone()) {
                continue;
            }
            let outcome = overlays
                .get(&key)
                .copied()
                .unwrap_or(DecisionOutcome::Open);
            candidates.push(
                creation
                    .to_decision_candidate(outcome)
                    .map_err(KernelError::from)?,
            );
        }
        let lifecycle_integrations =
            DecisionCandidateLifecycleIntegration::derive_batch(&candidates);
        let persisted_eval_origins = Self::load_evaluation_origin_contracts(db, ws)?;
        let evaluation_origin_contracts = Self::project_evaluation_origin_contracts(
            &candidates,
            &lifecycle_integrations,
            &persisted_eval_origins,
        );
        let persisted_resolutions = Self::load_evaluation_resolutions(db, ws)?;
        let evaluation_resolutions = Self::project_evaluation_resolutions(
            &candidates,
            &lifecycle_integrations,
            &evaluation_origin_contracts,
            &intake_candidates,
            &persisted_resolutions,
        );
        let candidate_scores = Self::load_candidate_scores(db, ws)?;
        let candidate_ranking = Self::project_candidate_ranking(
            ws,
            &candidates,
            &lifecycle_integrations,
            &intake_candidates,
            &candidate_scores,
        )?;
        let persisted_selections = Self::load_candidate_selections(db, ws)?;
        let candidate_selections = Self::project_candidate_selections(
            &candidates,
            &lifecycle_integrations,
            &intake_candidates,
            &candidate_scores,
            &candidate_ranking,
            &persisted_selections,
        );
        let persisted_progression = Self::load_progression_requests(db, ws)?;
        let progression_requests = Self::project_progression_requests(
            &candidates,
            &lifecycle_integrations,
            &intake_candidates,
            &candidate_selections,
            &persisted_progression,
        );
        let persisted_acks = Self::load_progression_acknowledgements(db, ws)?;
        let progression_acknowledgements = Self::project_progression_acknowledgements(
            &candidates,
            &lifecycle_integrations,
            &intake_candidates,
            &progression_requests,
            &persisted_acks,
        );
        let state = DecisionEngineState::from_candidates(ws, context, candidates)
            .with_intake_receipts(intake_receipts)
            .with_intake_assessments(intake_assessments)
            .with_intake_eligibilities(intake_eligibilities)
            .with_intake_candidates(intake_candidates)
            .with_intake_evaluations(intake_evaluations)
            .with_intake_dispositions(intake_dispositions)
            .with_intake_promotion_boundaries(intake_promotion_boundaries)
            .with_candidate_creation_requests(candidate_creation_requests)
            .with_candidate_creations(candidate_creations)
            .with_lifecycle_integrations(lifecycle_integrations)
            .with_evaluation_origin_contracts(evaluation_origin_contracts)
            .with_evaluation_resolutions(evaluation_resolutions)
            .with_candidate_scores(candidate_scores)
            .with_candidate_ranking(candidate_ranking)
            .with_candidate_selections(candidate_selections)
            .with_progression_requests(progression_requests)
            .with_progression_acknowledgements(progression_acknowledgements);

        // Enforce resilience invariants at runtime (not just debug builds).
        // These invariants are critical to prevent RE/DE boundary violations.
        crate::services::validate_decision_engine_state_integrity(&state)?;

        Self::audit_generated(db, actor, &state)?;
        Self::audit_rank_changes(db, actor, &state, &previous_ranks)?;
        Ok(state)
    }

    /// Read-only observation + assessment + eligibility of accepted RE sealed packages.
    /// Never mutates Recommendation Engine overlays. Never creates candidates.
    fn project_recommendation_intake_observations(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<(
        Vec<DecisionEngineIntakeReceipt>,
        Vec<DecisionEngineIntakeAssessment>,
        Vec<DecisionEngineIntakeEligibility>,
    )> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let overlays =
            RecommendationLifecycleRepository::new(&guard).list_overlays(workspace_id)?;
        drop(guard);
        let mut inputs = Vec::new();
        for overlay in overlays {
            let (Some(acceptance), Some(seal)) = (
                overlay.decision_engine_acceptance.as_ref(),
                overlay.decision_intake_package_seal.as_ref(),
            ) else {
                continue;
            };
            let Some(receipt) = DecisionEngineIntakeReceipt::try_observe(acceptance, seal) else {
                continue;
            };
            crate::services::validate_intake_receipt_observational(&receipt)?;
            let acceptance_active = acceptance.acceptance_state
                == workspace_domain::RecommendationDecisionEngineAcceptance::STATE_ACCEPTED
                && acceptance.revoked_at.is_none()
                && !acceptance.ownership_transferred;
            let handoff_active = overlay
                .decision_handoff_request
                .as_ref()
                .map(|h| h.is_active_request())
                .unwrap_or(false);
            let prep_active = overlay
                .decision_intake_adapter_preparation
                .as_ref()
                .map(|p| p.is_active_preparation())
                .unwrap_or(false);
            let receipt_current =
                acceptance_active && handoff_active && prep_active && receipt.seal_aligned;
            let lifecycle_superseded =
                overlay.lifecycle_state == RecommendationLifecycleState::Superseded
                    || overlay.resolution_type
                        == Some(workspace_domain::RecommendationResolutionType::Superseded);
            inputs.push(DecisionEngineIntakeAssessmentInput {
                receipt,
                acceptance_active,
                lifecycle_superseded,
                receipt_current,
            });
        }
        inputs.sort_by(|a, b| a.receipt.recommendation_id.cmp(&b.receipt.recommendation_id));
        let assessments = DecisionEngineIntakeAssessment::assess_batch(&inputs);
        let receipts: Vec<_> = inputs.into_iter().map(|i| i.receipt).collect();
        let eligibilities =
            DecisionEngineIntakeEligibility::derive_batch(&receipts, &assessments);
        Ok((receipts, assessments, eligibilities))
    }

    /// Materialize DE-owned intake candidates for eligible intakes only.
    /// Never mutates Recommendation Engine overlays. Never creates DecisionCandidates.
    fn sync_intake_candidates(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        receipts: &[DecisionEngineIntakeReceipt],
        assessments: &[DecisionEngineIntakeAssessment],
        eligibilities: &[DecisionEngineIntakeEligibility],
    ) -> Result<Vec<DecisionEngineIntakeCandidate>> {
        let now = Utc::now().to_rfc3339();
        let created = DecisionEngineIntakeCandidate::create_batch(
            receipts,
            assessments,
            eligibilities,
            &now,
        );
        let eligibility_by_rec: HashMap<&str, &DecisionEngineIntakeEligibility> = eligibilities
            .iter()
            .map(|e| (e.recommendation_id.as_str(), e))
            .collect();

        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = DecisionEngineRepository::new(&guard);
        let mut existing = repo.list_intake_candidates(workspace_id)?;

        // Create or refresh eligible intake candidates (digest uniqueness blocks duplicates).
        // Preserve withdrawn/invalidated DE lifecycle — never auto-reactivate invalidated.
        for mut candidate in created {
            if let Some(prior) =
                repo.get_intake_candidate_by_digest(workspace_id, &candidate.package_seal_digest)?
            {
                if prior.recommendation_reference != candidate.recommendation_reference {
                    // Duplicate seal digest already acknowledged for another recommendation.
                    continue;
                }
                candidate.created_at = prior.created_at;
                if prior.lifecycle.is_invalidated() || prior.lifecycle.is_withdrawn() {
                    candidate.lifecycle = prior.lifecycle;
                    candidate.state = prior.state;
                } else {
                    candidate.state = DecisionEngineIntakeCandidate::STATE_READY.into();
                }
            }
            crate::services::validate_intake_candidate_phase(&candidate)?;
            repo.upsert_intake_candidate(&candidate)?;
        }

        // Reevaluate previously stored intake candidates against current eligibility/acceptance.
        let overlays =
            RecommendationLifecycleRepository::new(&guard).list_overlays(workspace_id)?;
        let acceptance_by_rec: HashMap<String, String> = overlays
            .into_iter()
            .filter_map(|o| {
                o.decision_engine_acceptance
                    .map(|a| (a.recommendation_id, a.acceptance_state))
            })
            .collect();

        existing = repo.list_intake_candidates(workspace_id)?;
        for mut stored in existing {
            let eligibility = eligibility_by_rec
                .get(stored.recommendation_reference.as_str())
                .copied();
            let acceptance_state = acceptance_by_rec
                .get(&stored.recommendation_reference)
                .map(String::as_str);
            stored.apply_source_reevaluation(eligibility, acceptance_state, &now);
            repo.upsert_intake_candidate(&stored)?;
        }

        let mut out = repo.list_intake_candidates(workspace_id)?;
        drop(guard);
        out.sort_by(|a, b| a.intake_candidate_id.cmp(&b.intake_candidate_id));
        Ok(out)
    }

    fn load_intake_evaluations(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<DecisionEngineIntakeEvaluation>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let mut evaluations =
            DecisionEngineRepository::new(&guard).list_intake_evaluations(workspace_id)?;
        drop(guard);
        evaluations.sort_by(|a, b| a.evaluation_id.cmp(&b.evaluation_id));
        Ok(evaluations)
    }

    /// Persist a DE-owned intake evaluation for an active intake candidate only.
    /// Never mutates Recommendation Engine overlays. Never creates DecisionCandidates.
    pub(crate) fn evaluate_intake_candidate(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
        intake_candidate_id: impl Into<String>,
        evaluation_state: impl Into<String>,
        evaluation_reason: impl Into<String>,
    ) -> Result<DecisionEngineIntakeEvaluation> {
        let workspace_id = workspace_id.into();
        let intake_candidate_id = intake_candidate_id.into();
        let evaluation_state = evaluation_state.into();
        let evaluation_reason = evaluation_reason.into();
        let now = Utc::now().to_rfc3339();

        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = DecisionEngineRepository::new(&guard);
        let candidate = repo
            .get_intake_candidate(&workspace_id, &intake_candidate_id)?
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let evaluation = DecisionEngineIntakeEvaluation::try_evaluate(
            &candidate,
            evaluation_state,
            evaluation_reason,
            now,
        )
        .map_err(KernelError::from)?;
        crate::services::validate_intake_evaluation_phase(&evaluation)?;
        repo.upsert_intake_evaluation(&evaluation)?;
        drop(guard);
        Ok(evaluation)
    }

    fn load_intake_dispositions(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<DecisionEngineIntakeDisposition>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let mut dispositions =
            DecisionEngineRepository::new(&guard).list_intake_dispositions(workspace_id)?;
        drop(guard);
        dispositions.sort_by(|a, b| a.disposition_id.cmp(&b.disposition_id));
        Ok(dispositions)
    }

    /// Project promotion readiness from intake aggregates + seal/acceptance facts.
    /// Never creates DecisionCandidates. Never mutates Recommendation Engine overlays.
    fn project_intake_promotion_boundaries(
        candidates: &[DecisionEngineIntakeCandidate],
        evaluations: &[DecisionEngineIntakeEvaluation],
        dispositions: &[DecisionEngineIntakeDisposition],
        receipts: &[DecisionEngineIntakeReceipt],
        eligibilities: &[DecisionEngineIntakeEligibility],
        created_intake_ids: &[String],
    ) -> Vec<DecisionEngineIntakePromotionBoundary> {
        let evaluation_by_id: HashMap<&str, &DecisionEngineIntakeEvaluation> = evaluations
            .iter()
            .map(|e| (e.intake_candidate_id.as_str(), e))
            .collect();
        let disposition_by_id: HashMap<&str, &DecisionEngineIntakeDisposition> = dispositions
            .iter()
            .map(|d| (d.intake_candidate_id.as_str(), d))
            .collect();
        let receipt_by_rec: HashMap<&str, &DecisionEngineIntakeReceipt> = receipts
            .iter()
            .map(|r| (r.recommendation_id.as_str(), r))
            .collect();
        let eligibility_by_rec: HashMap<&str, &DecisionEngineIntakeEligibility> = eligibilities
            .iter()
            .map(|e| (e.recommendation_id.as_str(), e))
            .collect();
        let created: std::collections::HashSet<&str> =
            created_intake_ids.iter().map(|s| s.as_str()).collect();

        let inputs: Vec<DecisionEngineIntakePromotionBoundaryInput> = candidates
            .iter()
            .map(|candidate| {
                let rec = candidate.recommendation_reference.as_str();
                let eligibility = eligibility_by_rec.get(rec).copied();
                let receipt = receipt_by_rec.get(rec).copied();
                let acceptance_active = eligibility
                    .map(|e| e.acceptance_active)
                    .unwrap_or(false);
                let seal_aligned = eligibility
                    .map(|e| e.seal_aligned)
                    .or_else(|| receipt.map(|r| r.seal_aligned && r.is_observed()))
                    .unwrap_or(false);
                DecisionEngineIntakePromotionBoundaryInput {
                    candidate: candidate.clone(),
                    evaluation: evaluation_by_id
                        .get(candidate.intake_candidate_id.as_str())
                        .copied()
                        .cloned(),
                    disposition: disposition_by_id
                        .get(candidate.intake_candidate_id.as_str())
                        .copied()
                        .cloned(),
                    acceptance_active,
                    seal_aligned,
                    previously_promoted: created.contains(candidate.intake_candidate_id.as_str()),
                }
            })
            .collect();
        DecisionEngineIntakePromotionBoundary::derive_batch(&inputs)
    }

    fn load_candidate_creations(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<DecisionEngineCandidateCreation>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let mut creations =
            DecisionEngineRepository::new(&guard).list_candidate_creations(workspace_id)?;
        drop(guard);
        creations.sort_by(|a, b| a.creation_id.cmp(&b.creation_id));
        Ok(creations)
    }

    fn project_candidate_creations(
        intake_candidates: &[DecisionEngineIntakeCandidate],
        boundaries: &[DecisionEngineIntakePromotionBoundary],
        requests: &[DecisionEngineCandidateCreationRequest],
        persisted: &[DecisionEngineCandidateCreation],
    ) -> Vec<DecisionEngineCandidateCreation> {
        let boundary_by_id: HashMap<&str, &DecisionEngineIntakePromotionBoundary> = boundaries
            .iter()
            .map(|b| (b.intake_candidate_id.as_str(), b))
            .collect();
        let request_by_id: HashMap<&str, &DecisionEngineCandidateCreationRequest> = requests
            .iter()
            .map(|r| (r.intake_candidate_id.as_str(), r))
            .collect();
        let persisted_by_id: HashMap<&str, &DecisionEngineCandidateCreation> = persisted
            .iter()
            .map(|c| (c.intake_candidate_id.as_str(), c))
            .collect();

        let inputs: Vec<DecisionEngineCandidateCreationInput> = intake_candidates
            .iter()
            .filter_map(|intake| {
                let id = intake.intake_candidate_id.as_str();
                let boundary = boundary_by_id.get(id).copied()?;
                let request = request_by_id.get(id).copied()?;
                Some(DecisionEngineCandidateCreationInput {
                    creation_request: request.clone(),
                    intake_candidate: intake.clone(),
                    promotion_boundary: boundary.clone(),
                    package_seal_digest: intake.package_seal_digest.clone(),
                    existing_creation: persisted_by_id.get(id).copied().cloned(),
                })
            })
            .collect();
        DecisionEngineCandidateCreation::derive_batch(&inputs)
    }

    fn load_evaluation_origin_contracts(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<DecisionCandidateEvaluationOriginContract>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let mut contracts =
            DecisionEngineRepository::new(&guard).list_evaluation_origin_contracts(workspace_id)?;
        drop(guard);
        contracts.sort_by(|a, b| a.evaluation_id.cmp(&b.evaluation_id));
        Ok(contracts)
    }

    fn project_evaluation_origin_contracts(
        candidates: &[DecisionCandidate],
        integrations: &[DecisionCandidateLifecycleIntegration],
        persisted: &[DecisionCandidateEvaluationOriginContract],
    ) -> Vec<DecisionCandidateEvaluationOriginContract> {
        let integration_by_id: HashMap<&str, &DecisionCandidateLifecycleIntegration> = integrations
            .iter()
            .map(|i| (i.decision_candidate_id.as_str(), i))
            .collect();
        let persisted_by_id: HashMap<&str, &DecisionCandidateEvaluationOriginContract> = persisted
            .iter()
            .map(|c| (c.decision_candidate_id.as_str(), c))
            .collect();
        let inputs: Vec<DecisionCandidateEvaluationOriginInput> = candidates
            .iter()
            .filter_map(|candidate| {
                let id = candidate.id.as_str();
                let integration = integration_by_id.get(id).copied()?;
                Some(DecisionCandidateEvaluationOriginInput {
                    candidate: candidate.clone(),
                    lifecycle_integration: integration.clone(),
                    existing_evaluated: persisted_by_id.get(id).copied().cloned(),
                })
            })
            .collect();
        DecisionCandidateEvaluationOriginContract::derive_batch(&inputs)
    }

    fn load_evaluation_resolutions(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<DecisionCandidateEvaluationResolution>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let mut resolutions =
            DecisionEngineRepository::new(&guard).list_evaluation_resolutions(workspace_id)?;
        drop(guard);
        resolutions.sort_by(|a, b| a.resolution_id.cmp(&b.resolution_id));
        Ok(resolutions)
    }

    fn project_evaluation_resolutions(
        candidates: &[DecisionCandidate],
        integrations: &[DecisionCandidateLifecycleIntegration],
        evaluations: &[DecisionCandidateEvaluationOriginContract],
        intake_candidates: &[DecisionEngineIntakeCandidate],
        persisted: &[DecisionCandidateEvaluationResolution],
    ) -> Vec<DecisionCandidateEvaluationResolution> {
        let integration_by_id: HashMap<&str, &DecisionCandidateLifecycleIntegration> = integrations
            .iter()
            .map(|i| (i.decision_candidate_id.as_str(), i))
            .collect();
        let evaluation_by_id: HashMap<&str, &DecisionCandidateEvaluationOriginContract> =
            evaluations
                .iter()
                .map(|e| (e.decision_candidate_id.as_str(), e))
                .collect();
        let intake_by_id: HashMap<&str, &DecisionEngineIntakeCandidate> = intake_candidates
            .iter()
            .map(|c| (c.intake_candidate_id.as_str(), c))
            .collect();
        let persisted_by_id: HashMap<&str, &DecisionCandidateEvaluationResolution> = persisted
            .iter()
            .map(|r| (r.decision_candidate_id.as_str(), r))
            .collect();
        let inputs: Vec<DecisionCandidateEvaluationResolutionInput> = candidates
            .iter()
            .filter_map(|candidate| {
                let id = candidate.id.as_str();
                let integration = integration_by_id.get(id).copied()?;
                let evaluation = evaluation_by_id.get(id).copied()?;
                let intake_withdrawn_or_invalidated = candidate
                    .intake_candidate_id
                    .as_deref()
                    .and_then(|intake_id| intake_by_id.get(intake_id).copied())
                    .is_some_and(|intake| {
                        intake.lifecycle.is_withdrawn() || intake.lifecycle.is_invalidated()
                    });
                Some(DecisionCandidateEvaluationResolutionInput {
                    candidate: candidate.clone(),
                    lifecycle_integration: integration.clone(),
                    evaluation_origin: evaluation.clone(),
                    intake_withdrawn_or_invalidated,
                    existing_resolution: persisted_by_id.get(id).copied().cloned(),
                })
            })
            .collect();
        DecisionCandidateEvaluationResolution::derive_batch(&inputs)
    }

    /// Resolve evaluation toward scoring-path admission without creating scores.
    pub(crate) fn resolve_decision_candidate_evaluation(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        candidate_id: impl Into<String>,
        resolution: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<(DecisionCandidateEvaluationResolution, DecisionCandidate)> {
        let workspace_id = workspace_id.into();
        let candidate_id = candidate_id.into();
        let resolution = resolution.into();
        let reason = reason.into();
        let now = Utc::now().to_rfc3339();
        let state = Self::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let candidate = state
            .candidates
            .iter()
            .find(|c| c.id.as_str() == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let integration = state
            .lifecycle_integrations
            .iter()
            .find(|i| i.decision_candidate_id == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let evaluation = state
            .evaluation_origin_contracts
            .iter()
            .find(|e| e.decision_candidate_id == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let existing = state
            .evaluation_resolutions
            .iter()
            .find(|r| {
                r.decision_candidate_id == candidate_id
                    && (r.is_accepted_for_scoring() || r.is_rejected_for_scoring())
            })
            .cloned();
        let intake_withdrawn_or_invalidated = candidate
            .intake_candidate_id
            .as_deref()
            .and_then(|intake_id| {
                state
                    .intake_candidates
                    .iter()
                    .find(|c| c.intake_candidate_id == intake_id)
            })
            .is_some_and(|intake| {
                intake.lifecycle.is_withdrawn() || intake.lifecycle.is_invalidated()
            });
        let input = DecisionCandidateEvaluationResolutionInput {
            candidate: candidate.clone(),
            lifecycle_integration: integration,
            evaluation_origin: evaluation,
            intake_withdrawn_or_invalidated,
            existing_resolution: existing,
        };
        let (resolved, unchanged) =
            DecisionCandidateEvaluationResolution::try_resolve(&input, resolution, reason, now)
                .map_err(KernelError::from)?;
        crate::services::validate_evaluation_resolution_only(&resolved)?;
        crate::services::validate_candidate_score_unchanged(&candidate, &unchanged)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        DecisionEngineRepository::new(&guard).upsert_evaluation_resolution(&resolved)?;
        drop(guard);
        Ok((resolved, unchanged))
    }

    /// Create a DE-owned DecisionScore for an accepted_for_scoring candidate.
    /// Never ranks, selects, plans, executes, or mutates Recommendation Engine.
    pub(crate) fn score_decision_candidate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        candidate_id: impl Into<String>,
    ) -> Result<(DecisionCandidateScore, DecisionCandidate)> {
        let workspace_id = workspace_id.into();
        let candidate_id = candidate_id.into();
        let now = Utc::now().to_rfc3339();
        let state = Self::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let candidate = state
            .candidates
            .iter()
            .find(|c| c.id.as_str() == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let integration = state
            .lifecycle_integrations
            .iter()
            .find(|i| i.decision_candidate_id == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let resolution = state
            .evaluation_resolutions
            .iter()
            .find(|r| r.decision_candidate_id == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let existing = state
            .candidate_scores
            .iter()
            .find(|s| s.decision_candidate_id == candidate_id)
            .cloned();
        let intake_withdrawn_or_invalidated = candidate
            .intake_candidate_id
            .as_deref()
            .and_then(|intake_id| {
                state
                    .intake_candidates
                    .iter()
                    .find(|c| c.intake_candidate_id == intake_id)
            })
            .is_some_and(|intake| {
                intake.lifecycle.is_withdrawn() || intake.lifecycle.is_invalidated()
            });
        let input = DecisionCandidateScoreInput {
            candidate: candidate.clone(),
            resolution,
            lifecycle_integration: integration,
            intake_withdrawn_or_invalidated,
            existing_score: existing,
        };
        let (score, unchanged) =
            DecisionCandidateScore::try_create(&input, now).map_err(KernelError::from)?;
        crate::services::validate_candidate_score_only(&score)?;
        crate::services::validate_candidate_score_outcome_unchanged(&candidate, &unchanged)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        DecisionEngineRepository::new(&guard).upsert_candidate_score(&score)?;
        drop(guard);
        Ok((score, unchanged))
    }

    /// Resolve DE-owned selection after ranking — never executes or planner-handoffs.
    pub(crate) fn resolve_decision_candidate_selection(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        candidate_id: impl Into<String>,
        action: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<(DecisionCandidateSelection, DecisionCandidate)> {
        let workspace_id = workspace_id.into();
        let candidate_id = candidate_id.into();
        let action = action.into();
        let reason = reason.into();
        let now = Utc::now().to_rfc3339();
        let state = Self::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let candidate = state
            .candidates
            .iter()
            .find(|c| c.id.as_str() == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let integration = state
            .lifecycle_integrations
            .iter()
            .find(|i| i.decision_candidate_id == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let score = state
            .candidate_scores
            .iter()
            .find(|s| s.decision_candidate_id == candidate_id)
            .cloned();
        let ranking = state.candidate_ranking.clone();
        let ranking_entry = ranking.as_ref().and_then(|r| {
            r.entries
                .iter()
                .find(|e| e.decision_candidate_id == candidate_id)
                .cloned()
        });
        let existing = state
            .candidate_selections
            .iter()
            .find(|s| {
                s.decision_candidate_id == candidate_id && (s.is_selected() || s.is_rejected())
            })
            .cloned();
        let intake_withdrawn_or_invalidated = candidate
            .intake_candidate_id
            .as_deref()
            .and_then(|intake_id| {
                state
                    .intake_candidates
                    .iter()
                    .find(|c| c.intake_candidate_id == intake_id)
            })
            .is_some_and(|intake| {
                intake.lifecycle.is_withdrawn() || intake.lifecycle.is_invalidated()
            });
        let input = DecisionCandidateSelectionInput {
            candidate: candidate.clone(),
            score,
            ranking,
            ranking_entry,
            lifecycle_integration: integration,
            intake_withdrawn_or_invalidated,
            existing_selection: existing,
        };
        let (selection, unchanged) =
            DecisionCandidateSelection::try_select(&input, action, reason, now)
                .map_err(KernelError::from)?;
        crate::services::validate_candidate_selection_bounded(&selection)?;
        crate::services::validate_candidate_score_outcome_unchanged(&candidate, &unchanged)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        DecisionEngineRepository::new(&guard).upsert_candidate_selection(&selection)?;
        drop(guard);
        Ok((selection, unchanged))
    }

    /// Issue a DE-owned progression request after selection — never planner/execution.
    pub(crate) fn request_decision_candidate_progression(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        candidate_id: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<(DecisionCandidateProgressionRequest, DecisionCandidate)> {
        let workspace_id = workspace_id.into();
        let candidate_id = candidate_id.into();
        let reason = reason.into();
        let now = Utc::now().to_rfc3339();
        let state = Self::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let candidate = state
            .candidates
            .iter()
            .find(|c| c.id.as_str() == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let integration = state
            .lifecycle_integrations
            .iter()
            .find(|i| i.decision_candidate_id == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let selection = state
            .candidate_selections
            .iter()
            .find(|s| s.decision_candidate_id == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let existing = state
            .progression_requests
            .iter()
            .find(|r| {
                r.decision_candidate_id == candidate_id && (r.is_requested() || r.is_cancelled())
            })
            .cloned();
        let intake_withdrawn_or_invalidated = candidate
            .intake_candidate_id
            .as_deref()
            .and_then(|intake_id| {
                state
                    .intake_candidates
                    .iter()
                    .find(|c| c.intake_candidate_id == intake_id)
            })
            .is_some_and(|intake| {
                intake.lifecycle.is_withdrawn() || intake.lifecycle.is_invalidated()
            });
        let input = DecisionCandidateProgressionRequestInput {
            candidate: candidate.clone(),
            selection,
            lifecycle_integration: integration,
            intake_withdrawn_or_invalidated,
            existing_request: existing,
        };
        let (request, unchanged) =
            DecisionCandidateProgressionRequest::try_request(&input, reason, now)
                .map_err(KernelError::from)?;
        crate::services::validate_progression_request_bounded(&request)?;
        crate::services::validate_candidate_score_outcome_unchanged(&candidate, &unchanged)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        DecisionEngineRepository::new(&guard).upsert_progression_request(&request)?;
        drop(guard);
        Ok((request, unchanged))
    }

    /// Acknowledge a DE-owned progression request — never planner/execution.
    pub(crate) fn acknowledge_decision_candidate_progression(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        candidate_id: impl Into<String>,
        action: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<(DecisionCandidateProgressionAcknowledgement, DecisionCandidate)> {
        let workspace_id = workspace_id.into();
        let candidate_id = candidate_id.into();
        let action = action.into();
        let reason = reason.into();
        let now = Utc::now().to_rfc3339();
        let state = Self::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let candidate = state
            .candidates
            .iter()
            .find(|c| c.id.as_str() == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let integration = state
            .lifecycle_integrations
            .iter()
            .find(|i| i.decision_candidate_id == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let progression_request = state
            .progression_requests
            .iter()
            .find(|r| r.decision_candidate_id == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let existing = state
            .progression_acknowledgements
            .iter()
            .find(|a| {
                a.decision_candidate_id == candidate_id
                    && (a.is_acknowledged() || a.is_rejected())
            })
            .cloned();
        let intake_withdrawn_or_invalidated = candidate
            .intake_candidate_id
            .as_deref()
            .and_then(|intake_id| {
                state
                    .intake_candidates
                    .iter()
                    .find(|c| c.intake_candidate_id == intake_id)
            })
            .is_some_and(|intake| {
                intake.lifecycle.is_withdrawn() || intake.lifecycle.is_invalidated()
            });
        let input = DecisionCandidateProgressionAcknowledgementInput {
            candidate: candidate.clone(),
            progression_request,
            lifecycle_integration: integration,
            intake_withdrawn_or_invalidated,
            existing_acknowledgement: existing,
        };
        let (acknowledgement, unchanged) =
            DecisionCandidateProgressionAcknowledgement::try_acknowledge(
                &input, action, reason, now,
            )
            .map_err(KernelError::from)?;
        crate::services::validate_progression_acknowledgement_only(&acknowledgement)?;
        crate::services::validate_candidate_score_outcome_unchanged(&candidate, &unchanged)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        DecisionEngineRepository::new(&guard)
            .upsert_progression_acknowledgement(&acknowledgement)?;
        drop(guard);
        Ok((acknowledgement, unchanged))
    }

    fn load_candidate_scores(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<DecisionCandidateScore>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let mut scores =
            DecisionEngineRepository::new(&guard).list_candidate_scores(workspace_id)?;
        drop(guard);
        scores.sort_by(|a, b| a.score_id.cmp(&b.score_id));
        Ok(scores)
    }

    /// Project DE-owned comparative ranking from scored candidates — never selects.
    fn project_candidate_ranking(
        workspace_id: &str,
        candidates: &[DecisionCandidate],
        integrations: &[DecisionCandidateLifecycleIntegration],
        intake_candidates: &[DecisionEngineIntakeCandidate],
        scores: &[DecisionCandidateScore],
    ) -> Result<DecisionCandidateRanking> {
        let integration_by_id: HashMap<&str, &DecisionCandidateLifecycleIntegration> = integrations
            .iter()
            .map(|i| (i.decision_candidate_id.as_str(), i))
            .collect();
        let score_by_id: HashMap<&str, &DecisionCandidateScore> = scores
            .iter()
            .map(|s| (s.decision_candidate_id.as_str(), s))
            .collect();
        let intake_by_id: HashMap<&str, &DecisionEngineIntakeCandidate> = intake_candidates
            .iter()
            .map(|c| (c.intake_candidate_id.as_str(), c))
            .collect();
        let inputs: Vec<DecisionCandidateRankingMemberInput> = candidates
            .iter()
            .filter_map(|candidate| {
                let id = candidate.id.as_str();
                let integration = integration_by_id.get(id).copied()?;
                let intake_withdrawn_or_invalidated = candidate
                    .intake_candidate_id
                    .as_deref()
                    .and_then(|intake_id| intake_by_id.get(intake_id).copied())
                    .is_some_and(|intake| {
                        intake.lifecycle.is_withdrawn() || intake.lifecycle.is_invalidated()
                    });
                Some(DecisionCandidateRankingMemberInput {
                    candidate: candidate.clone(),
                    score: score_by_id.get(id).copied().cloned(),
                    lifecycle_integration: integration.clone(),
                    intake_withdrawn_or_invalidated,
                })
            })
            .collect();
        let ranking = DecisionCandidateRanking::derive(
            workspace_id,
            Utc::now().to_rfc3339(),
            &inputs,
        );
        crate::services::validate_candidate_ranking_only(&ranking)?;
        Ok(ranking)
    }

    fn load_candidate_selections(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<DecisionCandidateSelection>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let mut selections =
            DecisionEngineRepository::new(&guard).list_candidate_selections(workspace_id)?;
        drop(guard);
        selections.sort_by(|a, b| a.selection_id.cmp(&b.selection_id));
        Ok(selections)
    }

    fn project_candidate_selections(
        candidates: &[DecisionCandidate],
        integrations: &[DecisionCandidateLifecycleIntegration],
        intake_candidates: &[DecisionEngineIntakeCandidate],
        scores: &[DecisionCandidateScore],
        ranking: &DecisionCandidateRanking,
        persisted: &[DecisionCandidateSelection],
    ) -> Vec<DecisionCandidateSelection> {
        let integration_by_id: HashMap<&str, &DecisionCandidateLifecycleIntegration> = integrations
            .iter()
            .map(|i| (i.decision_candidate_id.as_str(), i))
            .collect();
        let score_by_id: HashMap<&str, &DecisionCandidateScore> = scores
            .iter()
            .map(|s| (s.decision_candidate_id.as_str(), s))
            .collect();
        let entry_by_id: HashMap<&str, &workspace_domain::DecisionCandidateRankingEntry> = ranking
            .entries
            .iter()
            .map(|e| (e.decision_candidate_id.as_str(), e))
            .collect();
        let intake_by_id: HashMap<&str, &DecisionEngineIntakeCandidate> = intake_candidates
            .iter()
            .map(|c| (c.intake_candidate_id.as_str(), c))
            .collect();
        let persisted_by_id: HashMap<&str, &DecisionCandidateSelection> = persisted
            .iter()
            .map(|s| (s.decision_candidate_id.as_str(), s))
            .collect();
        let inputs: Vec<DecisionCandidateSelectionInput> = candidates
            .iter()
            .filter_map(|candidate| {
                let id = candidate.id.as_str();
                let integration = integration_by_id.get(id).copied()?;
                let intake_withdrawn_or_invalidated = candidate
                    .intake_candidate_id
                    .as_deref()
                    .and_then(|intake_id| intake_by_id.get(intake_id).copied())
                    .is_some_and(|intake| {
                        intake.lifecycle.is_withdrawn() || intake.lifecycle.is_invalidated()
                    });
                Some(DecisionCandidateSelectionInput {
                    candidate: candidate.clone(),
                    score: score_by_id.get(id).copied().cloned(),
                    ranking: Some(ranking.clone()),
                    ranking_entry: entry_by_id.get(id).copied().cloned(),
                    lifecycle_integration: integration.clone(),
                    intake_withdrawn_or_invalidated,
                    existing_selection: persisted_by_id.get(id).copied().cloned(),
                })
            })
            .collect();
        DecisionCandidateSelection::derive_batch(&inputs)
    }

    fn load_progression_requests(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<DecisionCandidateProgressionRequest>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let mut requests =
            DecisionEngineRepository::new(&guard).list_progression_requests(workspace_id)?;
        drop(guard);
        requests.sort_by(|a, b| a.request_id.cmp(&b.request_id));
        Ok(requests)
    }

    fn project_progression_requests(
        candidates: &[DecisionCandidate],
        integrations: &[DecisionCandidateLifecycleIntegration],
        intake_candidates: &[DecisionEngineIntakeCandidate],
        selections: &[DecisionCandidateSelection],
        persisted: &[DecisionCandidateProgressionRequest],
    ) -> Vec<DecisionCandidateProgressionRequest> {
        let integration_by_id: HashMap<&str, &DecisionCandidateLifecycleIntegration> = integrations
            .iter()
            .map(|i| (i.decision_candidate_id.as_str(), i))
            .collect();
        let selection_by_id: HashMap<&str, &DecisionCandidateSelection> = selections
            .iter()
            .map(|s| (s.decision_candidate_id.as_str(), s))
            .collect();
        let intake_by_id: HashMap<&str, &DecisionEngineIntakeCandidate> = intake_candidates
            .iter()
            .map(|c| (c.intake_candidate_id.as_str(), c))
            .collect();
        let persisted_by_id: HashMap<&str, &DecisionCandidateProgressionRequest> = persisted
            .iter()
            .map(|r| (r.decision_candidate_id.as_str(), r))
            .collect();
        let inputs: Vec<DecisionCandidateProgressionRequestInput> = candidates
            .iter()
            .filter_map(|candidate| {
                let id = candidate.id.as_str();
                let integration = integration_by_id.get(id).copied()?;
                let selection = selection_by_id.get(id).copied()?;
                let intake_withdrawn_or_invalidated = candidate
                    .intake_candidate_id
                    .as_deref()
                    .and_then(|intake_id| intake_by_id.get(intake_id).copied())
                    .is_some_and(|intake| {
                        intake.lifecycle.is_withdrawn() || intake.lifecycle.is_invalidated()
                    });
                Some(DecisionCandidateProgressionRequestInput {
                    candidate: candidate.clone(),
                    selection: selection.clone(),
                    lifecycle_integration: integration.clone(),
                    intake_withdrawn_or_invalidated,
                    existing_request: persisted_by_id.get(id).copied().cloned(),
                })
            })
            .collect();
        DecisionCandidateProgressionRequest::derive_batch(&inputs)
    }

    fn load_progression_acknowledgements(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<Vec<DecisionCandidateProgressionAcknowledgement>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let mut acks =
            DecisionEngineRepository::new(&guard).list_progression_acknowledgements(workspace_id)?;
        drop(guard);
        acks.sort_by(|a, b| a.acknowledgement_id.cmp(&b.acknowledgement_id));
        Ok(acks)
    }

    fn project_progression_acknowledgements(
        candidates: &[DecisionCandidate],
        integrations: &[DecisionCandidateLifecycleIntegration],
        intake_candidates: &[DecisionEngineIntakeCandidate],
        requests: &[DecisionCandidateProgressionRequest],
        persisted: &[DecisionCandidateProgressionAcknowledgement],
    ) -> Vec<DecisionCandidateProgressionAcknowledgement> {
        let integration_by_id: HashMap<&str, &DecisionCandidateLifecycleIntegration> = integrations
            .iter()
            .map(|i| (i.decision_candidate_id.as_str(), i))
            .collect();
        let request_by_id: HashMap<&str, &DecisionCandidateProgressionRequest> = requests
            .iter()
            .map(|r| (r.decision_candidate_id.as_str(), r))
            .collect();
        let intake_by_id: HashMap<&str, &DecisionEngineIntakeCandidate> = intake_candidates
            .iter()
            .map(|c| (c.intake_candidate_id.as_str(), c))
            .collect();
        let persisted_by_id: HashMap<&str, &DecisionCandidateProgressionAcknowledgement> = persisted
            .iter()
            .map(|a| (a.decision_candidate_id.as_str(), a))
            .collect();
        let inputs: Vec<DecisionCandidateProgressionAcknowledgementInput> = candidates
            .iter()
            .filter_map(|candidate| {
                let id = candidate.id.as_str();
                let integration = integration_by_id.get(id).copied()?;
                let progression_request = request_by_id.get(id).copied()?;
                let intake_withdrawn_or_invalidated = candidate
                    .intake_candidate_id
                    .as_deref()
                    .and_then(|intake_id| intake_by_id.get(intake_id).copied())
                    .is_some_and(|intake| {
                        intake.lifecycle.is_withdrawn() || intake.lifecycle.is_invalidated()
                    });
                Some(DecisionCandidateProgressionAcknowledgementInput {
                    candidate: candidate.clone(),
                    progression_request: progression_request.clone(),
                    lifecycle_integration: integration.clone(),
                    intake_withdrawn_or_invalidated,
                    existing_acknowledgement: persisted_by_id.get(id).copied().cloned(),
                })
            })
            .collect();
        DecisionCandidateProgressionAcknowledgement::derive_batch(&inputs)
    }

    /// Acknowledge origin evaluation contract for a DecisionCandidate.
    /// Never scores, ranks, plans, or mutates Recommendation Engine overlays.
    pub(crate) fn evaluate_decision_candidate_origin(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        candidate_id: impl Into<String>,
    ) -> Result<(DecisionCandidateEvaluationOriginContract, DecisionCandidate)> {
        let workspace_id = workspace_id.into();
        let candidate_id = candidate_id.into();
        let now = Utc::now().to_rfc3339();
        let state = Self::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let candidate = state
            .candidates
            .iter()
            .find(|c| c.id.as_str() == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let integration = state
            .lifecycle_integrations
            .iter()
            .find(|i| i.decision_candidate_id == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let existing = state
            .evaluation_origin_contracts
            .iter()
            .find(|c| c.decision_candidate_id == candidate_id && c.is_evaluated())
            .cloned();
        let input = DecisionCandidateEvaluationOriginInput {
            candidate: candidate.clone(),
            lifecycle_integration: integration,
            existing_evaluated: existing,
        };
        let (contract, unchanged) =
            DecisionCandidateEvaluationOriginContract::try_evaluate(&input, now)
                .map_err(KernelError::from)?;
        crate::services::validate_evaluation_origin_contract_only(&contract)?;
        crate::services::validate_candidate_score_unchanged(&candidate, &unchanged)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        DecisionEngineRepository::new(&guard).upsert_evaluation_origin_contract(&contract)?;
        drop(guard);
        Ok((contract, unchanged))
    }

    /// Persist DE-owned DecisionCandidate creation from an eligible creation request.
    /// Never scores, plans, grants, or mutates Recommendation Engine overlays.
    pub(crate) fn create_decision_candidate_from_intake(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
        intake_candidate_id: impl Into<String>,
    ) -> Result<(DecisionEngineCandidateCreation, DecisionCandidate)> {
        let workspace_id = workspace_id.into();
        let intake_candidate_id = intake_candidate_id.into();
        let now = Utc::now().to_rfc3339();

        let (intake_receipts, intake_assessments, intake_eligibilities) =
            Self::project_recommendation_intake_observations(db, &workspace_id)?;
        let intake_candidates = Self::sync_intake_candidates(
            db,
            &workspace_id,
            &intake_receipts,
            &intake_assessments,
            &intake_eligibilities,
        )?;
        let intake_evaluations = Self::load_intake_evaluations(db, &workspace_id)?;
        let intake_dispositions = Self::load_intake_dispositions(db, &workspace_id)?;
        let persisted_creations = Self::load_candidate_creations(db, &workspace_id)?;
        let created_intake_ids: Vec<String> = persisted_creations
            .iter()
            .filter(|c| c.is_created())
            .map(|c| c.intake_candidate_id.clone())
            .collect();
        let boundaries = Self::project_intake_promotion_boundaries(
            &intake_candidates,
            &intake_evaluations,
            &intake_dispositions,
            &intake_receipts,
            &intake_eligibilities,
            &created_intake_ids,
        );
        let requests = DecisionEngineCandidateCreationRequest::derive_batch_with_created(
            &boundaries,
            &created_intake_ids,
        );

        let intake = intake_candidates
            .iter()
            .find(|c| c.intake_candidate_id == intake_candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let package_seal_digest = intake.package_seal_digest.clone();
        let boundary = boundaries
            .iter()
            .find(|b| b.intake_candidate_id == intake_candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let request = requests
            .iter()
            .find(|r| r.intake_candidate_id == intake_candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let existing = persisted_creations
            .iter()
            .find(|c| c.intake_candidate_id == intake_candidate_id)
            .cloned();

        let input = DecisionEngineCandidateCreationInput {
            creation_request: request,
            intake_candidate: intake,
            promotion_boundary: boundary,
            package_seal_digest,
            existing_creation: existing,
        };

        let (creation, candidate) =
            DecisionEngineCandidateCreation::try_create(&input, now).map_err(KernelError::from)?;
        crate::services::validate_candidate_creation_bounded(&creation)?;
        crate::services::validate_new_intake_candidate_bootstrap(&candidate)?;

        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = DecisionEngineRepository::new(&guard);
        repo.upsert_candidate_creation(&creation)?;
        repo.upsert_overlay(&DecisionEngineOverlay {
            workspace_id: workspace_id.clone(),
            candidate_key: Self::candidate_key(&candidate),
            outcome: candidate.outcome,
            updated_at: creation.created_at.clone().unwrap_or_else(|| Utc::now().to_rfc3339()),
            actor_id: "decision_engine".into(),
        })?;
        drop(guard);
        Ok((creation, candidate))
    }

    /// Persist a DE-owned intake disposition for an active evaluated intake only.
    /// Never mutates Recommendation Engine overlays. Never creates DecisionCandidates.
    pub(crate) fn dispose_intake_candidate(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
        intake_candidate_id: impl Into<String>,
        disposition_state: impl Into<String>,
        disposition_reason: impl Into<String>,
    ) -> Result<DecisionEngineIntakeDisposition> {
        let workspace_id = workspace_id.into();
        let intake_candidate_id = intake_candidate_id.into();
        let disposition_state = disposition_state.into();
        let disposition_reason = disposition_reason.into();
        let now = Utc::now().to_rfc3339();

        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = DecisionEngineRepository::new(&guard);
        let candidate = repo
            .get_intake_candidate(&workspace_id, &intake_candidate_id)?
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;
        let evaluation = repo
            .get_intake_evaluation_for_candidate(&workspace_id, &intake_candidate_id)?
            .ok_or_else(|| {
                KernelError::from(DecisionEngineError::InvalidTransition {
                    from: "unevaluated".into(),
                    to: "dispose".into(),
                })
            })?;
        let disposition = DecisionEngineIntakeDisposition::try_dispose(
            &candidate,
            &evaluation,
            disposition_state,
            disposition_reason,
            now,
        )
        .map_err(KernelError::from)?;
        crate::services::validate_intake_disposition_only(&disposition)?;
        repo.upsert_intake_disposition(&disposition)?;
        drop(guard);
        Ok(disposition)
    }

    pub(crate) fn summary_projection(
        state: &DecisionEngineState,
        limit: usize,
    ) -> DecisionEngineSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn dismiss(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        candidate_id: impl Into<String>,
    ) -> Result<DecisionEngineActionResult> {
        Self::transition(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            candidate_id,
            DecisionOutcome::Dismissed,
            "decision.dismissed",
            false,
        )
    }

    pub(crate) fn postpone(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        candidate_id: impl Into<String>,
    ) -> Result<DecisionEngineActionResult> {
        // Postponed is a soft dismiss — audit under decision.dismissed with outcome metadata.
        Self::transition(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            candidate_id,
            DecisionOutcome::Postponed,
            "decision.dismissed",
            false,
        )
    }

    /// Select a recommendation — returns planner handoff only. Never executes.
    pub(crate) fn select(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        candidate_id: impl Into<String>,
    ) -> Result<DecisionEngineActionResult> {
        Self::transition(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            candidate_id,
            DecisionOutcome::Selected,
            "decision.selected",
            true,
        )
    }

    /// Explicit rejection of any execution attempt from this layer.
    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(DecisionEngineError::CannotExecute))
    }

    #[allow(clippy::too_many_arguments)]
    fn transition(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        candidate_id: impl Into<String>,
        to: DecisionOutcome,
        audit_event: &str,
        include_handoff: bool,
    ) -> Result<DecisionEngineActionResult> {
        let workspace_id = workspace_id.into();
        let candidate_id = candidate_id.into();
        let state = Self::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let candidate = state
            .candidates
            .iter()
            .find(|c| c.id.as_str() == candidate_id)
            .cloned()
            .ok_or_else(|| KernelError::from(DecisionEngineError::NotFound))?;

        let (integration, updated) =
            DecisionCandidateLifecycleIntegration::try_apply_outcome(&candidate, to)
                .map_err(KernelError::from)?;
        crate::services::validate_lifecycle_integration_only(&integration)?;
        crate::services::validate_provenance_retained(&candidate, &updated)?;

        let key = Self::candidate_key(&updated);
        Self::upsert_overlay(
            db,
            &DecisionEngineOverlay {
                workspace_id: workspace_id.clone(),
                candidate_key: key,
                outcome: to,
                updated_at: Utc::now().to_rfc3339(),
                actor_id: actor.actor.id.to_string(),
            },
        )?;

        Self::audit_lifecycle(
            db,
            actor,
            audit_event,
            &updated,
            json!({
                "origin": integration.origin,
                "integration_state": integration.integration_state,
                "provenance_immutable": true,
            }),
        )?;

        // Recommendation-intake candidates remain non-planner-connected in this boundary.
        let handoff = if include_handoff && updated.is_native_origin() {
            Some(DecisionEngineHandoff {
                candidate_id: updated.id.to_string(),
                next_command: DecisionCandidate::HANDOFF_SUBMIT_ASSISTANT_GOAL.into(),
                goal_statement: updated.goal_statement.clone(),
                workspace_id: workspace_id.clone(),
                note: "Decision Engine selected a recommendation. Call submit_assistant_goal to plan — never execute from Decision Engine.".into(),
                authority_effect: DecisionCandidate::AUTHORITY_EFFECT_NONE.into(),
            })
        } else {
            None
        };

        Ok(DecisionEngineActionResult {
            candidate: Some(updated),
            handoff,
            authority_effect: DecisionCandidate::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    fn candidate_key(candidate: &DecisionCandidate) -> String {
        candidate
            .id
            .as_str()
            .strip_prefix("engine_decision:")
            .unwrap_or(candidate.id.as_str())
            .to_string()
    }

    #[allow(clippy::too_many_arguments)]
    fn candidate_from_attention(
        ws: &str,
        item: &AttentionItem,
        goals: &[WorkGoal],
        memory: &[IntelligenceHighlight],
        prefs: &[IntelligenceHighlight],
        pending_approvals: &[&workspace_domain::DecisionItem],
        pending_plans: &[&workspace_domain::DecisionItem],
        overlays: &HashMap<String, DecisionOutcome>,
    ) -> Result<DecisionCandidate> {
        let key = format!("attention:{}", item.id);
        let outcome = overlays.get(&key).copied().unwrap_or(DecisionOutcome::Open);
        let attention_contribution = item.score.min(100);
        let memory_contribution = if memory.is_empty() { 0 } else { 8 };
        let personalization_contribution = if prefs.is_empty() { 0 } else { 10 };
        let goal_contribution = if goals.is_empty() { 0 } else { 12 };
        let plan_bonus = if pending_plans.is_empty() { 0 } else { 6 };
        let total = attention_contribution
            + memory_contribution
            + personalization_contribution
            + goal_contribution
            + plan_bonus;

        let mut reasons = vec![DecisionReason {
            kind: "attention".into(),
            summary: format!(
                "Current attention score is {} ({})",
                item.score,
                item.category.as_str()
            ),
            evidence_ref: Some(item.id.to_string()),
            attention_reason: None,
        }];
        // Attention's own structured reasons, carried whole and in Attention's order.
        // These replace the former `score_factors` strings, which narrated score math
        // rather than explaining why the item deserved focus.
        for attention_reason in &item.reasons {
            reasons.push(DecisionReason {
                kind: "attention_signal".into(),
                summary: format!(
                    "Attention signal {} from {} (weight {})",
                    attention_reason.signal.as_str(),
                    attention_reason.source.as_str(),
                    attention_reason.weight
                ),
                evidence_ref: Some(item.id.to_string()),
                attention_reason: Some(attention_reason.clone()),
            });
        }
        if let Some(goal) = goals.first() {
            reasons.push(DecisionReason {
                kind: "goal".into(),
                summary: format!("Related to current goal: {}", goal.description),
                evidence_ref: Some(goal.id.to_string()),
                attention_reason: None,
            });
        }
        if let Some(mem) = memory.first() {
            reasons.push(DecisionReason {
                kind: "memory".into(),
                summary: format!("Memory supports this: {}", mem.summary),
                evidence_ref: Some(mem.id.clone()),
                attention_reason: None,
            });
        }
        if let Some(pref) = prefs.first() {
            reasons.push(DecisionReason {
                kind: "personalization".into(),
                summary: format!("Preference: {} = {}", pref.label, pref.summary),
                evidence_ref: Some(pref.id.clone()),
                attention_reason: None,
            });
        }
        if !pending_plans.is_empty() {
            reasons.push(DecisionReason {
                kind: "plan".into(),
                summary: "Unfinished plan exists in Decision Queue.".into(),
                evidence_ref: Some(pending_plans[0].source_id.clone()),
                attention_reason: None,
            });
        }
        if !pending_approvals.is_empty() {
            reasons.push(DecisionReason {
                kind: "approval".into(),
                summary: format!(
                    "{} pending approval(s) in workspace.",
                    pending_approvals.len()
                ),
                evidence_ref: Some(pending_approvals[0].source_id.clone()),
                attention_reason: None,
            });
        }

        let confidence = if total >= 80 {
            "high"
        } else if total >= 45 {
            "medium"
        } else {
            "low"
        };

        // `score.factors` stays string-based on purpose: it is the arithmetic trail behind
        // `total`, not the rationale. Attention's score math explains `attention_contribution`
        // the same way "memory +8" explains the rest.
        let mut factors = item.score_factors.clone();
        if memory_contribution > 0 {
            factors.push(format!("memory +{memory_contribution}"));
        }
        if personalization_contribution > 0 {
            factors.push(format!("personalization +{personalization_contribution}"));
        }
        if goal_contribution > 0 {
            factors.push(format!("goal +{goal_contribution}"));
        }
        if plan_bonus > 0 {
            factors.push(format!("pending plan +{plan_bonus}"));
        }

        let goal_statement = format!(
            "Recommend next step: {}. {}",
            item.title, item.explanation
        );
        let related_goal_ids: Vec<_> = goals.iter().take(3).map(|g| g.id.to_string()).collect();
        let pending_approval_ids: Vec<_> = pending_approvals
            .iter()
            .take(5)
            .map(|a| a.source_id.clone())
            .collect();

        Ok(DecisionCandidate {
            id: DecisionCandidate::synthetic_id(&key),
            workspace_id: WorkspaceId::new(ws).map_err(KernelError::Domain)?,
            title: item.title.clone(),
            goal_statement,
            originating_goal: goals.first().map(|g| g.description.clone()),
            attention_item_id: Some(item.id.to_string()),
            recommendation_id: Some(format!("rec-attention-{}", item.id)),
            intake_candidate_id: None,
            creation_request_id: None,
            package_seal_digest: None,
            origin: DecisionCandidate::ORIGIN_NATIVE.into(),
            score: DecisionScore {
                total,
                attention_contribution,
                memory_contribution,
                personalization_contribution,
                goal_contribution: goal_contribution + plan_bonus,
                factors,
            },
            explanation: DecisionExplanation {
                headline: format!("Recommended because attention prioritizes: {}", item.title),
                reasons,
                confidence: confidence.into(),
            },
            related_goal_ids,
            pending_approval_ids,
            outcome,
            created_at: Utc::now().to_rfc3339(),
            handoff_command: DecisionCandidate::HANDOFF_SUBMIT_ASSISTANT_GOAL.into(),
            authority_effect: DecisionCandidate::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    fn candidate_from_graph_node(
        ws: &str,
        node: &workspace_domain::TaskNode,
        goals: &[WorkGoal],
        memory: &[IntelligenceHighlight],
        prefs: &[IntelligenceHighlight],
        overlays: &HashMap<String, DecisionOutcome>,
    ) -> Result<DecisionCandidate> {
        let key = format!("graph:{}", node.task.id);
        let outcome = overlays.get(&key).copied().unwrap_or(DecisionOutcome::Open);
        let attention_contribution = match node.task.status {
            workspace_domain::WorkspaceTaskStatus::Blocked => 70,
            workspace_domain::WorkspaceTaskStatus::Waiting => 50,
            workspace_domain::WorkspaceTaskStatus::InProgress => 45,
            _ => 35,
        };
        let memory_contribution = if memory.is_empty() { 0 } else { 6 };
        let personalization_contribution = if prefs.is_empty() { 0 } else { 6 };
        let goal_contribution = if goals.is_empty() { 0 } else { 10 };
        let priority_bonus = u32::from(node.task.priority.rank()) * 5;
        let total = attention_contribution
            + memory_contribution
            + personalization_contribution
            + goal_contribution
            + priority_bonus;

        let mut reasons = vec![DecisionReason {
            kind: "task_graph".into(),
            summary: format!(
                "This recommendation targets an incomplete {}-priority task.",
                node.task.priority.as_str()
            ),
            evidence_ref: Some(node.task.id.to_string()),
            attention_reason: None,
        }];
        if let Some(reason) = &node.waiting_reason {
            reasons.push(DecisionReason {
                kind: "dependency".into(),
                summary: reason.clone(),
                evidence_ref: node.dependency_ids.first().cloned(),
                attention_reason: None,
            });
        }
        reasons.push(DecisionReason {
            kind: "progress".into(),
            summary: format!("Task is {}% complete.", node.task.progress_percent),
            evidence_ref: Some(node.task.id.to_string()),
            attention_reason: None,
        });

        let confidence = if total >= 80 {
            "high"
        } else if total >= 45 {
            "medium"
        } else {
            "low"
        };

        Ok(DecisionCandidate {
            id: DecisionCandidate::synthetic_id(&key),
            workspace_id: WorkspaceId::new(ws).map_err(KernelError::Domain)?,
            title: format!("Continue: {}", node.task.title),
            goal_statement: format!(
                "Advance Task Graph work: {}. {}",
                node.task.title, node.task.explanation
            ),
            originating_goal: goals.first().map(|g| g.description.clone()),
            attention_item_id: None,
            recommendation_id: Some(format!("rec-graph-{}", node.task.id)),
            intake_candidate_id: None,
            creation_request_id: None,
            package_seal_digest: None,
            origin: DecisionCandidate::ORIGIN_NATIVE.into(),
            score: DecisionScore {
                total,
                attention_contribution,
                memory_contribution,
                personalization_contribution,
                goal_contribution: goal_contribution + priority_bonus,
                factors: vec![
                    format!("graph status +{attention_contribution}"),
                    format!("priority bonus +{priority_bonus}"),
                ],
            },
            explanation: DecisionExplanation {
                headline: format!(
                    "Recommended because Task Graph shows incomplete work: {}",
                    node.task.title
                ),
                reasons,
                confidence: confidence.into(),
            },
            related_goal_ids: goals.iter().take(3).map(|g| g.id.to_string()).collect(),
            pending_approval_ids: Vec::new(),
            outcome,
            created_at: Utc::now().to_rfc3339(),
            handoff_command: DecisionCandidate::HANDOFF_SUBMIT_ASSISTANT_GOAL.into(),
            authority_effect: DecisionCandidate::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn candidate_from_goal(
        ws: &str,
        goal: &WorkGoal,
        attention: &WorkspaceAttentionState,
        memory: &[IntelligenceHighlight],
        prefs: &[IntelligenceHighlight],
        pending_approvals: &[&workspace_domain::DecisionItem],
        overlays: &HashMap<String, DecisionOutcome>,
    ) -> Result<DecisionCandidate> {
        let key = format!("goal:{}", goal.id);
        let outcome = overlays.get(&key).copied().unwrap_or(DecisionOutcome::Open);
        let attention_contribution = attention
            .top_items
            .first()
            .map(|i| (i.score / 2).min(40))
            .unwrap_or(10);
        let memory_contribution = if memory.is_empty() { 0 } else { 8 };
        let personalization_contribution = if prefs.is_empty() { 0 } else { 8 };
        let goal_contribution = 20u32;
        let total = attention_contribution
            + memory_contribution
            + personalization_contribution
            + goal_contribution;

        let mut reasons = vec![DecisionReason {
            kind: "goal".into(),
            summary: format!("Continue work toward goal: {}", goal.description),
            evidence_ref: Some(goal.id.to_string()),
            attention_reason: None,
        }];
        if let Some(item) = attention.top_items.first() {
            // A cross-reference to what Attention ranked first, not a projection of that
            // item's rationale — so this stays Decision Engine's own evidence.
            reasons.push(DecisionReason {
                kind: "attention".into(),
                summary: format!("Attention also highlights: {}", item.title),
                evidence_ref: Some(item.id.to_string()),
                attention_reason: None,
            });
        }
        if let Some(mem) = memory.first() {
            reasons.push(DecisionReason {
                kind: "memory".into(),
                summary: format!("Memory: {}", mem.summary),
                evidence_ref: Some(mem.id.clone()),
                attention_reason: None,
            });
        }
        if let Some(pref) = prefs.first() {
            reasons.push(DecisionReason {
                kind: "personalization".into(),
                summary: format!("Preference: {}", pref.label),
                evidence_ref: Some(pref.id.clone()),
                attention_reason: None,
            });
        }

        let confidence = if total >= 70 { "high" } else { "medium" };

        Ok(DecisionCandidate {
            id: DecisionCandidate::synthetic_id(&key),
            workspace_id: WorkspaceId::new(ws).map_err(KernelError::Domain)?,
            title: format!("Continue: {}", goal.description),
            goal_statement: format!("Continue progress on work goal: {}", goal.description),
            originating_goal: Some(goal.description.clone()),
            attention_item_id: attention.top_items.first().map(|i| i.id.to_string()),
            recommendation_id: None,
            intake_candidate_id: None,
            creation_request_id: None,
            package_seal_digest: None,
            origin: DecisionCandidate::ORIGIN_NATIVE.into(),
            score: DecisionScore {
                total,
                attention_contribution,
                memory_contribution,
                personalization_contribution,
                goal_contribution,
                factors: vec![
                    format!("goal base +{goal_contribution}"),
                    format!("attention share +{attention_contribution}"),
                ],
            },
            explanation: DecisionExplanation {
                headline: format!("Recommended because goal \"{}\" remains active", goal.description),
                reasons,
                confidence: confidence.into(),
            },
            related_goal_ids: vec![goal.id.to_string()],
            pending_approval_ids: pending_approvals
                .iter()
                .take(5)
                .map(|a| a.source_id.clone())
                .collect(),
            outcome,
            created_at: Utc::now().to_rfc3339(),
            handoff_command: DecisionCandidate::HANDOFF_SUBMIT_ASSISTANT_GOAL.into(),
            authority_effect: DecisionCandidate::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    fn bootstrap_candidate(
        ws: &str,
        goals: &[WorkGoal],
        memory: &[IntelligenceHighlight],
        prefs: &[IntelligenceHighlight],
        overlays: &HashMap<String, DecisionOutcome>,
    ) -> Result<DecisionCandidate> {
        let key = "bootstrap:define_work";
        let outcome = overlays.get(key).copied().unwrap_or(DecisionOutcome::Open);
        let memory_contribution = if memory.is_empty() { 0 } else { 5 };
        let personalization_contribution = if prefs.is_empty() { 0 } else { 5 };
        let total = 20 + memory_contribution + personalization_contribution;
        let mut reasons = vec![DecisionReason {
            kind: "bootstrap".into(),
            summary: "Attention has no scored items — define current work.".into(),
            evidence_ref: None,
            attention_reason: None,
        }];
        if let Some(goal) = goals.first() {
            reasons.push(DecisionReason {
                kind: "goal".into(),
                summary: format!("Existing goal available: {}", goal.description),
                evidence_ref: Some(goal.id.to_string()),
                attention_reason: None,
            });
        }

        Ok(DecisionCandidate {
            id: DecisionCandidate::synthetic_id(key),
            workspace_id: WorkspaceId::new(ws).map_err(KernelError::Domain)?,
            title: "Define current work".into(),
            goal_statement: "Define the current project and task so the Workspace can recommend next steps."
                .into(),
            originating_goal: goals.first().map(|g| g.description.clone()),
            attention_item_id: None,
            recommendation_id: Some("rec-idle".into()),
            intake_candidate_id: None,
            creation_request_id: None,
            package_seal_digest: None,
            origin: DecisionCandidate::ORIGIN_NATIVE.into(),
            score: DecisionScore {
                total,
                attention_contribution: 0,
                memory_contribution,
                personalization_contribution,
                goal_contribution: if goals.is_empty() { 0 } else { 5 },
                factors: vec!["bootstrap baseline +20".into()],
            },
            explanation: DecisionExplanation {
                headline: "Recommended because the workspace needs an active focus".into(),
                reasons,
                confidence: "low".into(),
            },
            related_goal_ids: goals.iter().take(3).map(|g| g.id.to_string()).collect(),
            pending_approval_ids: Vec::new(),
            outcome,
            created_at: Utc::now().to_rfc3339(),
            handoff_command: DecisionCandidate::HANDOFF_SUBMIT_ASSISTANT_GOAL.into(),
            authority_effect: DecisionCandidate::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    fn load_overlays(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
    ) -> Result<HashMap<String, DecisionOutcome>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let overlays = DecisionEngineRepository::new(&guard).list_overlays(workspace_id)?;
        Ok(overlays
            .into_iter()
            .map(|o| (o.candidate_key, o.outcome))
            .collect())
    }

    fn upsert_overlay(db: &Arc<Mutex<Database>>, overlay: &DecisionEngineOverlay) -> Result<()> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        DecisionEngineRepository::new(&guard).upsert_overlay(overlay)?;
        Ok(())
    }

    fn previous_open_order(
        _db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        _ws: &str,
    ) -> Vec<String> {
        // Rank-change detection is best-effort via audit comparison within a session;
        // empty baseline means first generation (no rank_changed events).
        Vec::new()
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &DecisionEngineState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "decision.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "candidate_count": state.candidates.len(),
                "open_count": state.top_candidates.len(),
                "top_ids": state.top_candidates.iter().map(|c| c.id.to_string()).collect::<Vec<_>>(),
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_rank_changes(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &DecisionEngineState,
        previous: &[String],
    ) -> Result<()> {
        if previous.is_empty() {
            return Ok(());
        }
        let current: Vec<_> = state
            .top_candidates
            .iter()
            .map(|c| c.id.to_string())
            .collect();
        if current == previous {
            return Ok(());
        }
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "decision.rank_changed",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "previous": previous,
                "current": current,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_lifecycle(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event: &str,
        candidate: &DecisionCandidate,
        extra: serde_json::Value,
    ) -> Result<()> {
        let mut metadata = json!({
            "workspace_id": candidate.workspace_id.as_str(),
            "candidate_id": candidate.id.as_str(),
            "outcome": candidate.outcome.as_str(),
            "authority_effect": "none",
        });
        if let Some(obj) = metadata.as_object_mut() {
            if let Some(extra_obj) = extra.as_object() {
                for (k, v) in extra_obj {
                    obj.insert(k.clone(), v.clone());
                }
            }
        }
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            event,
            true,
            metadata.to_string(),
        )
    }
}
