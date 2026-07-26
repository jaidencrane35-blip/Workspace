# Workspace Runtime Context & Integration

Sprints 170–181 + diagnostic provenance/continuity + diagnostic evolution — read-only runtime integration.

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
| Readiness Model | Preparedness for current work — distinct |
| Work Continuity Engine | Where work left off / resume — **not** diagnostic continuity |
| `RuntimeDiagnosticContinuityRecord` | Observational link between diagnostic snapshots |
| `RuntimeDiagnosticComparison` | Structured snapshot-to-snapshot deltas (diagnostic only) |
| `RuntimeDiagnosticLifecycleRecord` | Captured → Provenanced → ContinuityLinked → Superseded |

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
| Audit | `RuntimeDiagnosticLifecycleRecord` | Snapshot lineage phases; immutable |
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

Authority: `authority_effect: none` (`GOVERNANCE_AUTHORITY_EFFECT_NONE`).

Module: `packages/domain/src/workspace_runtime/` (`mod.rs` + `diagnostics.rs`).

### Diagnostic ownership (post–181 audit)

| Artifact | Owner / role |
|----------|----------------|
| Graph, capability map, snapshot, verification | Observational diagnostics |
| Attention / Decision scoring | Cognition subsystems (unchanged) |
| Kernel `WorkspaceHealth` | Kernel lifecycle (distinct) |
| `WorkspaceRuntimeHealth` | Observational observer only |
| Operator overview / explanation | Operator projection — never executes |

Canonical table: `RuntimeArchitectureOwnershipRegistry::canonical()` (`canonical:v1`).
Validation: `RuntimeArchitectureOwnershipValidation` (conflict detection only).

### Diagnostic lifecycle

```
Captured → Provenanced → ContinuityLinked → Compared → Evolved → Superseded → Archived
```

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

No hidden repair, auto-heal, or silent mutation paths.

---

## Related docs

- [WORKSPACE-GOVERNANCE.md](./WORKSPACE-GOVERNANCE.md)
- [WORKSPACE-PLATFORM-COHERENCE.md](./WORKSPACE-PLATFORM-COHERENCE.md)
- [WORKSPACE-COGNITION-PIPELINE-CONTRACT.md](./WORKSPACE-COGNITION-PIPELINE-CONTRACT.md)
- [WORKSPACE-CONTINUITY-ENGINE.md](./WORKSPACE-CONTINUITY-ENGINE.md) (work continuity ≠ diagnostic continuity)
- [WORKSPACE-READINESS-MODEL.md](./WORKSPACE-READINESS-MODEL.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
