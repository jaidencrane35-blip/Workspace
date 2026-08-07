# Production Gate Catalog

| Field | Value |
| --- | --- |
| **Generated from** | `production-gates-dependency.json` |
| **Schema** | `PRODUCTION_GATE_SPECIFICATION.md` |
| **Sequencing** | `PRODUCTION_DEPENDENCY_AUTHORITY.md` |
| **nextReadyNow** | `A1-artifact-checksums` |
| **Do not edit by hand** | Run `node scripts/sync-production-gate-catalog.mjs` |

Every unit below uses the **same field set**. Operational Acceptance is human-trust criteria (not Product Proof, not CI).

## A0-installer-foundation

| Field | Value |
| --- | --- |
| **Gate family** | A |
| **Production classification** | Complete |
| **Purpose** | Provide a professional NSIS install/upgrade/uninstall path for Workspace on Windows. |
| **User value** | A user can install, upgrade, repair, and remove Workspace without engineering assistance. |
| **Engineering value** | Reproducible installer package, hooks, and install-manifest version detection. |
| **Constitutional justification** | Production layer; Presentation/install surface only — no new Information Owner. |
| **Prerequisites** | — |
| **Dependents** | A1-artifact-checksums, A2-code-signing |
| **Blocking conditions** | — |
| **Entry criteria** | `PR1 accepted`; `Tauri 2 bundle available` |
| **Exit criteria** | `NSIS config`; `hooks.nsh`; `INSTALLER.md`; `verify:installer-foundation`; `setup.exe buildable` |
| **Verification** | `pnpm verify:installer-foundation`; `pnpm installer:build` |
| **Production readiness delta** | Engineering Complete for install foundation; not Release Ready (unsigned). |
| **Regression risks** | Broken hooks block upgrade; Silent uninstall data policy surprises |
| **Rollback strategy** | Revert tauri.conf/hooks; ship prior setup.exe. |
| **Success metrics** | setup.exe produced; verifier green |

### Operational Acceptance

- [ ] A user can run the installer without developer instructions
- [ ] A user can uninstall cleanly or knowingly keep data
- [ ] A user can reinstall/upgrade over an existing install

## C0-single-instance

| Field | Value |
| --- | --- |
| **Gate family** | C |
| **Production classification** | Complete |
| **Purpose** | Ensure one Workspace process owns local state at a time. |
| **User value** | Opening Workspace twice no longer risks corrupted Moments/DB behaviour. |
| **Engineering value** | Process-level Singular Truth before kernel init. |
| **Constitutional justification** | Singular Truth / Conservation at process boundary; Runtime only. |
| **Prerequisites** | — |
| **Dependents** | E1-tray-lifecycle, C1-config-db-ux |
| **Blocking conditions** | — |
| **Entry criteria** | `Installer foundation usable or tauri dev` |
| **Exit criteria** | `Plugin first`; `secondary launch focuses main`; `verify:single-instance` |
| **Verification** | `pnpm verify:single-instance`; `cargo check -p workspace-app` |
| **Production readiness delta** | Engineering Complete + Production Ready for process integrity. |
| **Regression risks** | Plugin order wrong allows dual kernel; Focus fails on odd window state |
| **Rollback strategy** | Remove plugin registration; accept dual-instance risk temporarily. |
| **Success metrics** | Second launch focuses existing window; No second kernel init |

### Operational Acceptance

- [ ] A user can relaunch Workspace and return to the existing session
- [ ] A user can avoid two competing Workspace windows editing the same data

## B1-diagnostics-support-bundle

| Field | Value |
| --- | --- |
| **Gate family** | B |
| **Production classification** | Complete |
| **Purpose** | Let operators produce privacy-preserving local diagnostics without a developer present. |
| **User value** | A user can export logs/version info for support when something fails. |
| **Engineering value** | File logs, rotation, export_support_bundle IPC, Conversation path. |
| **Constitutional justification** | Observability + Evidence; privacy — no Moments DB contents; no telemetry. |
| **Prerequisites** | — |
| **Dependents** | B3-crash-local |
| **Blocking conditions** | — |
| **Entry criteria** | `Dependency Authority lists B1 Ready Now` |
| **Exit criteria** | `file_log rotation`; `export_support_bundle`; `NL path`; `verify:support-bundle` |
| **Verification** | `pnpm verify:support-bundle`; `pnpm verify:production-dependency-authority` |
| **Production readiness delta** | Engineering Complete for support export; Production Ready pending Owner Operational Acceptance. |
| **Regression risks** | Log volume disk growth; Accidental PII in logs |
| **Rollback strategy** | Disable file logger attach; remove Conversation intent. |
| **Success metrics** | Support folder created; manifest databaseContentsIncluded=false |

### Operational Acceptance

- [ ] A user can ask Conversation to export a support package
- [ ] A user can understand the package excludes Moments
- [ ] A user can find the saved folder path from the reply

## A1-artifact-checksums

| Field | Value |
| --- | --- |
| **Gate family** | A |
| **Production classification** | ReadyNow |
| **Purpose** | Publish integrity hashes for installer artifacts so downloads can be verified. |
| **User value** | A user (or CI) can confirm the setup.exe was not tampered with in transit. |
| **Engineering value** | SHA-256 sidecars + verify:artifact-checksums feeding release automation. |
| **Constitutional justification** | Integrity / Observability of distributed Effects artifacts — Production/Repository Standards. |
| **Prerequisites** | A0-installer-foundation |
| **Dependents** | F1-ci-automation |
| **Blocking conditions** | — |
| **Entry criteria** | `Gate Spec schema satisfied`; `A0 Complete`; `Owner authorizes next unit` |
| **Exit criteria** | `SHA-256 file beside setup.exe`; `documented verify command`; `verifier green` |
| **Verification** | `pnpm verify:artifact-checksums (to be added at implementation)` |
| **Production readiness delta** | Toward Release Ready integrity; not Release Ready alone. |
| **Regression risks** | Stale checksums after rebuild; Path mismatch in CI |
| **Rollback strategy** | Stop publishing sidecars; remove verify from CI. |
| **Success metrics** | Checksum matches rebuilt artifact; CI can fail on mismatch |

### Operational Acceptance

- [ ] A user can verify a downloaded installer hash against a published checksum
- [ ] A release engineer can regenerate checksums in one command

## D1-ipc-quarantine

| Field | Value |
| --- | --- |
| **Gate family** | D |
| **Production classification** | ReadyNow |
| **Purpose** | Reduce unused IPC attack surface to the commands Workspace actually needs. |
| **User value** | Less risk of obscure commands being abused; clearer product surface. |
| **Engineering value** | Hard quarantine or removal of unused registered commands; smaller surface-size risk. |
| **Constitutional justification** | Least privilege / Singular Truth of entry — Repository Standards + Runtime. |
| **Prerequisites** | — |
| **Dependents** | D2-permission-audit |
| **Blocking conditions** | Regression risk on experimental/developer surfaces |
| **Entry criteria** | `Gate Spec satisfied`; `B1 available for diagnosing fallout`; `Owner authorizes` |
| **Exit criteria** | `Documented quarantine list enforced`; `Product Proof IPC path green`; `surface-size improved` |
| **Verification** | `pnpm verify:ipc-tiers`; `pnpm test`; `product conversation smoke` |
| **Production readiness delta** | Security hardening toward Production/Release Ready. |
| **Regression risks** | Break developer/diagnostic tools; Miss a still-needed command |
| **Rollback strategy** | Restore command registration from git; loosen quarantine list. |
| **Success metrics** | Registered count down; No product path regressions |

### Operational Acceptance

- [ ] A user can complete supported Conversation workflows after quarantine
- [ ] A developer can still run required diagnostics without inventing new IPC

## E1-tray-lifecycle

| Field | Value |
| --- | --- |
| **Gate family** | E |
| **Production classification** | ReadyNow |
| **Purpose** | Provide persistent desktop presence that serves Conversation without becoming a second product. |
| **User value** | Workspace remains reachable from the tray; Exit/Show are obvious. |
| **Engineering value** | Tray icon, restore, Exit policy foundation for C2. |
| **Constitutional justification** | Product Gravity — tray supports Conversation; Presentation/Production only. |
| **Prerequisites** | C0-single-instance |
| **Dependents** | C2-shutdown-policy |
| **Blocking conditions** | — |
| **Entry criteria** | `Gate Spec satisfied`; `C0 Complete`; `Owner authorizes` |
| **Exit criteria** | `Tray Show Conversation`; `Exit`; `restore`; `no catalogue chrome` |
| **Verification** | `Manual Operational Acceptance`; `verify script for tray wiring` |
| **Production readiness delta** | Major perceived Production Ready lift for desktop presence. |
| **Regression risks** | Zombie processes; Competing with Conversation attention |
| **Rollback strategy** | Disable tray plugin; keep window-only lifecycle. |
| **Success metrics** | Tray appears on launch; Exit terminates process |

### Operational Acceptance

- [ ] A user can restore Conversation from the tray
- [ ] A user can Exit Workspace from the tray without confusion
- [ ] A user can use the tray without seeing a second app compete with Conversation

## C1-config-db-ux

| Field | Value |
| --- | --- |
| **Gate family** | C |
| **Production classification** | ReadyNow |
| **Purpose** | Tell the truth when session/DB recovery yields empty or corrupt state. |
| **User value** | A user understands recovery happened instead of silent empty desktop. |
| **Engineering value** | Presentation of existing kernel checksum recovery Facts. |
| **Constitutional justification** | Observability / truthful Experience — Presentation over Evidence. |
| **Prerequisites** | C0-single-instance |
| **Dependents** | — |
| **Blocking conditions** | — |
| **Entry criteria** | `Gate Spec satisfied`; `Owner authorizes` |
| **Exit criteria** | `Honest Conversation/UI on corrupt session`; `no invented success` |
| **Verification** | `Manual corrupt-session drill`; `unit tests for recovery path if added` |
| **Production readiness delta** | Reliability honesty toward Production Ready. |
| **Regression risks** | Alarm fatigue; False corrupt warnings |
| **Rollback strategy** | Revert copy to prior quiet recovery. |
| **Success metrics** | Corrupt fixture shows clear message |

### Operational Acceptance

- [ ] A user can understand that recovery occurred
- [ ] A user can see that success was not claimed when state was reset

## F1-ci-automation

| Field | Value |
| --- | --- |
| **Gate family** | F |
| **Production classification** | ReleaseOnly |
| **Purpose** | Automate validation and artifact checks on the active development branch. |
| **User value** | Indirect — fewer broken builds reach humans. |
| **Engineering value** | CI on v2-dev; checksum/verify jobs. |
| **Constitutional justification** | Repository Standards — no architectural change. |
| **Prerequisites** | A0-installer-foundation |
| **Dependents** | — |
| **Blocking conditions** | — |
| **Entry criteria** | `Gate Spec satisfied`; `Owner authorizes release engineering work` |
| **Exit criteria** | `Workflow covers v2-dev`; `verify:release or subset in CI` |
| **Verification** | `GitHub Actions green on sample PR` |
| **Production readiness delta** | Release engineering maturity; not Release Ready alone. |
| **Regression risks** | CI flakiness blocking merges |
| **Rollback strategy** | Narrow workflow triggers; make jobs advisory temporarily. |
| **Success metrics** | CI runs on v2-dev push/PR |

### Operational Acceptance

- [ ] A release engineer can see pass/fail without local full matrix
- [ ] A release engineer can see the pipeline fail when installer verify is broken

## A2-code-signing

| Field | Value |
| --- | --- |
| **Gate family** | A |
| **Production classification** | BlockedExternal |
| **Purpose** | Authenticode-sign Windows installers so users and OS trust the binary. |
| **User value** | SmartScreen/trust barriers drop for legitimate installs. |
| **Engineering value** | Signed artifacts unlock updater integrity and Release Ready. |
| **Constitutional justification** | Trust of Effects delivered to the OS — Production. |
| **Prerequisites** | A0-installer-foundation |
| **Dependents** | B2-auto-updater, F2-signed-release |
| **Blocking conditions** | Owner/org Authenticode certificate or equivalent |
| **Entry criteria** | `Certificate available`; `Gate Spec satisfied`; `Owner authorizes` |
| **Exit criteria** | `Signed setup.exe`; `signature verify path`; `docs for thumbprint/CI secret` |
| **Verification** | `signtool/ossl verify`; `WORKSPACE_REQUIRE_SIGNED=1 gate` |
| **Production readiness delta** | Critical step to Release Ready. |
| **Regression risks** | Cert expiry; Timestamp server outage |
| **Rollback strategy** | Ship unsigned with explicit Owner risk acceptance only. |
| **Success metrics** | Windows shows publisher; Signature valid |

### Operational Acceptance

- [ ] A user can install without unexplained SmartScreen distrust (normal channels)
- [ ] A release engineer can sign in CI or one documented command

## B2-auto-updater

| Field | Value |
| --- | --- |
| **Gate family** | B |
| **Production classification** | BlockedByGate |
| **Purpose** | Deliver trusted in-app updates without full reinstall. |
| **User value** | A user can stay current safely. |
| **Engineering value** | Tauri updater + signed manifests + failure recovery. |
| **Constitutional justification** | Production; Authority still gates Effects; no telemetry dependence. |
| **Prerequisites** | A2-code-signing |
| **Dependents** | A3-install-rollback |
| **Blocking conditions** | A2-code-signing incomplete |
| **Entry criteria** | `A2 Complete`; `Gate Spec satisfied`; `Owner authorizes` |
| **Exit criteria** | `Stable channel`; `integrity validation`; `honest update UX`; `failure recovery` |
| **Verification** | `Updater integration tests`; `manual update drill` |
| **Production readiness delta** | Major Release Ready unlock. |
| **Regression risks** | Broken update bricks install; Downgrade attacks |
| **Rollback strategy** | Disable updater endpoint; instruct manual A0 reinstall. |
| **Success metrics** | Update applies; Failed update recovers |

### Operational Acceptance

- [ ] A user can update without losing Moments
- [ ] A user can understand when an update fails
- [ ] A user can update without a developer workflow

## F2-signed-release

| Field | Value |
| --- | --- |
| **Gate family** | F |
| **Production classification** | BlockedByGate |
| **Purpose** | Automate signed artifact publication and verification. |
| **User value** | Indirect — trustworthy published builds. |
| **Engineering value** | Release workflow producing signed setup + notes. |
| **Constitutional justification** | Repository Standards / Production. |
| **Prerequisites** | A2-code-signing |
| **Dependents** | — |
| **Blocking conditions** | A2-code-signing incomplete |
| **Entry criteria** | `A2 Complete`; `Owner authorizes` |
| **Exit criteria** | `CI release job`; `signed artifacts`; `checksums`; `release notes template` |
| **Verification** | `Dry-run release pipeline` |
| **Production readiness delta** | Release Ready pipeline. |
| **Regression risks** | Secret leakage; Wrong channel publish |
| **Rollback strategy** | Disable publish job; yank bad release. |
| **Success metrics** | One-tag release produces signed setup |

### Operational Acceptance

- [ ] A release engineer can ship a signed build without tribal knowledge
- [ ] A user can obtain the intended channel artifact

## C2-shutdown-policy

| Field | Value |
| --- | --- |
| **Gate family** | C |
| **Production classification** | BlockedByGate |
| **Purpose** | Define close vs Exit vs tray-hide so shutdown matches user intent. |
| **User value** | Closing the window does not mysteriously kill or zombie Workspace. |
| **Engineering value** | Lifecycle policy coordinated with tray. |
| **Constitutional justification** | Presentation purity for chrome; Production lifecycle. |
| **Prerequisites** | E1-tray-lifecycle |
| **Dependents** | — |
| **Blocking conditions** | E1-tray-lifecycle incomplete |
| **Entry criteria** | `E1 Complete`; `Owner authorizes` |
| **Exit criteria** | `Documented + implemented close/Exit/tray semantics` |
| **Verification** | `Manual lifecycle matrix`; `verifier for policy constants` |
| **Production readiness delta** | Reliability/UX Production Ready. |
| **Regression risks** | Users lose work on unexpected Exit |
| **Rollback strategy** | Revert to explicit Exit-only policy. |
| **Success metrics** | Close≠crash; Exit terminates |

### Operational Acceptance

- [ ] A user can predict what Close does
- [ ] A user can fully quit Workspace when they intend to

## B3-crash-local

| Field | Value |
| --- | --- |
| **Gate family** | B |
| **Production classification** | OptionalPolish |
| **Purpose** | Capture local crash evidence for inclusion in support bundles. |
| **User value** | Hard crashes become diagnosable without reinventing the failure. |
| **Engineering value** | Local minidump/WER opt-in; attach to B1 bundle. |
| **Constitutional justification** | Observability; default no network telemetry. |
| **Prerequisites** | B1-diagnostics-support-bundle |
| **Dependents** | — |
| **Blocking conditions** | Priority below Ready Now critical path |
| **Entry criteria** | `B1 Complete`; `Owner prioritizes` |
| **Exit criteria** | `Local crash artifact`; `included in support export when present` |
| **Verification** | `Crash drill`; `bundle contains dump metadata` |
| **Production readiness delta** | Ops polish toward Production Ready. |
| **Regression risks** | Large dump disk use; Sensitive memory in dumps |
| **Rollback strategy** | Disable dump capture. |
| **Success metrics** | Crash produces local artifact |

### Operational Acceptance

- [ ] A user can export a support package that notes a crash occurred
- [ ] A user can keep crash dumps local with no network upload by default

## E2-polish

| Field | Value |
| --- | --- |
| **Gate family** | E |
| **Production classification** | OptionalPolish |
| **Purpose** | Improve animations, a11y, and micro-interactions without redesigning UI. |
| **User value** | Workspace feels calmer and more accessible. |
| **Engineering value** | Targeted Presentation polish. |
| **Constitutional justification** | Presentation; Product Gravity preserved. |
| **Prerequisites** | — |
| **Dependents** | — |
| **Blocking conditions** | Not on critical readiness path |
| **Entry criteria** | `Owner prioritizes polish over Ready Now units` |
| **Exit criteria** | `Scoped polish checklist done`; `no new chrome product` |
| **Verification** | `Manual a11y pass`; `visual regression as available` |
| **Production readiness delta** | Perceived quality only. |
| **Regression risks** | Motion sickness / reduced-motion ignored |
| **Rollback strategy** | Revert CSS/animation commits. |
| **Success metrics** | Keyboard paths work; reduced-motion respected |

### Operational Acceptance

- [ ] A user can operate primary flows by keyboard
- [ ] A user can use reduced motion without distracting animation

## A3-install-rollback

| Field | Value |
| --- | --- |
| **Gate family** | A |
| **Production classification** | BlockedByGate |
| **Purpose** | Recover from a failed upgrade to a known-good install. |
| **User value** | A bad update does not strand the user. |
| **Engineering value** | Rollback path tied to updater/OS previous version. |
| **Constitutional justification** | Reversibility / recoverability — Production. |
| **Prerequisites** | B2-auto-updater |
| **Dependents** | — |
| **Blocking conditions** | B2-auto-updater incomplete |
| **Entry criteria** | `B2 Complete`; `Owner authorizes` |
| **Exit criteria** | `Documented + tested rollback` |
| **Verification** | `Failed-upgrade drill` |
| **Production readiness delta** | Release Ready resilience. |
| **Regression risks** | Rollback data mismatch |
| **Rollback strategy** | Manual A0 reinstall from last good setup.exe. |
| **Success metrics** | Failed upgrade returns to prior version |

### Operational Acceptance

- [ ] A user can return to a working Workspace after a failed update
- [ ] A user can learn what to do if automatic rollback fails

## D2-permission-audit

| Field | Value |
| --- | --- |
| **Gate family** | D |
| **Production classification** | BlockedByGate |
| **Purpose** | Audit remaining IPC/capabilities for least privilege after quarantine. |
| **User value** | Fewer surprising permission prompts and tighter OS privilege use. |
| **Engineering value** | Permission matrix + capability review. |
| **Constitutional justification** | Explicit Authority; least privilege. |
| **Prerequisites** | D1-ipc-quarantine |
| **Dependents** | — |
| **Blocking conditions** | D1-ipc-quarantine incomplete |
| **Entry criteria** | `D1 Complete`; `Owner authorizes` |
| **Exit criteria** | `Audit doc`; `fixes for over-broad permissions` |
| **Verification** | `Audit checklist sign-off`; `targeted tests` |
| **Production readiness delta** | Security Production/Release hardening. |
| **Regression risks** | Over-tight permissions break Voice/OS features |
| **Rollback strategy** | Restore prior capability grants with documented exception. |
| **Success metrics** | No unjustified broad permissions remain |

### Operational Acceptance

- [ ] A user can complete supported workflows after permission tightening
- [ ] A user can see truthful guidance when OS permission is denied

