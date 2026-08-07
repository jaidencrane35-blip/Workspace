# PR1 — Workspace Production Readiness Program

| Field | Value |
| --- | --- |
| **Kind** | Production program (Sustainable Engineering Operations) |
| **Date** | 2026-08-07 |
| **Max layer** | Production + Repository Standards + Documentation |
| **Branch** | `v2-dev` |
| **Commit** | `8907f6a` (`v2-dev`) — assessment + verifier; no Phase 2 runtime impl |
| **Machine matrix** | `docs/production/production-readiness.json` |
| **Verifier** | `pnpm verify:production-readiness` |
| **Spec / governance / P17 / Product Proof redesign?** | No |

**Repository truth (not reopened):** Constitution complete · Governance self-sustaining · No Review Trigger · Product Proof is the product gate · Track A = production maturity.

---

## 1. Production Readiness Assessment

### 1.1 Executive verdict

| Question | Answer | Confidence |
| --- | --- | --- |
| Production-ready today? | **No** | Verified |
| Suitable for Owner live Product Proof via `tauri dev`? | **Yes** (eng-assisted) | Supported |
| Suitable for broad public release? | **No** | Verified |
| Architecture blocking production? | **No** | Supported |
| Greatest gap class | Install / update / tray / diagnostics / signing / IPC quarantine | Verified |

A new user **cannot** yet install, update, diagnose, and remove Workspace without engineering assistance. Session/Moment recovery and local privacy are stronger than distribution surfaces.

### 1.2 Area maturity (summary)

Maturity key: **Complete** · **MinorImprovement** · **TrackARequired** · **FutureScale** · **OutOfScope**

| Area | Maturity | Confidence | Evidence (path) |
| --- | --- | --- | --- |
| Installer | TrackARequired | Verified | `tauri.conf.json` bundle only; no repair/silent/signing |
| Packaging | MinorImprovement | Verified | icons + `targets: all`; version `0.1.0` |
| Code signing | TrackARequired | Verified | absent from config + CI |
| Auto-update | TrackARequired | Verified | no updater plugin |
| Rollback | FutureScale | Verified | no updater |
| Crash recovery (process) | MinorImprovement | Supported | session checksum recovery; no crash reporter |
| Diagnostics / support bundle | TrackARequired | Verified/Supported | domain diagnostics; no support export UX |
| Logging / rotation | MinorImprovement / TrackARequired | Verified | `env_logger`; no rotation |
| Telemetry | Complete (none by design) | Supported | experience tests forbid network telemetry |
| Privacy | Complete (local-first posture) | Supported | product docs + tests |
| Error reporting | TrackARequired | Verified | no minidump pipeline |
| System tray | TrackARequired | Verified | health `tray-integration` |
| Background lifecycle | TrackARequired | Supported | operator `skipTaskbar`; no tray policy |
| Startup / shutdown | MinorImprovement | Supported | window defs; `exit_workspace` |
| Update channels | FutureScale | Verified | no updater |
| Version management | MinorImprovement | Verified | `0.1.0`; no release job |
| Settings / config | MinorImprovement | Supported | settings IPC; no import/export/reset UX |
| First-run | MinorImprovement | Supported | `first_run` flag; tour polish debt |
| Accessibility | MinorImprovement | Supported | aria on operator; no a11y gate |
| High-DPI | MinorImprovement | Hypothesis | WebView2 default |
| Multi-monitor | MinorImprovement | Supported | Window provider closed; relative polish |
| Window persistence | MinorImprovement | Supported | fixed geometry in conf |
| Performance budgets | TrackARequired | Verified | none published |
| Installer repair / uninstall | TrackARequired | Verified/Hypothesis | untested |
| Release automation | TrackARequired | Verified | `ci-pr.yml` validate-only |
| Security / IPC | TrackARequired | Verified | CSP yes; 197 vs 20 IPC |
| Dead code | MinorImprovement | Supported | kernel unused Track A |
| Production docs | TrackARequired | Verified | this program is first SST |

Full machine rows: `production-readiness.json`.

---

## 2. Installer Program

| Field | Value |
| --- | --- |
| Goal | Professional Windows install / upgrade / repair / uninstall |
| Lowest layer | Production (Tauri bundle / NSIS or MSI) |
| Gate | Phase 2 (public release) — not required for Owner Product Proof |

| Work item | Risk | Benefit | Effort | User impact | Priority |
| --- | --- | --- | --- | --- | --- |
| Choose NSIS vs MSI (or both); document | Low | Clear ship path | S | Trust | Critical (Phase 2) |
| Branded installer UX + shortcuts | Low | Perceived quality | M | Delight | High |
| Upgrade in place | Med | Continuity | M | Trust | Critical |
| Repair | Med | Supportability | M | Recover | High |
| Silent install (`/S`) | Low | IT / scale | S | 1k+ users | Medium → Phase 3 |
| Clean uninstall (data policy explicit) | Med | Trust | M | Remove | Critical |
| WebView2 / dependency validation | Med | First-run success | M | Trust | Critical |
| Code signing readiness (Authenticode) | High process | SmartScreen trust | L | Trust | Critical |
| Rollback of failed upgrade | Med | Recover | L | Trust | Deferred → Phase 3–4 |

**Shall not:** invent a custom installer architecture; escalate to Constitution.

---

## 3. Auto-Updater Program

| Field | Value |
| --- | --- |
| Goal | Trusted in-app updates with integrity |
| Lowest layer | Production (Tauri updater plugin + signed artifacts) |
| Evidence of gap | No updater in `Cargo.toml` / `tauri.conf.json` |

| Work item | Priority | Phase |
| --- | --- | --- |
| Enable Tauri updater + pubkey | Critical | 2 |
| Stable channel + signed manifests | Critical | 2 |
| Update UX (non-blocking, honest) | High | 2 |
| Integrity validation before apply | Critical | 2 |
| Failed-update recovery | High | 3 |
| Staged rollout / percentage | Medium | 4 |
| Beta channel | Low | 4–5 |
| Forced critical security updates | Medium | 3 |

**User experience:** Presentation-only chrome; no provider orchestration in update UI. Product Gravity: do not build an “update dashboard.”

---

## 4. Diagnostics Program

| Field | Value |
| --- | --- |
| Goal | Operator-readable diagnosis without engineering SSH |
| Lowest layer | Production + Presentation (+ existing Evidence/diagnostics domain) |

| Work item | Evidence | Priority | Phase |
| --- | --- | --- | --- |
| Structured log sinks (file) | `env_logger` console-oriented today | Critical | 2 |
| Log rotation / size caps | absent | High | 2 |
| Support bundle export (logs + versions + health JSON, redacted) | absent UX | Critical | 2 |
| Crash capture (local minidump or Windows Error Reporting opt-in) | absent | High | 3 |
| Operator-readable “something went wrong” copy | Conversation honesty exists | Medium | 2 |
| Developer diagnostics remain behind non-default surface | ExperienceEvidenceDashboard pattern | Medium | 2 |
| Optional remote error reporting | **Out of Scope by default** (privacy Complete) | Deferred / never default | — |

---

## 5. Reliability Program

| Work item | Lowest layer | Evidence | Priority | Phase |
| --- | --- | --- | --- | --- |
| Preserve session checksum recovery | Runtime (exists) | `workspace_session_store` | Maintain | 1–2 |
| Moment approve-before-restore | Presentation (exists) | Product Proof | Maintain | 1 |
| Unexpected shutdown → honest restart | Runtime + Presentation | partial | High | 2 |
| OS restart / sleep Voice residual | WRAP limitation | Voice docs F9/COM | Document + soft recover | 2 |
| DB integrity checks on open | Runtime | SQLite + checksums | High | 2 |
| Corruption detection UX | Presentation | recovery empty path | Medium | 2 |
| Single-instance policy | Production | not found | High | 2 |

---

## 6. System Tray Program

| Field | Value |
| --- | --- |
| Goal | Persistent, trustworthy Workspace presence |
| Evidence | `outstandingProductDebt.tray-integration`; no TrayIcon |
| Lowest layer | Production / Presentation |
| Product Gravity | Tray supports Conversation; must not become a second product |

| Work item | Priority | Phase |
| --- | --- | --- |
| Tray icon + Show Conversation / Exit | Critical | 2 |
| Minimize-to-tray / restore | High | 2 |
| Shutdown policy (Exit vs hide) | Critical | 2 |
| Notification click → Conversation | Medium | 2–3 |
| Launch at login (opt-in) | Medium | 3 |
| Background lifecycle when window closed | High | 2 |

---

## 7. Security Hardening Program

| Work item | Evidence | Priority | Phase |
| --- | --- | --- | --- |
| IPC quarantine (197 → product-used + justified) | health `surface-size` | Critical | 2 |
| Keep CSP (already present) | `tauri.conf.json` security.csp | Maintain | — |
| Permission validation review on remaining IPC | tiers exist | High | 2 |
| Code signing + tamper-evident updates | absent | Critical | 2 |
| Least privilege OS capabilities | Windows integration ports | Medium | 2–3 |
| Dead-code / unused kernel command reduction | Track A warnings | Medium | 3 |

**Never:** ambient capture, default network telemetry, provider→provider calls.

---

## 8. Performance Assessment

| Metric | Current | Confidence | Action |
| --- | --- | --- | --- |
| Cold startup (installed) | Unknown | Unknown | Measure Phase 2; set budget |
| Warm startup | Supported OK for eng | Supported | Budget + regress |
| Voice Click→Ready | Engineered; Owner live Unknown | Supported/Unknown | Product Proof timings |
| Memory / CPU / GPU / idle | No published budgets | Verified gap | Define Phase 2–3 |
| Large session / Moments | Partial eng tests | Supported | Load tests Phase 3 |
| Background idle with tray | N/A until tray | — | After tray |

---

## 9. UX Polish Assessment

| Item | Maturity | Priority | Phase | Notes |
| --- | --- | --- | --- | --- |
| Window show/hide animations | Track A debt | Medium | 2–3 | `window-animations` |
| Loading / progress honesty | MinorImprovement | Medium | 2 | Voice phases model |
| Error presentation | Supported strength | Maintain | — | No invented success |
| Accessibility / keyboard | MinorImprovement | High | 2–3 | a11y pass |
| Focus / visual consistency | MinorImprovement | Medium | 2 | Operator surface |
| First-run permission tour | Track A | Medium | 2 | After Owner PP defects |
| Micro-interactions | Polish | Low | 3 | After Critical |

Polish must not compete with Conversation (Product Gravity).

---

## 10. Release Engineering Plan

| Work item | Evidence | Priority | Phase |
| --- | --- | --- | --- |
| CI validate (exists) | `ci-pr.yml` | Maintain | — |
| CI on `v2-dev` (today: `main` only) | workflow `branches: [main]` | High | 2 |
| Signed release workflow | absent | Critical | 2 |
| Artifact checksums + SBOM-lite | absent | High | 3 |
| SemVer + release notes | `0.1.0` only | High | 2 |
| `pnpm verify:release` in release job | script exists | High | 2 |
| Reproducible builds | Hypothesis | Medium | 4 |
| Rollback runbook | absent | High | 3 |

---

## 11. Production Roadmap

### Phase 1 — Before Product Complete (P16 Owner Accept)

| Item | Why |
| --- | --- |
| Owner live Product Proof (package + O2 workbook) | Product gate — not production code |
| Fix only Owner-evidenced defects (lowest layer) | Evidence Before Modification |
| Keep production plans current (this program) | Sequencing honesty |

**Not required for Product Complete:** signed installer, updater, tray (Owner may use `tauri dev`).

### Phase 2 — Before broad public release

Installer (branded + upgrade + uninstall) · Authenticode signing · Auto-updater stable channel · Tray + lifecycle · Support bundle + log files/rotation · IPC quarantine · Single-instance · Release workflow · Basic performance budgets · a11y pass · End-user install/remove docs.

### Phase 3 — Before ~1,000 users

Silent install · Update failure recovery · Local crash capture · Launch-at-login opt-in · Load/session stress · Release notes discipline · Uninstall cleanliness verification · Support runbook.

### Phase 4 — Before ~10,000 users

Staged rollout · Beta channel · Deeper telemetry **only if** explicit opt-in and Constitution-compatible (default remains off) · Reproducible builds · Stronger SBOM / supply chain · Animation polish.

### Phase 5 — Before ~100,000+ users

Enterprise silent/deploy at scale · Advanced staged canaries · Regional update CDNs · Formal support SLAs · Accessibility certification depth · Multi-locale production (if product expands).

---

## 12. Engineering Priority Matrix

| Priority | Items | Gate |
| --- | --- | --- |
| **Critical** | Owner Product Proof; signing; updater; installer upgrade/uninstall; tray+exit policy; support bundle; IPC quarantine | Phase 1–2 |
| **High** | Log rotation; single-instance; CI release job; DB integrity UX; a11y; versioning/notes; update recovery | Phase 2–3 |
| **Medium** | Silent install; launch-at-login; window animations; first-run tour; crash minidump; performance budgets | Phase 2–3 |
| **Low** | Micro-interaction polish; beta channel UX | Phase 3–4 |
| **Deferred** | Staged % rollout; reproducible builds; 100k ops | Phase 4–5 |
| **Never** | Default network telemetry; ambient mic; architecture redesign for production; provider coupling | — |

---

## 13. Settings Program (brief)

| Work item | Layer | Phase |
| --- | --- | --- |
| Production defaults documented | Documentation | 2 |
| Reset to defaults | Presentation + settings IPC | 2 |
| Import/export (local file, redacted secrets) | Presentation + Runtime | 3 |
| Settings migration on version bump | Runtime | 2–3 |
| Profiles | FutureScale / OutOfScope until need evidenced | 4+ |

---

## 14. Explicit answers

| Question | Answer |
| --- | --- |
| Is Workspace production-ready today? | **No** (Verified) |
| What prevents broad public release? | No signed installer/updater, no tray lifecycle, weak diagnostics/support export, large IPC surface, no release automation, Product Proof still open |
| Greatest perceived quality lift? | Tray presence + honest update/install trust + Conversation polish from Owner PP |
| Greatest operational reliability lift? | Signed updates + support bundle + session/DB recovery UX + single-instance |
| Mandatory? | Phase 1 Owner PP; Phase 2 Critical matrix rows |
| Polish? | Animations, micro-interactions, tour chrome |
| After P17? | File-centric production concerns; deeper automation deploy; not a reason to delay Phase 2 Critical |
| Never implement? | Default telemetry; ambient mic; constitutional redesign “for production” |
| Support first 1,000 users after this program? | **Yes, after Phase 2–3 complete** (Supported) — not after docs alone |

---

## 15. Constitutional / governance confirmation

| Check | Result |
| --- | --- |
| Spec modified? | No |
| Governance redesigned? | No |
| Product Proof reopened as redesign? | No — sequencing preserved |
| P17 begun? | No |
| Review Trigger? | **No Constitutional Review Trigger.** |
| Architecture drift introduced? | No — Production layer only |

---

## 16. Repository Health / next engineering

| Item | Status |
| --- | --- |
| Handoff | Production program PR1 active as Track A authority; P16 PP still pending |
| Machine matrix | `production-readiness.json` |
| Verifier | `verify-production-readiness` |
| Implementation in PR1? | **Assessment + plans + verifier only** — no tray/updater code in this commit |
| Next implementation programs | After Owner directs: Phase 2 Critical slices **one program at a time** (EES) OR Owner Product Proof first |

**Recommended Owner order:** Product Proof session → accept/reject → then Phase 2 Critical production slices (Installer/Signing → Updater → Tray → Diagnostics → IPC).

---

## Declaration

PR1 establishes production truth and an evidence-backed roadmap. Workspace is **engineering-capable** and **not distribution-ready**. Improving production excellence must not reopen Constitution, governance, or P17.

**STOP.** Await Product Owner review.
