# Experience Research Package — EXP-001

Status: Research complete; adoption decision pending
Research ID: EXP-001
Version: 1.0
Date: 2026-08-02
External evidence accessed: 2026-08-02

Authority: Subordinate to Blueprint, Experience Roadmap (`21`), Product Proof
behaviour, and Open Source Registry conventions. This package does **not**
select runtime dependencies, change contracts, or authorise capability growth.

Session mode: Experience research and planning only. No runtime mutation.

---

## Purpose

Decompose the intended Workspace Experience into reusable components, evaluate
existing implementations and licences, and produce a curated catalogue that
Experience Phase 2+ can draw from in a repository-driven way.

## Package contents

| # | Artefact | Path |
|---|---|---|
| 1 | Component Catalogue | `01_Component_Catalogue.md` |
| 2 | Licensing Matrix | `02_Licensing_Matrix.md` |
| 3 | Inventory & Gap Analysis | `03_Component_Inventory_and_Gap_Analysis.md` |
| 4 | Interaction Pattern Research | `04_Interaction_Pattern_Research.md` |
| 5 | Design Token Proposal | `05_Design_Token_Proposal.md` |
| 6 | Recommended OSS Dependencies | `06_Recommended_OSS_Dependencies.md` |
| 7 | Experience Build Plan | `07_Experience_Build_Plan.md` |
| 8 | Phase 2 Implementation Plan | `08_Phase_2_Implementation_Plan.md` |
| — | Master research record | `../EXPERIENCE_COMPONENT_RESEARCH.md` |
| — | Catalogue entry | `EXP-001` in `03_Research_Catalogue.md` |

## Binding constraints (research and later build)

- No ambient observation or invented activity.
- No AI-generated handoffs.
- Product Proof Save/Continue honesty unchanged.
- Prefer MIT / Apache-2.0 / BSD / ISC / MPL; reject GPL unless registry accepts.
- Exact dependency-level legal review required before adoption.
- CSP (`pnpm test` verifier) and Tauri WebView constraints apply to any CSS/JS.

## Traceability

- Concept boards (Aug 2026) — composition target
- Participant #1 screenshots + review after LEDGER-0034
- `21_Experience_Roadmap.md` Phases 1–4
- Current Experience surfaces under `app/src/` (React 18 + CSS; no component lib)

## Decision status

**No technology selected or approved.** Recommendations below are ranked
candidates for a later adoption decision when an Experience implementation
session is authorised. Phase 2 can begin from presentation structure alone even
before OSS adoption if needed; the recommended stack accelerates quality.
