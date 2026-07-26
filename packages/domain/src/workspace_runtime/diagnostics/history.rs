//! Runtime diagnostics — observational only.
//!
//! No execution, automation, repair, recommendations, or authority.

use serde::{Deserialize, Serialize};

use crate::action_proposal::GOVERNANCE_AUTHORITY_EFFECT_NONE as AUTH_NONE;
use crate::workspace_runtime::{
    WorkspaceRuntimeCoherence, WorkspaceRuntimeContext, WorkspaceRuntimeError,
    WorkspaceRuntimeHealth,
};

use super::{
    OperatorRuntimeOverview, RuntimeCapabilityMap, RuntimeConsistencySeverity,
    RuntimeConsistencyVerification, RuntimeDependencyGraph, RuntimeDiagnosticSnapshot,
};

// ---------------------------------------------------------------------------
// Runtime Diagnostic Provenance & Continuity (post–181 audit)
// ---------------------------------------------------------------------------
//
// Highest-value gap after 170–181: snapshots were point-in-time without provenance
// of inputs or historical continuity between captures. Distinct from the work
// Continuity Engine (`workspace_continuity`) — this layer is diagnostic-only.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticSourceKind {
    RuntimeContext,
    RuntimeHealth,
    DependencyGraph,
    CapabilityMap,
    ConsistencyVerification,
    CoherenceReview,
    OperatorOverview,
}

impl RuntimeDiagnosticSourceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeContext => "runtime_context",
            Self::RuntimeHealth => "runtime_health",
            Self::DependencyGraph => "dependency_graph",
            Self::CapabilityMap => "capability_map",
            Self::ConsistencyVerification => "consistency_verification",
            Self::CoherenceReview => "coherence_review",
            Self::OperatorOverview => "operator_overview",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticSourceRef {
    pub kind: RuntimeDiagnosticSourceKind,
    pub artifact_id: String,
}

/// Ownership roles for diagnostic vs subsystem vs lifecycle health — boundary only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticOwnershipRole {
    /// Owns observational diagnostic contracts (graph, map, snapshot, verify).
    ObservationalDiagnostics,
    /// Owns scoring/reasoning — diagnostics observe, never mutate.
    CognitionSubsystem,
    /// Kernel lifecycle health label — distinct from WorkspaceRuntimeHealth.
    KernelLifecycleHealth,
    /// Projects labels for operators — never owns sources or executes.
    OperatorProjection,
}

impl RuntimeDiagnosticOwnershipRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ObservationalDiagnostics => "observational_diagnostics",
            Self::CognitionSubsystem => "cognition_subsystem",
            Self::KernelLifecycleHealth => "kernel_lifecycle_health",
            Self::OperatorProjection => "operator_projection",
        }
    }
}

/// Declares who observes vs who owns facts for runtime diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticOwnershipBoundary {
    pub snapshot_owner: RuntimeDiagnosticOwnershipRole,
    pub health_observer: RuntimeDiagnosticOwnershipRole,
    pub lifecycle_health_owner: RuntimeDiagnosticOwnershipRole,
    pub operator_role: RuntimeDiagnosticOwnershipRole,
    pub may_mutate_sources: bool,
    pub may_prescribe_healing: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticOwnershipBoundary {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn canonical() -> Self {
        Self {
            snapshot_owner: RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
            health_observer: RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
            lifecycle_health_owner: RuntimeDiagnosticOwnershipRole::KernelLifecycleHealth,
            operator_role: RuntimeDiagnosticOwnershipRole::OperatorProjection,
            may_mutate_sources: false,
            may_prescribe_healing: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn boundaries_respected(&self) -> bool {
        !self.may_mutate_sources
            && !self.may_prescribe_healing
            && self.snapshot_owner == RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics
            && self.operator_role == RuntimeDiagnosticOwnershipRole::OperatorProjection
            && self.lifecycle_health_owner == RuntimeDiagnosticOwnershipRole::KernelLifecycleHealth
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn attempt_mutate_sources(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticHistoryImmutable)
    }

    pub fn attempt_heal(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticHistoryImmutable)
    }
}

/// Provenance for one diagnostic snapshot — cites inputs; does not own them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticProvenance {
    pub id: String,
    pub workspace_id: String,
    pub snapshot_id: String,
    pub captured_at: String,
    pub source_refs: Vec<RuntimeDiagnosticSourceRef>,
    pub ownership: RuntimeDiagnosticOwnershipBoundary,
    pub authority_effect: String,
}

impl RuntimeDiagnosticProvenance {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn from_capture(
        snapshot: &RuntimeDiagnosticSnapshot,
        ctx: &WorkspaceRuntimeContext,
        health: &WorkspaceRuntimeHealth,
        graph: &RuntimeDependencyGraph,
        capabilities: &RuntimeCapabilityMap,
        verification: &RuntimeConsistencyVerification,
        coherence: &WorkspaceRuntimeCoherence,
        overview: Option<&OperatorRuntimeOverview>,
    ) -> Self {
        let mut source_refs = vec![
            RuntimeDiagnosticSourceRef {
                kind: RuntimeDiagnosticSourceKind::RuntimeContext,
                artifact_id: ctx.id.clone(),
            },
            RuntimeDiagnosticSourceRef {
                kind: RuntimeDiagnosticSourceKind::RuntimeHealth,
                artifact_id: health.id.clone(),
            },
            RuntimeDiagnosticSourceRef {
                kind: RuntimeDiagnosticSourceKind::DependencyGraph,
                artifact_id: graph.id.clone(),
            },
            RuntimeDiagnosticSourceRef {
                kind: RuntimeDiagnosticSourceKind::CapabilityMap,
                artifact_id: capabilities.id.clone(),
            },
            RuntimeDiagnosticSourceRef {
                kind: RuntimeDiagnosticSourceKind::ConsistencyVerification,
                artifact_id: verification.id.clone(),
            },
            RuntimeDiagnosticSourceRef {
                kind: RuntimeDiagnosticSourceKind::CoherenceReview,
                artifact_id: coherence.id.clone(),
            },
        ];
        if let Some(o) = overview {
            source_refs.push(RuntimeDiagnosticSourceRef {
                kind: RuntimeDiagnosticSourceKind::OperatorOverview,
                artifact_id: o.id.clone(),
            });
        }
        Self {
            id: format!("runtime_diagnostic_provenance:{}", snapshot.id),
            workspace_id: snapshot.workspace_id.clone(),
            snapshot_id: snapshot.id.clone(),
            captured_at: snapshot.captured_at.clone(),
            source_refs,
            ownership: RuntimeDiagnosticOwnershipBoundary::canonical(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn cites_snapshot(&self, snapshot_id: &str) -> bool {
        self.snapshot_id == snapshot_id
    }

    pub fn may_rewrite_history(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_rewrite_history(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticHistoryImmutable)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Runtime Diagnostic Evolution (post–continuity audit)
// ---------------------------------------------------------------------------
//
// Provenance + continuity link snapshots, but lacked structured comparison,
// lifecycle phases, and validation that continuity matches observational diffs.
// All evolution contracts remain diagnostic-only.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticDeltaKind {
    HealthOverall,
    DependencyTopology,
    CapabilityInventory,
    GatewayAuthoritySignal,
    CognitionProjectionSet,
    GovernanceSummary,
    None,
}

impl RuntimeDiagnosticDeltaKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HealthOverall => "health_overall",
            Self::DependencyTopology => "dependency_topology",
            Self::CapabilityInventory => "capability_inventory",
            Self::GatewayAuthoritySignal => "gateway_authority_signal",
            Self::CognitionProjectionSet => "cognition_projection_set",
            Self::GovernanceSummary => "governance_summary",
            Self::None => "none",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticDelta {
    pub kind: RuntimeDiagnosticDeltaKind,
    pub detail: String,
    pub severity: RuntimeConsistencySeverity,
}

/// Structured observational comparison of two diagnostic snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticComparison {
    pub id: String,
    pub before_snapshot_id: String,
    pub after_snapshot_id: String,
    pub deltas: Vec<RuntimeDiagnosticDelta>,
    pub authority_effect: String,
}

impl RuntimeDiagnosticComparison {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn compare(
        before: &RuntimeDiagnosticSnapshot,
        after: &RuntimeDiagnosticSnapshot,
    ) -> Self {
        let mut deltas = Vec::new();
        if before.health_overall != after.health_overall {
            deltas.push(RuntimeDiagnosticDelta {
                kind: RuntimeDiagnosticDeltaKind::HealthOverall,
                detail: format!(
                    "health_overall:{}→{}",
                    before.health_overall.as_str(),
                    after.health_overall.as_str()
                ),
                severity: RuntimeConsistencySeverity::Warning,
            });
        }
        if before.dependency_node_count != after.dependency_node_count
            || before.dependency_edge_count != after.dependency_edge_count
            || before.dependency_has_cycles != after.dependency_has_cycles
        {
            let severity = if before.dependency_has_cycles != after.dependency_has_cycles {
                RuntimeConsistencySeverity::Error
            } else {
                RuntimeConsistencySeverity::Info
            };
            deltas.push(RuntimeDiagnosticDelta {
                kind: RuntimeDiagnosticDeltaKind::DependencyTopology,
                detail: format!(
                    "dependency_counts:{}n/{}e→{}n/{}e cycles:{}→{}",
                    before.dependency_node_count,
                    before.dependency_edge_count,
                    after.dependency_node_count,
                    after.dependency_edge_count,
                    before.dependency_has_cycles,
                    after.dependency_has_cycles
                ),
                severity,
            });
        }
        if before.capability_entry_count != after.capability_entry_count {
            deltas.push(RuntimeDiagnosticDelta {
                kind: RuntimeDiagnosticDeltaKind::CapabilityInventory,
                detail: format!(
                    "capability_entry_count:{}→{}",
                    before.capability_entry_count, after.capability_entry_count
                ),
                severity: RuntimeConsistencySeverity::Info,
            });
        }
        if before.gateway_sole_execution != after.gateway_sole_execution {
            deltas.push(RuntimeDiagnosticDelta {
                kind: RuntimeDiagnosticDeltaKind::GatewayAuthoritySignal,
                detail: format!(
                    "gateway_sole_execution:{}→{}",
                    before.gateway_sole_execution, after.gateway_sole_execution
                ),
                severity: RuntimeConsistencySeverity::Error,
            });
        }
        if before.cognition_projection_kinds != after.cognition_projection_kinds {
            deltas.push(RuntimeDiagnosticDelta {
                kind: RuntimeDiagnosticDeltaKind::CognitionProjectionSet,
                detail: "cognition_projection_kinds_changed".into(),
                severity: RuntimeConsistencySeverity::Info,
            });
        }
        if before.governance != after.governance {
            deltas.push(RuntimeDiagnosticDelta {
                kind: RuntimeDiagnosticDeltaKind::GovernanceSummary,
                detail: "governance_summary_changed".into(),
                severity: RuntimeConsistencySeverity::Warning,
            });
        }
        if deltas.is_empty() {
            deltas.push(RuntimeDiagnosticDelta {
                kind: RuntimeDiagnosticDeltaKind::None,
                detail: "no_observational_delta".into(),
                severity: RuntimeConsistencySeverity::Info,
            });
        }
        Self {
            id: format!(
                "runtime_diagnostic_comparison:{}:{}",
                before.id, after.id
            ),
            before_snapshot_id: before.id.clone(),
            after_snapshot_id: after.id.clone(),
            deltas,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn operator_safe_details(&self) -> Vec<String> {
        self.deltas.iter().map(|d| d.detail.clone()).collect()
    }

    pub fn has_authority_regression(&self) -> bool {
        self.deltas
            .iter()
            .any(|d| d.kind == RuntimeDiagnosticDeltaKind::GatewayAuthoritySignal)
    }

    pub fn is_authoritative(&self) -> bool {
        false
    }

    pub fn is_decision(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }

    pub fn attempt_promote_to_decision(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticEvidenceNotAuthoritative)
    }
}

/// Lifecycle phases for diagnostic snapshot lineage — observational only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticLifecyclePhase {
    Captured,
    Provenanced,
    ContinuityLinked,
    Compared,
    Evolved,
    Superseded,
    /// Retention marker — history retained; never deletion.
    Archived,
}

impl RuntimeDiagnosticLifecyclePhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Captured => "captured",
            Self::Provenanced => "provenanced",
            Self::ContinuityLinked => "continuity_linked",
            Self::Compared => "compared",
            Self::Evolved => "evolved",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn may_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Captured, Self::Provenanced)
                | (Self::Provenanced, Self::ContinuityLinked)
                | (Self::ContinuityLinked, Self::Compared)
                | (Self::Compared, Self::Evolved)
                | (Self::ContinuityLinked, Self::Evolved)
                | (Self::Evolved, Self::Superseded)
                | (Self::Evolved, Self::Archived)
                | (Self::Superseded, Self::Archived)
                | (Self::ContinuityLinked, Self::Superseded)
                | (Self::Provenanced, Self::Superseded)
        )
    }

    /// Retention terminal — history kept; RestoredView is currency, not a phase.
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Archived)
    }

    pub fn is_invalid_standalone(self) -> bool {
        false
    }
}

/// Immutable lifecycle record for one diagnostic snapshot in a lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticLifecycleRecord {
    pub id: String,
    pub workspace_id: String,
    pub snapshot_id: String,
    pub provenance_id: Option<String>,
    pub continuity_id: Option<String>,
    pub phase: RuntimeDiagnosticLifecyclePhase,
    pub recorded_at: String,
    pub previous_snapshot_id: Option<String>,
    pub authority_effect: String,
}

impl RuntimeDiagnosticLifecycleRecord {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn captured(snapshot: &RuntimeDiagnosticSnapshot, at: impl Into<String>) -> Self {
        Self {
            id: format!("runtime_diagnostic_lifecycle:{}:captured", snapshot.id),
            workspace_id: snapshot.workspace_id.clone(),
            snapshot_id: snapshot.id.clone(),
            provenance_id: None,
            continuity_id: None,
            phase: RuntimeDiagnosticLifecyclePhase::Captured,
            recorded_at: at.into(),
            previous_snapshot_id: None,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn provenanced(
        snapshot: &RuntimeDiagnosticSnapshot,
        provenance: &RuntimeDiagnosticProvenance,
        at: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("runtime_diagnostic_lifecycle:{}:provenanced", snapshot.id),
            workspace_id: snapshot.workspace_id.clone(),
            snapshot_id: snapshot.id.clone(),
            provenance_id: Some(provenance.id.clone()),
            continuity_id: None,
            phase: RuntimeDiagnosticLifecyclePhase::Provenanced,
            recorded_at: at.into(),
            previous_snapshot_id: None,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn continuity_linked(
        snapshot: &RuntimeDiagnosticSnapshot,
        provenance: &RuntimeDiagnosticProvenance,
        continuity: &RuntimeDiagnosticContinuityRecord,
        at: impl Into<String>,
    ) -> Self {
        Self {
            id: format!(
                "runtime_diagnostic_lifecycle:{}:continuity_linked",
                snapshot.id
            ),
            workspace_id: snapshot.workspace_id.clone(),
            snapshot_id: snapshot.id.clone(),
            provenance_id: Some(provenance.id.clone()),
            continuity_id: Some(continuity.id.clone()),
            phase: RuntimeDiagnosticLifecyclePhase::ContinuityLinked,
            recorded_at: at.into(),
            previous_snapshot_id: continuity.previous_snapshot_id.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn superseded(
        prior: &RuntimeDiagnosticSnapshot,
        successor_id: impl Into<String>,
        at: impl Into<String>,
    ) -> Self {
        let successor_id = successor_id.into();
        Self {
            id: format!(
                "runtime_diagnostic_lifecycle:{}:superseded:{}",
                prior.id, successor_id
            ),
            workspace_id: prior.workspace_id.clone(),
            snapshot_id: prior.id.clone(),
            provenance_id: None,
            continuity_id: None,
            phase: RuntimeDiagnosticLifecyclePhase::Superseded,
            recorded_at: at.into(),
            previous_snapshot_id: Some(successor_id),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn evolved(
        snapshot: &RuntimeDiagnosticSnapshot,
        provenance: &RuntimeDiagnosticProvenance,
        continuity: &RuntimeDiagnosticContinuityRecord,
        at: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("runtime_diagnostic_lifecycle:{}:evolved", snapshot.id),
            workspace_id: snapshot.workspace_id.clone(),
            snapshot_id: snapshot.id.clone(),
            provenance_id: Some(provenance.id.clone()),
            continuity_id: Some(continuity.id.clone()),
            phase: RuntimeDiagnosticLifecyclePhase::Evolved,
            recorded_at: at.into(),
            previous_snapshot_id: continuity.previous_snapshot_id.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn archived(
        snapshot: &RuntimeDiagnosticSnapshot,
        at: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("runtime_diagnostic_lifecycle:{}:archived", snapshot.id),
            workspace_id: snapshot.workspace_id.clone(),
            snapshot_id: snapshot.id.clone(),
            provenance_id: None,
            continuity_id: None,
            phase: RuntimeDiagnosticLifecyclePhase::Archived,
            recorded_at: at.into(),
            previous_snapshot_id: None,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_mutate(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_mutate(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticHistoryImmutable)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

/// Canonical ownership table for runtime architecture artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeArchitectureOwnershipEntry {
    pub artifact: String,
    pub owner: RuntimeDiagnosticOwnershipRole,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeArchitectureOwnershipRegistry {
    pub entries: Vec<RuntimeArchitectureOwnershipEntry>,
    pub authority_effect: String,
}

impl RuntimeArchitectureOwnershipRegistry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn canonical() -> Self {
        let entry = |artifact: &str, owner: RuntimeDiagnosticOwnershipRole, notes: &str| {
            RuntimeArchitectureOwnershipEntry {
                artifact: artifact.into(),
                owner,
                notes: notes.into(),
            }
        };
        Self {
            entries: vec![
                entry(
                    "RuntimeDependencyGraph",
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "diagnostics contract; not a cognition owner",
                ),
                entry(
                    "RuntimeCapabilityMap",
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "descriptive inventory only",
                ),
                entry(
                    "WorkspaceRuntimeHealth",
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "observes presence/readiness labels; never prescribes",
                ),
                entry(
                    "WorkspaceRuntimeCoherence",
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "structural chain review only",
                ),
                entry(
                    "GovernanceRuntimeSummary",
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "embedded non-authoritative labels",
                ),
                entry(
                    "OperatorContextProjection",
                    RuntimeDiagnosticOwnershipRole::OperatorProjection,
                    "read-only labels; not UI and not execution",
                ),
                entry(
                    "OperatorRuntimeOverview",
                    RuntimeDiagnosticOwnershipRole::OperatorProjection,
                    "read-only overview projection",
                ),
                entry(
                    "OperatorRuntimeExplanation",
                    RuntimeDiagnosticOwnershipRole::OperatorProjection,
                    "exposes what changed; never actions",
                ),
                entry(
                    "Attention/Decision scoring",
                    RuntimeDiagnosticOwnershipRole::CognitionSubsystem,
                    "diagnostics must not alter scoring",
                ),
                entry(
                    "Kernel WorkspaceHealth",
                    RuntimeDiagnosticOwnershipRole::KernelLifecycleHealth,
                    "distinct from WorkspaceRuntimeHealth",
                ),
            ],
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn owner_of(&self, artifact: &str) -> Option<RuntimeDiagnosticOwnershipRole> {
        self.entries
            .iter()
            .find(|e| e.artifact == artifact)
            .map(|e| e.owner)
    }

    pub fn operator_artifacts_are_projections_only(&self) -> bool {
        self.entries
            .iter()
            .filter(|e| e.artifact.starts_with("Operator"))
            .all(|e| e.owner == RuntimeDiagnosticOwnershipRole::OperatorProjection)
    }

    pub const REGISTRY_VERSION: &'static str = "canonical:v1";

    /// Descriptive ownership validation — conflict detection only; never mutates.
    pub fn validate(&self) -> RuntimeArchitectureOwnershipValidation {
        RuntimeArchitectureOwnershipValidation::validate(self)
    }
}

/// Result of validating the ownership registry — descriptive only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeArchitectureOwnershipValidation {
    pub registry_version: String,
    pub conflict_free: bool,
    pub diagnostics: Vec<String>,
    pub authority_effect: String,
}

impl RuntimeArchitectureOwnershipValidation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn validate(registry: &RuntimeArchitectureOwnershipRegistry) -> Self {
        let mut diagnostics = Vec::new();
        let mut seen: std::collections::HashMap<&str, RuntimeDiagnosticOwnershipRole> =
            std::collections::HashMap::new();
        for e in &registry.entries {
            if let Some(prior) = seen.insert(e.artifact.as_str(), e.owner) {
                if prior != e.owner {
                    diagnostics.push(format!(
                        "ownership conflict on {}: {:?} vs {:?}",
                        e.artifact, prior, e.owner
                    ));
                }
            }
            if e.artifact.starts_with("Operator")
                && e.owner != RuntimeDiagnosticOwnershipRole::OperatorProjection
            {
                diagnostics.push(format!(
                    "operator artifact {} must remain OperatorProjection",
                    e.artifact
                ));
            }
            if e.artifact.contains("scoring")
                && e.owner == RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics
            {
                diagnostics.push(format!(
                    "scoring artifact {} must not be owned by diagnostics",
                    e.artifact
                ));
            }
        }
        if registry.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            diagnostics.push("ownership registry authority_effect must be none".into());
        }
        let conflict_free = diagnostics.is_empty();
        if conflict_free {
            diagnostics.push("ownership registry validation passed".into());
        }
        Self {
            registry_version: RuntimeArchitectureOwnershipRegistry::REGISTRY_VERSION.into(),
            conflict_free,
            diagnostics,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_mutate_registry(&self) -> bool {
        false
    }
}

/// Validates and interprets diagnostic evolution — never repairs or executes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticEvolutionReport {
    pub id: String,
    pub workspace_id: String,
    pub lifecycle: RuntimeDiagnosticLifecycleRecord,
    pub comparison: Option<RuntimeDiagnosticComparison>,
    pub provenance_consistent: bool,
    pub continuity_consistent: bool,
    pub lifecycle_ordering_ok: bool,
    pub ownership_ok: bool,
    pub operator_safe_summary: Vec<String>,
    pub diagnostics: Vec<String>,
    pub authority_effect: String,
}

impl RuntimeDiagnosticEvolutionReport {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn evaluate(
        previous: Option<&RuntimeDiagnosticSnapshot>,
        current: &RuntimeDiagnosticSnapshot,
        provenance: &RuntimeDiagnosticProvenance,
        continuity: &RuntimeDiagnosticContinuityRecord,
        prior_phase: Option<RuntimeDiagnosticLifecyclePhase>,
        at: impl Into<String>,
    ) -> Self {
        let at = at.into();
        let mut diagnostics = Vec::new();
        let comparison = previous.map(|prev| RuntimeDiagnosticComparison::compare(prev, current));

        let provenance_consistent = provenance.cites_snapshot(&current.id)
            && provenance.ownership.boundaries_respected()
            && continuity.provenance_id == provenance.id;
        if !provenance_consistent {
            diagnostics.push("provenance/continuity linkage inconsistent".into());
        }

        let continuity_consistent = continuity.current_snapshot_id == current.id
            && continuity.previous_snapshot_id.as_deref() == previous.map(|p| p.id.as_str())
            && match &comparison {
                None => continuity.is_initial(),
                Some(cmp) => {
                    let expected = cmp.operator_safe_details();
                    continuity.what_changed == expected
                }
            };
        if !continuity_consistent {
            diagnostics.push("continuity deltas do not match structured comparison".into());
        }

        let lifecycle = RuntimeDiagnosticLifecycleRecord::evolved(
            current,
            provenance,
            continuity,
            &at,
        );
        let lifecycle_ordering_ok = match prior_phase {
            None => true,
            Some(phase) => {
                phase.may_transition_to(RuntimeDiagnosticLifecyclePhase::Evolved)
                    || phase.may_transition_to(RuntimeDiagnosticLifecyclePhase::ContinuityLinked)
                    || phase == RuntimeDiagnosticLifecyclePhase::Evolved
                    || phase == RuntimeDiagnosticLifecyclePhase::ContinuityLinked
            }
        };
        if !lifecycle_ordering_ok {
            diagnostics.push(format!(
                "invalid lifecycle ordering from {}",
                prior_phase.map(|p| p.as_str()).unwrap_or("none")
            ));
        }

        let registry = RuntimeArchitectureOwnershipRegistry::canonical();
        let ownership_validation = registry.validate();
        let ownership_ok = ownership_validation.conflict_free
            && registry.operator_artifacts_are_projections_only()
            && provenance.ownership.boundaries_respected();
        if !ownership_ok {
            diagnostics.push("ownership registry/boundary violation".into());
        }

        let mut operator_safe_summary = vec![
            format!("lifecycle={}", lifecycle.phase.as_str()),
            "interpretation=diagnostic_only".into(),
            "authoritative=false".into(),
            "actions=none".into(),
            "execution_authority=permission_gateway_only".into(),
        ];
        if let Some(cmp) = &comparison {
            for d in &cmp.deltas {
                operator_safe_summary.push(format!(
                    "delta:{}:{}:{}",
                    d.kind.as_str(),
                    d.severity.as_str(),
                    d.detail
                ));
            }
            if cmp.has_authority_regression() {
                operator_safe_summary
                    .push("signal:gateway_authority_observation_changed".into());
            }
        } else {
            operator_safe_summary.push("delta:initial_diagnostic_snapshot".into());
        }

        // Strip accidental action-like phrasing from operator surface.
        operator_safe_summary.retain(|s| {
            let lower = s.to_lowercase();
            !lower.contains("repair")
                && !lower.contains("auto_heal")
                && !lower.contains("approve:")
                && !lower.starts_with("action:")
                && !lower.contains("may_execute=true")
        });

        if diagnostics.is_empty() {
            diagnostics.push("diagnostic evolution validation passed".into());
        }

        Self {
            id: format!("runtime_diagnostic_evolution:{}:{}", current.workspace_id, at),
            workspace_id: current.workspace_id.clone(),
            lifecycle,
            comparison,
            provenance_consistent,
            continuity_consistent,
            lifecycle_ordering_ok,
            ownership_ok,
            operator_safe_summary,
            diagnostics,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn passed(&self) -> bool {
        self.provenance_consistent
            && self.continuity_consistent
            && self.lifecycle_ordering_ok
            && self.ownership_ok
            && !self.operator_safe_summary.iter().any(|s| {
                let l = s.to_lowercase();
                l.contains("repair")
                    || l.contains("auto_heal")
                    || l.starts_with("action:")
                    || l.contains("may_execute=true")
            })
    }

    pub fn is_authoritative(&self) -> bool {
        false
    }

    pub fn is_decision(&self) -> bool {
        false
    }

    pub fn may_repair(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_repair(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticEvolutionReadOnly)
    }

    pub fn attempt_promote_to_decision(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticEvidenceNotAuthoritative)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}


pub struct RuntimeDiagnosticContinuityRecord {
    pub id: String,
    pub workspace_id: String,
    pub previous_snapshot_id: Option<String>,
    pub current_snapshot_id: String,
    pub recorded_at: String,
    pub what_changed: Vec<String>,
    pub provenance_id: String,
    pub authority_effect: String,
}

impl RuntimeDiagnosticContinuityRecord {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    /// Link prior → current snapshot with observational field diffs only.
    pub fn link(
        previous: Option<&RuntimeDiagnosticSnapshot>,
        current: &RuntimeDiagnosticSnapshot,
        provenance: &RuntimeDiagnosticProvenance,
        recorded_at: impl Into<String>,
    ) -> Self {
        let recorded_at = recorded_at.into();
        let what_changed = match previous {
            None => vec!["initial_diagnostic_snapshot".into()],
            Some(prev) => RuntimeDiagnosticComparison::compare(prev, current)
                .operator_safe_details(),
        };
        Self {
            id: format!(
                "runtime_diagnostic_continuity:{}:{}",
                current.workspace_id, recorded_at
            ),
            workspace_id: current.workspace_id.clone(),
            previous_snapshot_id: previous.map(|p| p.id.clone()),
            current_snapshot_id: current.id.clone(),
            recorded_at,
            what_changed,
            provenance_id: provenance.id.clone(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn is_initial(&self) -> bool {
        self.previous_snapshot_id.is_none()
    }

    pub fn may_mutate_prior(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_mutate_prior(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticHistoryImmutable)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Runtime Diagnostic Evidence Integrity & Retention (post–evolution audit)
// ---------------------------------------------------------------------------
//
// Evolution validates change, but comparisons/reports lacked sealed evidence
// references and append-only retention. Distinct from GovernanceArchive and
// work Continuity Engine.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticEvidenceKind {
    Snapshot,
    Provenance,
    Continuity,
    Comparison,
    Evolution,
    Lifecycle,
    OwnershipRegistry,
}

impl RuntimeDiagnosticEvidenceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
            Self::Provenance => "provenance",
            Self::Continuity => "continuity",
            Self::Comparison => "comparison",
            Self::Evolution => "evolution",
            Self::Lifecycle => "lifecycle",
            Self::OwnershipRegistry => "ownership_registry",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticEvidenceRef {
    pub kind: RuntimeDiagnosticEvidenceKind,
    pub artifact_id: String,
    pub digest: String,
}

/// Immutable sealed evidence for one diagnostic evolution chain — not a decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticEvidenceBundle {
    pub id: String,
    pub workspace_id: String,
    pub sealed_at: String,
    pub refs: Vec<RuntimeDiagnosticEvidenceRef>,
    pub evolution_report_id: String,
    pub comparison_id: Option<String>,
    pub authority_effect: String,
}

impl RuntimeDiagnosticEvidenceBundle {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    fn digest_for(kind: RuntimeDiagnosticEvidenceKind, artifact_id: &str) -> String {
        format!("digest:{}:{}", kind.as_str(), artifact_id)
    }

    pub fn seal(
        evolution: &RuntimeDiagnosticEvolutionReport,
        provenance: &RuntimeDiagnosticProvenance,
        continuity: &RuntimeDiagnosticContinuityRecord,
        at: impl Into<String>,
    ) -> Self {
        let sealed_at = at.into();
        let mut refs = vec![
            RuntimeDiagnosticEvidenceRef {
                kind: RuntimeDiagnosticEvidenceKind::Snapshot,
                artifact_id: evolution.lifecycle.snapshot_id.clone(),
                digest: Self::digest_for(
                    RuntimeDiagnosticEvidenceKind::Snapshot,
                    &evolution.lifecycle.snapshot_id,
                ),
            },
            RuntimeDiagnosticEvidenceRef {
                kind: RuntimeDiagnosticEvidenceKind::Provenance,
                artifact_id: provenance.id.clone(),
                digest: Self::digest_for(RuntimeDiagnosticEvidenceKind::Provenance, &provenance.id),
            },
            RuntimeDiagnosticEvidenceRef {
                kind: RuntimeDiagnosticEvidenceKind::Continuity,
                artifact_id: continuity.id.clone(),
                digest: Self::digest_for(RuntimeDiagnosticEvidenceKind::Continuity, &continuity.id),
            },
            RuntimeDiagnosticEvidenceRef {
                kind: RuntimeDiagnosticEvidenceKind::Evolution,
                artifact_id: evolution.id.clone(),
                digest: Self::digest_for(RuntimeDiagnosticEvidenceKind::Evolution, &evolution.id),
            },
            RuntimeDiagnosticEvidenceRef {
                kind: RuntimeDiagnosticEvidenceKind::Lifecycle,
                artifact_id: evolution.lifecycle.id.clone(),
                digest: Self::digest_for(
                    RuntimeDiagnosticEvidenceKind::Lifecycle,
                    &evolution.lifecycle.id,
                ),
            },
            RuntimeDiagnosticEvidenceRef {
                kind: RuntimeDiagnosticEvidenceKind::OwnershipRegistry,
                artifact_id: RuntimeArchitectureOwnershipRegistry::REGISTRY_VERSION.into(),
                digest: Self::digest_for(
                    RuntimeDiagnosticEvidenceKind::OwnershipRegistry,
                    RuntimeArchitectureOwnershipRegistry::REGISTRY_VERSION,
                ),
            },
        ];
        let comparison_id = evolution.comparison.as_ref().map(|c| c.id.clone());
        if let Some(cid) = &comparison_id {
            refs.push(RuntimeDiagnosticEvidenceRef {
                kind: RuntimeDiagnosticEvidenceKind::Comparison,
                artifact_id: cid.clone(),
                digest: Self::digest_for(RuntimeDiagnosticEvidenceKind::Comparison, cid),
            });
        }
        Self {
            id: format!(
                "runtime_diagnostic_evidence:{}:{}",
                evolution.workspace_id, sealed_at
            ),
            workspace_id: evolution.workspace_id.clone(),
            sealed_at,
            refs,
            evolution_report_id: evolution.id.clone(),
            comparison_id,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn cites_evolution(&self, evolution_id: &str) -> bool {
        self.evolution_report_id == evolution_id
    }

    pub fn is_complete(&self) -> bool {
        let kinds: std::collections::HashSet<_> = self.refs.iter().map(|r| r.kind).collect();
        kinds.contains(&RuntimeDiagnosticEvidenceKind::Snapshot)
            && kinds.contains(&RuntimeDiagnosticEvidenceKind::Provenance)
            && kinds.contains(&RuntimeDiagnosticEvidenceKind::Continuity)
            && kinds.contains(&RuntimeDiagnosticEvidenceKind::Evolution)
            && kinds.contains(&RuntimeDiagnosticEvidenceKind::Lifecycle)
    }

    pub fn is_authoritative(&self) -> bool {
        false
    }

    pub fn is_decision(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_promote_to_decision(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticEvidenceNotAuthoritative)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticRetentionPolicy {
    pub retain_indefinitely: bool,
    pub may_delete: bool,
    pub may_mutate_archived: bool,
}

impl RuntimeDiagnosticRetentionPolicy {
    pub fn canonical() -> Self {
        Self {
            retain_indefinitely: true,
            may_delete: false,
            may_mutate_archived: false,
        }
    }

    pub fn allows_deletion(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticArchiveKind {
    SnapshotSeal,
    EvolutionSeal,
    SupersessionMarker,
    OwnershipRegistryVersion,
}

impl RuntimeDiagnosticArchiveKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SnapshotSeal => "snapshot_seal",
            Self::EvolutionSeal => "evolution_seal",
            Self::SupersessionMarker => "supersession_marker",
            Self::OwnershipRegistryVersion => "ownership_registry_version",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticArchiveEntry {
    pub id: String,
    pub kind: RuntimeDiagnosticArchiveKind,
    pub source_reference: String,
    pub evidence_bundle_id: Option<String>,
    pub digest: String,
    pub archived_at: String,
    pub superseded_by: Option<String>,
}

/// Append-only diagnostic retention archive — not governance archive, not work continuity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticArchive {
    pub id: String,
    pub workspace_id: String,
    pub entries: Vec<RuntimeDiagnosticArchiveEntry>,
    pub append_only: bool,
    pub retention: RuntimeDiagnosticRetentionPolicy,
    pub authority_effect: String,
}

impl RuntimeDiagnosticArchive {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn new(workspace_id: impl Into<String>) -> Self {
        let workspace_id = workspace_id.into();
        Self {
            id: format!("runtime_diagnostic_archive:{workspace_id}"),
            workspace_id,
            entries: Vec::new(),
            append_only: true,
            retention: RuntimeDiagnosticRetentionPolicy::canonical(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn archive_evidence(
        &mut self,
        evidence: &RuntimeDiagnosticEvidenceBundle,
        at: impl Into<String>,
    ) -> Result<&RuntimeDiagnosticArchiveEntry, WorkspaceRuntimeError> {
        if !self.append_only || self.retention.may_delete || self.retention.may_mutate_archived {
            return Err(WorkspaceRuntimeError::DiagnosticArchiveImmutable);
        }
        let archived_at = at.into();
        let entry = RuntimeDiagnosticArchiveEntry {
            id: format!(
                "{}:{}:{}",
                self.id,
                RuntimeDiagnosticArchiveKind::EvolutionSeal.as_str(),
                self.entries.len()
            ),
            kind: RuntimeDiagnosticArchiveKind::EvolutionSeal,
            source_reference: evidence.evolution_report_id.clone(),
            evidence_bundle_id: Some(evidence.id.clone()),
            digest: format!("digest:archive:{}", evidence.id),
            archived_at,
            superseded_by: None,
        };
        self.entries.push(entry);
        Ok(self.entries.last().unwrap())
    }

    pub fn mark_superseded(
        &mut self,
        prior_snapshot_id: impl Into<String>,
        successor_snapshot_id: impl Into<String>,
        at: impl Into<String>,
    ) -> Result<&RuntimeDiagnosticArchiveEntry, WorkspaceRuntimeError> {
        if !self.append_only {
            return Err(WorkspaceRuntimeError::DiagnosticArchiveImmutable);
        }
        let prior = prior_snapshot_id.into();
        let successor = successor_snapshot_id.into();
        let entry = RuntimeDiagnosticArchiveEntry {
            id: format!(
                "{}:{}:{}",
                self.id,
                RuntimeDiagnosticArchiveKind::SupersessionMarker.as_str(),
                self.entries.len()
            ),
            kind: RuntimeDiagnosticArchiveKind::SupersessionMarker,
            source_reference: prior,
            evidence_bundle_id: None,
            digest: format!("digest:supersede:{}", successor),
            archived_at: at.into(),
            superseded_by: Some(successor),
        };
        self.entries.push(entry);
        Ok(self.entries.last().unwrap())
    }

    pub fn may_delete(&self) -> bool {
        false
    }

    pub fn may_mutate_entries(&self) -> bool {
        false
    }

    pub fn attempt_delete(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticArchiveImmutable)
    }

    pub fn attempt_mutate_entry(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticArchiveImmutable)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

/// Historical integrity over sealed evidence + retention archive — diagnostics only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticHistoricalIntegrity {
    pub id: String,
    pub archive_append_only_ok: bool,
    pub evidence_complete: bool,
    pub reports_non_authoritative: bool,
    pub ownership_conflict_free: bool,
    pub retention_forbids_deletion: bool,
    pub diagnostics: Vec<String>,
    pub authority_effect: String,
}

impl RuntimeDiagnosticHistoricalIntegrity {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn verify(
        evidence: &RuntimeDiagnosticEvidenceBundle,
        archive: &RuntimeDiagnosticArchive,
        evolution: &RuntimeDiagnosticEvolutionReport,
        comparison: Option<&RuntimeDiagnosticComparison>,
        ownership: &RuntimeArchitectureOwnershipValidation,
    ) -> Self {
        let mut diagnostics = Vec::new();
        let archive_append_only_ok = archive.append_only && !archive.may_delete();
        if !archive_append_only_ok {
            diagnostics.push("archive must remain append-only".into());
        }
        let evidence_complete = evidence.is_complete()
            && evidence.cites_evolution(&evolution.id)
            && evidence.comparison_id.as_deref() == comparison.map(|c| c.id.as_str());
        if !evidence_complete {
            diagnostics.push("evidence bundle incomplete or mismatched".into());
        }
        let reports_non_authoritative = !evolution.is_authoritative()
            && !evolution.is_decision()
            && !evidence.is_authoritative()
            && comparison.map(|c| !c.is_authoritative() && !c.is_decision()).unwrap_or(true);
        if !reports_non_authoritative {
            diagnostics.push("diagnostic reports must not be authoritative decisions".into());
        }
        let ownership_conflict_free = ownership.conflict_free;
        if !ownership_conflict_free {
            diagnostics.push("ownership registry has conflicts".into());
        }
        let retention_forbids_deletion = !archive.retention.allows_deletion()
            && archive.retention.retain_indefinitely
            && !archive.retention.may_mutate_archived;
        if !retention_forbids_deletion {
            diagnostics.push("retention must forbid deletion and mutation".into());
        }
        if diagnostics.is_empty() {
            diagnostics.push("diagnostic historical integrity passed".into());
        }
        Self {
            id: format!(
                "runtime_diagnostic_historical_integrity:{}",
                evidence.workspace_id
            ),
            archive_append_only_ok,
            evidence_complete,
            reports_non_authoritative,
            ownership_conflict_free,
            retention_forbids_deletion,
            diagnostics,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn passed(&self) -> bool {
        self.archive_append_only_ok
            && self.evidence_complete
            && self.reports_non_authoritative
            && self.ownership_conflict_free
            && self.retention_forbids_deletion
    }

    pub fn may_repair(&self) -> bool {
        false
    }

    pub fn attempt_repair(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticArchiveImmutable)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

