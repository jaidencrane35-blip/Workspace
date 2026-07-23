//! Model provider registry + routing — intelligence only (Sprints 62–63).
//!
//! Providers generate proposal candidates. They never execute, grant, or bypass
//! the Permission Gateway.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    ActorContext, AiActionProposal, AiModelError, AiPlan, AiPlanningContext, ApplicationId,
    IntentContext, ModelInvocationSummary, ModelProposalCandidate, ModelProviderAvailability,
    ModelProviderCapability, ModelProviderDescriptor, ModelRequest, ModelResponse,
    ModelResponseStatus,
};

use crate::error::{KernelError, Result};
use crate::services::AuditService;

/// Kernel-side provider interface. Implementations supply intelligence only.
pub(crate) trait ModelProvider: Send + Sync {
    fn descriptor(&self) -> &ModelProviderDescriptor;
    fn complete(&self, request: &ModelRequest) -> std::result::Result<ModelResponse, AiModelError>;
}

/// Default deterministic provider — preserves existing planning behavior.
pub(crate) struct DeterministicModelProvider {
    descriptor: ModelProviderDescriptor,
}

impl DeterministicModelProvider {
    pub(crate) fn new() -> Self {
        let descriptor = ModelProviderDescriptor::new(
            "deterministic",
            "rules-v1",
            "Deterministic Planner",
            "1.0.0",
        )
        .expect("deterministic provider ids are valid")
        .with_capabilities(vec![
            ModelProviderCapability::PlanningAssist,
            ModelProviderCapability::StructuredProposals,
            ModelProviderCapability::ContextInterpretation,
        ])
        .with_runtime_metadata("local_deterministic_stub");
        Self { descriptor }
    }

    pub(crate) fn candidates_for(
        &self,
        context: &AiPlanningContext,
    ) -> Vec<ModelProposalCandidate> {
        // Personalization ranks first; memory remains a secondary informational hint.
        let mut preferred = context
            .personalization_awareness
            .as_ref()
            .map(|awareness| awareness.preferred_application_ids())
            .unwrap_or_default();
        let memory_preferred = context
            .memory_awareness
            .as_ref()
            .map(|awareness| awareness.preferred_application_ids())
            .unwrap_or_default();
        for id in memory_preferred {
            if !preferred.iter().any(|existing| existing == &id) {
                preferred.push(id);
            }
        }

        let mut candidates: Vec<(ApplicationId, String)> =
            if let Some(awareness) = &context.awareness {
                awareness
                    .applications
                    .iter()
                    .filter(|app| !app.appears_active)
                    .map(|app| {
                        let preferred_note = preference_note(context, app.id.as_str(), &app.name);
                        let explanation = format!(
                        "Goal '{}': launch '{}' (registered in workspace '{}', not appearing active){preferred_note}",
                        context.goal.statement, app.name, awareness.workspace_name
                    );
                        (app.id.clone(), explanation)
                    })
                    .collect()
            } else {
                context
                    .available_application_ids
                    .iter()
                    .map(|application_id| {
                        let preferred_note =
                            preference_note(context, application_id.as_str(), application_id.as_str());
                        let explanation = format!(
                            "Goal '{}': launch application to prepare workspace{preferred_note}",
                            context.goal.statement
                        );
                        (application_id.clone(), explanation)
                    })
                    .collect()
            };

        candidates.sort_by_key(|(id, _)| {
            if preferred
                .iter()
                .any(|preferred_id| preferred_id == id.as_str())
            {
                0_u8
            } else {
                1_u8
            }
        });

        candidates
            .into_iter()
            .take(5)
            .map(|(id, explanation)| ModelProposalCandidate::application_launch(id, explanation))
            .collect()
    }
}

fn preference_note(context: &AiPlanningContext, application_id: &str, fallback_label: &str) -> String {
    if let Some(personalization) = &context.personalization_awareness {
        if let Some(explanation) = personalization.explanation_for_application(application_id) {
            return format!(" {explanation}");
        }
    }
    let memory_preferred = context
        .memory_awareness
        .as_ref()
        .map(|awareness| awareness.preferred_application_ids())
        .unwrap_or_default();
    if memory_preferred.iter().any(|id| id == application_id) {
        return format!(" (preferred from memory: {fallback_label})");
    }
    String::new()
}

impl ModelProvider for DeterministicModelProvider {
    fn descriptor(&self) -> &ModelProviderDescriptor {
        &self.descriptor
    }

    fn complete(&self, request: &ModelRequest) -> std::result::Result<ModelResponse, AiModelError> {
        if self.descriptor.availability == ModelProviderAvailability::Unavailable {
            return Err(AiModelError::ProviderUnavailable(
                self.descriptor.provider_id.to_string(),
            ));
        }
        let candidates: Vec<ModelProposalCandidate> = request
            .candidate_application_ids
            .iter()
            .take(5)
            .map(|id| {
                ModelProposalCandidate::application_launch(
                    id.clone(),
                    format!(
                        "Goal '{}': launch application to prepare workspace",
                        request.task
                    ),
                )
            })
            .collect();

        Ok(ModelResponse::success(
            request,
            &self.descriptor,
            candidates,
            Some(format!("deterministic response for: {}", request.task)),
        ))
    }
}

/// Secondary available provider for multi-provider selection tests/diagnostics.
pub(crate) struct EchoModelProvider {
    descriptor: ModelProviderDescriptor,
}

impl EchoModelProvider {
    pub(crate) fn new() -> Self {
        let descriptor = ModelProviderDescriptor::new("echo", "echo-v1", "Echo Provider", "1.0.0")
            .expect("echo provider ids are valid")
            .with_capabilities(vec![
                ModelProviderCapability::TextGeneration,
                ModelProviderCapability::StructuredProposals,
            ])
            .with_runtime_metadata("local_echo_stub");
        Self { descriptor }
    }
}

impl ModelProvider for EchoModelProvider {
    fn descriptor(&self) -> &ModelProviderDescriptor {
        &self.descriptor
    }

    fn complete(&self, request: &ModelRequest) -> std::result::Result<ModelResponse, AiModelError> {
        let candidates: Vec<ModelProposalCandidate> = request
            .candidate_application_ids
            .iter()
            .take(3)
            .map(|id| {
                ModelProposalCandidate::application_launch(
                    id.clone(),
                    format!("Echo provider suggestion for '{}'", request.task),
                )
            })
            .collect();
        Ok(ModelResponse::success(
            request,
            &self.descriptor,
            candidates,
            Some(format!("echo: {}", request.task)),
        ))
    }
}

/// Provider that always fails — used for graceful failure diagnostics/tests.
pub(crate) struct UnavailableModelProvider {
    descriptor: ModelProviderDescriptor,
}

impl UnavailableModelProvider {
    pub(crate) fn new() -> Self {
        let descriptor = ModelProviderDescriptor::new(
            "unavailable",
            "none",
            "Unavailable Provider",
            "0.0.0",
        )
        .expect("unavailable provider ids are valid")
        .with_capabilities(vec![ModelProviderCapability::PlanningAssist])
        .with_availability(ModelProviderAvailability::Unavailable)
        .with_runtime_metadata("forced_unavailable");
        Self { descriptor }
    }
}

impl ModelProvider for UnavailableModelProvider {
    fn descriptor(&self) -> &ModelProviderDescriptor {
        &self.descriptor
    }

    fn complete(&self, request: &ModelRequest) -> std::result::Result<ModelResponse, AiModelError> {
        let _ = request;
        Err(AiModelError::ProviderUnavailable(
            self.descriptor.provider_id.to_string(),
        ))
    }
}

/// Registry of model providers + routing (availability/capability only).
pub(crate) struct ModelProviderRegistry {
    providers: Vec<Arc<dyn ModelProvider>>,
    preferred_provider_id: Option<String>,
}

impl ModelProviderRegistry {
    pub(crate) fn default_registry() -> Self {
        Self {
            providers: vec![
                Arc::new(DeterministicModelProvider::new()),
                Arc::new(EchoModelProvider::new()),
                Arc::new(UnavailableModelProvider::new()),
            ],
            preferred_provider_id: Some("deterministic".into()),
        }
    }

    #[cfg(test)]
    pub(crate) fn with_providers(
        providers: Vec<Arc<dyn ModelProvider>>,
        preferred_provider_id: Option<String>,
    ) -> Self {
        Self {
            providers,
            preferred_provider_id,
        }
    }

    pub(crate) fn list_descriptors(&self) -> Vec<ModelProviderDescriptor> {
        self.providers
            .iter()
            .map(|provider| provider.descriptor().clone())
            .collect()
    }

    pub(crate) fn get(&self, provider_id: &str) -> Option<Arc<dyn ModelProvider>> {
        self.providers
            .iter()
            .find(|provider| provider.descriptor().provider_id.as_str() == provider_id)
            .cloned()
    }

    /// Selects a provider by availability + required capability. Never uses permissions.
    ///
    /// Explicit `preferred_provider_id` does not silently fall back (no autonomous switching).
    /// Default routing (None) uses registry preference, then first selectable provider.
    pub(crate) fn select_for(
        &self,
        required: ModelProviderCapability,
        preferred_provider_id: Option<&str>,
    ) -> Result<Arc<dyn ModelProvider>> {
        if let Some(preferred_id) = preferred_provider_id {
            let provider = self.get(preferred_id).ok_or_else(|| {
                KernelError::from(AiModelError::UnknownProvider(preferred_id.into()))
            })?;
            let descriptor = provider.descriptor();
            if !descriptor.availability.is_selectable() {
                return Err(KernelError::from(AiModelError::ProviderUnavailable(
                    preferred_id.into(),
                )));
            }
            if !descriptor.supports(required) {
                return Err(KernelError::from(AiModelError::UnsupportedCapability(
                    required.as_str().into(),
                )));
            }
            return Ok(provider);
        }

        if let Some(preferred_id) = self.preferred_provider_id.as_deref() {
            if let Some(provider) = self.get(preferred_id) {
                let descriptor = provider.descriptor();
                if descriptor.availability.is_selectable() && descriptor.supports(required) {
                    return Ok(provider);
                }
            }
        }

        self.providers
            .iter()
            .find(|provider| {
                let descriptor = provider.descriptor();
                descriptor.availability.is_selectable() && descriptor.supports(required)
            })
            .cloned()
            .ok_or_else(|| KernelError::from(AiModelError::NoProviderAvailable))
    }
}

/// Governed model provider service — intelligence boundary only.
pub(crate) struct ModelProviderService;

impl ModelProviderService {
    pub(crate) fn registry() -> ModelProviderRegistry {
        ModelProviderRegistry::default_registry()
    }

    pub(crate) fn list_providers() -> Vec<ModelProviderDescriptor> {
        Self::registry().list_descriptors()
    }

    pub(crate) fn get_provider_metadata(provider_id: &str) -> Result<ModelProviderDescriptor> {
        Self::registry()
            .get(provider_id)
            .map(|provider| provider.descriptor().clone())
            .ok_or_else(|| KernelError::from(AiModelError::UnknownProvider(provider_id.into())))
    }

    pub(crate) fn invoke(
        request: &ModelRequest,
        preferred_provider_id: Option<&str>,
    ) -> Result<ModelResponse> {
        let registry = Self::registry();
        let provider = registry.select_for(
            ModelProviderCapability::StructuredProposals,
            preferred_provider_id,
        )?;
        provider.complete(request).map_err(KernelError::from)
    }

    pub(crate) fn invoke_with_audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        request: &ModelRequest,
        preferred_provider_id: Option<&str>,
    ) -> Result<ModelResponse> {
        let registry = Self::registry();
        let provider = match registry.select_for(
            ModelProviderCapability::StructuredProposals,
            preferred_provider_id,
        ) {
            Ok(provider) => provider,
            Err(error) => {
                Self::audit_failed(db, actor, request, None, None, &error.to_string())?;
                return Err(error);
            }
        };

        Self::audit_selected(db, actor, provider.descriptor())?;
        Self::audit_requested(db, actor, request, provider.descriptor())?;

        match provider.complete(request) {
            Ok(response) => {
                Self::audit_response(db, actor, &response)?;
                if response.status == ModelResponseStatus::Failed {
                    return Err(KernelError::from(AiModelError::InvalidResponse(
                        response
                            .error_message
                            .clone()
                            .unwrap_or_else(|| "provider returned failed status".into()),
                    )));
                }
                Ok(response)
            }
            Err(error) => {
                Self::audit_failed(
                    db,
                    actor,
                    request,
                    Some(provider.descriptor().provider_id.as_str()),
                    Some(provider.descriptor().model_id.as_str()),
                    &error.to_string(),
                )?;
                Err(KernelError::from(error))
            }
        }
    }

    /// Planning integration: provider candidates → AiPlan proposals (no execution).
    pub(crate) fn plan_from_context(
        context: &AiPlanningContext,
        audit: Option<(&Arc<Mutex<Database>>, &ActorContext)>,
    ) -> Result<AiPlan> {
        let request = ModelRequest::from_planning_context(context).map_err(KernelError::from)?;
        let registry = Self::registry();
        let provider = registry.select_for(
            ModelProviderCapability::PlanningAssist,
            Some("deterministic"),
        )?;

        if let Some((db, actor)) = audit {
            Self::audit_selected(db, actor, provider.descriptor())?;
            Self::audit_requested(db, actor, &request, provider.descriptor())?;
        }

        let response = if provider.descriptor().provider_id.as_str() == "deterministic" {
            let deterministic = DeterministicModelProvider::new();
            let candidates = deterministic.candidates_for(context);
            let response = ModelResponse::success(
                &request,
                deterministic.descriptor(),
                candidates,
                Some(format!("deterministic plan for: {}", context.goal.statement)),
            );
            if let Some((db, actor)) = audit {
                Self::audit_response(db, actor, &response)?;
            }
            response
        } else {
            match provider.complete(&request) {
                Ok(response) => {
                    if let Some((db, actor)) = audit {
                        Self::audit_response(db, actor, &response)?;
                    }
                    if response.status == ModelResponseStatus::Failed {
                        let message = response
                            .error_message
                            .clone()
                            .unwrap_or_else(|| "provider failed".into());
                        if let Some((db, actor)) = audit {
                            Self::audit_failed(
                                db,
                                actor,
                                &request,
                                Some(response.provider_id.as_str()),
                                Some(response.model_id.as_str()),
                                &message,
                            )?;
                        }
                        return Err(KernelError::from(AiModelError::InvalidResponse(message)));
                    }
                    response
                }
                Err(error) => {
                    if let Some((db, actor)) = audit {
                        Self::audit_failed(
                            db,
                            actor,
                            &request,
                            Some(provider.descriptor().provider_id.as_str()),
                            Some(provider.descriptor().model_id.as_str()),
                            &error.to_string(),
                        )?;
                    }
                    return Err(KernelError::from(error));
                }
            }
        };

        let capability_note = context
            .action_awareness
            .as_ref()
            .and_then(|awareness| awareness.explain_capability_for_command("LaunchApplication"));
        let memory_notes = context
            .memory_awareness
            .as_ref()
            .map(|awareness| awareness.planning_notes())
            .unwrap_or_default();
        let personalization_notes = context
            .personalization_awareness
            .as_ref()
            .map(|awareness| awareness.applied_explanations())
            .unwrap_or_default();

        let mut proposals = Vec::new();
        for candidate in response
            .proposal_candidates
            .iter()
            .filter(|candidate| candidate.is_governed_launch_candidate())
            .take(5)
        {
            let application_id = candidate.application_id().map_err(KernelError::from)?;
            let mut explanation = candidate
                .explanation
                .clone()
                .unwrap_or_else(|| format!("Provider suggested launch of {application_id}"));
            if let Some(note) = &capability_note {
                explanation = format!("{explanation} {note}");
            }
            if !personalization_notes.is_empty()
                && !explanation.contains("Preferred because")
            {
                explanation = format!(
                    "{explanation} Personalization: {}",
                    personalization_notes.join("; ")
                );
            }
            if !memory_notes.is_empty() {
                explanation = format!("{explanation} Memory: {}", memory_notes.join("; "));
            }
            let proposal = AiActionProposal::propose_application_launch(
                &context.goal,
                &application_id,
                Some(explanation),
            )
            .map_err(KernelError::from)?;
            proposals.push(proposal);
        }

        Ok(AiPlan {
            goal: context.goal.clone(),
            proposals,
            model_invocation: Some(ModelInvocationSummary::from_response(&response)),
        })
    }

    fn audit_selected(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        descriptor: &ModelProviderDescriptor,
    ) -> Result<()> {
        let metadata = json!({
            "provider_id": descriptor.provider_id.as_str(),
            "model_id": descriptor.model_id.as_str(),
            "availability": descriptor.availability.as_str(),
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.model.provider_selected",
            true,
            metadata,
        )
    }

    fn audit_requested(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        request: &ModelRequest,
        descriptor: &ModelProviderDescriptor,
    ) -> Result<()> {
        let metadata = json!({
            "request_id": request.request_id,
            "kind": format!("{:?}", request.kind),
            "provider_id": descriptor.provider_id.as_str(),
            "model_id": descriptor.model_id.as_str(),
            "response_format": format!("{:?}", request.response_format),
            "candidate_application_count": request.candidate_application_ids.len(),
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.model.requested",
            true,
            metadata,
        )
    }

    fn audit_response(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        response: &ModelResponse,
    ) -> Result<()> {
        let metadata = json!({
            "request_id": response.request_id,
            "provider_id": response.provider_id.as_str(),
            "model_id": response.model_id.as_str(),
            "status": match response.status {
                ModelResponseStatus::Success => "success",
                ModelResponseStatus::Failed => "failed",
            },
            "candidate_count": response.proposal_candidates.len(),
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.model.response_received",
            response.status == ModelResponseStatus::Success,
            metadata,
        )
    }

    fn audit_failed(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        request: &ModelRequest,
        provider_id: Option<&str>,
        model_id: Option<&str>,
        error_message: &str,
    ) -> Result<()> {
        let metadata = json!({
            "request_id": request.request_id,
            "provider_id": provider_id,
            "model_id": model_id,
            "error": error_message,
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.model.failed",
            false,
            metadata,
        )
    }
}
