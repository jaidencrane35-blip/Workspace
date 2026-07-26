//! Sprints 176–181 — runtime dependency graph, capabilities, snapshot, verification, overview.
//!
//! Diagnostics only. No execution, automation, or automatic repair.

use serde::{Deserialize, Serialize};

use super::{
    CognitionContextProjection, GovernanceRuntimeSummary, OperatorContextProjection,
    WorkspaceHealthLevel, WorkspaceRuntimeCoherence, WorkspaceRuntimeContext,
    WorkspaceRuntimeError, WorkspaceRuntimeHealth, WorkspaceRuntimeIntegrationContract,
};

use crate::action_proposal::GOVERNANCE_AUTHORITY_EFFECT_NONE as AUTH_NONE;

// ---------------------------------------------------------------------------
// Sprint 176 — Runtime Dependency Graph
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDependencyKind {
    Required,
    Optional,
}

impl RuntimeDependencyKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeSubsystemNode {
    pub id: String,
    pub owner: String,
    pub layer: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDependencyEdge {
    pub from: String,
    pub to: String,
    pub kind: RuntimeDependencyKind,
    pub detail: String,
}

/// Canonical runtime dependency graph — diagnostics only (Sprint 176).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDependencyGraph {
    pub id: String,
    pub nodes: Vec<RuntimeSubsystemNode>,
    pub edges: Vec<RuntimeDependencyEdge>,
    pub circular_dependency_ids: Vec<String>,
    pub authority_effect: String,
}

impl RuntimeDependencyGraph {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    /// Canonical cognition → governance → operator dependency direction.
    pub fn canonical(workspace_id: impl Into<String>) -> Self {
        let workspace_id = workspace_id.into();
        let node = |id: &str, owner: &str, layer: &str| RuntimeSubsystemNode {
            id: id.into(),
            owner: owner.into(),
            layer: layer.into(),
        };
        let edge = |from: &str, to: &str, kind: RuntimeDependencyKind, detail: &str| {
            RuntimeDependencyEdge {
                from: from.into(),
                to: to.into(),
                kind,
                detail: detail.into(),
            }
        };
        let nodes = vec![
            node("WorkspaceState", "WorkspaceStateEngine", "facts"),
            node("Environment", "WorkspaceEnvironmentService", "projection"),
            node("Attention", "WorkspaceAttentionService", "cognition"),
            node("Intelligence", "WorkspaceIntelligenceService", "cognition"),
            node("Decision", "DecisionEngineService", "cognition"),
            node("Recommendation", "WorkspaceRecommendationEngineService", "cognition"),
            node("Experience", "WorkspaceExperienceService", "experience"),
            node("GovernanceProjections", "Governance (summaries)", "governance"),
            node("OperatorProjections", "OperatorContextProjection", "operator"),
            node("PermissionGateway", "PermissionGateway", "execution"),
        ];
        let edges = vec![
            edge(
                "Environment",
                "WorkspaceState",
                RuntimeDependencyKind::Required,
                "Environment projects from WorkspaceState",
            ),
            edge(
                "Attention",
                "Environment",
                RuntimeDependencyKind::Required,
                "Attention consumes environment/context facts",
            ),
            edge(
                "Intelligence",
                "Attention",
                RuntimeDependencyKind::Required,
                "Intelligence aggregates cognition summaries",
            ),
            edge(
                "Decision",
                "Intelligence",
                RuntimeDependencyKind::Optional,
                "Decision may use intelligence overlays",
            ),
            edge(
                "Decision",
                "Attention",
                RuntimeDependencyKind::Required,
                "Decision ranks from attention signals",
            ),
            edge(
                "Recommendation",
                "Attention",
                RuntimeDependencyKind::Required,
                "Recommendations ground in attention",
            ),
            edge(
                "Experience",
                "Recommendation",
                RuntimeDependencyKind::Optional,
                "Experience translates recommendation display",
            ),
            edge(
                "Experience",
                "Decision",
                RuntimeDependencyKind::Optional,
                "Experience translates decision display",
            ),
            edge(
                "GovernanceProjections",
                "Experience",
                RuntimeDependencyKind::Optional,
                "Governance summaries visible beside experience",
            ),
            edge(
                "OperatorProjections",
                "GovernanceProjections",
                RuntimeDependencyKind::Required,
                "Operator overview includes governance labels",
            ),
            edge(
                "OperatorProjections",
                "Intelligence",
                RuntimeDependencyKind::Optional,
                "Operator may view intelligence health labels",
            ),
            // Gateway is execution authority — cognition must not depend on Allow.
            edge(
                "PermissionGateway",
                "WorkspaceState",
                RuntimeDependencyKind::Optional,
                "Gateway executes commands; does not own cognition deps",
            ),
        ];
        let mut graph = Self {
            id: format!("runtime_dependency_graph:{workspace_id}"),
            nodes,
            edges,
            circular_dependency_ids: Vec::new(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        };
        graph.circular_dependency_ids = graph.detect_cycles();
        graph
    }

    pub fn detect_cycles(&self) -> Vec<String> {
        use std::collections::{HashMap, HashSet};
        let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
        for e in &self.edges {
            adj.entry(e.from.as_str()).or_default().push(e.to.as_str());
        }
        let mut cycles = Vec::new();
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();

        fn dfs(
            node: &str,
            adj: &HashMap<&str, Vec<&str>>,
            visiting: &mut HashSet<String>,
            visited: &mut HashSet<String>,
            stack: &mut Vec<String>,
            cycles: &mut Vec<String>,
        ) {
            if visited.contains(node) {
                return;
            }
            if visiting.contains(node) {
                if let Some(start) = stack.iter().position(|n| n == node) {
                    let path = stack[start..].join("→");
                    cycles.push(format!("{path}→{node}"));
                }
                return;
            }
            visiting.insert(node.into());
            stack.push(node.into());
            if let Some(nexts) = adj.get(node) {
                for n in nexts {
                    dfs(n, adj, visiting, visited, stack, cycles);
                }
            }
            stack.pop();
            visiting.remove(node);
            visited.insert(node.into());
        }

        let mut stack = Vec::new();
        for n in &self.nodes {
            dfs(
                &n.id,
                &adj,
                &mut visiting,
                &mut visited,
                &mut stack,
                &mut cycles,
            );
        }
        cycles.sort();
        cycles.dedup();
        cycles
    }

    pub fn has_cycles(&self) -> bool {
        !self.circular_dependency_ids.is_empty()
    }

    pub fn required_edges(&self) -> Vec<&RuntimeDependencyEdge> {
        self.edges
            .iter()
            .filter(|e| e.kind == RuntimeDependencyKind::Required)
            .collect()
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 177 — Runtime Capability Map
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeVisibilityScope {
    Internal,
    Cognition,
    Operator,
    Governance,
}

impl RuntimeVisibilityScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::Cognition => "cognition",
            Self::Operator => "operator",
            Self::Governance => "governance",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeAuthorityScope {
    None,
    /// Descriptive only — execution Allow remains Permission Gateway.
    ExecutionGatewayOnly,
}

impl RuntimeAuthorityScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ExecutionGatewayOnly => "execution_gateway_only",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeCapabilityEntry {
    pub subsystem: String,
    pub provides: Vec<String>,
    pub consumes: Vec<String>,
    pub visibility: RuntimeVisibilityScope,
    pub authority: RuntimeAuthorityScope,
}

/// Descriptive capability inventory — no execution (Sprint 177).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeCapabilityMap {
    pub id: String,
    pub entries: Vec<RuntimeCapabilityEntry>,
    pub authority_effect: String,
}

impl RuntimeCapabilityMap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn inventory(workspace_id: impl Into<String>) -> Self {
        let workspace_id = workspace_id.into();
        let entry = |subsystem: &str,
                     provides: &[&str],
                     consumes: &[&str],
                     visibility: RuntimeVisibilityScope,
                     authority: RuntimeAuthorityScope| RuntimeCapabilityEntry {
            subsystem: subsystem.into(),
            provides: provides.iter().map(|s| (*s).into()).collect(),
            consumes: consumes.iter().map(|s| (*s).into()).collect(),
            visibility,
            authority,
        };
        Self {
            id: format!("runtime_capability_map:{workspace_id}"),
            entries: vec![
                entry(
                    "WorkspaceState",
                    &["state_projection", "window_rows"],
                    &["observation_facts"],
                    RuntimeVisibilityScope::Internal,
                    RuntimeAuthorityScope::None,
                ),
                entry(
                    "Environment",
                    &["environment_summary"],
                    &["state_projection"],
                    RuntimeVisibilityScope::Cognition,
                    RuntimeAuthorityScope::None,
                ),
                entry(
                    "Attention",
                    &["attention_items", "attention_summary"],
                    &["environment_summary"],
                    RuntimeVisibilityScope::Cognition,
                    RuntimeAuthorityScope::None,
                ),
                entry(
                    "Intelligence",
                    &["intelligence_state"],
                    &["attention_summary", "decision_summary", "readiness_summary"],
                    RuntimeVisibilityScope::Cognition,
                    RuntimeAuthorityScope::None,
                ),
                entry(
                    "Decision",
                    &["decision_candidates"],
                    &["attention_items"],
                    RuntimeVisibilityScope::Cognition,
                    RuntimeAuthorityScope::None,
                ),
                entry(
                    "Experience",
                    &["display_reasons", "experience_summary"],
                    &["recommendation_candidates", "decision_candidates"],
                    RuntimeVisibilityScope::Cognition,
                    RuntimeAuthorityScope::None,
                ),
                entry(
                    "GovernanceProjections",
                    &["governance_runtime_summary"],
                    &["decision_package_refs", "compliance_labels"],
                    RuntimeVisibilityScope::Governance,
                    RuntimeAuthorityScope::None,
                ),
                entry(
                    "OperatorProjections",
                    &["operator_context", "runtime_overview"],
                    &["runtime_health", "governance_runtime_summary"],
                    RuntimeVisibilityScope::Operator,
                    RuntimeAuthorityScope::None,
                ),
                entry(
                    "PermissionGateway",
                    &["allow_deny_approval_required"],
                    &["intent_commands"],
                    RuntimeVisibilityScope::Internal,
                    RuntimeAuthorityScope::ExecutionGatewayOnly,
                ),
            ],
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn cognition_entries(&self) -> Vec<&RuntimeCapabilityEntry> {
        self.entries
            .iter()
            .filter(|e| e.visibility == RuntimeVisibilityScope::Cognition)
            .collect()
    }

    pub fn gateway_is_sole_execution_authority(&self) -> bool {
        let gateway = self
            .entries
            .iter()
            .filter(|e| e.authority == RuntimeAuthorityScope::ExecutionGatewayOnly)
            .collect::<Vec<_>>();
        gateway.len() == 1 && gateway[0].subsystem == "PermissionGateway"
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 178 — Runtime Diagnostic Snapshot
// ---------------------------------------------------------------------------

/// Immutable observational runtime snapshot (Sprint 178).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticSnapshot {
    pub id: String,
    pub workspace_id: String,
    pub captured_at: String,
    pub health_overall: WorkspaceHealthLevel,
    pub runtime_context_id: String,
    pub cognition_projection_kinds: Vec<String>,
    pub governance: GovernanceRuntimeSummary,
    pub dependency_node_count: u32,
    pub dependency_edge_count: u32,
    pub dependency_has_cycles: bool,
    pub capability_entry_count: u32,
    pub gateway_sole_execution: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn capture(
        ctx: &WorkspaceRuntimeContext,
        health: &WorkspaceRuntimeHealth,
        cognition: &[CognitionContextProjection],
        graph: &RuntimeDependencyGraph,
        capabilities: &RuntimeCapabilityMap,
        captured_at: impl Into<String>,
    ) -> Self {
        let captured_at = captured_at.into();
        Self {
            id: format!(
                "runtime_diagnostic_snapshot:{}:{}",
                ctx.workspace_id, captured_at
            ),
            workspace_id: ctx.workspace_id.clone(),
            captured_at,
            health_overall: health.overall,
            runtime_context_id: ctx.id.clone(),
            cognition_projection_kinds: cognition
                .iter()
                .map(|c| c.kind.as_str().into())
                .collect(),
            governance: ctx.governance.clone(),
            dependency_node_count: graph.nodes.len() as u32,
            dependency_edge_count: graph.edges.len() as u32,
            dependency_has_cycles: graph.has_cycles(),
            capability_entry_count: capabilities.entries.len() as u32,
            gateway_sole_execution: capabilities.gateway_is_sole_execution_authority(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 179 — Runtime Consistency Verification
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeConsistencyCheckKind {
    MissingDependency,
    InvalidOwnership,
    ProjectionViolation,
    CircularReference,
    InvalidAuthorityCrossing,
}

impl RuntimeConsistencyCheckKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingDependency => "missing_dependency",
            Self::InvalidOwnership => "invalid_ownership",
            Self::ProjectionViolation => "projection_violation",
            Self::CircularReference => "circular_reference",
            Self::InvalidAuthorityCrossing => "invalid_authority_crossing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeConsistencySeverity {
    Info,
    Warning,
    Error,
}

impl RuntimeConsistencySeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeConsistencyDiagnostic {
    pub kind: RuntimeConsistencyCheckKind,
    pub severity: RuntimeConsistencySeverity,
    pub message: String,
    pub reference: Option<String>,
}

/// Runtime architecture verifier — diagnostics only; never repairs (Sprint 179).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeConsistencyVerification {
    pub id: String,
    pub diagnostics: Vec<RuntimeConsistencyDiagnostic>,
    pub checked_at: String,
    pub authority_effect: String,
}

impl RuntimeConsistencyVerification {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn verify(
        graph: &RuntimeDependencyGraph,
        capabilities: &RuntimeCapabilityMap,
        ctx: &WorkspaceRuntimeContext,
        integration: &WorkspaceRuntimeIntegrationContract,
        snapshot: &RuntimeDiagnosticSnapshot,
        at: impl Into<String>,
    ) -> Self {
        let mut diagnostics = Vec::new();

        // Circular references
        for cycle in &graph.circular_dependency_ids {
            diagnostics.push(RuntimeConsistencyDiagnostic {
                kind: RuntimeConsistencyCheckKind::CircularReference,
                severity: RuntimeConsistencySeverity::Error,
                message: format!("circular dependency: {cycle}"),
                reference: Some(graph.id.clone()),
            });
        }

        // Missing required dependency targets
        let node_ids: std::collections::HashSet<_> =
            graph.nodes.iter().map(|n| n.id.as_str()).collect();
        for e in graph.required_edges() {
            if !node_ids.contains(e.to.as_str()) {
                diagnostics.push(RuntimeConsistencyDiagnostic {
                    kind: RuntimeConsistencyCheckKind::MissingDependency,
                    severity: RuntimeConsistencySeverity::Error,
                    message: format!("required dependency target missing: {}→{}", e.from, e.to),
                    reference: Some(e.to.clone()),
                });
            }
            if !node_ids.contains(e.from.as_str()) {
                diagnostics.push(RuntimeConsistencyDiagnostic {
                    kind: RuntimeConsistencyCheckKind::MissingDependency,
                    severity: RuntimeConsistencySeverity::Error,
                    message: format!("required dependency source missing: {}", e.from),
                    reference: Some(e.from.clone()),
                });
            }
        }

        // Ownership: empty owners invalid
        for n in &graph.nodes {
            if n.owner.trim().is_empty() {
                diagnostics.push(RuntimeConsistencyDiagnostic {
                    kind: RuntimeConsistencyCheckKind::InvalidOwnership,
                    severity: RuntimeConsistencySeverity::Error,
                    message: format!("subsystem {} has empty owner", n.id),
                    reference: Some(n.id.clone()),
                });
            }
        }

        // Projection violations
        if ctx.owns_workspace_state() {
            diagnostics.push(RuntimeConsistencyDiagnostic {
                kind: RuntimeConsistencyCheckKind::ProjectionViolation,
                severity: RuntimeConsistencySeverity::Error,
                message: "runtime context must not own WorkspaceState".into(),
                reference: Some(ctx.id.clone()),
            });
        }
        if !integration.all_non_authoritative() {
            diagnostics.push(RuntimeConsistencyDiagnostic {
                kind: RuntimeConsistencyCheckKind::ProjectionViolation,
                severity: RuntimeConsistencySeverity::Error,
                message: "governance integration points must not be authoritative".into(),
                reference: Some(integration.id.clone()),
            });
        }
        if !ctx.governance.publication_blocked || !snapshot.governance.publication_blocked {
            diagnostics.push(RuntimeConsistencyDiagnostic {
                kind: RuntimeConsistencyCheckKind::ProjectionViolation,
                severity: RuntimeConsistencySeverity::Error,
                message: "publication must remain blocked in summaries".into(),
                reference: None,
            });
        }

        // Authority crossings — only Gateway may declare ExecutionGatewayOnly
        if !capabilities.gateway_is_sole_execution_authority() {
            diagnostics.push(RuntimeConsistencyDiagnostic {
                kind: RuntimeConsistencyCheckKind::InvalidAuthorityCrossing,
                severity: RuntimeConsistencySeverity::Error,
                message: "PermissionGateway must be sole ExecutionGatewayOnly authority".into(),
                reference: Some(capabilities.id.clone()),
            });
        }
        for e in &capabilities.entries {
            if e.subsystem != "PermissionGateway"
                && e.authority == RuntimeAuthorityScope::ExecutionGatewayOnly
            {
                diagnostics.push(RuntimeConsistencyDiagnostic {
                    kind: RuntimeConsistencyCheckKind::InvalidAuthorityCrossing,
                    severity: RuntimeConsistencySeverity::Error,
                    message: format!(
                        "subsystem {} must not claim execution authority",
                        e.subsystem
                    ),
                    reference: Some(e.subsystem.clone()),
                });
            }
        }

        // Cognition must not list Gateway as a required inbound dep for Allow
        for e in &graph.edges {
            if e.from == "Attention" && e.to == "PermissionGateway"
                || e.from == "Decision" && e.to == "PermissionGateway"
            {
                diagnostics.push(RuntimeConsistencyDiagnostic {
                    kind: RuntimeConsistencyCheckKind::InvalidAuthorityCrossing,
                    severity: RuntimeConsistencySeverity::Error,
                    message: format!("cognition {} must not depend on PermissionGateway", e.from),
                    reference: Some(e.from.clone()),
                });
            }
        }

        if diagnostics.is_empty() {
            diagnostics.push(RuntimeConsistencyDiagnostic {
                kind: RuntimeConsistencyCheckKind::ProjectionViolation,
                severity: RuntimeConsistencySeverity::Info,
                message: "runtime consistency checks passed".into(),
                reference: Some(graph.id.clone()),
            });
        }

        Self {
            id: format!("runtime_consistency:{}", ctx.workspace_id),
            diagnostics,
            checked_at: at.into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == RuntimeConsistencySeverity::Error)
    }

    pub fn may_repair_automatically(&self) -> bool {
        false
    }

    pub fn attempt_repair(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::ConsistencyCannotMutate)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 180 — Operator Runtime Overview
// ---------------------------------------------------------------------------

/// Complete operator-facing runtime overview projection — not UI (Sprint 180).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorRuntimeOverview {
    pub id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub health_overall: WorkspaceHealthLevel,
    pub runtime_context_id: String,
    pub operator_context_id: String,
    pub diagnostic_snapshot_id: String,
    pub consistency_has_errors: bool,
    pub dependency_summary: String,
    pub capability_summary: String,
    pub governance_summary: String,
    pub coherence_ok: bool,
    pub publication_blocked: bool,
    pub authority_effect: String,
}

impl OperatorRuntimeOverview {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn project(
        ctx: &WorkspaceRuntimeContext,
        health: &WorkspaceRuntimeHealth,
        operator: &OperatorContextProjection,
        snapshot: &RuntimeDiagnosticSnapshot,
        verification: &RuntimeConsistencyVerification,
        graph: &RuntimeDependencyGraph,
        capabilities: &RuntimeCapabilityMap,
        coherence: &WorkspaceRuntimeCoherence,
    ) -> Self {
        Self {
            id: format!("operator_runtime_overview:{}", ctx.workspace_id),
            workspace_id: ctx.workspace_id.clone(),
            generated_at: ctx.generated_at.clone(),
            health_overall: health.overall,
            runtime_context_id: ctx.id.clone(),
            operator_context_id: operator.id.clone(),
            diagnostic_snapshot_id: snapshot.id.clone(),
            consistency_has_errors: verification.has_errors(),
            dependency_summary: format!(
                "nodes={} edges={} cycles={}",
                graph.nodes.len(),
                graph.edges.len(),
                graph.circular_dependency_ids.len()
            ),
            capability_summary: format!(
                "entries={} gateway_sole={}",
                capabilities.entries.len(),
                capabilities.gateway_is_sole_execution_authority()
            ),
            governance_summary: format!(
                "review={:?} compliance={:?} publication_blocked={}",
                ctx.governance.review_workflow_stage,
                ctx.governance.compliance_status,
                ctx.governance.publication_blocked
            ),
            coherence_ok: coherence.coherent,
            publication_blocked: true,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

// ---------------------------------------------------------------------------
// Sprint 181 — Runtime Architecture Coherence Review (extended)
// ---------------------------------------------------------------------------

/// Extended architecture review across diagnostics layer (Sprint 181).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeArchitectureReview {
    pub id: String,
    pub ownership_ok: bool,
    pub dependency_direction_ok: bool,
    pub projection_layering_ok: bool,
    pub authority_boundaries_ok: bool,
    pub aggregate_cohesion_ok: bool,
    pub notes: Vec<String>,
    pub authority_effect: String,
}

impl RuntimeArchitectureReview {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn review(
        graph: &RuntimeDependencyGraph,
        capabilities: &RuntimeCapabilityMap,
        verification: &RuntimeConsistencyVerification,
        overview: &OperatorRuntimeOverview,
        coherence: &WorkspaceRuntimeCoherence,
    ) -> Self {
        let mut notes: Vec<String> = Vec::new();
        let ownership_ok = graph.nodes.iter().all(|n| !n.owner.trim().is_empty());
        if !ownership_ok {
            notes.push("ownership gaps in dependency graph".into());
        }

        // Required edges: consumer → provider; consumer layer rank must be >= provider.
        let layer_rank = |id: &str| -> u8 {
            match id {
                "WorkspaceState" => 1,
                "Environment" => 2,
                "Attention" => 3,
                "Intelligence" | "Decision" | "Recommendation" => 4,
                "Experience" => 5,
                "GovernanceProjections" => 6,
                "OperatorProjections" => 7,
                "PermissionGateway" => 9,
                _ => 5,
            }
        };
        let mut dependency_direction_ok = true;
        for e in &graph.edges {
            if e.kind == RuntimeDependencyKind::Required
                && layer_rank(&e.from) < layer_rank(&e.to)
            {
                dependency_direction_ok = false;
                notes.push(format!(
                    "required edge reverses layering: {}→{}",
                    e.from, e.to
                ));
            }
        }

        let projection_layering_ok = overview.publication_blocked
            && coherence.coherent
            && !verification.has_errors();
        if !projection_layering_ok {
            notes.push("projection layering or coherence/verification failed".into());
        }

        let authority_boundaries_ok = capabilities.gateway_is_sole_execution_authority()
            && overview.authority_effect == Self::AUTHORITY_EFFECT_NONE;
        if !authority_boundaries_ok {
            notes.push("authority boundary violation".into());
        }

        let aggregate_cohesion_ok = graph.nodes.len() >= 8
            && capabilities.entries.len() >= 8
            && !graph.has_cycles();
        if !aggregate_cohesion_ok {
            notes.push("aggregate cohesion insufficient or cycles present".into());
        }

        if notes.is_empty() {
            notes.push("runtime architecture review passed".into());
        }

        Self {
            id: format!("runtime_architecture_review:{}", overview.workspace_id),
            ownership_ok,
            dependency_direction_ok,
            projection_layering_ok,
            authority_boundaries_ok,
            aggregate_cohesion_ok,
            notes,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn passed(&self) -> bool {
        self.ownership_ok
            && self.dependency_direction_ok
            && self.projection_layering_ok
            && self.authority_boundaries_ok
            && self.aggregate_cohesion_ok
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

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

// ---------------------------------------------------------------------------
// Runtime Diagnostic Consumption & Interpretation (post–integrity audit)
// ---------------------------------------------------------------------------
//
// Evidence/evolution/archives exist, but consumers lacked a contract forbidding
// command/recommendation/authority interpretations, and findings lacked
// structured confidence/scope/limitations. Restoration is read-only rehydration.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticConsumerKind {
    OperatorProjection,
    RuntimeOverview,
    ArchitectureReview,
    ArchiveInspect,
}

impl RuntimeDiagnosticConsumerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OperatorProjection => "operator_projection",
            Self::RuntimeOverview => "runtime_overview",
            Self::ArchitectureReview => "architecture_review",
            Self::ArchiveInspect => "archive_inspect",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticConsumptionMode {
    Observe,
    Explain,
    ArchiveInspect,
}

impl RuntimeDiagnosticConsumptionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Explain => "explain",
            Self::ArchiveInspect => "archive_inspect",
        }
    }

    pub fn is_allowed(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticForbiddenInterpretation {
    Command,
    Recommendation,
    Authority,
    Approval,
    ExperienceTranslation,
    GovernanceDecision,
}

impl RuntimeDiagnosticForbiddenInterpretation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Command => "command",
            Self::Recommendation => "recommendation",
            Self::Authority => "authority",
            Self::Approval => "approval",
            Self::ExperienceTranslation => "experience_translation",
            Self::GovernanceDecision => "governance_decision",
        }
    }
}

/// How diagnostic outputs may be consumed — observational only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticConsumptionContract {
    pub id: String,
    pub allowed_consumers: Vec<RuntimeDiagnosticConsumerKind>,
    pub allowed_modes: Vec<RuntimeDiagnosticConsumptionMode>,
    pub forbidden_interpretations: Vec<RuntimeDiagnosticForbiddenInterpretation>,
    pub may_drive_experience: bool,
    pub may_drive_governance: bool,
    pub may_enter_command_pipeline: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticConsumptionContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn canonical() -> Self {
        Self {
            id: "runtime_diagnostic_consumption:canonical".into(),
            allowed_consumers: vec![
                RuntimeDiagnosticConsumerKind::OperatorProjection,
                RuntimeDiagnosticConsumerKind::RuntimeOverview,
                RuntimeDiagnosticConsumerKind::ArchitectureReview,
                RuntimeDiagnosticConsumerKind::ArchiveInspect,
            ],
            allowed_modes: vec![
                RuntimeDiagnosticConsumptionMode::Observe,
                RuntimeDiagnosticConsumptionMode::Explain,
                RuntimeDiagnosticConsumptionMode::ArchiveInspect,
            ],
            forbidden_interpretations: vec![
                RuntimeDiagnosticForbiddenInterpretation::Command,
                RuntimeDiagnosticForbiddenInterpretation::Recommendation,
                RuntimeDiagnosticForbiddenInterpretation::Authority,
                RuntimeDiagnosticForbiddenInterpretation::Approval,
                RuntimeDiagnosticForbiddenInterpretation::ExperienceTranslation,
                RuntimeDiagnosticForbiddenInterpretation::GovernanceDecision,
            ],
            may_drive_experience: false,
            may_drive_governance: false,
            may_enter_command_pipeline: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn allows_consumer(&self, consumer: RuntimeDiagnosticConsumerKind) -> bool {
        self.allowed_consumers.contains(&consumer)
    }

    pub fn forbids(&self, interpretation: RuntimeDiagnosticForbiddenInterpretation) -> bool {
        self.forbidden_interpretations.contains(&interpretation)
    }

    pub fn is_safe(&self) -> bool {
        !self.may_drive_experience
            && !self.may_drive_governance
            && !self.may_enter_command_pipeline
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.forbids(RuntimeDiagnosticForbiddenInterpretation::Command)
            && self.forbids(RuntimeDiagnosticForbiddenInterpretation::Authority)
    }

    pub fn attempt_as_command(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticConsumptionForbidden)
    }

    pub fn attempt_as_recommendation(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticConsumptionForbidden)
    }

    pub fn attempt_enter_command_pipeline(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticConsumptionForbidden)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticConfidence {
    High,
    Medium,
    Low,
    Unknown,
}

impl RuntimeDiagnosticConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticFindingScope {
    Workspace,
    Subsystem,
    AuthorityBoundary,
    Projection,
    Lifecycle,
}

impl RuntimeDiagnosticFindingScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Subsystem => "subsystem",
            Self::AuthorityBoundary => "authority_boundary",
            Self::Projection => "projection",
            Self::Lifecycle => "lifecycle",
        }
    }
}

/// Structured observational finding — not a recommendation or command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticFinding {
    pub id: String,
    pub severity: RuntimeConsistencySeverity,
    pub confidence: RuntimeDiagnosticConfidence,
    pub scope: RuntimeDiagnosticFindingScope,
    pub source: String,
    pub detail: String,
    pub limitations: Vec<String>,
    pub is_recommendation: bool,
    pub is_command: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticFinding {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn observational(
        id: impl Into<String>,
        severity: RuntimeConsistencySeverity,
        confidence: RuntimeDiagnosticConfidence,
        scope: RuntimeDiagnosticFindingScope,
        source: impl Into<String>,
        detail: impl Into<String>,
        limitations: Vec<String>,
    ) -> Self {
        Self {
            id: id.into(),
            severity,
            confidence,
            scope,
            source: source.into(),
            detail: detail.into(),
            limitations,
            is_recommendation: false,
            is_command: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn is_actionable(&self) -> bool {
        false
    }
}

/// Operator-safe interpretation view derived from evolution — observational only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticInterpretationView {
    pub id: String,
    pub workspace_id: String,
    pub evolution_report_id: String,
    pub findings: Vec<RuntimeDiagnosticFinding>,
    pub consumption: RuntimeDiagnosticConsumptionContract,
    pub limitations: Vec<String>,
    pub authority_effect: String,
}

impl RuntimeDiagnosticInterpretationView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn from_evolution(evolution: &RuntimeDiagnosticEvolutionReport) -> Self {
        let consumption = RuntimeDiagnosticConsumptionContract::canonical();
        let mut findings = Vec::new();
        if let Some(cmp) = &evolution.comparison {
            for (i, d) in cmp.deltas.iter().enumerate() {
                let (confidence, scope, limitations) = match d.kind {
                    RuntimeDiagnosticDeltaKind::GatewayAuthoritySignal => (
                        RuntimeDiagnosticConfidence::High,
                        RuntimeDiagnosticFindingScope::AuthorityBoundary,
                        vec![
                            "observational_signal_only".into(),
                            "does_not_change_permission_gateway".into(),
                        ],
                    ),
                    RuntimeDiagnosticDeltaKind::HealthOverall => (
                        RuntimeDiagnosticConfidence::Medium,
                        RuntimeDiagnosticFindingScope::Workspace,
                        vec!["health_is_observational_not_prescriptive".into()],
                    ),
                    RuntimeDiagnosticDeltaKind::DependencyTopology => (
                        RuntimeDiagnosticConfidence::High,
                        RuntimeDiagnosticFindingScope::Subsystem,
                        vec!["topology_labels_only".into()],
                    ),
                    RuntimeDiagnosticDeltaKind::GovernanceSummary => (
                        RuntimeDiagnosticConfidence::Medium,
                        RuntimeDiagnosticFindingScope::Projection,
                        vec![
                            "governance_summary_not_authoritative".into(),
                            "not_a_governance_decision".into(),
                        ],
                    ),
                    RuntimeDiagnosticDeltaKind::None => (
                        RuntimeDiagnosticConfidence::High,
                        RuntimeDiagnosticFindingScope::Workspace,
                        vec!["no_delta_does_not_imply_health".into()],
                    ),
                    _ => (
                        RuntimeDiagnosticConfidence::Medium,
                        RuntimeDiagnosticFindingScope::Subsystem,
                        vec!["diagnostic_interpretation_only".into()],
                    ),
                };
                findings.push(RuntimeDiagnosticFinding::observational(
                    format!("finding:{}:{}", evolution.id, i),
                    d.severity,
                    confidence,
                    scope,
                    format!("comparison:{}", cmp.id),
                    d.detail.clone(),
                    limitations,
                ));
            }
        } else {
            findings.push(RuntimeDiagnosticFinding::observational(
                format!("finding:{}:initial", evolution.id),
                RuntimeConsistencySeverity::Info,
                RuntimeDiagnosticConfidence::Medium,
                RuntimeDiagnosticFindingScope::Lifecycle,
                evolution.id.clone(),
                "initial_diagnostic_snapshot",
                vec![
                    "no_prior_snapshot".into(),
                    "not_a_recommendation".into(),
                ],
            ));
        }
        let limitations = vec![
            "findings_are_observational".into(),
            "not_commands".into(),
            "not_recommendations".into(),
            "not_experience_translation".into(),
            "not_governance_authority".into(),
            "permission_gateway_sole_execution".into(),
        ];
        Self {
            id: format!("runtime_diagnostic_interpretation:{}", evolution.workspace_id),
            workspace_id: evolution.workspace_id.clone(),
            evolution_report_id: evolution.id.clone(),
            findings,
            consumption,
            limitations,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn all_findings_non_actionable(&self) -> bool {
        self.findings.iter().all(|f| {
            !f.is_actionable() && !f.is_command && !f.is_recommendation
        }) && self.consumption.is_safe()
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_as_recommendation(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticConsumptionForbidden)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeProjectionBoundaryLayer {
    RuntimeDiagnostics,
    OperatorProjection,
    Experience,
    GovernanceProjection,
    AuditHistory,
    WorkContinuity,
}

impl RuntimeProjectionBoundaryLayer {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeDiagnostics => "runtime_diagnostics",
            Self::OperatorProjection => "operator_projection",
            Self::Experience => "experience",
            Self::GovernanceProjection => "governance_projection",
            Self::AuditHistory => "audit_history",
            Self::WorkContinuity => "work_continuity",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeProjectionBoundaryEntry {
    pub layer: RuntimeProjectionBoundaryLayer,
    pub may_consume_diagnostics: bool,
    pub may_emit_commands: bool,
    pub owns_scoring: bool,
    pub notes: String,
}

/// Projection boundary registry — keeps diagnostics distinct from Experience/Governance/Audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeProjectionBoundaryRegistry {
    pub entries: Vec<RuntimeProjectionBoundaryEntry>,
    pub authority_effect: String,
}

impl RuntimeProjectionBoundaryRegistry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn canonical() -> Self {
        let entry = |layer: RuntimeProjectionBoundaryLayer,
                     may_consume_diagnostics: bool,
                     may_emit_commands: bool,
                     owns_scoring: bool,
                     notes: &str| RuntimeProjectionBoundaryEntry {
            layer,
            may_consume_diagnostics,
            may_emit_commands,
            owns_scoring,
            notes: notes.into(),
        };
        Self {
            entries: vec![
                entry(
                    RuntimeProjectionBoundaryLayer::RuntimeDiagnostics,
                    true,
                    false,
                    false,
                    "produces observational findings only",
                ),
                entry(
                    RuntimeProjectionBoundaryLayer::OperatorProjection,
                    true,
                    false,
                    false,
                    "may explain diagnostics; never executes",
                ),
                entry(
                    RuntimeProjectionBoundaryLayer::Experience,
                    false,
                    false,
                    false,
                    "experience translation ownership unchanged; diagnostics must not drive it",
                ),
                entry(
                    RuntimeProjectionBoundaryLayer::GovernanceProjection,
                    false,
                    false,
                    false,
                    "governance summaries visible elsewhere; diagnostics do not decide",
                ),
                entry(
                    RuntimeProjectionBoundaryLayer::AuditHistory,
                    false,
                    false,
                    false,
                    "AuditService command trail ≠ diagnostic archive",
                ),
                entry(
                    RuntimeProjectionBoundaryLayer::WorkContinuity,
                    false,
                    false,
                    false,
                    "work resume facets ≠ diagnostic continuity",
                ),
            ],
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn boundaries_respected(&self) -> bool {
        self.entries.iter().all(|e| !e.may_emit_commands)
            && self
                .entries
                .iter()
                .find(|e| e.layer == RuntimeProjectionBoundaryLayer::Experience)
                .is_some_and(|e| !e.may_consume_diagnostics && !e.owns_scoring)
            && self
                .entries
                .iter()
                .find(|e| e.layer == RuntimeProjectionBoundaryLayer::GovernanceProjection)
                .is_some_and(|e| !e.may_consume_diagnostics)
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Read-only rehydration of archived diagnostic evidence — never mutates archive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticRestorationView {
    pub id: String,
    pub workspace_id: String,
    pub archive_entry_id: String,
    pub evidence_bundle_id: Option<String>,
    pub restored_refs: Vec<String>,
    pub restored_at: String,
    pub may_mutate: bool,
    pub may_delete: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticRestorationView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn rehydrate(
        archive: &RuntimeDiagnosticArchive,
        entry: &RuntimeDiagnosticArchiveEntry,
        evidence: Option<&RuntimeDiagnosticEvidenceBundle>,
        at: impl Into<String>,
    ) -> Self {
        let mut restored_refs = vec![entry.source_reference.clone(), entry.digest.clone()];
        if let Some(eid) = &entry.evidence_bundle_id {
            restored_refs.push(eid.clone());
        }
        if let Some(ev) = evidence {
            for r in &ev.refs {
                restored_refs.push(r.artifact_id.clone());
            }
        }
        Self {
            id: format!(
                "runtime_diagnostic_restoration:{}:{}",
                archive.workspace_id, entry.id
            ),
            workspace_id: archive.workspace_id.clone(),
            archive_entry_id: entry.id.clone(),
            evidence_bundle_id: entry.evidence_bundle_id.clone(),
            restored_refs,
            restored_at: at.into(),
            may_mutate: false,
            may_delete: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn attempt_mutate(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticArchiveImmutable)
    }

    pub fn attempt_delete(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticArchiveImmutable)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

/// Observational continuity between diagnostic snapshots — not work continuity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

/// Operator-facing explanation of runtime diagnostics — projection only, not UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorRuntimeExplanation {
    pub id: String,
    pub workspace_id: String,
    pub overview_id: String,
    pub provenance_id: String,
    pub continuity_id: String,
    pub why: Vec<String>,
    pub publication_blocked: bool,
    pub authority_effect: String,
}

impl OperatorRuntimeExplanation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn explain(
        overview: &OperatorRuntimeOverview,
        provenance: &RuntimeDiagnosticProvenance,
        continuity: &RuntimeDiagnosticContinuityRecord,
    ) -> Self {
        let mut why = vec![
            format!("health={}", overview.health_overall.as_str()),
            format!("coherence_ok={}", overview.coherence_ok),
            format!("consistency_has_errors={}", overview.consistency_has_errors),
            format!("dependency={}", overview.dependency_summary),
            format!("capability={}", overview.capability_summary),
            format!("governance={}", overview.governance_summary),
            format!("provenance_sources={}", provenance.source_refs.len()),
        ];
        for delta in &continuity.what_changed {
            why.push(format!("continuity:{delta}"));
        }
        why.push("execution_authority=permission_gateway_only".into());
        Self {
            id: format!("operator_runtime_explanation:{}", overview.workspace_id),
            workspace_id: overview.workspace_id.clone(),
            overview_id: overview.id.clone(),
            provenance_id: provenance.id.clone(),
            continuity_id: continuity.id.clone(),
            why,
            publication_blocked: true,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Explain using evolution report safe summaries — still not an execution surface.
    pub fn explain_with_evolution(
        overview: &OperatorRuntimeOverview,
        provenance: &RuntimeDiagnosticProvenance,
        continuity: &RuntimeDiagnosticContinuityRecord,
        evolution: &RuntimeDiagnosticEvolutionReport,
    ) -> Self {
        let mut base = Self::explain(overview, provenance, continuity);
        for line in &evolution.operator_safe_summary {
            if !base.why.iter().any(|w| w == line) {
                base.why.push(line.clone());
            }
        }
        base.why.push(format!("evolution_passed={}", evolution.passed()));
        base
    }

    /// Explain with structured interpretation findings — never recommendations/actions.
    pub fn explain_with_interpretation(
        overview: &OperatorRuntimeOverview,
        provenance: &RuntimeDiagnosticProvenance,
        continuity: &RuntimeDiagnosticContinuityRecord,
        evolution: &RuntimeDiagnosticEvolutionReport,
        interpretation: &RuntimeDiagnosticInterpretationView,
    ) -> Self {
        let mut base = Self::explain_with_evolution(overview, provenance, continuity, evolution);
        base.why.push(format!(
            "findings={} non_actionable={}",
            interpretation.findings.len(),
            interpretation.all_findings_non_actionable()
        ));
        for limit in &interpretation.limitations {
            let line = format!("limitation:{limit}");
            if !base.why.iter().any(|w| w == &line) {
                base.why.push(line);
            }
        }
        base.why.push("consumption=observe_explain_only".into());
        base
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn is_execution_surface(&self) -> bool {
        false
    }

    pub fn exposes_actions(&self) -> bool {
        self.why.iter().any(|w| {
            let l = w.to_lowercase();
            l.contains("repair")
                || l.contains("auto_heal")
                || l.contains("approve:")
                || l.starts_with("action:")
                || l.contains("may_execute=true")
        })
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_runtime::{
        CognitionContextProjection, GovernanceRuntimeSummary, OperatorContextProjection,
        WorkspaceRuntimeCoherence, WorkspaceRuntimeContext, WorkspaceRuntimeHealth,
        WorkspaceRuntimeIntegrationContract,
    };

    fn sample_bundle() -> (
        WorkspaceRuntimeContext,
        WorkspaceRuntimeHealth,
        RuntimeDependencyGraph,
        RuntimeCapabilityMap,
        RuntimeDiagnosticSnapshot,
        RuntimeConsistencyVerification,
        OperatorRuntimeOverview,
        RuntimeArchitectureReview,
    ) {
        let ctx = WorkspaceRuntimeContext::assemble(
            "ws-diag",
            "t0",
            None,
            None,
            None,
            None,
            None,
            GovernanceRuntimeSummary::empty(),
        );
        let health = WorkspaceRuntimeHealth::observe(&ctx);
        let cognition = CognitionContextProjection::project_all(&ctx);
        let graph = RuntimeDependencyGraph::canonical("ws-diag");
        let capabilities = RuntimeCapabilityMap::inventory("ws-diag");
        let snapshot = RuntimeDiagnosticSnapshot::capture(
            &ctx,
            &health,
            &cognition,
            &graph,
            &capabilities,
            "t-snap",
        );
        let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-diag");
        let verification = RuntimeConsistencyVerification::verify(
            &graph,
            &capabilities,
            &ctx,
            &integration,
            &snapshot,
            "t-check",
        );
        let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
        let coherence =
            WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
        let overview = OperatorRuntimeOverview::project(
            &ctx,
            &health,
            &operator,
            &snapshot,
            &verification,
            &graph,
            &capabilities,
            &coherence,
        );
        let review = RuntimeArchitectureReview::review(
            &graph,
            &capabilities,
            &verification,
            &overview,
            &coherence,
        );
        (
            ctx,
            health,
            graph,
            capabilities,
            snapshot,
            verification,
            overview,
            review,
        )
    }

    #[test]
    fn dependency_graph_has_no_cycles_and_required_edges() {
        let graph = RuntimeDependencyGraph::canonical("ws");
        assert!(!graph.has_cycles());
        assert!(!graph.required_edges().is_empty());
        assert!(RuntimeDependencyGraph::attempt_execute().is_err());
    }

    #[test]
    fn capability_map_keeps_gateway_sole_execution() {
        let map = RuntimeCapabilityMap::inventory("ws");
        assert!(map.gateway_is_sole_execution_authority());
        assert!(!map.cognition_entries().is_empty());
    }

    #[test]
    fn snapshot_verification_overview_review_are_read_only() {
        let (_ctx, _h, graph, _cap, snapshot, verification, overview, review) = sample_bundle();
        assert!(!snapshot.dependency_has_cycles);
        assert!(snapshot.gateway_sole_execution);
        assert!(!verification.has_errors());
        assert!(!verification.may_repair_automatically());
        assert!(verification.attempt_repair().is_err());
        assert!(overview.publication_blocked);
        assert!(overview.coherence_ok);
        assert!(review.passed());
        assert!(!graph.has_cycles());
    }

    #[test]
    fn diagnostic_provenance_and_continuity_are_immutable() {
        let (ctx, health, graph, capabilities, snapshot, verification, overview, _review) =
            sample_bundle();
        let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-diag");
        let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
        let coherence =
            WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
        let provenance = RuntimeDiagnosticProvenance::from_capture(
            &snapshot,
            &ctx,
            &health,
            &graph,
            &capabilities,
            &verification,
            &coherence,
            Some(&overview),
        );
        assert!(provenance.cites_snapshot(&snapshot.id));
        assert!(provenance.ownership.boundaries_respected());
        assert!(!provenance.may_rewrite_history());
        assert!(provenance.attempt_rewrite_history().is_err());
        assert!(RuntimeDiagnosticOwnershipBoundary::canonical()
            .attempt_heal()
            .is_err());

        let continuity = RuntimeDiagnosticContinuityRecord::link(
            None,
            &snapshot,
            &provenance,
            "t-cont-0",
        );
        assert!(continuity.is_initial());
        assert!(!continuity.may_mutate_prior());
        assert!(continuity.attempt_mutate_prior().is_err());

        let snapshot2 = RuntimeDiagnosticSnapshot::capture(
            &ctx,
            &health,
            &CognitionContextProjection::project_all(&ctx),
            &graph,
            &capabilities,
            "t-snap-2",
        );
        let continuity2 = RuntimeDiagnosticContinuityRecord::link(
            Some(&snapshot),
            &snapshot2,
            &provenance,
            "t-cont-1",
        );
        assert!(!continuity2.is_initial());
        assert!(continuity2
            .what_changed
            .iter()
            .any(|d| d == "no_observational_delta"));

        let explanation =
            OperatorRuntimeExplanation::explain(&overview, &provenance, &continuity);
        assert!(!explanation.is_execution_surface());
        assert!(explanation.publication_blocked);
        assert!(OperatorRuntimeExplanation::attempt_execute().is_err());
    }

    #[test]
    fn diagnostic_evolution_compares_validates_and_stays_read_only() {
        let (ctx, health, graph, capabilities, snapshot, verification, overview, _review) =
            sample_bundle();
        let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-diag");
        let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
        let coherence =
            WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
        let provenance = RuntimeDiagnosticProvenance::from_capture(
            &snapshot,
            &ctx,
            &health,
            &graph,
            &capabilities,
            &verification,
            &coherence,
            Some(&overview),
        );
        let continuity =
            RuntimeDiagnosticContinuityRecord::link(None, &snapshot, &provenance, "e0");
        let evolution = RuntimeDiagnosticEvolutionReport::evaluate(
            None,
            &snapshot,
            &provenance,
            &continuity,
            Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
            "e-eval-0",
        );
        assert!(evolution.passed());
        assert!(evolution.lifecycle_ordering_ok);
        assert!(evolution.may_repair() == false);
        assert!(evolution.attempt_repair().is_err());

        let snapshot2 = RuntimeDiagnosticSnapshot::capture(
            &ctx,
            &health,
            &CognitionContextProjection::project_all(&ctx),
            &graph,
            &capabilities,
            "t-snap-evo-2",
        );
        let comparison = RuntimeDiagnosticComparison::compare(&snapshot, &snapshot2);
        assert!(comparison
            .deltas
            .iter()
            .any(|d| d.kind == RuntimeDiagnosticDeltaKind::None));
        let continuity2 = RuntimeDiagnosticContinuityRecord::link(
            Some(&snapshot),
            &snapshot2,
            &provenance,
            "e1",
        );
        // Provenance cites snap1; evolution against snap2 should fail provenance consistency.
        let evolution_mismatch = RuntimeDiagnosticEvolutionReport::evaluate(
            Some(&snapshot),
            &snapshot2,
            &provenance,
            &continuity2,
            Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
            "e-eval-1",
        );
        assert!(!evolution_mismatch.provenance_consistent);

        let provenance2 = RuntimeDiagnosticProvenance::from_capture(
            &snapshot2,
            &ctx,
            &health,
            &graph,
            &capabilities,
            &verification,
            &coherence,
            Some(&overview),
        );
        let continuity3 = RuntimeDiagnosticContinuityRecord::link(
            Some(&snapshot),
            &snapshot2,
            &provenance2,
            "e2",
        );
        let evolution_ok = RuntimeDiagnosticEvolutionReport::evaluate(
            Some(&snapshot),
            &snapshot2,
            &provenance2,
            &continuity3,
            Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
            "e-eval-2",
        );
        assert!(evolution_ok.passed());
        assert!(evolution_ok.continuity_consistent);

        let registry = RuntimeArchitectureOwnershipRegistry::canonical();
        assert!(registry.operator_artifacts_are_projections_only());
        assert_eq!(
            registry.owner_of("RuntimeDependencyGraph"),
            Some(RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics)
        );

        let explanation = OperatorRuntimeExplanation::explain_with_evolution(
            &overview,
            &provenance2,
            &continuity3,
            &evolution_ok,
        );
        assert!(!explanation.is_execution_surface());
        assert!(!explanation.exposes_actions());
        assert!(RuntimeDiagnosticLifecyclePhase::Captured
            .may_transition_to(RuntimeDiagnosticLifecyclePhase::Provenanced));
        assert!(!RuntimeDiagnosticLifecyclePhase::Captured
            .may_transition_to(RuntimeDiagnosticLifecyclePhase::Superseded));
        assert!(RuntimeDiagnosticLifecyclePhase::ContinuityLinked
            .may_transition_to(RuntimeDiagnosticLifecyclePhase::Compared));
        assert!(RuntimeDiagnosticLifecyclePhase::Evolved
            .may_transition_to(RuntimeDiagnosticLifecyclePhase::Archived));
    }

    #[test]
    fn diagnostic_evidence_archive_and_integrity_are_non_authoritative() {
        let (ctx, health, graph, capabilities, snapshot, verification, overview, _review) =
            sample_bundle();
        let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-diag");
        let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
        let coherence =
            WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
        let provenance = RuntimeDiagnosticProvenance::from_capture(
            &snapshot,
            &ctx,
            &health,
            &graph,
            &capabilities,
            &verification,
            &coherence,
            Some(&overview),
        );
        let continuity =
            RuntimeDiagnosticContinuityRecord::link(None, &snapshot, &provenance, "arch0");
        let evolution = RuntimeDiagnosticEvolutionReport::evaluate(
            None,
            &snapshot,
            &provenance,
            &continuity,
            Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
            "arch-eval",
        );
        assert!(evolution.passed());
        assert!(!evolution.is_authoritative());
        assert!(!evolution.is_decision());
        assert!(evolution.attempt_promote_to_decision().is_err());
        assert_eq!(evolution.lifecycle.phase, RuntimeDiagnosticLifecyclePhase::Evolved);

        let evidence = RuntimeDiagnosticEvidenceBundle::seal(
            &evolution,
            &provenance,
            &continuity,
            "seal0",
        );
        assert!(evidence.is_complete());
        assert!(!evidence.is_authoritative());
        assert!(evidence.attempt_promote_to_decision().is_err());

        let mut archive = RuntimeDiagnosticArchive::new("ws-diag");
        assert!(archive.archive_evidence(&evidence, "a0").is_ok());
        assert!(archive.mark_superseded(&snapshot.id, "snap-next", "a1").is_ok());
        assert!(!archive.may_delete());
        assert!(archive.attempt_delete().is_err());
        assert!(archive.attempt_mutate_entry().is_err());

        let ownership = RuntimeArchitectureOwnershipRegistry::canonical().validate();
        assert!(ownership.conflict_free);
        assert_eq!(
            ownership.registry_version,
            RuntimeArchitectureOwnershipRegistry::REGISTRY_VERSION
        );

        let integrity = RuntimeDiagnosticHistoricalIntegrity::verify(
            &evidence,
            &archive,
            &evolution,
            None,
            &ownership,
        );
        assert!(integrity.passed());
        assert!(!integrity.may_repair());
        assert!(integrity.attempt_repair().is_err());
    }

    #[test]
    fn diagnostic_consumption_interpretation_and_restoration_are_read_only() {
        let (ctx, health, graph, capabilities, snapshot, verification, overview, _review) =
            sample_bundle();
        let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-diag");
        let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
        let coherence =
            WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
        let provenance = RuntimeDiagnosticProvenance::from_capture(
            &snapshot,
            &ctx,
            &health,
            &graph,
            &capabilities,
            &verification,
            &coherence,
            Some(&overview),
        );
        let continuity =
            RuntimeDiagnosticContinuityRecord::link(None, &snapshot, &provenance, "c-cons");
        let evolution = RuntimeDiagnosticEvolutionReport::evaluate(
            None,
            &snapshot,
            &provenance,
            &continuity,
            Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
            "eval-cons",
        );

        let contract = RuntimeDiagnosticConsumptionContract::canonical();
        assert!(contract.is_safe());
        assert!(contract.allows_consumer(RuntimeDiagnosticConsumerKind::OperatorProjection));
        assert!(contract.attempt_as_command().is_err());
        assert!(contract.attempt_as_recommendation().is_err());
        assert!(contract.attempt_enter_command_pipeline().is_err());

        let interpretation = RuntimeDiagnosticInterpretationView::from_evolution(&evolution);
        assert!(interpretation.all_findings_non_actionable());
        assert!(!interpretation.findings.is_empty());
        assert!(interpretation
            .findings
            .iter()
            .all(|f| !f.limitations.is_empty()));
        assert!(interpretation.attempt_as_recommendation().is_err());

        let boundaries = RuntimeProjectionBoundaryRegistry::canonical();
        assert!(boundaries.boundaries_respected());

        let evidence = RuntimeDiagnosticEvidenceBundle::seal(
            &evolution,
            &provenance,
            &continuity,
            "seal-cons",
        );
        let mut archive = RuntimeDiagnosticArchive::new("ws-diag");
        let entry = archive.archive_evidence(&evidence, "a-cons").unwrap().clone();
        let restoration = RuntimeDiagnosticRestorationView::rehydrate(
            &archive,
            &entry,
            Some(&evidence),
            "rest0",
        );
        assert!(!restoration.may_mutate);
        assert!(!restoration.may_delete);
        assert!(restoration.attempt_mutate().is_err());
        assert!(restoration.attempt_delete().is_err());

        let explanation = OperatorRuntimeExplanation::explain_with_interpretation(
            &overview,
            &provenance,
            &continuity,
            &evolution,
            &interpretation,
        );
        assert!(!explanation.is_execution_surface());
        assert!(!explanation.exposes_actions());
    }
}
