# Workspace Cognition Pipeline Contract

Sprint 136 — complete layer contract for the cognition stack before any autonomous
action capability is introduced.

## Permanent pipeline

```
Observation
    ↓
Delta
    ↓
WorkspaceStateEngine
    ↓
WorkspaceState
    ↓
Environment
    ↓
Attention
    ↓
Intelligence
    ↓
Decision
    ↓
Experience
    ↓
UI
```

Governance for **actions** (separate from cognition):

```
Human Intent / Recommendation accept
    ↓
Prepare Intent
    ↓
Command Pipeline
    ↓
Permission Gateway
    ↓
Execution
    ↓
Audit
```

Cognition **never** jumps to Execution. See
[WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md).

---

## Layer contracts

### Observation

| | |
|--|--|
| **Responsibility** | Capture desktop/workspace facts (windows, monitors, apps) |
| **Inputs** | OS / capture triggers (startup, schedule, manual) |
| **Outputs** | Observation pass / snapshot |
| **Ownership** | Observation services + durable observation records |
| **Forbidden** | Scoring attention; inventing recommendations; launching apps; bypassing Pipeline/Gateway |

### WorkspaceState (via Delta + WorkspaceStateEngine)

| | |
|--|--|
| **Responsibility** | Canonical derived workspace fact model from observation deltas |
| **Inputs** | Observation snapshots / deltas |
| **Outputs** | `WorkspaceState` |
| **Ownership** | WorkspaceStateEngine |
| **Forbidden** | Presentation wording; Attention scoring; Decision synthesis; execution |

### Environment

| | |
|--|--|
| **Responsibility** | Interpret WorkspaceState into environment understanding (focus, apps, gaps) |
| **Inputs** | WorkspaceState |
| **Outputs** | Environment model / projections |
| **Ownership** | Environment aggregator (`authority_effect: none`) |
| **Forbidden** | Re-running Observation; Attention ranking; UI rationale invention; execute |

### Attention

| | |
|--|--|
| **Responsibility** | Prioritize what deserves focus; emit structured `AttentionReason` |
| **Inputs** | Environment + upstream fact aggregators (tasks, decisions, continuity, …) |
| **Outputs** | `WorkspaceAttentionState`, scored items, `AttentionReason[]` |
| **Ownership** | Attention Engine aggregator |
| **Forbidden** | Mutating source facts; Decision accept/handoff; Experience catalog wording; execute / grant |

### Intelligence

| | |
|--|--|
| **Responsibility** | Aggregate read-only understanding for Work / Assistant surfaces |
| **Inputs** | Attention, Decision Engine summaries, Recommendation Engine, Session-adjacent models |
| **Outputs** | `WorkspaceIntelligenceState` (projections only) |
| **Ownership** | Intelligence aggregator |
| **Forbidden** | Owning durable authority; launching; approving permissions; inventing DisplayReason |

### Decision

| | |
|--|--|
| **Responsibility** | Synthesize ranked next-step candidates; structured `DecisionReason` |
| **Inputs** | Attention, memory/preferences highlights, goals, approvals (informational) |
| **Outputs** | `DecisionEngineState` / candidates; accept → planner handoff only |
| **Ownership** | Decision Engine aggregator (+ lifecycle overlay) |
| **Forbidden** | Executing candidates; granting capabilities; bypassing Gateway; Experience wording ownership |

Distinct: **Decision Queue** is the human inbox aggregator over pending decisions —
also `attempt_execute` → CannotExecute. Not an execution queue.

### Experience

| | |
|--|--|
| **Responsibility** | Translate structured meaning → presentation (`DisplayReason`, Session experience) |
| **Inputs** | `AttentionReason` / `DecisionReason` / Session fields; explanation catalog |
| **Outputs** | `DisplayReason`, optional `ExperienceTranslationTrace` (developer-only) |
| **Ownership** | Experience Layer + catalog contract |
| **Forbidden** | Scoring; ranking; cognition inference; Domain wording ownership; execute; exposing traces on Work/Assistant |

### UI

| | |
|--|--|
| **Responsibility** | Render Experience outputs and engine summary metadata |
| **Inputs** | `DisplayReason`, Experience / Intelligence projections |
| **Outputs** | Layout / interaction (no new meaning) |
| **Ownership** | App surfaces (Work, Assistant, Operator) |
| **Forbidden** | Reconstructing rationale from `explanation_key` / scores; importing resolver internals (Work/Assistant); silent execution |

Operator is a **diagnostic** surface — scores and traces allowed; still no cognition→execute.

---

## Ownership summary

| Layer | Owns meaning? | Owns presentation? | Owns execution? |
|-------|---------------|--------------------|-----------------|
| Observation / WorkspaceState | Facts | No | No |
| Attention / Intelligence / Decision | Structured reasoning | No | No |
| Experience | No (reads meaning) | Yes | No |
| UI | No | Rendering only | No |
| Pipeline + Gateway | No | No | Authorization boundary |
| Execution services | Outcomes | No | Post-Allow only |

---

## Forbidden cross-layer dependencies

| From → To | Forbidden |
|-----------|-----------|
| Experience → Attention scoring | Must not re-score |
| UI → Domain reasoning fields as copy | Must use Experience translation |
| Attention / Decision / Experience → Launch / Grant APIs | Must not call |
| Observation → Decision / Experience | Skip Environment/Attention path |
| Recommendation accept → OS spawn | Must go Intent → Pipeline → Gateway |
| Cognition generate → `attempt_execute` success | All hard-fail CannotExecute |

---

## Related docs

- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
- [WORKSPACE-PLATFORM-COHERENCE.md](./WORKSPACE-PLATFORM-COHERENCE.md)
- [WORKSPACE-SEMANTIC-INTEGRITY.md](./WORKSPACE-SEMANTIC-INTEGRITY.md)
- [WORKSPACE-EXPERIENCE-CONTRACT.md](./WORKSPACE-EXPERIENCE-CONTRACT.md)
- [WORKSPACE-EXPERIENCE-DEBUGGING.md](./WORKSPACE-EXPERIENCE-DEBUGGING.md)
- [WORKSPACE-COGNITION-INTEGRITY-AUDIT.md](./WORKSPACE-COGNITION-INTEGRITY-AUDIT.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
