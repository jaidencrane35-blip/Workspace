# Projection Integrity & Immutable Evidence Consumption

| Field | Value |
|-------|-------|
| **Purpose** | Protect consumption of protected lifecycle evidence |
| **Owner** | Platform Kernel |
| **Status** | Active |

## Principle

Domain state remains canonical. Projections and React surfaces may **display**
lifecycle evidence but must not invent, mutate, or erase it.

| Layer | Responsibility |
|-------|----------------|
| Services / domain | Lifecycle authority and continuity writes |
| Repositories | Persistence guards for transitions and immutable identity |
| Summaries / IPC | Project canonical state; actionable surfaces exclude terminals without dropping history |
| React | Projection-only rendering; no lifecycle ownership |

## Identity vocabulary

| Kind | Meaning | Examples |
|------|---------|----------|
| **Source identity** | Owning subsystem key for a live decision | `source_type` + `source_id` |
| **Overlay identity** | Decision Queue lifecycle row for presentation | `DecisionLifecycleOverlay` / `DecisionOverlayHistoryEntry` (`decision_state`, `orphaned`) |
| **Outcome identity** | Recommendation Engine terminal outcome evidence | `RecommendationHistoryEntry.outcome.outcome_id` |

Do not conflate RE outcome identity with DQ overlay identity.

## Rules

1. **Actionable ≠ absent evidence.** Compact summaries filter active candidates
   but must carry `history` / `history_count` so consumers cannot infer “no
   evidence” from an empty actionable list.
2. **`history_count` is authoritative.** `history` is a truncated projection
   window (`summary(limit)` / generate window). `history.length` must never be
   treated as the full evidence count; an empty `history` array with
   `history_count > 0` still means retained evidence exists outside the window.
3. **Decision Queue orphans retain overlays.** Missing live sources expire open
   overlays (`Expired`) and keep already-terminal overlays. Overlays are **not
   deleted** — there is no Decision Queue overlay deletion API.
4. **Actionable / historical separation (full queue and summary).**
   - `items` — only actionable overlay states (`pending` / `viewed` / `deferred`)
   - `history` — terminal overlay evidence (`dismissed` / `expired`, with
     `orphaned` when the live source is absent)
   - The same overlay must not appear in both channels.
5. **History is non-actionable and non-executable.** Entries always project
   `actionable: false` and `authority_effect: none`. They are not
   `DecisionItem`s, cannot be converted into actionable items, and must not
   drive lifecycle mutations or execution.
6. **Frontend responsibilities.** React consumes projected fields only.
   Recommendation active filtering mirrors `RecommendationItem::is_active_lifecycle`
   (unknown states inactive). Decision Queue helpers only verify projected
   non-actionability — they do not own lifecycle or invent overlay state.
7. **Missing evidence is not inferred truth.** RE terminal history requires
   projected outcome identity. DQ terminal history requires projected overlay
   identity (`source_type` / `source_id` / `decision_state`). UI helpers must
   not synthesize resolutions or overlays.

## Related

- [Persistence Boundary Governance](./PERSISTENCE-BOUNDARY-GOVERNANCE.md)
- [Lifecycle Governance](./LIFECYCLE-GOVERNANCE.md)
- [Workspace Experience Contract](../05-AI/WORKSPACE-EXPERIENCE-CONTRACT.md)
