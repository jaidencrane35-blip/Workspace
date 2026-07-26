# Workspace Runtime Context & Integration

Sprints 170–181 + runtime diagnostic subsystem (provenance → maturity) — read-only.

**Governance is visible, never authoritative. No execution. No automation. No runtime publication.
Published remains BLOCKED. Scoring and WorkspaceState ownership unchanged.**

Execution authority remains:

```
Intent → Command Pipeline → Permission Gateway → Allow → Execution
```

---

## Naming

| Type | Role |
|------|------|
| `WorkspaceContext` (Sprint 19) | Execution/capability read composition — unchanged |
| `WorkspaceRuntimeContext` (Sprint 171) | Canonical cognition operating context |
| Kernel `WorkspaceHealth` | Lifecycle health label |
| `WorkspaceRuntimeHealth` (Sprint 174) | Observational runtime health aggregate |
| Readiness Model | Preparedness for current work — distinct from diagnostic maturity |
| Work Continuity Engine | Where work left off / resume — **not** diagnostic continuity |
| `RuntimeDiagnosticSubsystemBoundary` | Hard invariant: diagnostics observe/explain only |

---

## Aggregate chain (Sprint 175)

```
WorkspaceState
    → Environment
    → Attention
    → Intelligence
    → Decision
    → Experience
    → Governance projections
    → Operator projections
```

---

## Contracts

| Sprint | Type | Rule |
|--------|------|------|
| 170 | `WorkspaceRuntimeIntegrationContract` | Governance observable at audited surfaces; `authoritative == false` |
| 171 | `WorkspaceRuntimeContext` + `GovernanceRuntimeSummary` | Read-only; does not own WorkspaceState |
| 172 | `CognitionContextProjection` / `CognitionContextBundle` | Context only; no scoring/reasoning changes |
| 173 | `OperatorContextProjection` | Domain projection only; not UI |
| 174 | `WorkspaceRuntimeHealth` | Observational; never prescriptive |
| 175 | `WorkspaceRuntimeCoherence` | Structural chain review |
| 176 | `RuntimeDependencyGraph` | Subsystem nodes/edges; cycle detection; diagnostics only |
| 177 | `RuntimeCapabilityMap` | Provided/consumed services; visibility/authority scopes; descriptive |
| 178 | `RuntimeDiagnosticSnapshot` | Immutable observational snapshot |
| 179 | `RuntimeConsistencyVerification` | Detects architecture issues; never auto-repairs |
| 180 | `OperatorRuntimeOverview` | Operator domain projection of runtime state; not UI |
| 181 | `RuntimeArchitectureReview` | Ownership, layering, authority, cohesion review |
| Audit | `RuntimeDiagnosticProvenance` | Cites snapshot inputs; history immutable |
| Audit | `RuntimeDiagnosticOwnershipBoundary` | Diagnostics observe; kernel lifecycle health distinct; operator projects |
| Audit | `RuntimeDiagnosticContinuityRecord` | Prior→current snapshot deltas; no rewrite/heal |
| Audit | `OperatorRuntimeExplanation` | Explains overview via provenance/continuity; not an execution surface |
| Audit | `RuntimeDiagnosticComparison` | Structured compare; severity labels; no actions |
| Audit | `RuntimeDiagnosticLifecycleRecord` | Captured → … → Archived; immutable |
| Audit | `RuntimeArchitectureOwnershipRegistry` | Explicit artifact→owner table |
| Audit | `RuntimeDiagnosticEvolutionReport` | Validates provenance/continuity/lifecycle; operator-safe interpretation |
| Audit | `RuntimeDiagnosticEvidenceBundle` | Sealed immutable refs + digests; never a decision |
| Audit | `RuntimeDiagnosticArchive` | Append-only retention; no delete/mutate |
| Audit | `RuntimeArchitectureOwnershipValidation` | Conflict detection; descriptive only |
| Audit | `RuntimeDiagnosticHistoricalIntegrity` | Evidence + archive + non-authoritative checks |
| Audit | `RuntimeDiagnosticConsumptionContract` | Allowed consumers/modes; forbids command/recommendation/authority |
| Audit | `RuntimeDiagnosticInterpretationView` | Findings with severity/confidence/scope/source/limitations |
| Audit | `RuntimeProjectionBoundaryRegistry` | Diagnostics ≠ Experience ≠ Governance ≠ Audit ≠ Continuity |
| Audit | `RuntimeDiagnosticRestorationView` | Read-only rehydration from archive; no mutate/delete |
| Audit | `RuntimeDiagnosticContractIdentity` | Family + `runtime_diagnostics:v1` + schema_version |
| Audit | `RuntimeDiagnosticCompatibilityContract` | Identity-only compatibility; no migration apply |
| Audit | `RuntimeDiagnosticLineageRecord` | Lifecycle steps + currency (current/historical/restored) |
| Audit | `RuntimeDiagnosticTrustRecord` | Producer version, limitations, currency — informational |
| Audit | `RuntimeDiagnosticLineageValidation` | Ordering + restoration read-only checks |
| Audit | `RuntimeDiagnosticLifecycleClosure` | Terminal/invalid transitions; RestoredView = currency not phase |
| Audit | `RuntimeDiagnosticContractCatalog` | Static registration/discovery; none executable |
| Audit | `RuntimeDiagnosticCompatibilityReport` | Catalog vs compatibility identity report |
| Audit | `RuntimeDiagnosticInteropContract` | Read-only coexistence with Governance/Audit/Continuity/Experience |
| Audit | `RuntimeDiagnosticExplanationIntegrity` | what/why/who/version/currency for operator explanations |
| Audit | `RuntimeDiagnosticCatalogIntegrity` | Registration integrity + dependency ordering |
| Audit | `RuntimeDiagnosticReferenceIntegrity` | Cross-domain read-only reference validation |
| Audit | `RuntimeDiagnosticExplanationConsistency` | Explanation ↔ trust/lineage consistency |
| Audit | `RuntimeDiagnosticMaturityAssessment` | Meta-diagnostic readiness/completeness/health |
| Consol. | `RuntimeDiagnosticSubsystemBoundary` | Observes only; never owns facts/scoring/Experience/Gateway |
| Audit | `RuntimeDiagnosticModuleLayering` | foundation→history→surface→meta; overview by artifact id |

Authority: `authority_effect: none` (`GOVERNANCE_AUTHORITY_EFFECT_NONE`).

### Module layout

```
packages/domain/src/workspace_runtime/
  mod.rs                 # runtime context, health, coherence (170–175)
  diagnostics/
    mod.rs               # flat re-exports + SubsystemBoundary + ModuleLayering
    foundation.rs        # graph, capabilities, snapshot, consistency
    history.rs           # provenance, evolution, continuity, evidence, archive
    surface.rs           # consumption, interpretation, boundaries, restoration,
                         # operator overview, architecture review
    meta.rs              # trust, lineage, closure, catalog, interop, maturity, explanation
    tests.rs             # domain unit tests
```

Dependency direction (hard): `foundation → history → surface → meta`.
Lower layers must not import higher-layer types. History cites operator overview
by artifact id only (`overview_id: Option<&str>`), never by surface type.

Public types remain available via `workspace_domain::*` / `workspace_runtime::*` (flat exports).

### Ownership boundaries

| Domain | Owns | Diagnostics may |
|--------|------|-----------------|
| WorkspaceState | Facts | Observe presence only |
| Cognition | Reasoning / scoring | Cite labels; never change scores |
| Experience | Translation | Must not drive |
| Governance | Change review | Cite non-authoritative summaries |
| Permission Gateway | Execution authority | Never enter / never Allow |
| Operator projections | Read-only labels | Explain; never execute |
| Diagnostics | Observational contracts | Observe, compare, archive, explain |

Canonical table: `RuntimeArchitectureOwnershipRegistry::canonical()` (`canonical:v1`).
Validation: `RuntimeArchitectureOwnershipValidation` + `RuntimeDiagnosticSubsystemBoundary`
+ `RuntimeDiagnosticModuleLayering`.

### Diagnostic lifecycle

```
Captured → Provenanced → ContinuityLinked → Compared → Evolved → Superseded → Archived
```

`RestoredView` is **currency**, not a lifecycle phase.

`RuntimeDiagnosticEvolutionReport` validates that continuity deltas match
`RuntimeDiagnosticComparison`, provenance cites the current snapshot, and
lifecycle ordering is respected. Interpretation is diagnostic only — no repair,
heal, approve, or execute pathways.

### Evidence integrity & retention

| Concept | Role | Distinct from |
|---------|------|---------------|
| `RuntimeDiagnosticEvidenceBundle` | Sealed refs for snapshot/provenance/continuity/comparison/evolution | Governance decision evidence |
| `RuntimeDiagnosticArchive` | Append-only retention of seals / supersession markers | `GovernanceArchiveContract` |
| Work Continuity Engine | Resume / where work left off | Diagnostic continuity & archive |

Reports and evidence bundles always `is_authoritative() == false` and
`is_decision() == false`. No deletion or mutation of archived history.

### Diagnostic consumption

| Allowed | Forbidden |
|---------|-----------|
| Observe / Explain / ArchiveInspect | Command, Recommendation, Authority, Approval |
| Operator / Overview / ArchitectureReview consumers | Driving Experience translation |
| Read-only restoration from archive | Entering Command Pipeline / Gateway |

`RuntimeDiagnosticInterpretationView` structures findings with severity, confidence,
scope, source, and limitations — observational meaning only.

### Trust & compatibility identity

| Field | Meaning |
|-------|---------|
| `RUNTIME_DIAGNOSTICS_CONTRACT_VERSION` | `runtime_diagnostics:v1` |
| `RUNTIME_DIAGNOSTICS_SCHEMA_VERSION` | `1` (compatibility floor identity) |
| Currency | `current` / `historical` / `restored_view` |

`RuntimeDiagnosticCompatibilityContract` reuses schema-version as identity only —
`may_migrate == false`; `attempt_migrate` hard-fails. Distinct from
`GovernanceCompatibilityContract` and from AuditService / Continuity Engine archives.

### Closure & interoperability

| Rule | Detail |
|------|--------|
| Terminal phase | `Archived` (`is_terminal`); `RestoredView` is currency only |
| Contract catalog | Static discoverable entries; `executable == false`; no dynamic load |
| Interop | May cite Governance/Audit/Continuity/Operator read-only; must not own or drive Experience |
| Explanation integrity | Operator answers what/why/who/version/currency via trust + lineage |

### Maturity (meta-diagnostic)

`RuntimeDiagnosticMaturityAssessment` scores catalog integrity, reference integrity,
explanation consistency, and lifecycle closure. It is observational only — distinct
from the Workspace Readiness Model and never prescriptive.

No hidden repair, auto-heal, or silent mutation paths.

---

## Related docs

- [WORKSPACE-GOVERNANCE.md](./WORKSPACE-GOVERNANCE.md)
- [WORKSPACE-PLATFORM-COHERENCE.md](./WORKSPACE-PLATFORM-COHERENCE.md)
- [WORKSPACE-COGNITION-PIPELINE-CONTRACT.md](./WORKSPACE-COGNITION-PIPELINE-CONTRACT.md)
- [WORKSPACE-CONTINUITY-ENGINE.md](./WORKSPACE-CONTINUITY-ENGINE.md) (work continuity ≠ diagnostic continuity)
- [WORKSPACE-READINESS-MODEL.md](./WORKSPACE-READINESS-MODEL.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
