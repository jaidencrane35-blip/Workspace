# A2 / F2 — Release Execution Playbook

| Field | Value |
| --- | --- |
| **Program** | Stage 3 Release Execution Playbook |
| **ID** | A2/F2 Playbook *(operational — not an implementation sprint)* |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Documentation only — **no signing, updater, runtime, or UX implementation** |
| **Prerequisites complete** | F1 Release Pipeline Hardening · R1 Readiness |
| **Authorities** | Dependency Authority · Gate Catalog · `RELEASE_PIPELINE.md` · R1 · Spec v2.1 · EES v1 |
| **Hard gate** | Authenticode certificate (EXTERNAL) |

**Do not execute A2/F2/B2 engineering until the certificate is available and Owner authorizes the workstream.**

**Repository posture:** **Release Hold** (`docs/production/R2_RELEASE_HOLD.md`). This playbook is dormant until resume trigger T3 (certificate + Owner auth) or another T1–T5 event.

---

## 1. Executive Summary

F1 made the **unsigned** release path deterministic (`ci-pr` + `ci-release` → setup.exe + `.sha256` + manifest). The remaining Stage 3 sequence that produces a **Production Release** is externally gated by an Authenticode certificate:

```text
Certificate in hand
    → A2  Sign the installer + verify path
    → F2  Publish signed artifacts repeatably
    → B2  Auto-updater (signed manifests) — unless Owner waives
```

This playbook is the **authoritative execution order** once the certificate exists. It defines entry/exit criteria, owners, artifacts, approvals, rollback, and failure recovery for A2, F2, and B2. It does **not** implement those gates.

**Until the certificate arrives:** no signing work; keep F1 green; Owner continues Product Proof / OA / cert procurement.

---

## 2. Stage Diagram

```text
F1 COMPLETE (unsigned)
  ci-pr · ci-release · verify:release · NSIS · sha256 · manifest
        |
        v
EXTERNAL: Authenticode certificate available
        |
        v
OWNER: authorize A2
        |
        v
A2 — Code Signing
  sign + timestamp + WORKSPACE_REQUIRE_SIGNED + secret docs
  → signed Workspace_*_x64-setup.exe
        |
        v
OWNER: authorize F2
        |
        v
F2 — Signed Release Publish
  tag → signed build → checksums → notes → GitHub Release / host
        |
        v
OWNER: channel policy + B2 authorize  OR  updater waiver
        |
        v
B2 — Auto-Updater (if not waived)
  plugin · endpoint · signed manifests · honest failure
        |
        v
Production Release (see §9)
```

**Forbidden order:** B2 or F2 before A2 Complete.  
**Parallel (not on this critical path):** D1 IPC quarantine; B3 crash-local; UX items — do not reopen P17–P20.

---

## Cross-cutting: roles

| Role | Responsibility |
| --- | --- |
| **Owner** | Product Proof Accept; authorize each workstream; procure/store cert; channel policy; OA checkboxes; ship/waiver decisions |
| **Release Engineer (Engineering)** | Execute A2/F2/B2 per this playbook; keep F1 green; never commit secrets |
| **CI** | Run `ci-pr` / extended `ci-release` (signed after A2); fail closed on verify |
| **External** | Certificate vendor / timestamp authority / update hosting |

---

## Cross-cutting: artifact flow (target state after A2/F2)

```text
Source (v2-dev or release tag)
  → pnpm verify:release
  → pnpm installer:build
  → Authenticode sign + timestamp          [A2]
  → pnpm checksums:generate
  → pnpm release:manifest (signed: true)
  → Publish setup.exe + .sha256 + notes    [F2]
  → (optional) Publish updater JSON        [B2]
```

**Current F1 artifact names (preserve):**

- `Workspace_<version>_x64-setup.exe`
- `Workspace_<version>_x64-setup.exe.sha256`
- `Workspace_<version>_release-manifest.json`

---

## 3. A2 Execution — Code Signing

### Purpose
Authenticode-sign the NSIS installer so Windows and users can trust the binary; unlock F2 and B2.

### Entry criteria
| # | Criterion |
| --- | --- |
| A2-E1 | F1 Complete and green on `v2-dev` |
| A2-E2 | Authenticode certificate (or org equivalent) **available to Release Engineer** |
| A2-E3 | Owner authorizes A2 workstream |
| A2-E4 | Signing secret storage decided (CI secret / local HSM / file outside repo) |
| A2-E5 | Timestamp server reachable from sign environment |

### Exit criteria
| # | Criterion |
| --- | --- |
| A2-X1 | Signed `Workspace_*_x64-setup.exe` produced |
| A2-X2 | Signature verify path documented and automated (`signtool` / equivalent) |
| A2-X3 | Thumbprint / CI secret injection documented (never committed) |
| A2-X4 | `WORKSPACE_REQUIRE_SIGNED=1` gate implemented and enforced in release verify |
| A2-X5 | Gate catalog: A2 → **Complete**; OA path ready for Owner |

### Blocking conditions
- Certificate unavailable, expired, or not trusted for code signing  
- Owner has not authorized A2  
- Inability to timestamp (treat as hard fail for ship builds)

### Inputs
| Input | Source |
| --- | --- |
| Unsigned setup.exe | F1 `installer:build` / `ci-release` |
| Certificate + private key / cloud sign identity | Owner / External (outside git) |
| Timestamp URL | Org standard or public TSA |
| Version metadata | `verify:version-consistency` |

### Outputs / expected artifacts
| Artifact | Notes |
| --- | --- |
| Signed `Workspace_*_x64-setup.exe` | Same naming; Authenticode embedded |
| Verify log / CI step output | Pass/fail of signature + timestamp |
| Signing runbook section (docs) | Thumbprint location, secret names, rotate procedure |
| Optional: dual unsigned retained only in internal CI | Never published as Production Release |

### Validation
| Check | Owner |
| --- | --- |
| `signtool verify /pa` (or documented equivalent) succeeds | Engineering |
| Publisher name matches intended org | Owner |
| `WORKSPACE_REQUIRE_SIGNED=1` fails unsigned artifacts | Engineering |
| F1 unsigned path still builds for internal smoke | Engineering |

### CI responsibilities (post-implementation)
- Load signing secret from GitHub Environment (restricted)  
- Sign after `installer:build`, before checksums  
- Fail job if verify fails or timestamp fails  
- Never echo secrets  

### Developer / Release Engineer responsibilities
- Never commit `.pfx`, passwords, or thumbprints into the repo  
- Run local sign drill once before enabling CI sign  
- Update gate JSON + readiness evidence when Complete  

### Manual approvals
| Approval | When |
| --- | --- |
| Owner authorize A2 | Before engineering starts |
| Owner accept first signed smoke install | Before declaring A2 Complete for ship |

### Rollback points
| Point | Action |
| --- | --- |
| Sign step fails | Keep prior unsigned F1 artifacts internal; do not publish |
| Bad cert / wrong publisher | Revoke publish; re-sign with correct cert; do not ship |
| Owner rejects OA | Remain Engineering Complete only; no Production Release claim |

### Responsible owner
| Phase | Owner |
| --- | --- |
| Cert procurement | **Owner / External** |
| Integration + verify gate | **Engineering** |
| Accept first signed binary | **Owner** |

---

## 4. F2 Execution — Signed Release Publish

### Purpose
Automate **signed** artifact publication so a release engineer can ship trustworthy builds without tribal knowledge.

### Entry criteria
| # | Criterion |
| --- | --- |
| F2-E1 | A2 Complete |
| F2-E2 | Owner authorizes F2 |
| F2-E3 | Publish destination decided (GitHub Releases and/or org host) |
| F2-E4 | Tagging policy agreed (e.g. `vMAJOR.MINOR.PATCH` from version sync) |

### Exit criteria
| # | Criterion |
| --- | --- |
| F2-X1 | One command or CI workflow: tag → signed setup + `.sha256` + notes |
| F2-X2 | Published artifacts downloadable; checksums match |
| F2-X3 | Signature verify on published setup.exe succeeds |
| F2-X4 | Release notes template filled (version, git SHA, unsigned=false) |
| F2-X5 | Gate catalog: F2 → **Complete** |

### Blocking conditions
- A2 incomplete  
- Publish credentials missing  
- Version drift (`verify:version-consistency` red)

### Inputs
| Input | Source |
| --- | --- |
| Signed setup.exe | A2 |
| Version + git SHA | repo / manifest |
| Release notes | Engineering draft; Owner may edit |

### Outputs / expected artifacts
| Artifact | Notes |
| --- | --- |
| Git tag `v*` | Matches package version |
| Published `Workspace_*_x64-setup.exe` | Signed |
| Published `.sha256` | Regenerated after sign |
| Published release notes | Human-readable |
| Updated release-manifest (`signed: true`) | From F1 emitter, extended |

### Validation
| Check | Owner |
| --- | --- |
| Download published setup → verify signature | Engineering |
| `sha256sum -c` / Windows equivalent on sidecar | Engineering |
| Fresh Windows VM install smoke | Engineering + Owner OA |
| `ci-pr` still green on release branch | CI |

### CI responsibilities
- Extend `ci-release` (or sibling) for **signed** path after A2  
- Attach artifacts to GitHub Release on tag  
- Fail if unsigned when `WORKSPACE_REQUIRE_SIGNED=1`  

### Developer responsibilities
- Bump version in lockstep before tag (`verify:version-consistency`)  
- Run `pnpm verify:release` locally or rely on CI  
- Do not manually overwrite published artifacts without Owner approval  

### Manual approvals
| Approval | When |
| --- | --- |
| Owner authorize F2 | Before publish automation ships |
| Owner approve first public/channel publish | Before announcing download |

### Rollback points
| Point | Action |
| --- | --- |
| Bad publish | Unlist/delete Release; yank download links; publish corrected signed build with new patch version if needed |
| Tag mistake | Do not reuse tag for different bits; cut `vX.Y.Z+1` |

### Responsible owner
| Phase | Owner |
| --- | --- |
| Workflow implementation | **Engineering** |
| Publish authorization | **Owner** |
| Hosting account | **Owner / Engineering** |

---

## 5. B2 Execution — Auto-Updater

### Purpose
Trusted in-app updates without full reinstall, using **signed** update artifacts and honest failure UX.

### Entry criteria
| # | Criterion |
| --- | --- |
| B2-E1 | A2 Complete (Dependency Authority — non-negotiable) |
| B2-E2 | F2 Complete recommended (publish path for update payloads) |
| B2-E3 | Owner authorizes B2 **or** issues written updater waiver for installer-only release |
| B2-E4 | Update channel policy (stable / beta) decided by Owner |
| B2-E5 | HTTPS update endpoint / storage available |

### Exit criteria
| # | Criterion |
| --- | --- |
| B2-X1 | Updater integrated behind Workspace contracts (no provider-to-provider) |
| B2-X2 | Update manifests integrity-validated (signed) |
| B2-X3 | Honest failure + recovery (no silent brick) |
| B2-X4 | Manual update drill passed (N → N+1) |
| B2-X5 | Moments/data preserved across successful update |
| B2-X6 | Gate catalog: B2 → **Complete** (or waiver filed) |

### Blocking conditions
- A2 incomplete  
- Owner neither authorizes nor waives  
- Unsigned update channel proposed (reject)

### Inputs
| Input | Source |
| --- | --- |
| Signed installers / update bundles | A2/F2 |
| Channel endpoints | Owner / hosting |
| Product copy for update states | Interaction Language |

### Outputs / expected artifacts
| Artifact | Notes |
| --- | --- |
| Updater config in Tauri (future impl) | Not present today — playbook only |
| Signed update manifest(s) | Per channel |
| Integration tests + drill record | Engineering |
| Waiver doc (if no B2) | Owner-signed |

### Validation
| Check | Owner |
| --- | --- |
| Update applies on test machine | Engineering |
| Failed update recovers / instructs reinstall | Engineering |
| Downgrade / tamper rejected | Engineering |
| OA: user can update without developer workflow | Owner |

### CI responsibilities
- Publish update manifests only from signed F2 pipeline  
- Never publish updater JSON for unsigned builds  

### Developer responsibilities
- Preserve Presentation Purity (no policy in UI chrome beyond honest copy)  
- Do not start B2 implementation before A2  

### Manual approvals
| Approval | When |
| --- | --- |
| Owner channel + B2 authorize **or** waiver | Before B2 eng or before Production Release without updater |
| Owner OA on update drill | Before B2 Complete |

### Rollback points
| Point | Action |
| --- | --- |
| Bad update channel | Disable endpoint; instruct manual F2 reinstall |
| Broken updater in field | Disable updater via endpoint; ship fixed signed installer via F2 |

### Responsible owner
| Phase | Owner |
| --- | --- |
| Channel policy / waive | **Owner** |
| Implementation | **Engineering** |
| Hosting | **Owner / Engineering** |

### Updater prerequisites (checklist before B2 eng starts)

- [ ] A2 Complete  
- [ ] F2 publish path proven once  
- [ ] Channel URL(s) reserved  
- [ ] Signing key for update manifests decided (same or subordinate to Authenticode policy)  
- [ ] Failure copy reviewed against Interaction Language  
- [ ] Owner authorize or waive  

---

## 6. Rollback Procedure

### Principle
Prefer **forward repair with a new patch version** over rewriting published bits. Never ship unsigned as Production Release without explicit Owner risk acceptance (A2 catalog rollback — exceptional only).

### Procedure matrix

| Situation | Immediate action | Follow-up |
| --- | --- | --- |
| Sign failure in CI | Fail closed; no publish | Fix cert/timestamp; retry |
| Published wrong signed build | Unlist Release; notify Owner | Publish `vX.Y.Z+1` signed |
| Updater serves bad payload | Disable update endpoint | Fix + re-sign + re-enable |
| Certificate compromise | Stop publish; revoke cert with vendor | Re-issue cert; re-sign latest; Owner comms |
| F1 regression (unsigned CI red) | Block merges to release branch | Fix before any A2/F2 run |
| Owner Rejects Product Proof | Pause Production Release claims | Return to Stage 2; do not treat F2 as product Accept |

### Rollback points on the critical path

1. **Before A2 start** — remain on F1 unsigned internal only.  
2. **After A2, before F2 publish** — signed bits exist locally/CI; not public.  
3. **After F2 publish** — unlist + patch version.  
4. **After B2 enable** — disable endpoint + F2 manual installer path.

---

## 7. Failure Matrix

| Failure | Detected by | Severity | Recovery |
| --- | --- | --- | --- |
| Version drift | `verify:version-consistency` | High | Fix manifests; re-run F1 |
| Unsigned artifact in signed job | `WORKSPACE_REQUIRE_SIGNED` (A2 exit) | Critical | Fail CI; do not publish |
| Timestamp server down | Sign step | High | Retry; alternate TSA if org-approved |
| Checksum mismatch after download | `.sha256` verify | Critical | Yank publish; rebuild |
| Invalid Authenticode on download | `signtool verify` | Critical | Yank; investigate supply chain |
| `ci-release` red | GitHub Actions | High | Fix before tag |
| Update fails mid-apply | B2 UX + logs | Critical | Honest failure; manual F2 install |
| SmartScreen still blocks | User report | High | Reputation/cert class; Owner/External |
| Secret leaked | Audit / scan | Critical | Rotate cert/secrets; revoke |
| Tag/version mismatch | Release checklist | High | Do not publish; retag correctly |

---

## 8. Release Checklist

### Pre-flight (every Production Release candidate)

- [ ] Owner Product Proof **Accepted** (or documented Accept-with-changes closed)  
- [ ] F1 green on release commit  
- [ ] `pnpm verify:version-consistency` → intended version  
- [ ] `pnpm verify:release` green  
- [ ] A2 Complete (signed verify path green)  
- [ ] F2 publish dry-run succeeded once on prior tag (or this is the dry-run)  
- [ ] B2 Complete **or** Owner updater waiver on file  
- [ ] OA boxes for A0/A1/B1/E1/F1 (and A2/F2/B2 as applicable) reviewed by Owner  
- [ ] No open Ship Blockers (data loss, security, startup, broken installer/updater)  
- [ ] Release notes drafted  

### Execute (signed path — after A2/F2 implemented)

- [ ] Owner authorize this version ship  
- [ ] Create git tag `v<version>` on release commit  
- [ ] CI signed pipeline produces setup.exe + `.sha256` + manifest (`signed: true`)  
- [ ] Verify signature on artifact  
- [ ] Verify checksum  
- [ ] Publish via F2 destination  
- [ ] Fresh-machine install smoke  
- [ ] Support bundle export smoke  
- [ ] Tray Show / Exit smoke  
- [ ] If B2: update drill N → N+1  
- [ ] Owner final OK to announce  

### Post-release

- [ ] Tag immutable; notes published  
- [ ] Monitor install/update failures  
- [ ] File any Ship Blockers immediately; defer polish  

---

## 9. Definition of Production Release

A **Production Release** is a build that may be distributed to real Owners as Workspace software. It requires:

1. **Stage 2:** Owner Product Proof Accepted (vision/feel).  
2. **A2 Complete:** Authenticode-signed installer with verify path.  
3. **F2 Complete:** Repeatable signed publish of setup.exe + `.sha256` + notes.  
4. **B2 Complete** **or** Owner **written waiver** for installer-only distribution.  
5. **Operational Acceptance** for shipped production units checked by Owner.  
6. **No open Ship Blockers** on the supported Windows path.

**Not a Production Release:**

- F1 unsigned `ci-release` artifacts  
- Local `pnpm installer:build` without sign  
- Engineering Complete on P17–P20 alone  
- Checksums without Authenticode  

**Related terms (Dependency Authority):**

| Term | Meaning |
| --- | --- |
| Engineering Complete | Workstream finished + verifiers; may lack OA/sign |
| Production Ready | Eng + OA for that unit’s trust claims |
| Release Ready | Distribution confidence (A2/F2/B2-or-waiver + Accept) |
| Production Release | The act of shipping a Release Ready build to Owners |

---

## 10. Remaining external dependencies

| Dependency | Blocks | Owner |
| --- | --- | --- |
| **Authenticode certificate** | A2, F2, B2, Production Release | Owner / External |
| Windows timestamp authority | Durable A2 signatures | Engineering ops / External |
| CI signing secret storage (GitHub Environment / HSM) | Automated A2/F2 | Owner + Engineering |
| Update artifact hosting (HTTPS) | B2 | Owner / Engineering |
| Owner Product Proof Accept | Production Release product gate | Owner |
| Owner channel policy or B2 waiver | B2 vs installer-only ship | Owner |
| Certificate vendor revocation/renewal process | Ongoing ship ability | Owner / External |

---

## Stage summary cards

### A2 — Code Signing

| Field | Value |
| --- | --- |
| **Entry** | Cert available; Owner authorize; F1 green |
| **Exit** | Signed setup; verify path; secret docs; `WORKSPACE_REQUIRE_SIGNED` |
| **Blocking** | No cert; no Owner auth |
| **Validation** | signtool verify; require-signed gate |
| **Artifacts** | Signed `Workspace_*_x64-setup.exe` |
| **Owner** | Cert: Owner/External · Impl: Engineering |

### F2 — Signed Release

| Field | Value |
| --- | --- |
| **Entry** | A2 Complete; Owner authorize; publish target |
| **Exit** | Tag → signed publish + checksums + notes |
| **Blocking** | A2 incomplete |
| **Validation** | Download verify signature + sha256; install smoke |
| **Artifacts** | GitHub Release (or host) assets |
| **Owner** | Impl: Engineering · Publish auth: Owner |

### B2 — Auto-Updater

| Field | Value |
| --- | --- |
| **Entry** | A2 Complete; Owner authorize **or** waive; channel |
| **Exit** | Signed updates + honest failure + drill **or** waiver |
| **Blocking** | A2 incomplete; no auth/waiver |
| **Validation** | N→N+1 drill; tamper reject |
| **Artifacts** | Update manifests + bundles |
| **Owner** | Policy: Owner · Impl: Engineering |

---

## Compliance

| Constraint | Posture |
| --- | --- |
| Do not implement signing | Observed (playbook only) |
| Do not implement updater | Observed |
| Do not modify runtime / UX | Observed |
| Dependency Authority A2 before B2/F2 | Codified |
| F1 remains unsigned engineering path | Preserved |

---

## Stop

Playbook complete. **Do not begin A2 implementation until the Authenticode certificate is available and Owner authorizes.**  
Await certificate + Owner go-ahead.
