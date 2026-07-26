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
| Persistence (today) | `recommendation_lifecycle` overlay keyed by `(workspace_id, native_id)` — payloads remain regenerable |
| Surfaces | Operator Console + Work Intelligence Present / Accept / Reject (decision records only); `RecommendationExplanationView` explains why shown |
| Explanation | Structured evidence / keys / lifecycle notes — never CoT, never authority, never Decision Engine handoff |

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

## Runtime wiring (Sprints 192–196)

| Step | Behavior |
|------|----------|
| Generate | Ensures `Available` overlay for each regenerable candidate; projects lifecycle onto `RecommendationItem` |
| Present | `Available → Presented` via `GateRecommendationEngineWrite` |
| Accept | Auto-presents if still Available, then `Presented → Accepted`; persists `RecommendationOutcome` |
| Reject | `Available\|Presented → Rejected`; persists `RecommendationOutcome` |
| Authority | `authority_effect` remains `none`; no score mutation; no Gateway bypass |

IPC: `present_recommendation` / `accept_recommendation` / `reject_recommendation`.

## Continuity on regenerate (Sprints 197–201)

| Rule | Behavior |
|------|----------|
| Orphan open overlay | Source id absent from live candidates → `Expired` + `RecommendationOutcome` (`expired_without_action`) |
| Content change (open) | Fingerprint mismatch on Available/Presented → `Superseded` + outcome, then new `Available` generation |
| Content change (terminal) | Accepted/Rejected/Expired/Superseded with new fingerprint → new `Available` generation (prior resolution stays in audit; Accepted cannot transition) |
| Active surfaces | `summary_projection` / Intelligence top candidates exclude terminal lifecycle states |
| Fingerprint | Deterministic from kind/title/reason/impact/evidence summaries — **not** a score; never mutates cognition |

Expire / supersede are continuity transitions only — never execute, never grant authority, never auto-learn.

## Outcome history (Sprints 207–211)

| Concern | Behavior |
|---------|----------|
| Current outcome | On overlay + projected onto terminal candidates |
| Prior outcomes | `prior_outcomes` retained when content supersedes or generation reopens |
| History list | `state.history` / `history_count` for Operator / Work (immutable feedback) |
| Decision Engine | Handoff still **deferred** — acceptance records outcomes only |

---

## Related docs

- [WORKSPACE-RECOMMENDATION-OUTCOME.md](./WORKSPACE-RECOMMENDATION-OUTCOME.md)
- [WORKSPACE-RECOMMENDATION-PROVENANCE.md](./WORKSPACE-RECOMMENDATION-PROVENANCE.md)
- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
- [WORKSPACE-RECOMMENDATION-ENGINE.md](./WORKSPACE-RECOMMENDATION-ENGINE.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
