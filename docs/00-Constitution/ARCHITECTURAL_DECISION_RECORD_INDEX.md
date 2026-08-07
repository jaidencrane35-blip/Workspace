# Architectural Decision Record Index

| Field | Value |
| --- | --- |
| **Kind** | Engineering Governance — institutional memory (not constitutional law) |
| **Purpose** | Answer “Why does Workspace work this way?” without reading sprint history |
| **Related** | `docs/09-Decisions/DECISION-LOG.md` (historical ADR stream); Spec; EES |
| **Rule** | New settled architectural/governance decisions MUST be indexed here |

This is an **index of settled decisions**, not a redesign surface. Individual rationale lives in the cited evidence documents.

---

## Active decisions

| Decision | Status | Authority | Evidence | Supersedes |
| --- | --- | --- | --- | --- |
| Workspace Constitutional Specification v2 | Active | Constitution | Spec v2.1; A1–A3 formalization/codification; tag `workspace-constitution-v2.1` | Prior peer “architecture” docs; Blueprint-as-highest-authority claims |
| Engineering Execution Standard v1 | Active | Governance | `WORKSPACE_ENGINEERING_EXECUTION_STANDARD_V1.md` | Ad-hoc engineering prompt reinventing lifecycle |
| Repository Confidence Model | Active | Governance | `REPOSITORY_CONFIDENCE_MODEL.md`; Operations Audit | Treating scoped audits as universal certainty |
| Constitutional Operations posture | Superseded | Governance | Operations Audit | Architecture Discovery |
| Sustainable Engineering Operations | Active | Governance | `P16_O3_GOVERNANCE_RESILIENCE_REPORT.md`; handoff | Continuous meta-governance / constitutional redesign programs |
| PR1 Production Readiness Program | Active | Production | `PR1_PRODUCTION_READINESS_PROGRAM.md`; `production-readiness.json` | Treating eng-complete as public-release-ready; mixing P17 with Phase 2 Critical |
| P16.PI1 Slice 1 Installer Foundation | Active (complete) | Production | `P16_PI1_SLICE1_INSTALLER_FOUNDATION.md`; NSIS hooks | Starting Slice 2 before Owner accepts S1; skipping signing forever |
| Product Gravity | Active (principle) | Constitution | Spec §9; `docs/ui/PRODUCT_GRAVITY_RULE.md` | Catalogue / launcher-first product identity |
| Capability Integration Standard | Active | Constitution | Spec §15 | Provider-specific bespoke architectures |
| Kernel Operator sole Effect composition | Active | Architecture Standard under Spec | `KERNEL_AUTHORITY_RULE.md`; Operations Audit | Conversation→provider IPC; provider→provider |
| Voice as Input (not Execution) | Active | Constitution / realization | Spec Information Owners; Voice WRAP frozen | Voice-as-orchestrator designs |
| WinRT ContinuousRecognitionSession WRAP | Active (realization) | Architecture Standard / Voice | Voice WRAP audits; frozen unless Voice-owned defect | Speculative STT migrations without measured evidence |
| Moments approve-before-restore | Active | Product / Architecture Standard | Product Proof shell; Operations Audit | Ambient capture; silent restore |
| P10–P15 providers closed | Active | Capability Standards | Handoff inventory; Provider Acceptance | Reopening closed providers except bugfixes |
| P16 Product Proof pending Owner | Active (gate) | Governance | `project-health.json`; handoff; `P16_O1_PRODUCT_PROOF_READINESS_REPORT.md` | Treating engineering green as Owner acceptance |
| P16.O1 Product Proof Readiness | Active (assessment) | Governance | `P16_O1_PRODUCT_PROOF_READINESS_REPORT.md` | Meta-architecture as substitute for live Owner feel |
| P16.O2 Owner PP Execution Authority | Active (gate) | Governance | `P16_O2_OWNER_PRODUCT_PROOF_EXECUTION_AUTHORITY.md`; session workbook | Treating eng green as Owner acceptance; inventing Owner observations |
| P17 File Provider deferred | Active (sequencing) | Governance | Handoff; roadmap | Starting P17 before P16 Owner close |

---

## How to use

1. **Before proposing architecture** — Check whether an Active decision already settles the question.  
2. **After settling a durable decision** — Add one row (Status, Authority, Evidence, Supersedes).  
3. **If a decision is replaced** — Set prior row to Superseded; point Supersedes column on the new row.  
4. **Do not** use this index to invent new constitutional owners or laws.

---

## Standing posture (Sustainable Engineering Operations)

- Architecture and governance are presumed stable by default.  
- Every engineering task begins by identifying the lowest Information Owner responsible.  
- Engineering proceeds within the Constitution unless objective evidence satisfies a Review Trigger.  
- Improvements target product quality, production readiness, capability growth, or UX — not architectural or governance novelty.  
- Do not open further meta-governance programs absent a demonstrated process failure with evidence.

---

## Historical ADR stream

Richer chronological ADRs and decision log entries remain in `docs/09-Decisions/DECISION-LOG.md` and accepted ADRs. This index is the **short map** of what still governs.
