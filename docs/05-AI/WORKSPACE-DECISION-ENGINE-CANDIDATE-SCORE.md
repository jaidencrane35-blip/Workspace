# DecisionScore (DecisionCandidateScore)

| Field | Value |
|-------|-------|
| **Purpose** | DE-owned scoring result for candidates admitted via `accepted_for_scoring` |
| **Status** | Persisted score identity; never ranks or selects |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Why this exists

`DecisionCandidateEvaluationResolution.accepted_for_scoring` is scoring-path
admission only. It does **not** produce a score.

`DecisionCandidateScore` is the genuine next artifact: a DE-owned DecisionScore
result with identity and explainability.

The embedded `DecisionScore` value type (total + contributions + factors) remains
the score breakdown payload used on candidates and inside this artifact.

## Boundary

```
EvaluationResolution (accepted_for_scoring)
        ↓ try_create
DecisionCandidateScore (DecisionScore result)
        ✗ score ≠ ranking / selection
        ✗ no planner / Gateway / goals / intents / execution
        ✗ never mutates recommendation_lifecycle
        ✗ never mutates candidate outcome or synthesis score
```

## Rules

| Gate | Required |
|------|----------|
| Resolution | `accepted_for_scoring` |
| Lifecycle | Integrated and valid |
| Provenance | Valid for origin |
| Candidate | Open or postponed; not withdrawn/invalidated |
| Source package | Valid for recommendation_intake |

Blocked: `rejected_for_scoring`, blocked resolution, withdrawn candidate,
invalid provenance, invalid source package.

## Persistence

Score history is not derivable from resolution alone → DE-owned table
`decision_candidate_score`.

Comparative ordering is a separate projected artifact — see
[WORKSPACE-DECISION-ENGINE-CANDIDATE-RANKING.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-RANKING.md).

## Related

- [WORKSPACE-DECISION-ENGINE-CANDIDATE-EVALUATION-RESOLUTION.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-EVALUATION-RESOLUTION.md)
- [WORKSPACE-DECISION-ENGINE-CANDIDATE-RANKING.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-RANKING.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
- [WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md](./WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md)
