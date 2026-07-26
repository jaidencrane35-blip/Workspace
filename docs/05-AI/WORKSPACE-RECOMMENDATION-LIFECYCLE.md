# Workspace Recommendation Identity & Lifecycle

Sprint 138 — continuity and lifecycle governance for recommendations before automation.

**Does not execute.** Acceptance is a lifecycle transition only — not Permission Gateway Allow.

## Principle

```
Native family ID (preserved)
    ↓
RecommendationIdentity (cross-references)
    ↓
RecommendationProvenance (immutable reasoning)
    +
RecommendationLifecycle (mutable state / times)
    ↓
[optional] ActionProposal (still authority_effect: none)
    ↓
Human / future Gateway path
```

Do **not** collapse existing ID namespaces without a dedicated migration sprint.

---

## Unified Recommendation Identity

Domain type: `RecommendationIdentity`

| Field | Meaning |
|-------|---------|
| `native_id` | Unchanged family-native id |
| `family` | `recommendation_engine` / `decision_engine` / `intelligence` / `adaptation` / `decision_queue` |
| `source_domain` | Originating model label (`attention`, `task_graph`, …) |
| `originating_reasoning_ref` | Attention item id or explanation key |
| `decision_ref` | Optional Decision Engine / Queue id |
| `action_proposal_ref` | Optional `action_proposal:…` once prepared |

### Native ID prefixes (audit)

| Family | Prefix / pattern | Example |
|--------|------------------|---------|
| Recommendation Engine | `recommendation:` | `recommendation:resolve_blocker:…` |
| Intelligence | `rec-attention-` / `rec-idle` | `rec-attention-{attention_id}` |
| Decision Engine | `engine_decision:` | `engine_decision:attention:…` |
| Decision Queue | Source-derived decision item ids | Aggregator synthetic ids |
| Adaptation | Projection ids (`adaptation:` convention) | Process-local status |
| ActionProposal | `action_proposal:` | `action_proposal:{native_recommendation_id}` |
| Intent Proposal | Trigger evaluator durable ids | Automation intent proposals |
| Permission Approval | `PermissionApprovalRequestId` | Gateway approval requests |

**Relationships:** Decision Engine may store `recommendation_id` pointing at Intelligence
`rec-attention-*` (not Recommendation Engine ids). ActionProposal refs Recommendation Engine
`native_id`. Identity records these links without merging namespaces.

**Gaps:** No durable first-seen identity store for Recommendation Engine; Decision
`created_at` is regenerate-time; Adaptation status not durable across restart.

---

## Lifecycle model

Domain type: `RecommendationLifecycleState`

| State | Meaning |
|-------|---------|
| **Created** | Governance record instantiated |
| **Available** | Eligible for surfaces |
| **Presented** | Shown to a human surface |
| **Accepted** | Human accepted (handoff may follow — still not execute) |
| **Rejected** | Human rejected / dismissed |
| **Expired** | Timed out / source gone |
| **Superseded** | Replaced by a newer recommendation |

### Valid transitions

```
Created → Available | Expired | Superseded
Available → Presented | Expired | Superseded | Rejected
Presented → Accepted | Rejected | Expired | Superseded | Available
```

Terminal for further progress: Accepted, Rejected, Expired, Superseded
(Accepted cannot return to Available).

### Ownership & authority

| Concern | Owner |
|---------|-------|
| State machine contract | Domain (`RecommendationLifecycle`) |
| Transition validation | Domain (`allows_transition` / `transition`) |
| Transition actor | Recorded as actor id metadata — **not** a capability grant |
| Persistence (today) | Decision Engine / Queue outcome overlays only; Recommendation Engine still ephemeral |
| Persistence (future) | Lifecycle overlay keyed by `RecommendationIdentity` — payloads remain regenerable |

### Lifecycle metadata (mutable)

| Field | Meaning |
|-------|---------|
| `created_at` | Record creation time |
| `presented_at` | First / last presentation stamp |
| `resolved_at` | Acceptance / rejection / expiry / supersede time |
| `resolution_type` | `accepted` / `rejected` / `expired` / `superseded` |
| `transition_actor_id` | Who drove the last transition |

### Provenance (immutable)

`RecommendationProvenance.reasoning_origins`, `explanation_keys`, and `source_evidence`
must not change across lifecycle transitions. Expiry and acceptance leave reasoning intact.

---

## Acceptance does not execute

```
Presented → Accepted
    ↓
authority_effect remains "none"
    ↓
optional handoff command (submit_assistant_goal, …)
    ↓
CommandPipeline → Permission Gateway (only if a privileged command is issued later)
```

`ActionProposal::attempt_execute()` and recommendation/decision `attempt_execute` guards
remain hard-fail.

---

## Governance record

`RecommendationGovernanceRecord` = Identity + Provenance + Lifecycle.

Used for contract tests and future durability — not wired to automation in Sprint 138.

---

## Related docs

- [WORKSPACE-RECOMMENDATION-PROVENANCE.md](./WORKSPACE-RECOMMENDATION-PROVENANCE.md)
- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
- [WORKSPACE-RECOMMENDATION-ENGINE.md](./WORKSPACE-RECOMMENDATION-ENGINE.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
