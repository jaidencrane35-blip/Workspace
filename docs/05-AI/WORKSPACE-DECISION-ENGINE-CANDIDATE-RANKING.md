# DecisionCandidateRanking

| Field | Value |
|-------|-------|
| **Purpose** | DE-owned comparative ordering of scored candidates |
| **Status** | Projected from valid DecisionCandidateScore artifacts |
| **Authority** | Always `none` |
| **Owner** | Decision Engine |

---

## Why this exists

`DecisionCandidateScore` is a per-candidate scoring result. It does **not**
compare peers.

`DecisionCandidateRanking` is the genuine next artifact: ordered comparison
output with ranking identity and explainability — without selection authority.

## Audit (score → rank)

| Question | Answer |
|----------|--------|
| Ranking admission wrapper required? | **No** — score validity gates belong in ranking |
| Score can feed ranking directly? | **Yes** |
| Ranking introduces selection? | **No** if it never chooses a winner / mutates outcome |
| Missing comparison step before rank? | **No** — ranking *is* that comparison |

Rejected: score wrappers, score permissions, score readiness, compatibility layers.

## Boundary

```
DecisionCandidateScore (valid)
        ↓ derive / try_rank_member
DecisionCandidateRanking (ordered entries)
        ✗ rank ≠ selection / winner
        ✗ no planner / Gateway / goals / intents / execution
        ✗ never mutates recommendation_lifecycle or candidate outcome
```

## Rules

| Include when | Exclude when |
|--------------|--------------|
| Valid DecisionCandidateScore | Missing score |
| Active (open/postponed) | Withdrawn / invalidated |
| Valid provenance + lifecycle | Invalid provenance |

Ordering: `score_total` desc, then `decision_candidate_id` asc (stable tiebreak).

## Persistence

Ranking is reconstructible from current scores + candidate/lifecycle state →
**projected only** (no table).

## Related

- [WORKSPACE-DECISION-ENGINE-CANDIDATE-SCORE.md](./WORKSPACE-DECISION-ENGINE-CANDIDATE-SCORE.md)
- [WORKSPACE-DECISION-ENGINE.md](./WORKSPACE-DECISION-ENGINE.md)
- [WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md](./WORKSPACE-RECOMMENDATION-DECISION-BOUNDARY.md)
