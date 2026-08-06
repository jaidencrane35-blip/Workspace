# Repository Commit Review
## Milestone: Conversational Operator Foundation

| Field | Value |
| --- | --- |
| **Date** | 2026-08-07 |
| **Branch** | `v2-dev` |
| **Milestone** | Conversational Operator Foundation |
| **Product authority** | `docs/00-Constitution/PRODUCT_CONSTITUTION.md` |
| **Architecture authority** | `architecture/ARCHITECTURAL_CONSTITUTION_V2.md` |

---

## Repository health (pre-commit)

| Check | Expectation |
| --- | --- |
| Overall | Healthy after Phase 2 validation |
| Machine state | `docs/project-health.json` (+ `app/public/project-health.json`) |
| Protocol | Constitutional execution v1.1 |
| Risk | Medium surface size (197 IPC); docs authority still fragmented (G3) |

---

## Classification legend

Each path is exactly one of: **Production** · **Documentation** · **Verification** · **Generated** · **Temporary** · **Experimental** · **Debug** · **Evidence**

---

## Files included

### Production

| Path | Notes |
| --- | --- |
| `app/src/App.tsx` | Operator wraps Product Proof |
| `app/src/App.css` | Operator presence + Mode 3 nesting |
| `app/src/components/operator/**` | Modes 1–3, health, proposals UI |
| `app/src/lib/intentBridge.ts` | Deterministic intent routing |
| `app/src/lib/capabilityEvolution.ts` | Proposal lifecycle (no self-rewrite) |
| `app/src/types/domain.ts` | Re-exports generated Product Proof contracts |
| `app/src/demo/**` | Demo IPC alignment with PP surfaces |
| `app/src-tauri/src/lib.rs` | App wiring as required by contracts |
| `packages/domain/**` | Product Proof contracts + domain types |
| `packages/kernel/src/commands/resume.rs` | ResumePlanPreview domain move |
| `Cargo.toml`, `Cargo.lock`, `package.json` | Deps / scripts for sync+verify |

### Documentation

| Path | Notes |
| --- | --- |
| `architecture/ARCHITECTURAL_CONSTITUTION_V2.md` | Binding architecture constitution |
| `docs/00-Constitution/PRODUCT_CONSTITUTION.md` | Binding product constitution |
| `docs/conversational-desktop-operator.md` | Operator definition |
| `docs/engineering-milestone-report.md` | Milestone report |
| `docs/constitutional-engineering-backlog.md` | Backlog |
| `docs/execution-program-*.md` | Completed program records |
| `docs/product-proof-*.md` | Review / refoundation |
| `docs/*-map.md`, strategic/kiro/gap docs | Architecture discovery corpus |
| `docs/repository-commit-review.md` | This review |
| `docs/capability-evolution/**` | Capability evolution registry + protocol |
| `architecture/14_Architecture_Guardian.md` | Guardian notes |
| `architecture/22_Experience_Implementation_Snapshot.md` | Snapshot |
| `architecture/0*.md`, `13_*.md`, `20_*.md` (modified) | Capability / fidelity docs |
| `architecture/research/**/*.md` | Research (text) |
| `.cursor/rules/constitutional-execution-protocol.mdc` | Always-apply protocol |

### Verification

| Path | Notes |
| --- | --- |
| `scripts/sync-ipc-tiers.mjs`, `verify-ipc-tiers.mjs`, `ipc-tiers-lib.mjs` | IPC tiering |
| `scripts/sync-product-contracts.mjs`, `verify-product-contracts.mjs` | PP contracts |
| `scripts/sync-project-health.mjs`, `verify-project-health.mjs` | Health registry |
| `scripts/verify-capability-evolution.mjs` | Capability evolution schema |
| `tests/intent-bridge.test.ts` | Intent bridge tests |
| `tests/capability-evolution.test.ts` | Proposal classifier tests |
| `docs/03-Engineering/IPC-SURFACE.md`, `ipc-tiers.json` | IPC authority |

### Generated (intentionally versioned)

| Path | Notes |
| --- | --- |
| `app/src/generated/ipcTiers.ts` | From IPC surface |
| `app/src/generated/productContracts.ts` | From Rust ts-rs |
| `docs/project-health.json` | Machine engineering state |
| `app/public/project-health.json` | In-app health consumption |

### Evidence (selective)

| Path | Notes |
| --- | --- |
| `architecture/evidence/*.json` | Updated product/runtime evidence |

---

## Files excluded

| Path | Class | Reason |
| --- | --- | --- |
| `architecture/research/experience/screenshots/snapshot-2026-08-02/` | Evidence / Temporary | ~23 MB PNG capture batch; not required for milestone runtime; prior screenshot sets already versioned |
| Local `target/`, `dist/`, `node_modules/` | Temporary | Build artifacts (gitignored) |
| Browser/Vite demo-only process state | Temporary | Not repository content |

---

## Reasoning

1. **Include** constitutional authorities, operator production code, verifiers/generators, and intentionally versioned generated artifacts — these are the milestone.
2. **Include** architecture discovery docs produced under the constitutional program (knowledge maps, Kiro study, gap analysis) so the branch is reviewable as one foundation.
3. **Exclude** bulk screenshot dumps that do not gate build, typecheck, or operator behaviour.
4. **Do not** commit secrets, credentials, or local IDE noise beyond the intentional `.cursor/rules` protocol file.

---

## Risk assessment

| Risk | Level | Mitigation |
| --- | --- | --- |
| Large docs + research corpus in one commit | Medium | Single milestone narrative; no product rewrite |
| Generated TS drift | Low | `pnpm verify:contracts` / `verify:ipc-tiers` |
| Operator UX regressions | Medium | Launch + owner cadence; Product Proof wrapped not rewritten |
| Capability evolution misused as auto-code | Low | Proposals only; execution remains constitutional engineering |
| Screenshot exclusion loses visual history | Low | Earlier screenshot trees remain; new dump re-capturable |

---

## Milestone summary

**Conversational Operator Foundation** establishes conversation as the product front door (Modes 1–3), a deterministic intent bridge into existing Save/Continue surfaces, developer-only repository health, and a non-rewriting capability evolution proposal pipeline — then records a clean repository milestone for the permanent commit → push → next-program cadence.
