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
`eligible_for_future_candidate`. Assessment is informational only — never
creates a candidate. See
[WORKSPACE-DECISION-ENGINE-INTAKE-ASSESSMENT.md](./WORKSPACE-DECISION-ENGINE-INTAKE-ASSESSMENT.md).

`DecisionEngineIntakeEligibility` is a separate projected gate answering
whether an assessment may ever become a future DecisionCandidate — still
without creating one. See
[WORKSPACE-DECISION-ENGINE-INTAKE-ELIGIBILITY.md](./WORKSPACE-DECISION-ENGINE-INTAKE-ELIGIBILITY.md).

When eligibility is `eligible`, Decision Engine may materialize a DE-owned
`DecisionEngineIntakeCandidate` (`engine_decision_intake:*`) — an intake
acknowledgement for future evaluation, not a ranked `DecisionCandidate`.
Its DE-owned lifecycle (`active` / `withdrawn` / `invalidated`) is managed
separately from DecisionCandidate outcomes. Active intake candidates may receive
a DE-owned `DecisionEngineIntakeEvaluation` (examination record only — not
planning authority), then a `DecisionEngineIntakeDisposition`
(`retained` / `dismissed` / `deferred`) recording what DE does with that
evaluation — still not planning authority. A projected
`DecisionEngineIntakePromotionBoundary` then answers whether retained intake
may become eligible for *future* DecisionCandidate promotion — without
performing promotion. When allowed, a projected
`DecisionEngineCandidateCreationRequest` records the creation request. An
explicit `DecisionEngineCandidateCreation` may then create a native
`DecisionCandidate` (`engine_decision:intake:*`) with intake provenance — still
without scoring, planner handoff, Gateway, goals, or intents.
`DecisionCandidateLifecycleIntegration` then admits candidates into the normal
DE outcome lifecycle while keeping `native` and `recommendation_intake` origins
distinct and provenance immutable.
`DecisionCandidateEvaluationOriginContract` defines origin-specific evaluation
eligibility without scoring or ranking.
`DecisionCandidateEvaluationResolution` then records whether DE admits an
evaluated candidate into a future scoring path — still without scoring.
When admitted (`accepted_for_scoring`), a DE-owned `DecisionCandidateScore`
(DecisionScore result) may be created — scoring only; never ranking, selection,
planner, Gateway, goals, or intents.
`DecisionCandidateRanking` then projects comparative ordering of valid scores —
still without selection, planner, Gateway, goals, or intents.
`DecisionCandidateSelection` then records whether DE chooses to progress a ranked
candidate — still without execution, planner handoff, Gateway, goals, or intents.
`DecisionCandidateProgressionRequest` then records that DE wants the selected
candidate considered for downstream progression — still without planner, Gateway,
goals, or intents.
See
[WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE.md](./WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE.md),
[WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE-LIFECYCLE.md](./WORKSPACE-DECISION-ENGINE-INTAKE-CANDIDATE-LIFECYCLE.md),
[WORKSPACE-DECISION-ENGINE-INTAKE-EVALUATION.md](./WORKSPACE-DECISION-ENGINE-INTAKE-EVALUATION.md),
[WORKSPACE-DECISION-ENGINE-INTAKE-DISPOSITION.md](./WORKSPACE-DECISION-ENGINE-INTAKE-DISPOSITION.md),
[WORKSPACE-DECISION-ENGINE-INTAKE-PROMOTION-BOUNDARY.md](./WORKSPACE-DECISION-ENGINE-INTAKE-PROMOTION-BOUNDARY.md),
[WORKSPACE-DECISION-ENGINE-CANDIDATE-CREATION-REQUEST.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-CREATION-REQUEST.md),
[WORKSPACE-DECISION-ENGINE-CANDIDATE-CREATION.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-CREATION.md),
[WORKSPACE-DECISION-ENGINE-CANDIDATE-LIFECYCLE-INTEGRATION.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-LIFECYCLE-INTEGRATION.md),
[WORKSPACE-DECISION-ENGINE-CANDIDATE-EVALUATION-ORIGIN.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-EVALUATION-ORIGIN.md),
[WORKSPACE-DECISION-ENGINE-CANDIDATE-EVALUATION-RESOLUTION.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-EVALUATION-RESOLUTION.md),
[WORKSPACE-DECISION-ENGINE-CANDIDATE-SCORE.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-SCORE.md),
[WORKSPACE-DECISION-ENGINE-CANDIDATE-RANKING.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-RANKING.md),
[WORKSPACE-DECISION-ENGINE-CANDIDATE-SELECTION.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-SELECTION.md),
and
[WORKSPACE-DECISION-ENGINE-CANDIDATE-PROGRESSION-REQUEST.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-PROGRESSION-REQUEST.md).

Namespaces remain separate
(`recommendation:*` vs `engine_decision:*` vs `engine_decision_intake:*`).
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
