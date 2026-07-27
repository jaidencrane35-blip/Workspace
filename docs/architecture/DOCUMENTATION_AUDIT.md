# Documentation Audit — Workspace

Date: 2026-07-27
Author: Documentation consistency review

## Executive summary

The `docs/**` collection is generally coherent around the key architecture boundaries: Recommendation Engine vs Decision Engine, governed permission boundaries, and lifecycle projections. Core AI terminology is well-defined in the AI documentation and the roadmap aligns with the implemented sprint phases.

However, several documentation artifacts are stale or ambiguous relative to the current repository state and the documented terminology. The highest-impact issues are outdated engineering policy language, missing cross-links from IPC inventory to AI boundary docs, and vocabulary inconsistency between UI copy guidance and product UX wording.

## Documents reviewed

- `docs/02-Architecture/ARCHITECTURE-PRINCIPLES.md`
- `docs/02-Architecture/SYSTEM-OVERVIEW.md`
- `docs/03-Engineering/IPC-SURFACE.md`
- `docs/03-Engineering/DEPENDENCY-POLICY.md`
- `docs/03-Engineering/CODING-STANDARDS.md`
- `docs/05-AI/AI-MODEL-PROVIDER-FOUNDATION.md`
- `docs/05-AI/WORKSPACE-AUTOMATION-READINESS.md`
- `docs/05-AI/WORKSPACE-ATTENTION-ENGINE.md`
- `docs/05-AI/WORKSPACE-DECISION-ENGINE.md`
- `docs/05-AI/WORKSPACE-RECOMMENDATION-ENGINE.md`
- `docs/05-AI/WORKSPACE-VOCABULARY.md`
- `docs/05-AI/WORKSPACE-COGNITION-INTEGRITY-AUDIT.md`
- `docs/05-AI/WORKSPACE-INTELLIGENCE-FOUNDATION.md`
- `docs/08-Roadmap/ROADMAP.md`
- `docs/04-UX/UX-PRINCIPLES.md`
- `docs/03-Engineering/CI-CD-PLAN.md`

Additional documents were scanned for terminology and status metadata across `docs/**`.

## Issues found

### P0 critical

- `docs/03-Engineering/DEPENDENCY-POLICY.md` still describes lock file handling as pending stack selection and comments on `.gitignore` entries, while the repository already commits `pnpm-lock.yaml` and uses pnpm. This is stale policy guidance that can mislead contributors about current package management decisions.

- `docs/02-Architecture/ARCHITECTURE-PRINCIPLES.md` retains the line `Desktop-native — Not a web app pretending to be desktop — technology choice TBD` despite the workspace being implemented as a Tauri desktop shell. This mismatch is stale and can create confusion about the selected architecture.

### P1 important

- `docs/03-Engineering/IPC-SURFACE.md` enumerates many product and diagnostic IPC commands but omits direct links to authoritative AI docs for the most important commands, including `generate_workspace_recommendation_engine`, `generate_decision_engine`, `get_suggestion_lifecycle`, and recommendation/decision candidate actions. This weakens discoverability and makes cross-document traceability harder.

- `docs/05-AI/WORKSPACE-VOCABULARY.md` explicitly recommends avoiding bare `Recommendation` in new UI, yet `docs/05-AI/WORKSPACE-DECISION-ENGINE.md` still uses the product UX label `Work tab Recommended Actions`. The gap between vocabulary guidance and product wording is a naming consistency risk.

- The vocabulary around `Attention priority`, `Recommendation Engine candidate`, `Decision candidate`, and `Suggestion` is generally strong, but several docs still use overlapping terms such as `recommended_actions`, `Recommendation Engine`, and `Recommended Actions` without a single canonical cross-reference. This invites future drift.

### P2 cleanup

- `docs/03-Engineering/CODING-STANDARDS.md` retains placeholder formatting guidance: `Automated formatter enforced via CI (specific tool TBD with stack selection)`. If a formatter is already in use or a toolchain chosen, this should be updated.

- `docs/04-UX/UX-PRINCIPLES.md` and several other documents still show owner metadata as `TBD`, which is a lower-severity but useful cleanup for documentation hygiene.

- `docs/05-AI/WORKSPACE-COGNITION-INTEGRITY-AUDIT.md` and `docs/05-AI/WORKSPACE-INTELLIGENCE-FOUNDATION.md` warn against treating `recommended_actions` as Recommendation Engine output, but those distinctions are spread across multiple docs. A stronger single glossary or reference section would reduce the chance of inconsistent UI copy.

## Severity summary

- P0 critical: 2
- P1 important: 3
- P2 cleanup: 3

## Recommended documentation fixes

1. Update `docs/03-Engineering/DEPENDENCY-POLICY.md` to reflect the chosen package manager and lock file policy in the current repo (`pnpm-lock.yaml` committed, package manager selected).
2. Update `docs/02-Architecture/ARCHITECTURE-PRINCIPLES.md` to remove `TBD` technology-selection language or explicitly document the selected desktop shell approach.
3. Add authoritative cross-links in `docs/03-Engineering/IPC-SURFACE.md` from key IPC commands to `WORKSPACE-DECISION-ENGINE.md`, `WORKSPACE-RECOMMENDATION-ENGINE.md`, `WORKSPACE-VOCABULARY.md`, and relevant lifecycle docs.
4. Align UI terminology in `docs/05-AI/WORKSPACE-DECISION-ENGINE.md` with the vocabulary guidance in `docs/05-AI/WORKSPACE-VOCABULARY.md`, especially around `Recommended Actions` vs `Attention priority` vs `Recommendation Candidate`.
5. Review and refresh placeholder ownership and status metadata across docs that still show `TBD` or future-phase placeholders.
6. Consider adding a single concise glossary or cross-reference section for Recommendation/Decision/Candidate/Lifecycle/Boundary terminology to make terminology usage explicit and centralized.

## Notes on consistency

- Core Recommendation Engine vs Decision Engine ownership and lifecycle separation is consistent across the reviewed AI documents. No major contradiction was found in the engine/core boundary definitions.

- The roadmap, system overview, and AI docs are aligned in describing the current sprint progression and architecture phases.

- The most noticeable documentation drift is in engineering policy language and UI vocabulary guidance rather than in the core architecture definitions.
