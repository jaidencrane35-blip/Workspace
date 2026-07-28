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

## Rules

1. **Actionable ≠ absent evidence.** Compact summaries filter active candidates
   but must carry `history` / `history_count` (Recommendation Engine) so
   consumers cannot infer “no evidence” from an empty actionable list.
2. **Decision Queue orphans retain overlays.** Missing live sources expire open
   overlays (`Expired`) and keep already-terminal overlays; overlays are not
   deleted.
3. **Decision Queue summaries** expose only actionable overlay states
   (`pending` / `viewed` / `deferred`). Full queue remains available for
   Operator surfaces.
4. **Frontend active filtering** mirrors `RecommendationItem::is_active_lifecycle`
   exactly (unknown states are inactive). Prefer projected `top_candidates`
   when a summary is already available.
5. **Missing evidence is not inferred truth.** Terminal history entries must
   include projected outcome identity; UI helpers must not synthesize
   resolutions.

## Related

- [Persistence Boundary Governance](./PERSISTENCE-BOUNDARY-GOVERNANCE.md)
- [Lifecycle Governance](./LIFECYCLE-GOVERNANCE.md)
- [Workspace Experience Contract](../05-AI/WORKSPACE-EXPERIENCE-CONTRACT.md)
