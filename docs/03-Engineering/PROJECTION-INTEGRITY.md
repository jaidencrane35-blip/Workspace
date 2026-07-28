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
| **Source identity** | Owning subsystem key for a live decision | DQ `source_type` + `source_id`; DE `candidate_key` |
| **Overlay identity** | Lifecycle row for presentation continuity | DQ `DecisionLifecycleOverlay` / `DecisionOverlayHistoryEntry`; DE `DecisionEngineOverlay` / `DecisionArtifactHistoryEntry` |
| **Outcome identity** | Recommendation Engine terminal outcome evidence | `RecommendationHistoryEntry.outcome.outcome_id` |

Do not conflate RE outcome identity with DQ/DE overlay/artifact identity.

## Rules

1. **Actionable ≠ absent evidence.** Compact summaries filter active candidates
   but must carry `history` / `history_count` (Recommendation Engine, Decision
   Queue, and Decision Engine) so consumers cannot infer “no evidence” from an
   empty actionable list.
2. **`history_count` is authoritative.** `history` is a truncated projection
   window (`summary(limit)`). Full generate may return the complete history
   (`history_count == history.length`). `history.length` must never be treated
   as the full evidence count when a summary window is in use.
3. **Decision Queue orphans retain overlays.** Missing live sources expire open
   overlays (`Expired`) and keep already-terminal overlays. Overlays are **not
   deleted** — there is no Decision Queue overlay deletion API.
4. **Decision Engine terminal artifacts project into history.** Outcomes
   `selected` / `dismissed` / `expired` appear only in `history` (via
   `DecisionArtifactHistoryEntry`). Actionable `top_candidates` remain
   `open` / `postponed` only. Orphan terminal overlays without a live candidate
   still project into history.
5. **Actionable / historical separation.**
   - Actionable channel — open/interactive states only
   - History channel — terminal evidence only
   - The same identity must not appear in both channels.
6. **History is non-actionable and non-executable.** Entries always project
   `actionable: false` and `authority_effect: none`. They are not executable
   items/candidates, cannot be converted into command inputs, and must not
   drive lifecycle mutations or execution.
7. **Frontend responsibilities.** React consumes projected fields only.
   Recommendation active filtering mirrors `RecommendationItem::is_active_lifecycle`
   (unknown states inactive). DQ/DE history helpers only verify projected
   non-actionability — they do not own lifecycle or invent state.
8. **Missing evidence is not inferred truth.** RE terminal history requires
   projected outcome identity. DQ/DE terminal history requires projected
   overlay/artifact identity. UI helpers must not synthesize resolutions.

## Related

- [Persistence Boundary Governance](./PERSISTENCE-BOUNDARY-GOVERNANCE.md)
- [Lifecycle Governance](./LIFECYCLE-GOVERNANCE.md)
- [Workspace Experience Contract](../05-AI/WORKSPACE-EXPERIENCE-CONTRACT.md)
