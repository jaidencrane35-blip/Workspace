# R1 — Release Engineering Readiness

| Field | Value |
| --- | --- |
| **Program** | Release Engineering Readiness |
| **ID** | R1 |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Repository readiness assessment — **no product behaviour change** |
| **Stage** | Pre–Stage 3 discovery (Release Engineering) |
| **Not** | Product Proof · UX optimization · architecture review · implementation sprint |
| **Prior product-quality arc** | P17–P20 considered complete — **do not reopen** |
| **Authorities** | Spec v2.1 · EES v1 · `PRODUCTION_DEPENDENCY_AUTHORITY.md` · Gate Catalog · `production-readiness.json` · `INSTALLER.md` |

**Classification key:** every finding is exactly one of  
`READY` · `NEEDS ENGINEERING` · `EXTERNAL DEPENDENCY` · `OWNER DECISION` · `DEFERRED`

---

## 1. Executive Summary

Workspace can enter Stage 3 Release Engineering **after Owner Acceptance of Product Proof**, with **minimal additional discovery**. Installer foundation (NSIS), artifact checksum tooling, support-bundle export, single-instance, and tray lifecycle engineering units already exist and are verifier-backed.

**Release Ready is false today** (`productionReadyToday: false`, `releaseReady: false`).

| Critical path | Status |
| --- | --- |
| Product Proof engineering arc (P17–P20) | Complete — do not reopen |
| Owner Product Proof Accept (Voice + feel) | **Open** — Stage 2 |
| Authenticode certificate (A2) | **EXTERNAL DEPENDENCY** — hard blocker for signed ship + updater |
| CI release maturity (F1) | **NEEDS ENGINEERING** — can start when Owner authorizes RE (no cert required) |
| Signed release publish (F2) / Updater (B2) | **Blocked until A2** |

**Immediate post-Accept Stage 3 start:** F1 CI automation + release hygiene (docs/version sync).  
**Do not start:** B2 updater or UX optimization.  
**Owner parallel track:** procure Authenticode certificate and authorize RE work.

This report is sufficient for Stage 3 engineering to begin without a second discovery pass.

---

## 2. Release Readiness Scorecard

| Area | Classification | Notes |
| --- | --- | --- |
| Release configuration (Tauri/NSIS) | **READY** | `bundle.targets: ["nsis"]`; hooks.nsh; metadata present |
| Versioning (0.1.0 consistency) | **READY** (drift risk) | Aligned today; no sync automation |
| Version bump automation | **NEEDS ENGINEERING** | Manual multi-file bump |
| Build reproducibility | **NEEDS ENGINEERING** | Local Windows capable; CI does not build installer |
| Packaging (NSIS) | **READY** | `pnpm installer:build` |
| MSI second target | **DEFERRED** | Optional; docs/README may still mention MSI |
| Installer assets / hooks | **READY** | PRE/POST install; manifest; uninstall hygiene |
| Code signing readiness (integration points) | **NEEDS ENGINEERING** | No tauri/CI sign config yet |
| Code signing certificate | **EXTERNAL DEPENDENCY** | A2 BlockedExternal |
| Updater integration readiness | **NEEDS ENGINEERING** (after A2) | No plugin/config; B2 BlockedByGate |
| Release automation (F1) | **NEEDS ENGINEERING** | Only `ci-pr.yml` on `main` |
| Signed release pipeline (F2) | **EXTERNAL DEPENDENCY** → then eng | Needs A2 |
| Artifact generation | **READY** (manual) | Local `installer:build` |
| Artifact naming | **READY** | `Workspace_*_x64-setup.exe` convention |
| Checksums | **READY** | A1 Complete; generate/verify scripts |
| Version metadata / install-manifest | **READY** | hooks write version JSON |
| Crash reporting | **DEFERRED** | B3 OptionalPolish; no minidump pipeline |
| Diagnostics packaging | **READY** | File logs + rotation |
| Support bundle | **READY** | B1 eng-complete; OA pending Owner |
| Release documentation (eng) | **NEEDS ENGINEERING** | Strong gate docs; weak single runbook; README drift |
| End-user install guide | **DEFERRED** / **OWNER DECISION** | Matrix TrackARequired |
| Build scripts | **READY** | `installer:build`, `verify:release`, checksum scripts |
| CI/CD assumptions | **NEEDS ENGINEERING** | PR CI ≠ release CI; branch `main` only |
| Environment / secrets assumptions | **NEEDS ENGINEERING** | No signing secret runbook; `.env.example` absent |
| Windows packaging flow | **READY** | Documented NSIS path |
| Tauri release flow | **READY** (unsigned) | Signed flow missing |
| Repository cleanliness (RE-relevant) | **NEEDS ENGINEERING** | README MSI claim; health tray debt; CI-CD-PLAN aspirational |
| Developer onboarding for releases | **NEEDS ENGINEERING** | CONTRIBUTING lacks release section |
| Owner Product Proof Accept | **OWNER DECISION** | Blocks capability expansion; gates “ship as product” |
| Operational Acceptance (A0/B1/E1/A1) | **OWNER DECISION** | Engineering Complete ≠ OA |

---

## 3. Release Pipeline Inventory

### What exists today

```text
Developer machine (Windows)
  pnpm typecheck / build / test
  pnpm verify:release          → validate (no NSIS build required)
  pnpm installer:build         → NSIS setup.exe under target/.../bundle/nsis/
  pnpm checksums:generate      → Workspace_*_x64-setup.exe.sha256
  (manual) smoke install / uninstall

GitHub Actions (.github/workflows/ci-pr.yml)
  trigger: pull_request|push → main only
  jobs: pnpm install, typecheck, frontend build, cargo check/build/test, pnpm test
  missing: v2-dev, installer:build, checksum publish, sign, release upload
```

### Gate units (machine state)

| Unit | Classification | Eng status |
| --- | --- | --- |
| A0 installer foundation | Complete | READY |
| A1 artifact checksums | Complete | READY |
| C0 single-instance | Complete | READY |
| B1 diagnostics / support bundle | Complete | READY (OA open) |
| E1 tray lifecycle | Complete | READY (OA open) |
| F1 CI automation | ReleaseOnly | NEEDS ENGINEERING |
| A2 code signing | BlockedExternal | EXTERNAL DEPENDENCY |
| F2 signed release | BlockedByGate ← A2 | EXTERNAL then NEEDS ENGINEERING |
| B2 auto-updater | BlockedByGate ← A2 | NEEDS ENGINEERING after A2 |
| B3 crash-local | OptionalPolish | DEFERRED |
| A3 install rollback | BlockedByGate ← B2 | DEFERRED |
| D1 IPC quarantine | ReadyNow (`nextReadyNow`) | OWNER DECISION / security — not packaging |
| C2 shutdown policy | BlockedByGate (stale meta) | DEFERRED for Stage 3 packaging |

### Integration points (for Stage 3 implementers)

| Concern | Integration point | Present? |
| --- | --- | --- |
| Bundle target | `app/src-tauri/tauri.conf.json` → `bundle.targets` | Yes — `nsis` |
| NSIS hooks | `app/src-tauri/windows/hooks.nsh` | Yes |
| Sign config | tauri.conf certificate / CI secret | **No** |
| Updater plugin | `app/src-tauri/Cargo.toml` | **No** (`single-instance` only) |
| Updater endpoints | tauri.conf / publish host | **No** |
| Require-signed gate | `WORKSPACE_REQUIRE_SIGNED=1` (catalog) | **Not implemented** |
| Checksum scripts | `scripts/generate-artifact-checksums.mjs` | Yes |
| Support export | `export_support_bundle` IPC | Yes |

---

## 4. External Dependencies

| Dependency | Blocks | Classification | Owner |
| --- | --- | --- | --- |
| Authenticode code-signing certificate (or org equivalent) | A2, F2, B2, public install trust | **EXTERNAL DEPENDENCY** | Owner / org |
| Windows timestamp authority (when signing) | Durable signature validity | **EXTERNAL DEPENDENCY** | Engineering ops |
| Update artifact hosting (HTTPS channel) | B2 | **EXTERNAL DEPENDENCY** | Owner / Engineering |
| GitHub Actions secrets / environment for cert | F2 CI sign | **EXTERNAL DEPENDENCY** | Owner + Engineering |
| Owner Product Proof Accept | Permanent P16 close; File Provider; “product ship” confidence | **OWNER DECISION** | Owner |
| Operational Acceptance drills | Production Ready claims for completed gates | **OWNER DECISION** | Owner |

**Rule (Dependency Authority):** Do not implement B2 before A2 Complete.

---

## 5. Repository Readiness

### READY

- NSIS packaging configuration and installer hooks  
- Checksum generate/verify tooling (A1)  
- Support bundle + file log rotation (B1)  
- `pnpm installer:build` / `pnpm verify:release` / `pnpm test` verifiers for production gates already wired  
- Version currently consistent at `0.1.0` across package.json (root + app), tauri.conf, Cargo workspace  
- Artifact naming convention understood by checksum scripts  

### NEEDS ENGINEERING (pre/at Stage 3 start — no cert required)

| Item | Why |
| --- | --- |
| F1 CI on active branch (`v2-dev`) + release-relevant checks | Today CI is `main`-only; no installer build |
| Single Release Engineer runbook | Gate docs exist; one path for bump → build → checksum → tag |
| README packaging truth | Claims MSI + NSIS; config is NSIS-only |
| Version sync assertion script | Prevent silent version drift |
| Signing secret / thumbprint documentation skeleton | So A2 can land without rediscovery |
| project-health tray debt row accuracy | Stale “not implemented” undermines machine trust |

### EXTERNAL DEPENDENCY

- Authenticode certificate availability  

### OWNER DECISION

- Authorize Stage 3 Release Engineering work (F1 entry criterion)  
- Product Proof Accept / Accept with changes / Reject  
- Channel policy (stable/beta) before B2  
- Whether B3 crash dumps are in first public release  

### DEFERRED

- MSI target  
- B3 local crash reporter  
- A3 rollback  
- End-user marketing install guide  
- UX items (C2, dock gravity, toolDock) — not Stage 3  

---

## 6. Manual Release Steps (current unsigned path)

Until F1/F2 exist, a release engineer would:

1. Confirm clean tree on intended tag branch.  
2. Bump version in lockstep: root `package.json`, `app/package.json`, `app/src-tauri/tauri.conf.json`, Cargo workspace `version` (manual).  
3. Run `pnpm verify:release` on Windows.  
4. Run `pnpm installer:build`.  
5. Locate `Workspace_*_x64-setup.exe` under `target/release/bundle/nsis/` or `app/src-tauri/target/release/bundle/nsis/`.  
6. Run `pnpm checksums:generate`.  
7. Manually smoke: install → launch → support export → uninstall.  
8. (Future) Sign with Authenticode; verify signature.  
9. (Future) Publish setup.exe + `.sha256` (+ update manifest after B2).  
10. Tag git release; record notes.

**Gaps:** steps 2, 4–7 not in CI; steps 8–9 impossible without A2.

---

## 7. Automation Opportunities

| Opportunity | Gate / class | Cert required? |
| --- | --- | --- |
| CI on `v2-dev` + `verify:release` subset | F1 | No |
| CI `installer:build` + upload unsigned artifact | F1 / internal smoke | No |
| Auto checksum sidecar on artifact job | A1 + F1 | No |
| `sync:version` / assert version equality | Hygiene | No |
| CI sign + timestamp after secret present | A2 / F2 | **Yes** |
| Tag → GitHub Release attach setup + sha256 | F2 | Prefer signed |
| Publish updater JSON + signed artifacts | B2 | **Yes** |
| Secret scanning workflow | Hygiene | No |
| `WORKSPACE_REQUIRE_SIGNED=1` verify gate | A2 exit | Yes (to enforce) |

---

## 8. Risks

| Risk | Level | Mitigation direction |
| --- | --- | --- |
| SmartScreen / unsigned public install | **Critical** | A2 certificate + sign |
| B2 attempted before A2 | **Critical** | Forbidden by Dependency Authority |
| Installer regressions merge unnoticed | **High** | F1 installer verify / optional build |
| Version drift across manifests | **Medium** | Version sync script |
| README MSI misdirection | **Medium** | Doc hygiene in Stage 3 start |
| Signing secrets improvised at first ship | **High** | Runbook before cert arrives |
| Hard crashes undiagnosable | **Medium** | B3 deferred unless Owner requires |
| Treating F1 green as Release Ready | **High** | Definition §12 |
| Product Proof Reject after RE starts | **Medium** | Keep Stage 2 Accept before broad RE spend; F1 hygiene OK earlier if Owner authorizes |

---

## 9. Blockers

### Ship / Release Ready blockers

| Blocker | Classification |
| --- | --- |
| No Authenticode certificate | **EXTERNAL DEPENDENCY** |
| No signed setup.exe path (A2/F2) | **EXTERNAL DEPENDENCY** + **NEEDS ENGINEERING** |
| No updater (if “updateable product” is in Release Ready definition) | **NEEDS ENGINEERING** after A2 |
| Owner Product Proof not Accepted | **OWNER DECISION** |
| OA unpaid on production units (for Production Ready claims) | **OWNER DECISION** |

### Not blockers for starting Stage 3 engineering

| Item | Classification |
| --- | --- |
| F1 CI automation (unsigned) | **NEEDS ENGINEERING** — start when Owner authorizes RE |
| Release runbook / README sync | **NEEDS ENGINEERING** |
| B3 crash dumps | **DEFERRED** |
| MSI | **DEFERRED** |
| UX polish (C2, dock, toolDock) | **DEFERRED** — do not reopen P17–P20 |
| D1 IPC quarantine | **OWNER DECISION** timing — parallel security, not packaging prerequisite for F1 |

---

## 10. Recommended Stage 3 execution order

> Assumes Stage 2 Owner Accept (or explicit Owner authorization to begin RE in parallel with Accept).  
> Does **not** reopen P17–P20. Does **not** start UX optimization.

| Order | Workstream | Classification | Depends on |
| --- | --- | --- | --- |
| **0** | Owner: Accept Product Proof + authorize Stage 3 RE; start cert procurement | **OWNER DECISION** / **EXTERNAL DEPENDENCY** | — |
| **1** | Release hygiene: README NSIS truth, Release Engineer runbook, version-assert script, health debt accuracy | **NEEDS ENGINEERING** | Owner auth optional but recommended |
| **2** | **F1-ci-automation**: CI on `v2-dev` (or active branch); `verify:release` subset; installer foundation/checksum verifiers; optional unsigned `installer:build` artifact | **NEEDS ENGINEERING** | Owner authorizes RE |
| **3** | **A2-code-signing** (wire tauri/signtool, secret docs, verify path, `WORKSPACE_REQUIRE_SIGNED`) | **NEEDS ENGINEERING** | Certificate in hand |
| **4** | **F2-signed-release**: tag → signed setup + checksums + notes | **NEEDS ENGINEERING** | A2 |
| **5** | **B2-auto-updater**: plugin, channel, signed manifests, honest failure | **NEEDS ENGINEERING** | A2 + channel policy |
| **6** | **B3-crash-local** (optional) | **DEFERRED** unless Owner requires | B1 |
| **7** | **A3-install-rollback** | **DEFERRED** | B2 |

**Parallel (not packaging):** D1 IPC quarantine when Owner prioritizes security surface — do not block F1/A2 prep.

---

## 11. Estimated engineering effort (Stage 3 workstreams)

| Workstream | Effort | Calendar risk |
| --- | --- | --- |
| Release hygiene (docs + version assert) | **1–2 eng days** | Low |
| F1 CI automation (+ optional unsigned installer artifact) | **3–5 eng days** | Med (Tauri build time on GHA) |
| Authenticode procurement | **0 eng / 1–6+ weeks calendar** | **EXTERNAL** |
| A2 signing integration | **3–5 eng days** after cert | Med (timestamp/certs) |
| F2 signed release workflow | **3–5 eng days** | Med |
| B2 auto-updater | **1–2 eng weeks** | High (channels + UX honesty) |
| B3 crash-local | **3–7 eng days** | Med |
| A3 rollback | **3–7 eng days** | Med |

---

## 12. Definition of “Release Ready”

Workspace is **Release Ready** only when **all** of the following are true:

1. **Owner Product Proof Accepted** (Stage 2) — product vision/feel gate closed.  
2. **A2 Complete** — Authenticode-signed `Workspace_*_x64-setup.exe` with documented verify path.  
3. **F2 Complete** — repeatable signed publish path (CI or one documented command) producing setup.exe + `.sha256`.  
4. **B2 Complete** — if the release promise includes in-app update; otherwise Owner explicitly waives updater for a one-shot installer-only release (documented waiver).  
5. **Operational Acceptance** unpaid boxes for shipped units (A0/A1/B1/E1 at minimum) are checked by Owner.  
6. **No open Ship Blockers** (data loss, security, startup failure, broken installer/updater on the supported path).

**Explicitly insufficient alone:**

- F1 green  
- `pnpm verify:release` green  
- Engineering Complete on P17–P20  
- Unsigned NSIS build  
- Checksums without signatures  

**Release Ready ≠ Production Ready ≠ Engineering Complete** (Dependency Authority).

---

## Area detail (current · missing · deps · assumptions · manual · automation · blockers · non-blockers · risk · owner)

### Release configuration / Tauri packaging
- **Current:** NSIS-only; publisher metadata; WebView2 embedBootstrapper; allowDowngrades false.  
- **Missing:** Sign/updater blocks.  
- **Class:** READY (unsigned) · NEEDS ENGINEERING (signed/update).  
- **Risk:** Low unsigned / Critical public. · **Owner:** Engineering  

### Versioning
- **Current:** 0.1.0 aligned.  
- **Missing:** Sync script; changelog automation.  
- **Class:** READY + NEEDS ENGINEERING (automation). · **Owner:** Engineering  

### Build reproducibility / CI
- **Current:** Local Windows reproducible; CI validates code on `main` only.  
- **Missing:** Installer in CI; `v2-dev` coverage.  
- **Class:** NEEDS ENGINEERING (F1). · **Owner:** Engineering; entry OWNER DECISION  

### Installer assets
- **Current:** hooks.nsh, manifest, taskkill, uninstall purge rules.  
- **Missing:** OA smoke.  
- **Class:** READY · OA OWNER DECISION. · **Risk:** Med. · **Owner:** Engineering / Owner  

### Code signing
- **Current:** None.  
- **Missing:** Cert, config, CI secret, verify gate.  
- **Class:** EXTERNAL DEPENDENCY + NEEDS ENGINEERING. · **Risk:** Critical. · **Owner:** Owner then Engineering  

### Updater
- **Current:** Absent.  
- **Missing:** Plugin, endpoint, signed manifests, UX.  
- **Class:** NEEDS ENGINEERING after A2. · **Risk:** High if rushed. · **Owner:** Engineering; channel OWNER DECISION  

### Checksums / artifacts
- **Current:** A1 scripts + naming convention.  
- **Missing:** CI publish.  
- **Class:** READY. · **Owner:** Engineering  

### Crash / diagnostics / support
- **Current:** Logs + support bundle.  
- **Missing:** Minidumps (B3).  
- **Class:** READY (B1) · DEFERRED (B3). · **Owner:** Engineering / Owner  

### Secrets / docs / onboarding
- **Current:** gitignore for secrets; production gate docs strong; README/CONTRIBUTING weak for release.  
- **Missing:** Signing runbook; `.env.example` optional; README NSIS truth.  
- **Class:** NEEDS ENGINEERING. · **Owner:** Engineering  

---

## Compliance

| Constraint | Posture |
| --- | --- |
| No signing implementation | Observed |
| No updater implementation | Observed |
| No runtime / UX change | Observed |
| No new frameworks / governance | Observed |
| P17–P20 not reopened | Observed |
| Spec / EES | Unchanged |

---

## Stop

R1 assessment complete. **Do not implement Stage 3 workstreams in this program.**  
Await Owner Acceptance (Stage 2) and authorization to begin Stage 3 per §10.
