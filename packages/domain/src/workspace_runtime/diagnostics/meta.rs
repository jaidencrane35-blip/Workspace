//! Runtime diagnostics — observational only.
//!
//! No execution, automation, repair, recommendations, or authority.

use serde::{Deserialize, Serialize};

use crate::action_proposal::GOVERNANCE_AUTHORITY_EFFECT_NONE as AUTH_NONE;
use crate::workspace_runtime::{WorkspaceHealthLevel, WorkspaceRuntimeError};

use super::{
    OperatorRuntimeOverview, RuntimeDiagnosticContinuityRecord,
    RuntimeDiagnosticEvolutionReport, RuntimeDiagnosticInterpretationView,
    RuntimeDiagnosticLifecyclePhase, RuntimeDiagnosticOwnershipRole,
    RuntimeDiagnosticProvenance, RuntimeDiagnosticRestorationView, RuntimeDiagnosticSnapshot,
    RuntimeProjectionBoundaryRegistry,
};

// ---------------------------------------------------------------------------
// Runtime Diagnostic Trust, Compatibility & Lineage (post–consumption audit)
// ---------------------------------------------------------------------------
//
// Consumers could not determine contract version, producer identity, or whether
// findings were current vs historical. Compatibility is identity-only — no
// migration/apply behaviour. Distinct from GovernanceCompatibilityContract.

/// Shared contract family version for runtime diagnostics (compatibility identity).
pub const RUNTIME_DIAGNOSTICS_CONTRACT_VERSION: &str = "runtime_diagnostics:v1";
pub const RUNTIME_DIAGNOSTICS_SCHEMA_VERSION: i32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticContractFamily {
    Snapshot,
    Provenance,
    Continuity,
    Comparison,
    Evolution,
    Evidence,
    Archive,
    Consumption,
    Interpretation,
    Ownership,
    Restoration,
    Lineage,
    Trust,
}

impl RuntimeDiagnosticContractFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
            Self::Provenance => "provenance",
            Self::Continuity => "continuity",
            Self::Comparison => "comparison",
            Self::Evolution => "evolution",
            Self::Evidence => "evidence",
            Self::Archive => "archive",
            Self::Consumption => "consumption",
            Self::Interpretation => "interpretation",
            Self::Ownership => "ownership",
            Self::Restoration => "restoration",
            Self::Lineage => "lineage",
            Self::Trust => "trust",
        }
    }
}

/// Compatibility identity for a diagnostic contract family — not a migrator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticContractIdentity {
    pub family: RuntimeDiagnosticContractFamily,
    pub contract_version: String,
    pub schema_version: i32,
}

impl RuntimeDiagnosticContractIdentity {
    pub fn canonical(family: RuntimeDiagnosticContractFamily) -> Self {
        Self {
            family,
            contract_version: RUNTIME_DIAGNOSTICS_CONTRACT_VERSION.into(),
            schema_version: RUNTIME_DIAGNOSTICS_SCHEMA_VERSION,
        }
    }

    pub fn matches_floor(&self, floor: i32) -> bool {
        self.schema_version >= floor && floor >= 1
    }
}

/// Declares diagnostic contract compatibility — identity only; no apply path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticCompatibilityContract {
    pub id: String,
    pub identities: Vec<RuntimeDiagnosticContractIdentity>,
    pub schema_version_floor: i32,
    pub may_migrate: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticCompatibilityContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;
    pub const DEFAULT_SCHEMA_VERSION_FLOOR: i32 = RUNTIME_DIAGNOSTICS_SCHEMA_VERSION;

    pub fn canonical() -> Self {
        let families = [
            RuntimeDiagnosticContractFamily::Snapshot,
            RuntimeDiagnosticContractFamily::Provenance,
            RuntimeDiagnosticContractFamily::Continuity,
            RuntimeDiagnosticContractFamily::Comparison,
            RuntimeDiagnosticContractFamily::Evolution,
            RuntimeDiagnosticContractFamily::Evidence,
            RuntimeDiagnosticContractFamily::Archive,
            RuntimeDiagnosticContractFamily::Consumption,
            RuntimeDiagnosticContractFamily::Interpretation,
            RuntimeDiagnosticContractFamily::Ownership,
            RuntimeDiagnosticContractFamily::Restoration,
            RuntimeDiagnosticContractFamily::Lineage,
            RuntimeDiagnosticContractFamily::Trust,
        ];
        Self {
            id: "runtime_diagnostic_compatibility:canonical".into(),
            identities: families
                .into_iter()
                .map(RuntimeDiagnosticContractIdentity::canonical)
                .collect(),
            schema_version_floor: Self::DEFAULT_SCHEMA_VERSION_FLOOR,
            may_migrate: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn is_compatible(&self, identity: &RuntimeDiagnosticContractIdentity) -> bool {
        identity.contract_version == RUNTIME_DIAGNOSTICS_CONTRACT_VERSION
            && identity.matches_floor(self.schema_version_floor)
            && self.identities.iter().any(|i| i.family == identity.family)
    }

    pub fn all_current(&self) -> bool {
        !self.may_migrate
            && self.schema_version_floor >= 1
            && self
                .identities
                .iter()
                .all(|i| self.is_compatible(i))
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn attempt_migrate(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticCompatibilityReadOnly)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticCurrency {
    Current,
    Historical,
    RestoredView,
}

impl RuntimeDiagnosticCurrency {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Historical => "historical",
            Self::RestoredView => "restored_view",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticLineageStep {
    pub phase: RuntimeDiagnosticLifecyclePhase,
    pub artifact_id: String,
}

/// Immutable lineage across diagnostic lifecycle phases for one snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticLineageRecord {
    pub id: String,
    pub workspace_id: String,
    pub snapshot_id: String,
    pub steps: Vec<RuntimeDiagnosticLineageStep>,
    pub current_phase: RuntimeDiagnosticLifecyclePhase,
    pub previous_snapshot_id: Option<String>,
    pub currency: RuntimeDiagnosticCurrency,
    pub contract_identity: RuntimeDiagnosticContractIdentity,
    pub authority_effect: String,
}

impl RuntimeDiagnosticLineageRecord {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn assemble(
        snapshot: &RuntimeDiagnosticSnapshot,
        provenance: &RuntimeDiagnosticProvenance,
        continuity: &RuntimeDiagnosticContinuityRecord,
        evolution: &RuntimeDiagnosticEvolutionReport,
        archive_entry_id: Option<&str>,
        restoration_id: Option<&str>,
        superseded: bool,
    ) -> Self {
        let mut steps = vec![
            RuntimeDiagnosticLineageStep {
                phase: RuntimeDiagnosticLifecyclePhase::Captured,
                artifact_id: snapshot.id.clone(),
            },
            RuntimeDiagnosticLineageStep {
                phase: RuntimeDiagnosticLifecyclePhase::Provenanced,
                artifact_id: provenance.id.clone(),
            },
            RuntimeDiagnosticLineageStep {
                phase: RuntimeDiagnosticLifecyclePhase::ContinuityLinked,
                artifact_id: continuity.id.clone(),
            },
        ];
        if let Some(cmp) = &evolution.comparison {
            steps.push(RuntimeDiagnosticLineageStep {
                phase: RuntimeDiagnosticLifecyclePhase::Compared,
                artifact_id: cmp.id.clone(),
            });
        }
        steps.push(RuntimeDiagnosticLineageStep {
            phase: RuntimeDiagnosticLifecyclePhase::Evolved,
            artifact_id: evolution.id.clone(),
        });
        if superseded {
            steps.push(RuntimeDiagnosticLineageStep {
                phase: RuntimeDiagnosticLifecyclePhase::Superseded,
                artifact_id: format!("superseded:{}", snapshot.id),
            });
        }
        if let Some(aid) = archive_entry_id {
            steps.push(RuntimeDiagnosticLineageStep {
                phase: RuntimeDiagnosticLifecyclePhase::Archived,
                artifact_id: aid.into(),
            });
        }
        let (current_phase, currency) = if restoration_id.is_some() {
            (
                RuntimeDiagnosticLifecyclePhase::Archived,
                RuntimeDiagnosticCurrency::RestoredView,
            )
        } else if superseded {
            (
                RuntimeDiagnosticLifecyclePhase::Superseded,
                RuntimeDiagnosticCurrency::Historical,
            )
        } else if archive_entry_id.is_some() {
            (
                RuntimeDiagnosticLifecyclePhase::Archived,
                RuntimeDiagnosticCurrency::Historical,
            )
        } else {
            (
                RuntimeDiagnosticLifecyclePhase::Evolved,
                RuntimeDiagnosticCurrency::Current,
            )
        };
        Self {
            id: format!("runtime_diagnostic_lineage:{}", snapshot.id),
            workspace_id: snapshot.workspace_id.clone(),
            snapshot_id: snapshot.id.clone(),
            steps,
            current_phase,
            previous_snapshot_id: continuity.previous_snapshot_id.clone(),
            currency,
            contract_identity: RuntimeDiagnosticContractIdentity::canonical(
                RuntimeDiagnosticContractFamily::Lineage,
            ),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn ordering_ok(&self) -> bool {
        let mut prev: Option<RuntimeDiagnosticLifecyclePhase> = None;
        for step in &self.steps {
            if let Some(p) = prev {
                if p != step.phase && !p.may_transition_to(step.phase) {
                    return false;
                }
            }
            prev = Some(step.phase);
        }
        true
    }

    pub fn may_mutate(&self) -> bool {
        false
    }

    pub fn attempt_mutate(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticHistoryImmutable)
    }
}

/// Informational trust envelope for an interpretation — not authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticTrustRecord {
    pub id: String,
    pub workspace_id: String,
    pub interpretation_id: String,
    pub evolution_report_id: String,
    pub producer: RuntimeDiagnosticContractIdentity,
    pub lineage_id: String,
    pub currency: RuntimeDiagnosticCurrency,
    pub limitations: Vec<String>,
    pub finding_ids: Vec<String>,
    pub compatibility_ok: bool,
    pub boundaries_ok: bool,
    pub consumption_safe: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticTrustRecord {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn attest(
        interpretation: &RuntimeDiagnosticInterpretationView,
        lineage: &RuntimeDiagnosticLineageRecord,
        compatibility: &RuntimeDiagnosticCompatibilityContract,
        boundaries: &RuntimeProjectionBoundaryRegistry,
    ) -> Self {
        let producer = RuntimeDiagnosticContractIdentity::canonical(
            RuntimeDiagnosticContractFamily::Interpretation,
        );
        let compatibility_ok = compatibility.is_compatible(&producer)
            && compatibility.is_compatible(&lineage.contract_identity)
            && compatibility.all_current();
        let boundaries_ok = boundaries.boundaries_respected();
        let consumption_safe = interpretation.consumption.is_safe()
            && interpretation.all_findings_non_actionable();
        let mut limitations = interpretation.limitations.clone();
        limitations.push(format!(
            "contract_version={}",
            RUNTIME_DIAGNOSTICS_CONTRACT_VERSION
        ));
        limitations.push(format!(
            "schema_version={}",
            RUNTIME_DIAGNOSTICS_SCHEMA_VERSION
        ));
        limitations.push(format!("currency={}", lineage.currency.as_str()));
        limitations.push("informational_only".into());
        Self {
            id: format!("runtime_diagnostic_trust:{}", interpretation.id),
            workspace_id: interpretation.workspace_id.clone(),
            interpretation_id: interpretation.id.clone(),
            evolution_report_id: interpretation.evolution_report_id.clone(),
            producer,
            lineage_id: lineage.id.clone(),
            currency: lineage.currency,
            limitations,
            finding_ids: interpretation.findings.iter().map(|f| f.id.clone()).collect(),
            compatibility_ok,
            boundaries_ok,
            consumption_safe,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn trustworthy(&self) -> bool {
        self.compatibility_ok
            && self.boundaries_ok
            && self.consumption_safe
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && !self.limitations.is_empty()
    }

    pub fn is_authoritative(&self) -> bool {
        false
    }

    pub fn is_decision(&self) -> bool {
        false
    }

    pub fn attempt_promote_to_decision(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticEvidenceNotAuthoritative)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

/// Validates lineage ordering and restoration read-only posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticLineageValidation {
    pub id: String,
    pub ordering_ok: bool,
    pub restoration_read_only: bool,
    pub currency_consistent: bool,
    pub diagnostics: Vec<String>,
    pub authority_effect: String,
}

impl RuntimeDiagnosticLineageValidation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn validate(
        lineage: &RuntimeDiagnosticLineageRecord,
        restoration: Option<&RuntimeDiagnosticRestorationView>,
    ) -> Self {
        let mut diagnostics = Vec::new();
        let ordering_ok = lineage.ordering_ok();
        if !ordering_ok {
            diagnostics.push("lineage lifecycle ordering invalid".into());
        }
        let restoration_read_only = match restoration {
            None => true,
            Some(r) => !r.may_mutate && !r.may_delete,
        };
        if !restoration_read_only {
            diagnostics.push("restoration must remain read-only".into());
        }
        let currency_consistent = match (lineage.currency, restoration.is_some()) {
            (RuntimeDiagnosticCurrency::RestoredView, true) => true,
            (RuntimeDiagnosticCurrency::RestoredView, false) => false,
            (_, true) => false,
            _ => true,
        };
        if !currency_consistent {
            diagnostics.push("currency inconsistent with restoration presence".into());
        }
        if diagnostics.is_empty() {
            diagnostics.push("diagnostic lineage validation passed".into());
        }
        Self {
            id: format!("runtime_diagnostic_lineage_validation:{}", lineage.id),
            ordering_ok,
            restoration_read_only,
            currency_consistent,
            diagnostics,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn passed(&self) -> bool {
        self.ordering_ok && self.restoration_read_only && self.currency_consistent
    }

    pub fn attempt_repair(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticHistoryImmutable)
    }
}

// ---------------------------------------------------------------------------
// Runtime Diagnostic Closure (post–trust audit)
// ---------------------------------------------------------------------------
//
// Trust/lineage cover producer version and currency, but lacked terminal-state
// closure, static contract catalog/discovery, cross-domain interoperability
// declarations, and explanation integrity tying trust into operator answers.

/// Lifecycle closure check — terminal + invalid transition detection; no mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticLifecycleClosure {
    pub id: String,
    pub snapshot_id: String,
    pub current_phase: RuntimeDiagnosticLifecyclePhase,
    pub terminal: bool,
    pub ordering_ok: bool,
    pub invalid_transitions: Vec<String>,
    pub restored_view_is_currency_not_phase: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticLifecycleClosure {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn evaluate(lineage: &RuntimeDiagnosticLineageRecord) -> Self {
        let mut invalid_transitions = Vec::new();
        let mut prev: Option<RuntimeDiagnosticLifecyclePhase> = None;
        for step in &lineage.steps {
            if let Some(p) = prev {
                if p != step.phase && !p.may_transition_to(step.phase) {
                    invalid_transitions.push(format!(
                        "{}→{}",
                        p.as_str(),
                        step.phase.as_str()
                    ));
                }
            }
            prev = Some(step.phase);
        }
        let terminal = lineage.current_phase.is_terminal()
            || (lineage.currency == RuntimeDiagnosticCurrency::Historical
                && lineage.current_phase == RuntimeDiagnosticLifecyclePhase::Superseded)
            || lineage.currency == RuntimeDiagnosticCurrency::RestoredView;
        Self {
            id: format!("runtime_diagnostic_lifecycle_closure:{}", lineage.snapshot_id),
            snapshot_id: lineage.snapshot_id.clone(),
            current_phase: lineage.current_phase,
            terminal,
            ordering_ok: invalid_transitions.is_empty() && lineage.ordering_ok(),
            invalid_transitions,
            restored_view_is_currency_not_phase: true,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn closed(&self) -> bool {
        self.ordering_ok
            && self.invalid_transitions.is_empty()
            && self.restored_view_is_currency_not_phase
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn attempt_mutate(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticClosureReadOnly)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticContractCatalogEntry {
    pub identity: RuntimeDiagnosticContractIdentity,
    pub owner: RuntimeDiagnosticOwnershipRole,
    pub discoverable: bool,
    pub executable: bool,
    pub notes: String,
}

/// Static registration/discovery catalog — no dynamic loading or execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticContractCatalog {
    pub id: String,
    pub entries: Vec<RuntimeDiagnosticContractCatalogEntry>,
    pub authority_effect: String,
}

impl RuntimeDiagnosticContractCatalog {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn canonical() -> Self {
        let entry = |family: RuntimeDiagnosticContractFamily,
                     owner: RuntimeDiagnosticOwnershipRole,
                     notes: &str| RuntimeDiagnosticContractCatalogEntry {
            identity: RuntimeDiagnosticContractIdentity::canonical(family),
            owner,
            discoverable: true,
            executable: false,
            notes: notes.into(),
        };
        Self {
            id: "runtime_diagnostic_contract_catalog:canonical".into(),
            entries: vec![
                entry(
                    RuntimeDiagnosticContractFamily::Snapshot,
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "point-in-time observational capture",
                ),
                entry(
                    RuntimeDiagnosticContractFamily::Provenance,
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "input citations",
                ),
                entry(
                    RuntimeDiagnosticContractFamily::Continuity,
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "diagnostic continuity ≠ work continuity",
                ),
                entry(
                    RuntimeDiagnosticContractFamily::Comparison,
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "structured deltas",
                ),
                entry(
                    RuntimeDiagnosticContractFamily::Evolution,
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "validation/interpretation",
                ),
                entry(
                    RuntimeDiagnosticContractFamily::Evidence,
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "sealed digests",
                ),
                entry(
                    RuntimeDiagnosticContractFamily::Archive,
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "append-only retention",
                ),
                entry(
                    RuntimeDiagnosticContractFamily::Consumption,
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "allowed consumers only",
                ),
                entry(
                    RuntimeDiagnosticContractFamily::Interpretation,
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "structured findings",
                ),
                entry(
                    RuntimeDiagnosticContractFamily::Ownership,
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "descriptive ownership",
                ),
                entry(
                    RuntimeDiagnosticContractFamily::Restoration,
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "read-only rehydration",
                ),
                entry(
                    RuntimeDiagnosticContractFamily::Lineage,
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "lifecycle steps + currency",
                ),
                entry(
                    RuntimeDiagnosticContractFamily::Trust,
                    RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics,
                    "informational trust envelope",
                ),
            ],
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn discover(&self) -> Vec<&RuntimeDiagnosticContractCatalogEntry> {
        self.entries.iter().filter(|e| e.discoverable).collect()
    }

    pub fn none_executable(&self) -> bool {
        self.entries.iter().all(|e| !e.executable)
    }

    pub fn compatibility_report(
        &self,
        compatibility: &RuntimeDiagnosticCompatibilityContract,
    ) -> RuntimeDiagnosticCompatibilityReport {
        RuntimeDiagnosticCompatibilityReport::from_catalog(self, compatibility)
    }

    pub fn attempt_dynamic_load(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticClosureReadOnly)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

/// Compatibility reporting over the static catalog — identity only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticCompatibilityReport {
    pub id: String,
    pub catalog_id: String,
    pub compatible_families: Vec<String>,
    pub incompatible_families: Vec<String>,
    pub schema_version_floor: i32,
    pub may_migrate: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticCompatibilityReport {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn from_catalog(
        catalog: &RuntimeDiagnosticContractCatalog,
        compatibility: &RuntimeDiagnosticCompatibilityContract,
    ) -> Self {
        let mut compatible_families = Vec::new();
        let mut incompatible_families = Vec::new();
        for e in &catalog.entries {
            if compatibility.is_compatible(&e.identity) {
                compatible_families.push(e.identity.family.as_str().into());
            } else {
                incompatible_families.push(e.identity.family.as_str().into());
            }
        }
        Self {
            id: format!("runtime_diagnostic_compatibility_report:{}", catalog.id),
            catalog_id: catalog.id.clone(),
            compatible_families,
            incompatible_families,
            schema_version_floor: compatibility.schema_version_floor,
            may_migrate: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn all_compatible(&self) -> bool {
        self.incompatible_families.is_empty() && !self.may_migrate
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticInteropDomain {
    GovernanceRecords,
    AuditService,
    WorkContinuity,
    OperatorProjection,
    ExperienceTranslation,
}

impl RuntimeDiagnosticInteropDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GovernanceRecords => "governance_records",
            Self::AuditService => "audit_service",
            Self::WorkContinuity => "work_continuity",
            Self::OperatorProjection => "operator_projection",
            Self::ExperienceTranslation => "experience_translation",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticInteropEntry {
    pub domain: RuntimeDiagnosticInteropDomain,
    pub may_reference_read_only: bool,
    pub may_own: bool,
    pub may_emit_commands: bool,
    pub notes: String,
}

/// Cross-domain coexistence — references read-only; ownership remains separate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticInteropContract {
    pub id: String,
    pub entries: Vec<RuntimeDiagnosticInteropEntry>,
    pub authority_effect: String,
}

impl RuntimeDiagnosticInteropContract {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn canonical() -> Self {
        let entry = |domain: RuntimeDiagnosticInteropDomain,
                     may_reference_read_only: bool,
                     notes: &str| RuntimeDiagnosticInteropEntry {
            domain,
            may_reference_read_only,
            may_own: false,
            may_emit_commands: false,
            notes: notes.into(),
        };
        Self {
            id: "runtime_diagnostic_interop:canonical".into(),
            entries: vec![
                entry(
                    RuntimeDiagnosticInteropDomain::GovernanceRecords,
                    true,
                    "may cite governance summary labels; never owns ledger/decisions",
                ),
                entry(
                    RuntimeDiagnosticInteropDomain::AuditService,
                    true,
                    "diagnostic archive ≠ AuditService command trail",
                ),
                entry(
                    RuntimeDiagnosticInteropDomain::WorkContinuity,
                    true,
                    "diagnostic continuity ≠ Continuity Engine resume facets",
                ),
                entry(
                    RuntimeDiagnosticInteropDomain::OperatorProjection,
                    true,
                    "operator may consume explanations; never executes",
                ),
                entry(
                    RuntimeDiagnosticInteropDomain::ExperienceTranslation,
                    false,
                    "diagnostics must not drive Experience translation",
                ),
            ],
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn boundaries_respected(&self) -> bool {
        self.entries.iter().all(|e| !e.may_own && !e.may_emit_commands)
            && self
                .entries
                .iter()
                .find(|e| e.domain == RuntimeDiagnosticInteropDomain::ExperienceTranslation)
                .is_some_and(|e| !e.may_reference_read_only)
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn attempt_own_foreign(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticClosureReadOnly)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

/// Operator explanation integrity — answers what/why/who/version/currency.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticExplanationIntegrity {
    pub id: String,
    pub workspace_id: String,
    pub what_changed: Vec<String>,
    pub why_changed: Vec<String>,
    pub produced_by: String,
    pub contract_version: String,
    pub schema_version: i32,
    pub currency: RuntimeDiagnosticCurrency,
    pub trust_id: String,
    pub lineage_id: String,
    pub answers_complete: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticExplanationIntegrity {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn compose(
        continuity: &RuntimeDiagnosticContinuityRecord,
        interpretation: &RuntimeDiagnosticInterpretationView,
        trust: &RuntimeDiagnosticTrustRecord,
        lineage: &RuntimeDiagnosticLineageRecord,
    ) -> Self {
        let what_changed = continuity.what_changed.clone();
        let mut why_changed = interpretation
            .findings
            .iter()
            .map(|f| format!("{}:{}", f.scope.as_str(), f.detail))
            .collect::<Vec<_>>();
        if why_changed.is_empty() {
            why_changed.push("no_structured_findings".into());
        }
        let answers_complete = !what_changed.is_empty()
            && !why_changed.is_empty()
            && !trust.producer.contract_version.is_empty()
            && trust.trustworthy()
            && lineage.ordering_ok();
        Self {
            id: format!(
                "runtime_diagnostic_explanation_integrity:{}",
                trust.workspace_id
            ),
            workspace_id: trust.workspace_id.clone(),
            what_changed,
            why_changed,
            produced_by: format!(
                "family={}:version={}",
                trust.producer.family.as_str(),
                trust.producer.contract_version
            ),
            contract_version: trust.producer.contract_version.clone(),
            schema_version: trust.producer.schema_version,
            currency: lineage.currency,
            trust_id: trust.id.clone(),
            lineage_id: lineage.id.clone(),
            answers_complete,
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
// Runtime Diagnostic Maturity (post–closure audit)
// ---------------------------------------------------------------------------
//
// Closure delivered catalog/interop/explanation surfaces, but lacked a
// meta-diagnostic readiness assessment: catalog registration integrity,
// contract dependency ordering, cross-domain reference integrity, and
// explanation consistency. Distinct from Workspace Readiness Model.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeDiagnosticMaturityLevel {
    Incomplete,
    Degraded,
    Ready,
    Mature,
}

impl RuntimeDiagnosticMaturityLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Incomplete => "incomplete",
            Self::Degraded => "degraded",
            Self::Ready => "ready",
            Self::Mature => "mature",
        }
    }
}

/// Required dependency ordering among catalog families (consumer → prerequisite).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticContractDependency {
    pub family: RuntimeDiagnosticContractFamily,
    pub requires: RuntimeDiagnosticContractFamily,
}

impl RuntimeDiagnosticContractDependency {
    pub fn canonical_chain() -> Vec<Self> {
        use RuntimeDiagnosticContractFamily::*;
        let edge = |family, requires| Self { family, requires };
        vec![
            edge(Provenance, Snapshot),
            edge(Continuity, Provenance),
            edge(Comparison, Continuity),
            edge(Evolution, Comparison),
            edge(Evidence, Evolution),
            edge(Archive, Evidence),
            edge(Consumption, Evolution),
            edge(Interpretation, Consumption),
            edge(Restoration, Archive),
            edge(Lineage, Evolution),
            edge(Trust, Lineage),
            edge(Trust, Interpretation),
        ]
    }
}

/// Registration integrity for the static contract catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticCatalogIntegrity {
    pub id: String,
    pub no_duplicate_families: bool,
    pub none_executable: bool,
    pub ownership_consistent: bool,
    pub required_families_present: bool,
    pub dependency_order_ok: bool,
    pub missing_families: Vec<String>,
    pub diagnostics: Vec<String>,
    pub authority_effect: String,
}

impl RuntimeDiagnosticCatalogIntegrity {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn verify(catalog: &RuntimeDiagnosticContractCatalog) -> Self {
        let mut diagnostics = Vec::new();
        let mut seen = std::collections::HashSet::new();
        let mut no_duplicate_families = true;
        for e in &catalog.entries {
            if !seen.insert(e.identity.family) {
                no_duplicate_families = false;
                diagnostics.push(format!(
                    "duplicate catalog family: {}",
                    e.identity.family.as_str()
                ));
            }
        }
        let none_executable = catalog.none_executable();
        if !none_executable {
            diagnostics.push("catalog contains executable entries".into());
        }
        let ownership_consistent = catalog.entries.iter().all(|e| {
            e.owner == RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics
        });
        if !ownership_consistent {
            diagnostics.push("catalog ownership must remain ObservationalDiagnostics".into());
        }

        let present: std::collections::HashSet<_> =
            catalog.entries.iter().map(|e| e.identity.family).collect();
        let required = [
            RuntimeDiagnosticContractFamily::Snapshot,
            RuntimeDiagnosticContractFamily::Provenance,
            RuntimeDiagnosticContractFamily::Continuity,
            RuntimeDiagnosticContractFamily::Comparison,
            RuntimeDiagnosticContractFamily::Evolution,
            RuntimeDiagnosticContractFamily::Evidence,
            RuntimeDiagnosticContractFamily::Archive,
            RuntimeDiagnosticContractFamily::Consumption,
            RuntimeDiagnosticContractFamily::Interpretation,
            RuntimeDiagnosticContractFamily::Ownership,
            RuntimeDiagnosticContractFamily::Restoration,
            RuntimeDiagnosticContractFamily::Lineage,
            RuntimeDiagnosticContractFamily::Trust,
        ];
        let mut missing_families = Vec::new();
        for f in required {
            if !present.contains(&f) {
                missing_families.push(f.as_str().into());
            }
        }
        let required_families_present = missing_families.is_empty();
        if !required_families_present {
            diagnostics.push(format!("missing catalog families: {missing_families:?}"));
        }

        let index_of = |f: RuntimeDiagnosticContractFamily| {
            catalog
                .entries
                .iter()
                .position(|e| e.identity.family == f)
        };
        let mut dependency_order_ok = true;
        for dep in RuntimeDiagnosticContractDependency::canonical_chain() {
            match (index_of(dep.requires), index_of(dep.family)) {
                (Some(req_i), Some(fam_i)) if req_i < fam_i => {}
                (Some(_), Some(_)) => {
                    dependency_order_ok = false;
                    diagnostics.push(format!(
                        "dependency order violated: {} requires {}",
                        dep.family.as_str(),
                        dep.requires.as_str()
                    ));
                }
                _ => {
                    // Missing families already reported.
                }
            }
        }

        if diagnostics.is_empty() {
            diagnostics.push("catalog integrity passed".into());
        }
        Self {
            id: format!("runtime_diagnostic_catalog_integrity:{}", catalog.id),
            no_duplicate_families,
            none_executable,
            ownership_consistent,
            required_families_present,
            dependency_order_ok,
            missing_families,
            diagnostics,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn passed(&self) -> bool {
        self.no_duplicate_families
            && self.none_executable
            && self.ownership_consistent
            && self.required_families_present
            && self.dependency_order_ok
    }
}

/// Cross-domain reference integrity — read-only refs; ownership unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticReferenceIntegrity {
    pub id: String,
    pub interop_ok: bool,
    pub experience_not_driven: bool,
    pub foreign_ownership_forbidden: bool,
    pub diagnostics: Vec<String>,
    pub authority_effect: String,
}

impl RuntimeDiagnosticReferenceIntegrity {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn verify(interop: &RuntimeDiagnosticInteropContract) -> Self {
        let mut diagnostics = Vec::new();
        let interop_ok = interop.boundaries_respected();
        if !interop_ok {
            diagnostics.push("interop boundaries violated".into());
        }
        let experience_not_driven = interop
            .entries
            .iter()
            .find(|e| e.domain == RuntimeDiagnosticInteropDomain::ExperienceTranslation)
            .is_some_and(|e| !e.may_reference_read_only && !e.may_own && !e.may_emit_commands);
        if !experience_not_driven {
            diagnostics.push("experience translation must not be driven by diagnostics".into());
        }
        let foreign_ownership_forbidden = interop.entries.iter().all(|e| !e.may_own);
        if !foreign_ownership_forbidden {
            diagnostics.push("foreign domain ownership claimed".into());
        }
        if diagnostics.is_empty() {
            diagnostics.push("reference integrity passed".into());
        }
        Self {
            id: format!("runtime_diagnostic_reference_integrity:{}", interop.id),
            interop_ok,
            experience_not_driven,
            foreign_ownership_forbidden,
            diagnostics,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn passed(&self) -> bool {
        self.interop_ok && self.experience_not_driven && self.foreign_ownership_forbidden
    }
}

/// Validates operator explanation integrity is internally consistent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticExplanationConsistency {
    pub id: String,
    pub answers_complete: bool,
    pub trust_link_ok: bool,
    pub lineage_link_ok: bool,
    pub currency_match: bool,
    pub version_match: bool,
    pub no_actions: bool,
    pub diagnostics: Vec<String>,
    pub authority_effect: String,
}

impl RuntimeDiagnosticExplanationConsistency {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn verify(
        integrity: &RuntimeDiagnosticExplanationIntegrity,
        trust: &RuntimeDiagnosticTrustRecord,
        lineage: &RuntimeDiagnosticLineageRecord,
        explanation: Option<&OperatorRuntimeExplanation>,
    ) -> Self {
        let mut diagnostics = Vec::new();
        let answers_complete = integrity.answers_complete;
        if !answers_complete {
            diagnostics.push("explanation integrity answers incomplete".into());
        }
        let trust_link_ok = integrity.trust_id == trust.id;
        if !trust_link_ok {
            diagnostics.push("explanation trust_id mismatch".into());
        }
        let lineage_link_ok = integrity.lineage_id == lineage.id;
        if !lineage_link_ok {
            diagnostics.push("explanation lineage_id mismatch".into());
        }
        let currency_match = integrity.currency == lineage.currency;
        if !currency_match {
            diagnostics.push("explanation currency mismatch".into());
        }
        let version_match = integrity.contract_version == trust.producer.contract_version
            && integrity.schema_version == trust.producer.schema_version;
        if !version_match {
            diagnostics.push("explanation contract version mismatch".into());
        }
        let no_actions = match explanation {
            None => true,
            Some(e) => !e.exposes_actions() && !e.is_execution_surface(),
        };
        if !no_actions {
            diagnostics.push("operator explanation exposes actions".into());
        }
        if diagnostics.is_empty() {
            diagnostics.push("explanation consistency passed".into());
        }
        Self {
            id: format!(
                "runtime_diagnostic_explanation_consistency:{}",
                integrity.id
            ),
            answers_complete,
            trust_link_ok,
            lineage_link_ok,
            currency_match,
            version_match,
            no_actions,
            diagnostics,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn passed(&self) -> bool {
        self.answers_complete
            && self.trust_link_ok
            && self.lineage_link_ok
            && self.currency_match
            && self.version_match
            && self.no_actions
    }
}

/// Meta-diagnostic maturity assessment of the diagnostic subsystem itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticMaturityAssessment {
    pub id: String,
    pub workspace_id: String,
    pub level: RuntimeDiagnosticMaturityLevel,
    pub completeness_score: u8,
    pub health: WorkspaceHealthLevel,
    pub catalog_integrity_ok: bool,
    pub reference_integrity_ok: bool,
    pub explanation_consistency_ok: bool,
    pub lifecycle_closure_ok: bool,
    pub diagnostics: Vec<String>,
    pub meta_diagnostic_only: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticMaturityAssessment {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn assess(
        workspace_id: impl Into<String>,
        catalog_integrity: &RuntimeDiagnosticCatalogIntegrity,
        reference_integrity: &RuntimeDiagnosticReferenceIntegrity,
        explanation_consistency: &RuntimeDiagnosticExplanationConsistency,
        lifecycle_closure: &RuntimeDiagnosticLifecycleClosure,
    ) -> Self {
        let workspace_id = workspace_id.into();
        let mut diagnostics = Vec::new();
        let catalog_integrity_ok = catalog_integrity.passed();
        let reference_integrity_ok = reference_integrity.passed();
        let explanation_consistency_ok = explanation_consistency.passed();
        let lifecycle_closure_ok = lifecycle_closure.closed();

        let mut score: u8 = 0;
        if catalog_integrity_ok {
            score += 30;
        } else {
            diagnostics.extend(catalog_integrity.diagnostics.iter().cloned());
        }
        if reference_integrity_ok {
            score += 20;
        } else {
            diagnostics.extend(reference_integrity.diagnostics.iter().cloned());
        }
        if explanation_consistency_ok {
            score += 25;
        } else {
            diagnostics.extend(explanation_consistency.diagnostics.iter().cloned());
        }
        if lifecycle_closure_ok {
            score += 25;
        } else {
            diagnostics.push("lifecycle closure not closed".into());
        }

        let level = match score {
            100 => RuntimeDiagnosticMaturityLevel::Mature,
            75..=99 => RuntimeDiagnosticMaturityLevel::Ready,
            40..=74 => RuntimeDiagnosticMaturityLevel::Degraded,
            _ => RuntimeDiagnosticMaturityLevel::Incomplete,
        };
        let health = match level {
            RuntimeDiagnosticMaturityLevel::Mature | RuntimeDiagnosticMaturityLevel::Ready => {
                WorkspaceHealthLevel::Healthy
            }
            RuntimeDiagnosticMaturityLevel::Degraded => WorkspaceHealthLevel::Degraded,
            RuntimeDiagnosticMaturityLevel::Incomplete => WorkspaceHealthLevel::Unknown,
        };
        if diagnostics.is_empty() {
            diagnostics.push("diagnostic maturity assessment passed".into());
        }
        Self {
            id: format!("runtime_diagnostic_maturity:{workspace_id}"),
            workspace_id,
            level,
            completeness_score: score,
            health,
            catalog_integrity_ok,
            reference_integrity_ok,
            explanation_consistency_ok,
            lifecycle_closure_ok,
            diagnostics,
            meta_diagnostic_only: true,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn ready(&self) -> bool {
        self.meta_diagnostic_only
            && matches!(
                self.level,
                RuntimeDiagnosticMaturityLevel::Ready | RuntimeDiagnosticMaturityLevel::Mature
            )
            && self.catalog_integrity_ok
            && self.reference_integrity_ok
            && self.explanation_consistency_ok
            && self.lifecycle_closure_ok
    }

    pub fn may_prescribe(&self) -> bool {
        false
    }

    pub fn may_execute(&self) -> bool {
        false
    }

    pub fn attempt_prescribe(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticMaturityReadOnly)
    }

    pub fn attempt_execute() -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}

/// Observational continuity between diagnostic snapshots — not work continuity.
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

    /// Explain with trust/lineage integrity — what/why/who/version/currency.
    pub fn explain_with_trust(
        overview: &OperatorRuntimeOverview,
        provenance: &RuntimeDiagnosticProvenance,
        continuity: &RuntimeDiagnosticContinuityRecord,
        evolution: &RuntimeDiagnosticEvolutionReport,
        interpretation: &RuntimeDiagnosticInterpretationView,
        integrity: &RuntimeDiagnosticExplanationIntegrity,
    ) -> Self {
        let mut base = Self::explain_with_interpretation(
            overview,
            provenance,
            continuity,
            evolution,
            interpretation,
        );
        base.why.push(format!("produced_by={}", integrity.produced_by));
        base.why.push(format!("contract_version={}", integrity.contract_version));
        base.why.push(format!("schema_version={}", integrity.schema_version));
        base.why.push(format!("currency={}", integrity.currency.as_str()));
        base.why.push(format!("trust_id={}", integrity.trust_id));
        base.why.push(format!(
            "explanation_integrity_complete={}",
            integrity.answers_complete
        ));
        for w in &integrity.what_changed {
            let line = format!("what_changed:{w}");
            if !base.why.iter().any(|x| x == &line) {
                base.why.push(line);
            }
        }
        for w in &integrity.why_changed {
            let line = format!("why_changed:{w}");
            if !base.why.iter().any(|x| x == &line) {
                base.why.push(line);
            }
        }
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
