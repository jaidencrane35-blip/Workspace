# Workspace Attention Engine

| Field | Value |
|-------|-------|
| **Purpose** | A governed prioritization layer over Workspace context |
| **Status** | Foundation — ranked context with structured explanations; contract locked (Sprint 127) |
| **Owner** | Lead Software Engineer |

---

## Permanent rule

Attention is informational. Attention grants nothing, authorizes nothing, executes nothing.

```
Human Intent → … → Permission Gateway → Execution → Audit
```

Attention is **not** an observer, desktop monitor, automation engine, or executor.

---

## Stack position

```
Observation
        ↓
WorkspaceStateEngine
        ↓
WorkspaceState
        ↓
Environment
        ↓
Decision Queue / Continuity / Activity / Task Graph / Composition / Purpose / Evolution
        ↓
Attention Engine (ranked context + structured reasons)
        ↓
Workspace Intelligence / Recommendations / Assistant
```

Desktop facts reach Attention **only** via Environment ← WorkspaceState.  
Attention never reads Observation snapshots, DesktopWindow DTOs, or Win32.

No new persistence. Synthetic ids: `attention:{source_type}:{source_id}`.

---

## Facts vs inference

| Layer | Role |
|-------|------|
| Environment, Decision Queue, Continuity, Activity, Task Graph, Composition, Purpose, Evolution | **Facts** (owned elsewhere) |
| Attention `score` / `priority` / `urgency` / `category` / ranked `top_items` | **Inference — ranking** |
| Attention `reasons` (`AttentionReason`) | **Inference — explanation** |

### Score vs explanation

| Field | Answers |
|-------|---------|
| `score` + `score_factors` | What priority value does this receive? |
| `reasons` | Why did it receive attention? |
| `explanation` | Short narrative from the source fact |

`AttentionReason`: `{ source, signal, weight, explanation_key }`  
Reasons are normalized: unique `explanation_key`, ordered weight DESC then key ASC.  
UI may display `explanation_key` without recomputing scores.

Reason weights **sum to `score`** — there is exactly one scoring path, and `reasons`
expose it rather than restate it. A reason with no score contribution is not a reason.

---

## Input signal owners

| Signal | Owner | Meaning | Notes |
|--------|-------|---------|-------|
| Outstanding / blocked decisions | Decision Queue | Pending human decisions | High confidence |
| Blocked / waiting / in-progress tasks | Task Graph | Open work units | High confidence |
| Interrupted / resumable / dormant / focus | Continuity / WorkflowContext | Resume narrative | Medium–high |
| Commitment pending | AutomationContract via Continuity | Contract status only | Surfaces; never executes |
| Environment disconnect / missing app | Environment ← WorkspaceState | Desktop–work gaps | Medium |
| Composition gaps | Composition | Logical membership gaps | Desktop-like kinds deferred to Environment when present |
| Purpose outcome / obstacles | Purpose | Why work exists / blockers | Medium |
| Evolution insights | Evolution | How work changed | Medium |
| Activity progress | Activity Graph | Recent timeline (≤3) | Low score informative |
| Recommendation / Pattern | RE / Pattern enrich | Post-base enrich only | Not base SoT |

No fabricated ML signals. No deadline signal until Task Graph carries real deadlines.

---

## Boundary with Intelligence / Recommendations

| System | Question |
|--------|----------|
| **Attention** | What deserves focus? |
| **Recommendation Engine** | What could the user do? |
| Intelligence `recommended_actions` | Attention tops re-labeled for Assistant display — **not** a second Recommendation Engine |

Do not merge Attention into Recommendations. Do not treat `recommended_actions` as actionable automation.

### Consumer rules

| Consumer | Reads | Must not |
|----------|-------|----------|
| Intelligence | `top_items` order, `reasons` | Re-rank, re-score, or rewrite reasons |
| Decision Engine | `score` as one contribution to its own candidate score | Treat Attention rank as its own rank |
| Operating State | `top_items`, `score_factors` as evidence | Re-threshold or re-band |
| Session / Experience | `category`, `priority` | Re-derive banding from raw `score` |

Consumers read `priority` for banding. A raw `score` threshold outside Attention is a
duplicated prioritization rule and will drift from `score_to_priority`.

---

## Canonical APIs

| API | Role |
|-----|------|
| `generate_with_task_graph` | Preferred shared-input path (Intelligence / Operating State) |
| `generate` | Standalone / IPC diagnostics — loads Environment via WorkspaceStateEngine once |

## Determinism contract

1. **Ordering** — score DESC → priority rank ASC → id ASC.
2. **Unique ids** — enforced by `WorkspaceAttentionState::from_items`; the highest-ranked
   projection of an id survives. Callers cannot emit a duplicate.
3. **Ordering-independent cuts** — per-source caps are applied by attention rank
   (score DESC, id ASC), never by upstream vector position. Truncating raw input would let
   an upstream reordering silently change which facts reach Attention.
4. **One exception, by design** — Activity Graph items all score equally, so recency is the
   selection criterion. Timeline order is Activity Graph's contract, not incidental order.

Determinism is a property of Attention **over fixed facts**. Two consecutive `generate`
calls are not input-identical: each call audits, the Activity Graph grows, and
Purpose/Evolution legitimately re-infer.

---

## Audits (`authority_effect: none`)

- `workspace.attention.generated`
- `workspace.attention.summary.generated`

---

## Related

- [Workspace Environment Model](WORKSPACE-ENVIRONMENT-MODEL.md)
- [Workspace Continuity Engine](WORKSPACE-CONTINUITY-ENGINE.md)
- [Workspace Platform Coherence](WORKSPACE-PLATFORM-COHERENCE.md)
- [Governed Decision Queue](GOVERNED-DECISION-QUEUE.md)
- [Workspace Intelligence Foundation](WORKSPACE-INTELLIGENCE-FOUNDATION.md)
- [Workspace Recommendation Engine](WORKSPACE-RECOMMENDATION-ENGINE.md)
