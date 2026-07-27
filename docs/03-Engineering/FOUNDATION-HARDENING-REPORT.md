# Foundation Hardening Report

> Historical snapshot (2026-07-24). Metrics and status statements in this
> report are point-in-time and may differ from current repository state.

| Field | Value |
|-------|-------|
| **Date** | 2026-07-24 |
| **HEAD baseline** | `28d89a4` (Sprint 37–38) + this hardening pass |
| **Scope** | Audit + safe consolidation only — no new end-user features |
| **Owner** | Lead Software Engineer |

---

## Executive Summary

Workspace Phase 1 is a **real, layered foundation**: domain / database / kernel / windows-integration / Tauri IPC / React shell with canvas + diagnostic operator. Architecture crate hygiene is strong and the dependency graph is acyclic.

The main risks are **intentional Phase 1 stubs** (AllowAll gate/policy, DEC-011 in-process host) and **surface sprawl** (many IPC commands registered ahead of UI). This hardening pass sealed internal service mutators, renamed a colliding intent validator, removed an unused app→database dependency, and documented the IPC quarantine — without redesigning working systems.

**Foundation readiness: Foundation Complete with Minor Risks.**

---

## Architecture Score (out of 10)

| Dimension | Score | Notes |
|-----------|------:|-------|
| Architecture | 8 | Clear crate boundaries; Win32 isolated; UI→IPC→kernel→DB |
| Maintainability | 7 | CRUD/audit-scan clones remain; kernel is large |
| Consistency | 7 | Improved by rename + IPC inventory; TS mirrors still hand-synced |
| Modularity | 8 | Packages clean; future `domain-*` extraction still open |
| Scalability | 6 | DEC-011 multi-process deferred; kernel packed in Tauri host |
| Technical Debt | 6 | AllowAll + unused IPC + CRUD duplication tracked |
| Documentation | 8 | ROADMAP/DEC-011 honesty + IPC surface + this report |
| Testing | 8 | `cargo test --workspace` in CI; 380 Rust tests; FE placeholder only |
| Security | 6 | Least-privilege path structure OK; enforcement still AllowAll |
| Performance | 7 | Bounded audit scans; no measured hotspots; no micro-opts |

---

## Issues Found

### Critical (deferred — would be Phase 2 / redesign)

| Issue | Impact | Resolution |
|-------|--------|------------|
| `AllowAllPermissionGate` + `AlwaysAllowPolicy` defaults | Mutate path never denies | **Deferred** — Permission Gateway is Phase 2; do not fake-enforce |
| Public service mutators (pre-harden) | External crates could skip pipeline | **Fixed** — sealed to `pub(crate)` |

### High

| Issue | Impact | Resolution |
|-------|--------|------------|
| `IntentExecutionService` vs `GovernedIntentExecutionService` name collision | Mis-wiring risk | **Fixed** — renamed Sprint 15 service to `ActionIntentValidationService` |
| `app` depended on `workspace-database` unused | Invites UI→DB bypass | **Fixed** — dependency removed |
| 17 IPC commands unused by React | Maintenance / attack surface | **Documented** — kept for CommandHandler parity; see IPC-SURFACE.md |
| `get_settings` / status / health outside full pipeline | Weaker audit on reads | **Deferred** — documented Phase 1 pattern (Sprint 04) |

### Medium

| Issue | Impact | Resolution |
|-------|--------|------------|
| Zone/Application/Widget near-clone CRUD | Copy-paste drift | Deferred — extraction would be churn without product gain |
| Audit-scan projection twins | Duplicated scan/filter | Deferred — shared helper later |
| ~28 `ready_ctx` test helpers | Drift | Deferred — extract when next suite touch |
| Hand-synced `app/src/types/domain.ts` | Serde drift | Deferred — codegen when IPC surface stabilizes |
| DEC-011 multi-process not implemented | Plugins/AI still in-host | Intentional Phase 1 (Sprint 37 note) |

### Low

| Issue | Impact | Resolution |
|-------|--------|------------|
| `tests/placeholder.test.ts` | No real FE E2E | Deferred |
| Future package folders in REPOSITORY-STRUCTURE | Doc ahead of tree | Expected |
| Noisy `COMMAND:` / `EVENT:` logs | Ops noise | Deferred — tune log levels later |

---

## Refactors Performed

1. **Sealed resource/graph/config mutators** to `pub(crate)` so external crates cannot skip `CommandPipeline` (`workspace`, `zone`, `application`, `widget`, `layout`, `graph`, `configuration`, plus suggestion/execution prepare helpers).
2. **Renamed** `IntentExecutionService` → `ActionIntentValidationService` (Sprint 15 validator); left `GovernedIntentExecutionService` for Sprint 24.
3. **Removed** unused `workspace-database` dependency from `app/src-tauri`.
4. **Documented** used vs quarantined IPC in `docs/03-Engineering/IPC-SURFACE.md`; grouped registration in `lib.rs` with comments.
5. **Indexed** IPC surface + this report from `docs/README.md`.

---

## Files Modified

| Path | Change |
|------|--------|
| `packages/kernel/src/services/{workspace,zone,application,widget,layout,graph,configuration}.rs` | Mutators `pub(crate)` |
| `packages/kernel/src/services/{intent_execution,suggestion_intent,execution_cancellation}.rs` | Prepare helpers `pub(crate)` |
| `packages/kernel/src/intent/validation.rs` | Rename to `ActionIntentValidationService` |
| `packages/kernel/src/intent/mod.rs` | Re-export rename |
| `packages/kernel/src/lib.rs` | Public export rename |
| `packages/kernel/src/commands/{pipeline,intent_tests}.rs` | Import rename |
| `packages/kernel/src/services/suggestion_intent.rs` | Import rename |
| `app/src-tauri/Cargo.toml` | Drop `workspace-database` |
| `app/src-tauri/src/lib.rs` | IPC grouping comments |
| `docs/03-Engineering/IPC-SURFACE.md` | New |
| `docs/03-Engineering/FOUNDATION-HARDENING-REPORT.md` | This report |
| `docs/README.md` | Index links |
| `docs/10-Sprints/sprints/2026-07-24-sprint-39.md` | Sprint record |
| `docs/08-Roadmap/ROADMAP.md` | Harden checkbox |

---

## Remaining Risks (intentionally deferred)

1. **Permission Gateway** — still AllowAll until Phase 2.
2. **DEC-011 true multi-process** — crate+IPC only in Tauri host.
3. **Unused IPC quarantine** — registered but not React-wired; remove only when a surface pass deletes them deliberately.
4. **CRUD / audit-scan duplication** — consolidate only with a dedicated refactor sprint.
5. **TS/Rust dual types** — no codegen yet.
6. **Encryption** — NoOp provider (DEC-015 Tier 0).

---

## Validation

```text
cargo test --workspace   → 380 passed
pnpm typecheck           → clean
```

---

## Foundation Readiness

**Foundation Complete with Minor Risks**

Reasoning: Phase 1 product shell (canvas + layout persist + diagnostics + Windows enum) works behind clean package boundaries. Hardening removed the worst “skip the pipeline” footgun and clarified intent naming. Remaining risks are **known Phase 2 work** (gateway, multi-process, AI) or **optional maintainability refactors**, not blockers for building the next capability carefully on this base.
