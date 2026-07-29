# Workspace Evidence Observational Scaffold (Programme IV — Batch 10)

**Status:** Active  
**Audience:** Architecture, Kernel, Frontend, Governance  
**Depends on:** [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md), [Programme IV Maintainability Audit](../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md)

---

## Purpose

Batches 1–9 delivered nine observational evidence engines. Batch 10 does **not** answer a new evidence question.

It answers a maintainability question:

> **"How do we keep Programme IV extensible without another clone-and-rename wave?"**

Batch 10 extracts shared observational scaffolding so future conversational / assistant surfaces can build **on top of** the evidence family without duplicating envelope, guard, and projection boilerplate.

---

## Primary principle

**Share contracts. Do not collapse ownership.**

Scaffolding may factor repeated mechanics. Each evidence engine remains the authority for its DTOs, classification, persistence tables, and commands.

---

## Ownership

### Owns

| Artefact | Location |
|---|---|
| Shared digest / authority helpers | `packages/domain/src/workspace_evidence_contract/` |
| Data-driven evidence family guards | `scripts/architecture-governance-lib.mjs` (`EVIDENCE_ENGINE_GUARD_SPECS`) |
| Shared React projection contract | `app/src/components/evidenceProjectionContract.ts` |
| Engine thin projection wrappers | `app/src/components/evidence*Projection.ts` |

### Does not own

- Any evidence snapshot / observation / gap semantics
- Persistence tables or migrations
- CommandPipeline mutations
- Semantic retrieval, explanations, planning, reasoning, policy, execution, permissions, lifecycle
- A new source of truth

---

## Explicit non-deliverables

Batch 10 intentionally does **not** add:

- Migration `073_*`
- New `Generate*` mutation (baseline remains **82**)
- New history/projection DTO inventory entries (remains **33**)
- A tenth observational assessment engine
- Generic repository macros (deferred — see audit)
- Kernel service scaffold macros (deferred — see audit)

---

## Human navigation map

| Question | Answer |
|---|---|
| What does scaffolding own? | Shared helpers only — see Ownership |
| Where is the architecture contract? | This document |
| Where are domain helpers? | `packages/domain/src/workspace_evidence_contract/` |
| Where are engine DTOs still owned? | `packages/domain/src/workspace_evidence_*/` |
| Where are tests for helpers? | Domain `#[cfg(test)]` in contract module; Vitest projection-integrity |
| Where are projections? | `evidenceProjectionContract.ts` + thin engine wrappers |
| Where are persistence rules? | Still per-engine repositories / migrations 064–072 |

---

## Adoption

- **Pilot:** `workspace_evidence_reliability` uses contract `stable_digest` + forbidden-phrase baseline.
- **Frontend:** all eight evidence projection helpers delegate to the shared contract.
- **Governance:** eight guard bodies replaced by one spec table + runner.

Further engine adoption of domain helpers is incremental and optional.

---

## Explicit confirmation

> Programme IV Batch 10 observes maintainability needs only.
> It never becomes an evidence authority, never determines truth, never executes, and never replaces engine ownership with a mega-SoT.
