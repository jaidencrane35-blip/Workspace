# Workspace Runtime Context & Integration

Sprints 170–181 — read-only runtime integration, diagnostics, and operator overview.

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

Authority: `authority_effect: none` (`GOVERNANCE_AUTHORITY_EFFECT_NONE`).

Module: `packages/domain/src/workspace_runtime/` (`mod.rs` + `diagnostics.rs`).

---

## Related docs

- [WORKSPACE-GOVERNANCE.md](./WORKSPACE-GOVERNANCE.md)
- [WORKSPACE-PLATFORM-COHERENCE.md](./WORKSPACE-PLATFORM-COHERENCE.md)
- [WORKSPACE-COGNITION-PIPELINE-CONTRACT.md](./WORKSPACE-COGNITION-PIPELINE-CONTRACT.md)
- [WORKSPACE-READINESS-MODEL.md](./WORKSPACE-READINESS-MODEL.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
