# Experience Component Library Research

Status: Research complete; decision pending
Research ID: EXP-001
Version: 1.0
Date opened: 2026-08-02
Last reviewed: 2026-08-02
External evidence accessed: 2026-08-02

This record is the Experience analogue of capability technology research, but
it covers **presentation infrastructure and interaction patterns**, not a
domain capability. It does not select a technology for production, approve
npm installs, or change architecture/contracts.

Detailed artefacts live in `architecture/research/experience/`.

---

## A. Identity and scope

- Surface: Experience (companion presentation)
- Question: What reusable components and OSS building blocks should Experience
  Phase 2–4 draw from?
- Catalogue entry: `EXP-001`
- Related authorities: Blueprint companion principles; `21_Experience_Roadmap.md`;
  LEDGER-0034/0035; Product Proof Save/Continue honesty; CSP packaging
- Out of scope: Runtime implementation; capability expansion; ambient sensing;
  AI-generated handoffs; contract changes

---

## B. Objective

Accelerate Experience convergence by replacing ad-hoc screen design with a
curated component catalogue, licence-cleared dependency candidates, interaction
principles, tokens, and a Phase 2 plan that can start immediately when
authorised.

---

## C. Findings (executive)

1. **Decomposition:** 70+ presentation components identified; seven concept-board
   temptations explicitly marked Not Needed for Product Proof phases.
2. **Gap:** Phase 2 hinges on dashboard canvas, hero centrepiece, varied moment
   cards, and honest empty structure — not new backend power.
3. **Stack recommendation (pending adoption):** Tailwind CSS + shadcn/ui (Radix
   or Base UI primitives) + Lucide (ISC) + date-fns; Motion deferred mainly to
   Phase 4. Reject GPL kits and Ant/Material as product shell.
4. **Patterns:** Dashboard-first hubs (VS Code recents × Notion empty structure ×
   concept masonry), intention-before-mechanism, progressive disclosure.
5. **Licensing:** Prefer MIT/Apache/BSD/ISC; record attribution; no approval
   until exact-version legal review.
6. **Phase 2 plan:** Ready in `experience/08_Phase_2_Implementation_Plan.md`.

---

## D. Decision

**No technology selected or approved.** Retain recommendations for a bounded
adoption decision at the start of an authorised Experience Phase 2
implementation session.

---

## E. Package index

See `architecture/research/experience/00_INDEX.md`.

| Artefact | Path |
|---|---|
| Component Catalogue | `experience/01_Component_Catalogue.md` |
| Licensing Matrix | `experience/02_Licensing_Matrix.md` |
| Inventory & Gap Analysis | `experience/03_Component_Inventory_and_Gap_Analysis.md` |
| Interaction Patterns | `experience/04_Interaction_Pattern_Research.md` |
| Design Token Proposal | `experience/05_Design_Token_Proposal.md` |
| Recommended OSS | `experience/06_Recommended_OSS_Dependencies.md` |
| Build Plan | `experience/07_Experience_Build_Plan.md` |
| Phase 2 Implementation Plan | `experience/08_Phase_2_Implementation_Plan.md` |

---

## F. Review date

2027-02-01, or earlier on Experience Roadmap change, material licence/
maintenance change in a recommended dependency, CSP policy change, or new
Participant evidence.
