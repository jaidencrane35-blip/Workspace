# Execution Program: IPC Tiering

| Field | Value |
| --- | --- |
| **Program** | Tier and simplify the IPC surface (labels + verification) |
| **Backlog** | Phase A #2 |
| **Gap** | G2, G10 |
| **Date** | 2026-08-07 |
| **Behaviour change** | None |

---

## Phase 5 — Implementation Planning

### Why selected

1. Constitution §4.8 requires Product / Developer / Diagnostic / Experimental separation; tiers exist only as narrative in knowledge maps.
2. Completable in one focused pass with a verifier (same pattern as explanation-catalog).
3. Rust→TS contract generation (A#1) is higher absolute value but needs multi-sprint attribute coverage; a half-pipeline would add debt. A#2 unblocks surface governance immediately with low arch risk.
4. Prefer simplification and evidence over speculative systems.

### Expected benefits

| Kind | Benefit |
| --- | --- |
| Architectural | Enforces Law VIII / §4.8; clarifies which commands are Product Proof vs quarantine |
| Engineering | CI fails if Tier A drifts from Tauri handlers or experience catalog |
| Maintenance | New commands must be classified; accidental Product surface growth is visible |

### Risks

| Risk | Mitigation |
| --- | --- |
| Mis-tiering a command | Conservative: Experience catalog = Product; known Operator/diagnostic = Diagnostic/Developer; remainder Experimental until reviewed |
| False CI failures | Verify only structural rules (registration, catalog ⊆ Product, disjoint tiers) |
| Scope creep into deleting commands | Out of scope — labels only |

### Constitutional risks

| Risk | Assessment |
| --- | --- |
| Changing invoke behaviour | None — no handler changes |
| Expanding ambient/AI | None |
| Vendor binding | None |

### Rollback

Delete generated `app/src/generated/ipcTiers.ts`, remove verify script from `package.json` / test suite, keep markdown inventory if useful. No DB or IPC behaviour to roll back.

### Validation strategy

1. `node scripts/verify-ipc-tiers.mjs`
2. `pnpm typecheck`
3. `pnpm test` (includes verifier)
4. `pnpm build`
5. Confirm no Tauri command registration changes in diff

---

## Tier model (Workspace names)

Constitution names: Product · Developer · Diagnostic · Experimental.

Mapped from prior A/B/C narrative:

| Constitution tier | Prior label | Meaning |
| --- | --- | --- |
| **Product** | Tier A | Frozen experience / Product Proof path |
| **Developer** | Tier B (subset) | Operator / engineering console surfaces |
| **Diagnostic** | Tier B (subset) | Health, status, debug inspection |
| **Experimental** | Tier C | Mounted-or-not AI/canvas/intelligence; not Product Proof |

Quarantine: Experimental commands that must not be treated as Product without ADR.

---

## Implementation report (Phase 8)

### Summary

Implemented constitutional IPC tiering with zero runtime behaviour change. Product Proof commands remain the Experience catalog (20). All 197 `generate_handler!` commands are partitioned into Product / Developer / Diagnostic / Experimental; quarantine (16) is verified as a non-Product unused-by-React flag. CI enforces sync via `pnpm verify:ipc-tiers`.

### Files changed

| Path | Role |
| --- | --- |
| `docs/engineering-capability-ownership.md` | Phase 1 ownership matrix |
| `docs/constitutional-gap-analysis.md` | Phase 2 gaps |
| `docs/constitutional-engineering-backlog.md` | Phase 3 backlog |
| `docs/execution-program-ipc-tiering.md` | Planning + this report |
| `docs/03-Engineering/ipc-tiers.json` | Diagnostic + quarantine authority |
| `docs/03-Engineering/IPC-SURFACE.md` | Tier pointer + quarantine note |
| `docs/ipc-surface-map.md` | Enforced tier counts |
| `scripts/ipc-tiers-lib.mjs` | Parse/classify/render |
| `scripts/sync-ipc-tiers.mjs` | Generator |
| `scripts/verify-ipc-tiers.mjs` | Verifier |
| `app/src/generated/ipcTiers.ts` | Generated registry |
| `app/src/demo/experienceIpcCatalog.ts` | Product authority comment |
| `app/src-tauri/src/lib.rs` | Quarantine comment clarification only |
| `package.json` | `sync:ipc-tiers` / `verify:ipc-tiers` in `test` |

### Architectural impact

- Constitution §4.8 is now machine-enforced.
- Clarifies that historical IPC-SURFACE “Product” language ≠ constitutional Product tier.
- Corrects `get_action_catalog` quarantine mislabel (Developer).

### Technical debt reduced

- G2 / G10: tiers were narrative-only; now verified in CI.
- Stale parity comment on `get_action_catalog` documented.

### Remaining backlog (do not start until reviewed)

1. Rust → TypeScript contract generation (G1)  
3. Documentation authority convergence (G3)  
4. WorkspaceState naming (G4)  
5–10. AgentToolGate, AuditIntegrity, AI boundaries, ModelProvider, BackgroundWorkerSupervisor, RFC workflow  

### Lessons learned

- Experience catalog length is **20**, not 21 (prior narrative error).
- React `invokeIpc<T>("cmd")` requires generic-aware parsing for evidence-backed Developer tier.
- Prefer completable governance enforcement over incomplete codegen for the first execution program.

### Constitutional compliance assessment

| Law / principle | Status |
| --- | --- |
| §4.8 IPC tiers | Satisfied (labels + CI) |
| No behaviour change | Satisfied (no handler/logic changes) |
| Law VIII presentation boundary | Strengthened (Product surface explicit) |
| Law XII ambient | Untouched (still off) |
| Prefer one excellent program | Satisfied |

### Validation run

- `pnpm typecheck` — pass  
- `pnpm test` (342 + explanation + **ipc-tiers** + UI boundary + CSP) — pass  
- `pnpm build` — pass  
- Counts: registered=197 product=20 developer=112 diagnostic=37 experimental=28 quarantine=16
