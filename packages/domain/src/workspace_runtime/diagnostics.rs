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
}
