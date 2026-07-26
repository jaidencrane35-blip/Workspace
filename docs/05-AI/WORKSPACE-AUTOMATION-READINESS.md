# Workspace Automation Readiness

Sprint 136 — readiness boundary before any autonomous action capability.

**Status:** Cognition pipeline is stable enough for planning autonomy *design*.  
**Status:** Autonomous execution is **not** introduced in this sprint (or prior).

## Verdict (Sprint 136 audit)

| Class | Finding |
|-------|---------|
| **A — Observation-only** | Observation, WorkspaceState, Environment, Continuity, Activity, Pattern, Operating State, Session projections |
| **B — Recommendation-only** | Attention, Intelligence, Decision Engine/Queue, Recommendation Engine, Adaptation, Trigger → Intent Proposal, Automation Contract prepare |
| **C — Permission-gated execution** | Command Pipeline → Permission Gateway → Allow → launch / mutate / decide_approval / execute_intent_request |
| **D — Unsafe autonomous execution** | **None found** — no cognition → silent action path |

---

## Safe future path

```
Cognition (Observation → … → Decision)
    ↓
Recommendation / Decision Candidate (human-visible)
    ↓
Accept / Prepare Intent (explicit)
    ↓
Permission Gate (Command Pipeline → Permission Gateway)
    ↓
Execution (post-Allow only)
    ↓
Audit / Execution Outcome
```

Every privileged mutation must declare capabilities and pass Gateway. AI, Automation,
and Plugin actors have empty default capability sets — ApprovalRequired until human grant.

## Forbidden path

```
Cognition
    ↓
silent action   ← NEVER
```

Also forbidden:

- Automation Contract approval ⇒ auto-launch
- Attention score ⇒ execute
- Experience DisplayReason ⇒ execute
- Recommendation Engine candidate ⇒ OS action without Intent + Gateway
- Trigger evaluation ⇒ `execution_authorized: true` without Pipeline
- Direct service calls that bypass `CommandPipeline`

---

## Concept classification (audit)

### A. Observation-only

| Concept | Notes |
|---------|-------|
| Desktop Observation / scheduler | Capture only; `attempt_execute` fails |
| WorkspaceStateEngine / Delta | Fact derivation only |
| Environment / Continuity / Activity Graph | Aggregators; `authority_effect: none` |
| Pattern / Working Style / Operating State | Observe / project; never prepare launch |

### B. Recommendation-only

| Concept | Notes |
|---------|-------|
| Attention Engine | Prioritize; structured reasons; no grant |
| Intelligence | Read-only aggregate |
| Decision Engine | Ranked candidates; accept → `submit_assistant_goal` handoff |
| Decision Queue | Human inbox; accept → proposal / approval handoff |
| Recommendation Engine | Typed next-steps; no accept-into-planner execute |
| Adaptation | Proposals; accept → Intent handoff |
| Automation Contract | Durable **definition**; approval ≠ capability grant |
| Trigger Evaluator / Intent Proposal | Relevance → proposal; audits `execution_authorized: false` |
| AI Planning / Action Proposals | Proposals until `submit_ai_*` through Pipeline |

### C. Permission-gated execution

| Concept | Notes |
|---------|-------|
| `CommandPipeline` | Sole privileged command entry |
| `PermissionGateway` | Sole Allow / Deny / ApprovalRequired |
| `LaunchApplication` | OS spawn after Allow |
| `ExecuteIntentRequest` | Mapped command still fully gated |
| `DecideApproval` | LocalUser allow-once / deny |
| Resource CRUD (zone/app/widget/…) | Capability-gated mutations |
| `submit_ai_*` | AI as actor through same Pipeline |

### D. Unsafe autonomous execution

| Finding | Status |
|---------|--------|
| Cognition → silent OS action | **None** |
| Contract auto-run workers | **Not implemented** (explicit non-goal) |
| Aggregator launch creep | **None** (`attempt_execute` guards) |
| Experience / Attention / Decision execute | **Hard-fail CannotExecute** |

---

## Experience traces ↔ future governance

Sprint 135 traces answer: *What cognition produced this displayed explanation?*

Future governed actions should preserve a parallel provenance chain:

| Stage | Preserve |
|-------|----------|
| Cognition | Structured reason (`AttentionReason` / `DecisionReason`) + `explanation_key` |
| Recommendation | Candidate / proposal id + source model refs |
| Experience (optional debug) | `ExperienceTranslationTrace` — resolver path + DisplayReason (not authority) |
| Permission | Gateway decision (Allow / Deny / ApprovalRequired) + actor + capability |
| Execution | Outcome (completed / failed / cancelled) + audit record |

**Rules:**

1. Traces are **evidence of translation**, not authorization.
2. Permission decisions are **independent** of catalog match / DisplayReason.
3. Execution outcomes must reference the Intent / command that passed Gateway — never an Attention item id alone as authority.
4. When automation ships, every action must remain reconstructible as:

```
reason → recommendation → permission decision → execution outcome
```

Optional Experience traces attach to the *reason → explanation* segment for debugging,
without becoming a grant.

See [WORKSPACE-EXPERIENCE-DEBUGGING.md](./WORKSPACE-EXPERIENCE-DEBUGGING.md).

---

## Readiness checklist (before introducing autonomy)

- [x] Cognition layers separated (Observation → … → Experience → UI)
- [x] Experience translation-only with catalog + UI boundary enforcement
- [x] Experience traces available for developer debugging
- [x] Aggregators hard-fail `attempt_execute`
- [x] Pipeline → Gateway universal for privileged commands
- [ ] Explicit Automation actor scopes (future)
- [ ] Action provenance record linking reason → permission → outcome (future)
- [ ] Confidence policy L3–L4 automation within approved scope only (policy exists; runtime path not opened)

**Do not open autonomy until provenance + Gateway path are designed end-to-end.**

**Sprint 137:** Recommendation provenance contract and ActionProposal architecture —
[WORKSPACE-RECOMMENDATION-PROVENANCE.md](./WORKSPACE-RECOMMENDATION-PROVENANCE.md).

**Sprint 138:** Recommendation identity & lifecycle —
[WORKSPACE-RECOMMENDATION-LIFECYCLE.md](./WORKSPACE-RECOMMENDATION-LIFECYCLE.md).

**Sprint 139:** Recommendation outcome & feedback —
[WORKSPACE-RECOMMENDATION-OUTCOME.md](./WORKSPACE-RECOMMENDATION-OUTCOME.md).

---

## Related docs

- [WORKSPACE-COGNITION-PIPELINE-CONTRACT.md](./WORKSPACE-COGNITION-PIPELINE-CONTRACT.md)
- [WORKSPACE-RECOMMENDATION-PROVENANCE.md](./WORKSPACE-RECOMMENDATION-PROVENANCE.md)
- [WORKSPACE-PLATFORM-COHERENCE.md](./WORKSPACE-PLATFORM-COHERENCE.md)
- [WORKSPACE-EXPERIENCE-DEBUGGING.md](./WORKSPACE-EXPERIENCE-DEBUGGING.md)
- [GOVERNED-AUTOMATION-CONTRACTS.md](./GOVERNED-AUTOMATION-CONTRACTS.md)
- [GOVERNED-TRIGGER-EVALUATION.md](./GOVERNED-TRIGGER-EVALUATION.md)
- [CONFIDENCE-POLICY.md](./CONFIDENCE-POLICY.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
