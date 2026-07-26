# Workspace Readiness Model

Phase 5 Batch 13 / Sprint 92.

## Principle

Readiness describes whether the Workspace is prepared for the user's current work. It does not prepare, fix, launch, move windows, or execute.

## Domain

- `WorkspaceReadinessState` — full snapshot
- `ReadinessAssessment` — typed assessment (Environment / Context / Task / Decision / Purpose)
- `ReadinessSignal` — evidence from existing systems
- `ReadinessGap` — what prevents full readiness
- `ReadinessSummary` — human-facing status lines
- `ReadinessStatus` — `ready` | `partially_ready` | `blocked`

## Sources (consumed, not owned)

Operating State · Environment · Composition · Task Graph · Purpose · Continuity · Evolution · Patterns · Decision Queue

## Downstream consumers

- Recommendation Engine may cite readiness gaps as evidence
- Adaptation Proposals may cite readiness gaps as evidence
- Workspace Intelligence embeds `readiness` summary

## Governance

- Zero authority (`authority_effect: none`)
- Distinct from runtime `WorkspaceHealth` / `workspace_health` label
- Audits: `workspace.readiness.generated`, `.assessed`, `.updated`
- Assistant explains only — never prepares or executes

## Position

```
Operating State / Environment / Composition / Task Graph / Purpose /
Continuity / Evolution / Patterns / Decision Queue
        ↓
Workspace Readiness
        ↓
Recommendation Engine · Adaptation Proposal · Workspace Intelligence
        ↓
Human Decision → Intent → Gateway → Execution
```
