# P16.PI2 — Production Gate Execution
## Selected gate: C — Reliability (Single-Instance Process Integrity)

| Field | Value |
| --- | --- |
| **Program** | P16.PI2 |
| **Selected gate** | **C — Reliability** (minimum unit: single-instance) |
| **Not selected** | Updater (Gate B) — not assumed next |
| **Max layer** | Production / Runtime |
| **P17** | Not begun |
| **Commit** | `87dd540` (`v2-dev`) |

---

## 1. Before implementation — four questions

| Question | Answer |
| --- | --- |
| Highest production risk? | Concurrent processes can open the same SQLite/`workspace.db` and Moments state → silent corruption / Singular Truth violation at process boundary |
| Why now? | Installer foundation (Gate A partial) accepted; users can install and launch twice; signing cannot close without Owner cert; updater depends on signing |
| User problem that disappears? | “I opened Workspace twice and my data acted strangely / restore fought itself” |
| Success measured how? | Second launch focuses existing window and does not start a second kernel; `verify:single-instance` green; dependency present |

---

## 2. Re-rank (repository evidence)

| Candidate | User risk ↓ | Trust ↑ | Reliability | Effort | Dependency | Ship now? | Verdict |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Gate A Signing | Highest (SmartScreen) | Highest | Med | **Blocked** (Owner cert) | After S1 | No (without cert) | Defer — scaffolding alone does not remove user risk |
| Gate A Artifact checksums | Med | Med | Low | Low | After S1 | Yes | Lower than dual-instance data risk |
| Gate B Updater | High | High | High | High | **Needs signing** | No | Not next |
| Gate B Diagnostics / support bundle | Med | Med-High | High | Med | None | Yes | Strong; prevents less than dual-instance |
| **Gate C Single-instance** | **High (data)** | Med-High | **Highest** | Med | None | **Yes** | **Selected** |
| Gate D IPC quarantine | Med-High | Med | Med | High | Careful | Partial | Later |
| Gate E Tray | Med | High feel | Med | Med | None | Yes | Experience after reliability |
| Gate F Release CI | Med | Med | Med | Med | Signing | Partial | Later |

**Objective next gate:** Gate C — Reliability via **single-instance process integrity**.

Constitutional compliance: preserves Singular Truth / no competing desktop authorities at the process layer. No Spec change. No Review Trigger.

---

## 3. Implementation (minimum)

| Change | Path |
| --- | --- |
| `tauri-plugin-single-instance` | `app/src-tauri/Cargo.toml` |
| Plugin registered **first**; secondary launch focuses `main` | `app/src-tauri/src/lib.rs` |
| Verifier | `scripts/verify-single-instance.mjs` |
| Gates model | `docs/production/PRODUCTION_GATES.md` |

No updater. No tray. No IPC rewrite. No signing theater.

---

## 4. After implementation

| Question | Result |
| --- | --- |
| Readiness improved? | **Yes** — dual-kernel launch path closed |
| Independently verified? | `pnpm verify:single-instance` + cargo check |
| Can it ship? | **Yes** with next installer rebuild (plugin in binary) |
| Complexity vs value? | One WRAP plugin + focus helper — value ≫ complexity |

### Production readiness delta

| Area | Before | After |
| --- | --- | --- |
| Process singular truth | Absent (Verified gap) | Single-instance enforced |
| `startupBehaviour` | MinorImprovement | Improved (secondary → focus) |
| Signing / updater | Unchanged | Unchanged (correctly deferred) |

---

## 5. Remaining gates (not started)

- Gate A — Signing (+ full Distribution closure)  
- Gate B — Operations (updater, diagnostics, support bundle, crash)  
- Gate C — Remainder (startup budgets, shutdown policy with tray, config/DB validation UX)  
- Gate D — Security (IPC)  
- Gate E — Experience (tray, a11y, …)  
- Gate F — Release automation  

---

## 6. Stop

**Do not begin another production gate.** Await Product Owner review.
