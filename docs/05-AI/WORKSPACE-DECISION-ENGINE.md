# Workspace Decision Engine

| Field | Value |
|-------|-------|
| **Purpose** | Define the governed recommendation synthesis layer |
| **Owner** | Architecture |
| **Status** | Phase 5 Batch 1 foundation (Sprints 80–81) |
| **Authority** | Never creates authority (`authority_effect: none`) |

---

## Principle

```
Human Intent
        ↓
Workspace Intelligence
        ↓
Attention Engine
        ↓
Decision Engine   ← this document
        ↓
AI Planning
        ↓
Proposal Evaluation
        ↓
Permission Gateway
        ↓
Execution
```

The Decision Engine answers: **“What should the Workspace recommend next?”**

It does **not** plan, authorize, or execute.

---

## Distinct from Decision Queue

| Layer | Role |
|-------|------|
| **Decision Queue** | Human inbox of pending decisions (`DecisionItem`) |
| **Decision Engine** | Ranked recommendation candidates (`DecisionCandidate`) for planning opportunities |

Both are informational aggregators. Neither expands authority.

---

## Model (Sprint 80)

- `DecisionCandidate` — ranked recommendation with scores + explanation
- `DecisionContext` — snapshot of inputs (attention, memory, prefs, goals, approvals)
- `DecisionReason` — structured reason only (never chain-of-thought)
- `DecisionScore` — attention / memory / personalization / goal contributions
- `DecisionExplanation` — headline + reasons + confidence
- `DecisionOutcome` — open | selected | dismissed | postponed | expired

Lifecycle overlay: `decision_engine_lifecycle` (presentation only).

---

## Synthesis (Sprint 81)

Input order:

```
Attention → memory/prefs/goals/queue context → Decision Engine → ranked candidates
```

Accept returns a **planner handoff** (`submit_assistant_goal`). The UI (or caller) invokes the Planner explicitly. Decision Engine never creates plans itself.

### Distinct from Recommendation Engine accept

Recommendation Engine accept remains agreement-only. After RE confirmation → seal →
handoff request → `DecisionEngineAcceptance`, Decision Engine may **observe** the
accepted sealed package as a `DecisionEngineIntakeReceipt`.

Observation is DE-owned and informational only:

- Does **not** create `DecisionCandidate`, goals, or intents
- Does **not** transfer ownership (RE remains `current_owner`)
- Does **not** set `handoff_command` / invoke planner
- Does **not** mutate Recommendation Engine overlays
- Seal mismatch → `seal_mismatch` receipt; still no candidate

`DecisionEngineIntakeAssessment` then evaluates each receipt (projected, not
persisted): blocked / superseded / duplicate / stale /
`eligible_for_future_candidate`. Eligibility is informational only — never
creates a candidate. See
[WORKSPACE-DECISION-ENGINE-INTAKE-ASSESSMENT.md](./WORKSPACE-DECISION-ENGINE-INTAKE-ASSESSMENT.md).

Namespaces remain separate (`recommendation:*` vs `engine_decision:*`).
Existing Attention/graph/goal synthesis and scoring are unchanged.

---

## Audits

| Event | Meaning |
|-------|---------|
| `decision.generated` | Snapshot produced |
| `decision.rank_changed` | Top ranking changed vs prior |
| `decision.selected` | User accepted → handoff |
| `decision.dismissed` | Dismissed or postponed |
| `decision.expired` | Reserved for expiry |

All carry `authority_effect: none`.

---

## Product UX

Work tab **Recommended Actions**:

- Why this matters now
- Confidence + supporting reasons
- Related goals / pending approvals
- Alternatives
- Accept / Postpone / Dismiss / Regenerate

Operator Console diagnostics: generate, inspect graph, inspect scoring, replay.
