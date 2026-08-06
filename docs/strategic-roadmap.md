# Strategic Roadmap

| Field | Value |
| --- | --- |
| **Purpose** | Staged future direction for Workspace architecture |
| **Parent** | `docs/workspace-strategic-review.md` |
| **Date** | 2026-08-07 |
| **Constraint** | Roadmap only — no implementation in this task |

---

## North star

Ship and sustain **trusted interruption recovery on Windows**, on a **permissioned local kernel**, without letting experimental surfaces redefine the product.

---

## Stage 1 — Stabilise

### Purpose

Make the repository tell one architectural truth and protect Product Proof behaviour from accidental expansion.

### Expected outcome

- Contributors know PP chrome vs experimental platform vs legacy UI.
- Documentation phase language matches V1 baseline + V2 tip reality.
- IPC tiers A/B/C are governance, not folklore.
- No ambient capture enablement; no new architectural layers.

### Dependencies

- Knowledge maps already written (`docs/*-map.md`, strategic pack).
- Owner acceptance of S-001…S-003 in strategic review.

### Risks

| Risk | Mitigation |
| --- | --- |
| Doc updates reopen product debates | Limit to status/authority; no UX redesign |
| Tier labelling treated as delete mandate | Explicit: Stabilise does not delete IPC |
| Conflating V2 presentation with PP | Separate briefs; handoff constraints |

### Success criteria

- [ ] `docs/README.md` current phase aligned with `architecture/32_*` + V2 tip
- [ ] Strategic + knowledge docs linked from docs index
- [ ] IPC tier policy written (A/B/C) and referenced by CONTRIBUTING or eng standards
- [ ] PP evidence still regenerable; no intentional PP behaviour change
- [ ] Moratorium stated: no new `generate_*` / certification systems without brief

---

## Stage 2 — Consolidate

### Purpose

Reduce accidental complexity and dual ownership without reducing Save/Resume capability.

### Expected outcome

- Legacy UI gravity contained (DEV-only or documented non-product).
- Contract-generation approach chosen (ADR).
- Kernel/package READMEs match runtime-service-map.
- Experimental surface growth stopped.

### Dependencies

- Stage 1 complete.
- Consolidation plan workstreams C2–C5.

### Risks

| Risk | Mitigation |
| --- | --- |
| Engineers delete Tier C too early | Delete only in late Modernise/Expand with ADR |
| Contract ADR bikeshed | Bound options to assessment doc |
| Legacy panels still taught as “the app” | Update IPC-SURFACE consumer language |

### Success criteria

- [ ] ADR or Decision Log: domain contract pipeline approach
- [ ] IPC-SURFACE.md consumer section matches mounted reality
- [ ] Legacy components labelled in eng docs
- [ ] No net new experimental IPC families
- [ ] `pnpm test` + kernel lib tests remain green

---

## Stage 3 — Modernise

### Purpose

Improve maintainability and release safety of preserved platforms; close known gates that matter to the product claim.

### Expected outcome

- Generated (or CI-verified) TS domain contracts.
- Critical RE/DE invariants release-safe (not only `debug_assert!`).
- Multi-monitor topology decision: fix toward PASS or explicitly narrow product claim.
- Encryption tier decision recorded (stay Tier 0 or schedule Tier 1).
- Optional WRAP spikes (ModelProvider / codegen toolchain) behind interfaces.

### Dependencies

- Stage 2 ADR for contracts.
- Replaceability matrix Band 2 items.
- Windows hardware for monitor evidence if pursuing PASS.

### Risks

| Risk | Mitigation |
| --- | --- |
| Codegen breaks UI | Parallel generate + verify; gradual import cutover |
| Multi-monitor scope explosion | Bound to placement honesty, not full VD/tabs |
| Model WRAP becomes agent product | Pattern B only; gateway mandatory |

### Success criteria

- [ ] Domain contract verifier in CI
- [ ] Documented release-safe invariant strategy for RE/DE
- [ ] Multi-monitor: gate PASS **or** baseline claim updated
- [ ] Encryption Decision Log entry
- [ ] PP E2E + product-proof evidence still valid
- [ ] No ambient capture regression

---

## Stage 4 — Expand

### Purpose

Grow capability **only** through the permission platform, after the recovery wedge is stable and measurable.

### Expected outcome

- Selective promotion of experimental surfaces to mounted product **with briefs**.
- Automation/AI features that remain definition→approval→execute.
- Plugin host design (if any) with ApprovalRequired actors — not full-trust plugins.
- Optional Pattern A advisor integration study outcomes applied carefully.

### Dependencies

- Stages 1–3.
- Pilot/product evidence that recovery wedge works for users.
- Explicit Expand brief (V2 handoff: no Sprint 75 without brief).

### Risks

| Risk | Mitigation |
| --- | --- |
| Feature gravity returns (second product) | Mounted consumer requirement |
| External platform swallows identity | kiro-integration-strategy Never Leave list |
| Ambient “for AI quality” | Requires constitutional-level decision |

### Success criteria

- [ ] Each expansion has Decision Log / ADR + mounted UX
- [ ] Gateway + audit cover new mutations
- [ ] PP recovery path regressions = release blockers
- [ ] No bypass of Win32 crate boundary
- [ ] Plugin/AI process isolation decided before multi-process claims

---

## Cross-stage invariants

These hold in every stage:

1. PermissionGateway on mutations.  
2. UI never owns desktop truth.  
3. Ambient capture remains off unless Product Owner revises Product Proof.  
4. Experience catalog remains the mounted contract unless deliberately versioned.  
5. External maturity ⇒ WRAP evaluation, not silent REPLACE.  
6. No new architectural layers without authorised brief.

---

## Mapping to prior programmes

| Prior track | Relationship |
| --- | --- |
| V1 baseline (`architecture/23–32`) | Stage 1 protects; Stage 3 closes open gates |
| V2 confidence (`40–65`, handoff) | Stage 1–2 align; no architecture expansion until Expand brief |
| Product Proof mission | Stages 1–3 serve it; Stage 4 may extend it |
| Capability research catalogue | Research may continue; selection ≠ Expand mount |

---

## What this roadmap explicitly defers

- Adopting Kiro or any named external platform  
- Deleting large IPC sets in Stage 1  
- Multi-process plugin runtime delivery  
- Cloud sync  
- Virtual desktop / browser-tab fidelity  
- Autonomous automation
