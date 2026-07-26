# Workspace Recommendation Provenance

Sprint 137 — governance bridge between cognition and future permission-controlled action.

**Does not execute.** Recommendations may inform humans and future ActionProposals;
they never bypass Permission Gateway.

## Principle

Every recommendation must be reconstructible as:

```
Evidence
    ↓
Reasoning (AttentionReason / DecisionReason)
    ↓
Recommendation (identity + metadata)
    ↓
Experience translation (DisplayReason / optional trace)
    ↓
Human decision (accept / reject / defer / ignore)
    ↓
[future] ActionProposal → Permission Gateway → Execution
```

Question this contract answers:

> Why was this recommendation created?

---

## Required provenance fields

| Field | Meaning | Current source |
|-------|---------|----------------|
| **Source evidence** | Facts / model refs that ground the suggestion | `RecommendationEvidence`, `DecisionReason.evidence_ref`, Adaptation evidence |
| **Reasoning origin** | Structured cognition reasons | `AttentionReason`, `DecisionReason.attention_reason` |
| **Recommendation identity** | Stable id within its family | `RecommendationItem.id`, `DecisionCandidate.id`, `WorkspaceRecommendation.id` |
| **Experience translation reference** | Catalog keys / optional trace match keys | `explanation_key` → Experience; `ExperienceTranslationTrace.resolver_path.match_key` |
| **Confidence / priority metadata** | Display/strength hints — not authority | `RecommendationConfidence`, Attention priority/score, DecisionScore |
| **Optional future capability target** | What capability a future ActionProposal might request | Architecture-only on `ActionProposal.requested_capability` — **not a grant** |

Domain helper: `RecommendationProvenance` / `ActionProposal` in
`packages/domain/src/action_proposal/` (Sprint 137 architecture types).

---

## Recommendation families (do not conflate)

| Family | Type | Accept path today | Executes? |
|--------|------|-------------------|-----------|
| Recommendation Engine | `RecommendationItem` | None (informational) | No |
| Intelligence | `WorkspaceRecommendation` | None | No |
| Decision Engine | `DecisionCandidate` | Select → `submit_assistant_goal` handoff | No (handoff only) |
| Decision Queue | `DecisionItem` | Accept → source handoff / `decide_approval` | No |
| Adaptation | `AdaptationProposal` | Accept → `submit_assistant_goal` handoff | No |

Three id namespaces remain distinct by design:
`recommendation:*`, `rec-attention-*`, `engine_decision:*`.

---

## Lifecycle audit (Sprint 137)

### Recommendation Engine

| Stage | Behavior |
|-------|----------|
| **Creation** | Ephemeral projection (`generate` / `generate_with_inputs`) |
| **Presentation** | Intelligence summary + Work / Operator surfaces |
| **Acceptance** | Not supported |
| **Rejection** | Not supported |
| **Expiry** | N/A — rebuilt each generate |

**Persistence:** none for payloads. Audit: `workspace.recommendation_engine.generated`.  
**Immutable fields:** entire candidate payload (regenerated).  
**Mutable fields:** none.

Attention-derived items clone `attention_reasons` verbatim; other sources leave them empty
(reason + evidence only).

### Intelligence `recommended_actions`

| Stage | Behavior |
|-------|----------|
| Creation | Top Attention items → `WorkspaceRecommendation` with `reasons` |
| Presentation | Work / Assistant |
| Accept / reject / expiry | None |

**Persistence:** none. Bootstrap `rec-idle` has empty reasons.

### Decision Engine

| Stage | Behavior |
|-------|----------|
| Creation | Synthesize from Attention / graph / goals |
| Presentation | Work Intelligence / Operator |
| Acceptance | `Selected` overlay + handoff `submit_assistant_goal` |
| Rejection | `Dismissed` |
| Deferral | `Postponed` (re-openable) |
| Expiry | Domain has `Expired`; **no service path sets it yet** |

**Persistence ownership:**
- Payloads: ephemeral (rebuilt)
- Outcome overlay: durable `decision_engine_lifecycle` (outcome, actor, timestamps only)

**Immutable (projection):** title, explanation, scores, reasons, ids.  
**Mutable:** `outcome` (+ overlay metadata).  
**Gap:** `created_at` is regenerate-time, not first-seen.

### Decision Queue

| Stage | Behavior |
|-------|----------|
| Creation | Aggregate approvals / proposals / plans |
| Presentation | Work inbox |
| Accept / reject | Source-specific handoffs; never grants permissions itself |
| Overlay states | pending / viewed / deferred / dismissed (+ accepted/rejected via source) |

**Persistence:** lifecycle overlay only — not full item payloads.

### Adaptation

| Stage | Behavior |
|-------|----------|
| Creation | Projection from Pattern / Recommendations / OS / Composition |
| Accept | Handoff to `submit_assistant_goal` |
| Status | Process-local overlays (lost on restart) |

---

## Future ActionProposal boundary

Defined in Domain as `ActionProposal` — **architecture only**.

Must contain:

| Field | Role |
|-------|------|
| `recommendation_ref` | Points at recommendation family id |
| `provenance` | Full `RecommendationProvenance` |
| `requested_capability` | Optional future capability string |
| `permission_requirements` | List for future Gateway evaluation |
| `risk` | Level / summary / reversible |
| `authority_effect` | Always `"none"` until Gateway Allow |

```
Recommendation (+ provenance)
    ↓
ActionProposal (architecture)
    ↓
[future] privileged command via CommandPipeline
    ↓
Permission Gateway
    ↓
Execution (post-Allow)
```

**Forbidden:** constructing an ActionProposal that launches, grants, or sets
`authority_effect` to anything other than `"none"` without Gateway.

`ActionProposal::attempt_execute()` hard-fails `CannotExecute`.

---

## Experience traces ↔ provenance

Sprint 135 traces answer translation. Provenance answers creation.

```
AttentionReason.explanation_key
    ↓ Experience resolve_*_traced
ExperienceTranslationTrace.resolver_path.match_key
    ↓ attach to RecommendationProvenance.experience_trace_match_keys
Human sees DisplayReason
    ↓ accept (Decision / Adaptation)
Handoff command (still gated)
```

Debugging flow:

1. Recommendation id + family  
2. Evidence + `attention_reasons` / `DecisionReason`  
3. Experience match keys / DisplayReason  
4. Human decision overlay  
5. [future] ActionProposal → permission decision → execution outcome  

Traces never authorize. Permissions never depend on catalog match success.

---

## Known gaps (documented, not fixed this sprint)

1. No durable per-candidate creation history (regenerated projections).
2. Decision Engine `Expired` unused; Adaptation status not durable.
3. Non-Attention Recommendation Engine items lack structured reasons.
4. Intelligence ≠ Recommendation Engine id namespaces.
5. Accept overlays do not freeze the explanation snapshot used at decision time.

Future work should close provenance durability **before** autonomy — see
[WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md).

---

## Related docs

- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
- [WORKSPACE-COGNITION-PIPELINE-CONTRACT.md](./WORKSPACE-COGNITION-PIPELINE-CONTRACT.md)
- [WORKSPACE-RECOMMENDATION-ENGINE.md](./WORKSPACE-RECOMMENDATION-ENGINE.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
- [WORKSPACE-EXPERIENCE-DEBUGGING.md](./WORKSPACE-EXPERIENCE-DEBUGGING.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
