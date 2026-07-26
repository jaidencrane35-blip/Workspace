# Workspace Governance Architecture (Canonical)

Sprints 165–169 — consolidated governance architecture index.

**This is the canonical entry point.** Specialized docs retain detail for ledger,
policy, risk, evidence, and publication safety. Thin contract notes redirect here
or to a single specialized page to reduce drift.

**No runtime activation. Governance never grants execution authority.
Permission Gateway remains the only execution authority. Published remains BLOCKED.**

---

## Aggregate roots

| Aggregate | Responsibility | Primary types / modules |
|-----------|----------------|-------------------------|
| **Ledger** | Historical refs, publish request labels | `GovernanceRecord`, `PublishRequest`, `GovernanceTimeline` |
| **Policy & risk** | Review rules, routing, decisions, evidence | `GovernancePolicy`, `GovernanceRisk`, `GovernanceReviewDecision`, `GovernanceDecisionEvidence` |
| **Publication prep** | Readiness, environment, safety gates | `PublicationReadiness`, `PublicationEnvironment`, `PublicationSafetyContract` |
| **Preconditions** | Conditions, compatibility, integrity, archive | `action_proposal::governance::contracts` |
| **Review ops** | Workflow, conflict, package, compliance, dashboard | `action_proposal::governance::workflow` |
| **Observability** | Notifications, delegation, metrics, reports, export | `action_proposal::governance::ops` |

Code layout (Sprint 165):

```text
packages/domain/src/action_proposal/governance/
  mod.rs          — aggregate map, ownership, dependency direction
  authority.rs    — shared GOVERNANCE_AUTHORITY_EFFECT_NONE
  contracts.rs    — preconditions aggregate
  workflow.rs     — review-ops aggregate
  ops.rs          — observability aggregate
```

Ledger / policy / publication-prep types remain in `action_proposal/mod.rs` and are
consumed inward by the governance submodule. Public crate exports are unchanged.

---

## Ownership boundaries

| Owner | Owns | Must not own |
|-------|------|--------------|
| **Governance** | Review, ledger, conditions, compliance, archive, export projections | Command execution, Gateway Allow |
| **Experience** | DisplayReason / translation traces | Governance decisions |
| **Permission Gateway** | Execution Allow / Deny / ApprovalRequired | Adaptation publish activation |
| **Domain facts** | Observation, WorkspaceState schema facts | Governance authority |
| **UI** | Presentation of governance projections | Approval that grants execution |

Named markers: `GovernanceAggregateRoot`, `GovernanceBoundaryOwner`
(documentation / audit helpers — not runtime registries).

---

## Dependency direction

```text
ops ──► workflow ──► contracts ──► ledger / policy / publication types
                ╲         │
                 ╲        ▼
                  └► RecommendationProvenance (immutable)
```

Nothing in governance depends on UI or Gateway Allow paths.

---

## Authority

All governance artifacts use `authority_effect: none`
(`GOVERNANCE_AUTHORITY_EFFECT_NONE`). Per-type `AUTHORITY_EFFECT_NONE` constants
alias the shared marker for API stability.

---

## Observability (consolidated)

Formerly separate Metrics / Reporting / Export notes:

| Concern | Type | Rule |
|---------|------|------|
| Metrics | `GovernanceMetricsContract` | Append-only samples; no optimise / adapt |
| Reports | `GovernanceReportContract` | Projections only; not UI |
| Export | `GovernanceExportPackage` | Read-only; no import / sync / publish |
| Notifications | `GovernanceNotificationContract` | Informational only |
| Delegation | `GovernanceDelegationContract` | Review scope only; no execution transfer |

---

## Architecture review (Sprint 169)

| Dimension | Assessment |
|-----------|------------|
| Cohesion | Types grouped by aggregate (ledger → prep → preconditions → review-ops → observability) |
| Coupling | Inward dependencies only; observability does not feed ledger mutation |
| Naming | `Governance*` for governance; `Publication*` for prep; Gateway remains separate |
| Extensibility | New prep/review concerns extend aggregates; do not invent parallel authority paths |
| Immutability | Provenance frozen; timeline/archive/metrics append-only; sealed decision package |

Safe structural changes in this batch: module consolidation, shared authority marker,
canonical documentation. No behaviour or capability removal.

---

## Runtime integration (Sprints 170–175)

Governance summaries surface through read-only runtime contracts — see
[WORKSPACE-RUNTIME-CONTEXT.md](./WORKSPACE-RUNTIME-CONTEXT.md).
Governance remains visible, never authoritative.

---

## Specialized docs (canonical detail)

| Topic | Doc |
|-------|-----|
| Runtime context & health | [WORKSPACE-RUNTIME-CONTEXT.md](./WORKSPACE-RUNTIME-CONTEXT.md) |
| Ledger & publish labels | [WORKSPACE-GOVERNANCE-LEDGER.md](./WORKSPACE-GOVERNANCE-LEDGER.md) |
| Policy | [WORKSPACE-GOVERNANCE-POLICY.md](./WORKSPACE-GOVERNANCE-POLICY.md) |
| Risk | [WORKSPACE-GOVERNANCE-RISK.md](./WORKSPACE-GOVERNANCE-RISK.md) |
| Evidence & readiness | [WORKSPACE-GOVERNANCE-EVIDENCE.md](./WORKSPACE-GOVERNANCE-EVIDENCE.md) |
| Lifecycle & timeline | [WORKSPACE-GOVERNANCE-LIFECYCLE.md](./WORKSPACE-GOVERNANCE-LIFECYCLE.md) |
| Workspace surface | [WORKSPACE-GOVERNANCE-WORKSPACE.md](./WORKSPACE-GOVERNANCE-WORKSPACE.md) |
| Publication safety | [WORKSPACE-PUBLICATION-SAFETY.md](./WORKSPACE-PUBLICATION-SAFETY.md) |
| Failure handling | [WORKSPACE-GOVERNANCE-FAILURE-HANDLING.md](./WORKSPACE-GOVERNANCE-FAILURE-HANDLING.md) |
| Permission Gateway | [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md) |

Contract detail pages below are thin pointers — prefer this index for terminology.
