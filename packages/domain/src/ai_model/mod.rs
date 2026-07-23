//! Model provider abstraction — intelligence only (Sprints 62–63).
//!
//! Models generate intelligence. The system controls authority.
//! Provider output answers "What could be done?" — never "Is it allowed?".

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ai_planning::AiPlanningContext;
use crate::errors::DomainError;
use crate::ids::{ApplicationId, ModelId, ModelProviderId};

/// Model-provider validation / operational errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AiModelError {
    #[error("Model provider unavailable: {0}")]
    ProviderUnavailable(String),

    #[error("Model provider timed out: {0}")]
    Timeout(String),

    #[error("Invalid model response: {0}")]
    InvalidResponse(String),

    #[error("Malformed structured model output: {0}")]
    MalformedStructuredOutput(String),

    #[error("Unsupported model capability: {0}")]
    UnsupportedCapability(String),

    #[error("No model provider available for request")]
    NoProviderAvailable,

    #[error("Unknown model provider: {0}")]
    UnknownProvider(String),

    #[error("Model request task must not be empty")]
    EmptyTask,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

/// Intelligence capabilities a provider may advertise (not system permissions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelProviderCapability {
    TextGeneration,
    StructuredProposals,
    PlanningAssist,
    ContextInterpretation,
}

impl ModelProviderCapability {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TextGeneration => "text_generation",
            Self::StructuredProposals => "structured_proposals",
            Self::PlanningAssist => "planning_assist",
            Self::ContextInterpretation => "context_interpretation",
        }
    }
}

/// Availability of a registered provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelProviderAvailability {
    Available,
    Unavailable,
    Degraded,
}

impl ModelProviderAvailability {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Unavailable => "unavailable",
            Self::Degraded => "degraded",
        }
    }

    pub fn is_selectable(self) -> bool {
        matches!(self, Self::Available | Self::Degraded)
    }
}

/// Descriptor for a model provider instance (metadata only — not authority).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelProviderDescriptor {
    pub provider_id: ModelProviderId,
    pub model_id: ModelId,
    pub display_name: String,
    pub version: String,
    pub capabilities: Vec<ModelProviderCapability>,
    pub availability: ModelProviderAvailability,
    /// Runtime notes (local/stub/deterministic). Never permissions.
    pub runtime_metadata: Option<String>,
}

impl ModelProviderDescriptor {
    pub fn new(
        provider_id: impl Into<String>,
        model_id: impl Into<String>,
        display_name: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Self, AiModelError> {
        Ok(Self {
            provider_id: ModelProviderId::new(provider_id)?,
            model_id: ModelId::new(model_id)?,
            display_name: display_name.into(),
            version: version.into(),
            capabilities: Vec::new(),
            availability: ModelProviderAvailability::Available,
            runtime_metadata: None,
        })
    }

    pub fn with_capabilities(mut self, capabilities: Vec<ModelProviderCapability>) -> Self {
        self.capabilities = capabilities;
        self
    }

    pub fn with_availability(mut self, availability: ModelProviderAvailability) -> Self {
        self.availability = availability;
        self
    }

    pub fn with_runtime_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.runtime_metadata = Some(metadata.into());
        self
    }

    pub fn supports(&self, capability: ModelProviderCapability) -> bool {
        self.capabilities.contains(&capability)
    }
}

/// Expected response shape for a model request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelResponseFormat {
    Text,
    StructuredProposals,
}

/// Kind of intelligence task (not an authority decision).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelRequestKind {
    PlanningAssist,
    TextCompletion,
    StructuredProposal,
}

/// Abstraction for provider input — prompt/context + structured task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelRequest {
    pub request_id: String,
    pub kind: ModelRequestKind,
    pub task: String,
    pub context_summary: Option<String>,
    pub response_format: ModelResponseFormat,
    /// Application ids available for proposal candidates (informational).
    pub candidate_application_ids: Vec<String>,
    pub created_at: String,
}

impl ModelRequest {
    pub fn planning_assist(
        task: impl Into<String>,
        candidate_application_ids: Vec<String>,
        context_summary: Option<String>,
    ) -> Result<Self, AiModelError> {
        let task = task.into();
        if task.trim().is_empty() {
            return Err(AiModelError::EmptyTask);
        }
        Ok(Self {
            request_id: uuid::Uuid::new_v4().to_string(),
            kind: ModelRequestKind::PlanningAssist,
            task,
            context_summary,
            response_format: ModelResponseFormat::StructuredProposals,
            candidate_application_ids,
            created_at: Utc::now().to_rfc3339(),
        })
    }

    pub fn from_planning_context(context: &AiPlanningContext) -> Result<Self, AiModelError> {
        let candidate_application_ids = if let Some(awareness) = &context.awareness {
            awareness
                .applications
                .iter()
                .filter(|app| !app.appears_active)
                .map(|app| app.id.to_string())
                .collect()
        } else {
            context
                .available_application_ids
                .iter()
                .map(|id| id.to_string())
                .collect()
        };

        let mut summary_parts = Vec::new();
        if let Some(awareness) = &context.awareness {
            summary_parts.push(format!(
                "workspace={} apps={}",
                awareness.workspace_name,
                awareness.applications.len()
            ));
        }
        if let Some(memory) = &context.memory_awareness {
            summary_parts.push(format!("memory_entries={}", memory.entries.len()));
        }

        Self::planning_assist(
            context.goal.statement.clone(),
            candidate_application_ids,
            if summary_parts.is_empty() {
                None
            } else {
                Some(summary_parts.join("; "))
            },
        )
    }
}

/// Structured proposal candidate from a model — suggestion only, never authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelProposalCandidate {
    pub command_name: String,
    pub target_application_id: Option<String>,
    pub explanation: Option<String>,
}

impl ModelProposalCandidate {
    pub fn application_launch(
        application_id: impl Into<String>,
        explanation: impl Into<String>,
    ) -> Self {
        Self {
            command_name: "LaunchApplication".into(),
            target_application_id: Some(application_id.into()),
            explanation: Some(explanation.into()),
        }
    }

    /// Only LaunchApplication candidates are mapped into the governed proposal path.
    pub fn is_governed_launch_candidate(&self) -> bool {
        self.command_name == "LaunchApplication" && self.target_application_id.is_some()
    }

    pub fn application_id(&self) -> Result<ApplicationId, AiModelError> {
        let id = self
            .target_application_id
            .as_ref()
            .ok_or_else(|| AiModelError::MalformedStructuredOutput("missing target".into()))?;
        ApplicationId::new(id.clone()).map_err(AiModelError::from)
    }
}

/// Provider response status (operational — not permission outcome).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelResponseStatus {
    Success,
    Failed,
}

/// Abstraction for provider output — intelligence only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelResponse {
    pub request_id: String,
    pub provider_id: ModelProviderId,
    pub model_id: ModelId,
    pub status: ModelResponseStatus,
    pub text_output: Option<String>,
    pub proposal_candidates: Vec<ModelProposalCandidate>,
    pub metadata: Option<String>,
    pub error_message: Option<String>,
    pub created_at: String,
}

impl ModelResponse {
    pub fn success(
        request: &ModelRequest,
        provider: &ModelProviderDescriptor,
        proposal_candidates: Vec<ModelProposalCandidate>,
        text_output: Option<String>,
    ) -> Self {
        let candidate_count = proposal_candidates.len();
        Self {
            request_id: request.request_id.clone(),
            provider_id: provider.provider_id.clone(),
            model_id: provider.model_id.clone(),
            status: ModelResponseStatus::Success,
            text_output,
            proposal_candidates,
            metadata: Some(
                serde_json::json!({
                    "authority_effect": "none",
                    "candidate_count": candidate_count,
                })
                .to_string(),
            ),
            error_message: None,
            created_at: Utc::now().to_rfc3339(),
        }
    }

    pub fn failure(
        request: &ModelRequest,
        provider: &ModelProviderDescriptor,
        error_message: impl Into<String>,
    ) -> Self {
        Self {
            request_id: request.request_id.clone(),
            provider_id: provider.provider_id.clone(),
            model_id: provider.model_id.clone(),
            status: ModelResponseStatus::Failed,
            text_output: None,
            proposal_candidates: Vec::new(),
            metadata: Some(
                serde_json::json!({
                    "authority_effect": "none",
                })
                .to_string(),
            ),
            error_message: Some(error_message.into()),
            created_at: Utc::now().to_rfc3339(),
        }
    }
}

/// Compact invocation summary attached to plans for diagnostics/audits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelInvocationSummary {
    pub provider_id: String,
    pub model_id: String,
    pub status: String,
    pub candidate_count: usize,
    pub request_id: String,
}

impl ModelInvocationSummary {
    pub fn from_response(response: &ModelResponse) -> Self {
        Self {
            provider_id: response.provider_id.to_string(),
            model_id: response.model_id.to_string(),
            status: match response.status {
                ModelResponseStatus::Success => "success".into(),
                ModelResponseStatus::Failed => "failed".into(),
            },
            candidate_count: response.proposal_candidates.len(),
            request_id: response.request_id.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_tracks_capabilities_not_permissions() {
        let descriptor = ModelProviderDescriptor::new(
            "deterministic",
            "rules-v1",
            "Deterministic Planner",
            "1.0.0",
        )
        .unwrap()
        .with_capabilities(vec![
            ModelProviderCapability::PlanningAssist,
            ModelProviderCapability::StructuredProposals,
        ]);
        assert!(descriptor.supports(ModelProviderCapability::PlanningAssist));
        assert!(!descriptor.display_name.is_empty());
    }

    #[test]
    fn proposal_candidate_is_suggestion_only() {
        let candidate = ModelProposalCandidate::application_launch("app-1", "suggest launch");
        assert!(candidate.is_governed_launch_candidate());
        assert_eq!(candidate.command_name, "LaunchApplication");
    }
}
