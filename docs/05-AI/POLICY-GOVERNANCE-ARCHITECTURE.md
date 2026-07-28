# Policy & Governance Architecture (Programme III — Batch 2)

| Field | Value |
|-------|-------|
| **Purpose** | Policy reasoning layer — explainable governance recommendations |
| **Owner** | `PolicyGovernanceService` (DurableStore — **evaluation evidence only**) |
| **Status** | Active |
| **Permission owner** | No — `PermissionGateway` remains final authority |
| **Execution authority** | No |
| **Lifecycle owner** | No |

## Core principle

**Policy explains authority. It does not become authority.**

```
Workspace State Envelope
        │
        ▼
Policy & Governance Engine
        │
        ▼
Governance Decision Evidence
        │
        ▼
PermissionGateway
        │
        ▼
Allow / Deny / Require Approval
```

The Policy Engine advises. The Gateway authorises.

## What this owns

- policy definitions (versioned)
- policy evaluation results
- governance recommendations / explanations
- evaluation history evidence

## What this does not own

- capability ownership / permission grants
- execution decisions / command routing
- lifecycle state / source truth / user identity

## Result model

| Result | Meaning |
|--------|---------|
| `Compliant` | Active policies report compliance for observed context |
| `Violation` | Policy reports a violation |
| `RequiresReview` | Policy says approval/review required |
| `Unknown` | Incomplete/unavailable context — **fail closed** |
| `NotApplicable` | No active policy applied |

**Unknown context must not become approval.**

## Naming note

Domain type for Programme III rules is `PolicyDefinition` (avoids colliding with
`action_proposal::GovernancePolicy` used for outcome-adaptation review requirements).

## Commands

| Command | Kind | Capability |
|---------|------|------------|
| `GenerateGovernanceEvaluation` | Mutation | `work_context.write` |
| `GetGovernanceEvaluation` | Query | `work_context.read` |
| `GetGovernanceSummary` | Query | `work_context.read` |
| `ExplainGovernanceDecision` | Query | `work_context.read` |

Context source: `WorkspaceStateCompositionService::load_snapshot` only — never silent generate/refresh.

## Determinism

Same policy catalog revision + same workspace state revision ⇒ same evaluations / recommendation identity.

## Forbidden

- second permission system (`policy.allow(command)`)
- execute commands / create capabilities / grant permissions
- mutate lifecycle sources
- assume missing risk/context is safe

## Recovery

| Scenario | Result |
|----------|--------|
| Missing policy | PolicyUnavailable / no fabricated Compliant |
| Missing state | EvaluationIncomplete → Unknown |
| Corrupt evidence | EvaluationFailed |
| Restart | Preserve durable evaluation evidence |

## Related

- [Programme III — Coherent Workspace Runtime](./PROGRAMME-III-COHERENT-WORKSPACE-RUNTIME.md)
- [Unified Workspace State Architecture](./UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
