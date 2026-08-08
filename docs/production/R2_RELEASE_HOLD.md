# R2 — Engineering Standby & Release Lock

| Field | Value |
| --- | --- |
| **Program** | Engineering Standby & Release Lock |
| **ID** | R2 |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Documentation / repository lock — **no implementation** |
| **Engineering status** | **Release Hold** |
| **Current phase** | **Stage 2 — Owner Acceptance** |
| **Next engineering event** | **A2 (Code Signing)** after Owner Acceptance **and** Authenticode certificate availability |
| **Priority while held** | Maintain repository stability — do not begin new engineering work |

---

## 1. Release Hold confirmation

Workspace has completed the engineering arcs required to pause:

| Arc | Status |
| --- | --- |
| Constitutional Specification / Operations | Stable |
| Product Proof Engineering (P17–P20) | Engineering Complete — **do not reopen** |
| Release Readiness (R1) | Complete |
| Release Pipeline Hardening (F1) | Complete |
| A2/F2 Release Execution Playbook | Complete (docs only) |

**No active optimization or release-implementation program remains open.**  
Engineering resumes **only** on a documented trigger (§4).

---

## 2. Current truth (authoritative)

| Dimension | Value |
| --- | --- |
| **Current phase** | Stage 2 — Owner Acceptance |
| **Handoff status** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |
| **Engineering status** | Release Hold |
| **Unsigned pipeline** | F1 Complete (`RELEASE_PIPELINE.md`) |
| **Signed ship path** | Playbook ready; A2 **BlockedExternal** (certificate) |
| **Canonical security nextReadyNow** | `D1-ipc-quarantine` *(not a Release Hold resume trigger by itself)* |
| **File Provider / capability expansion** | Blocked until Owner Voice Product Proof Accept |
| **Product UX optimization** | **Stopped** — no new program by default |

---

## 3. Dependency & gate posture

| Gate / item | Classification | Hold implication |
| --- | --- | --- |
| F1-ci-automation | Complete | Do not reopen |
| A2-code-signing | BlockedExternal | Resume only on Trigger 3 (+ Owner auth) |
| F2-signed-release | BlockedByGate ← A2 | After A2 |
| B2-auto-updater | BlockedByGate ← A2 | After A2 (+ channel/waiver) |
| D1-ipc-quarantine | ReadyNow (`nextReadyNow`) | **Not** an automatic resume trigger under Release Hold |
| Owner Voice Product Proof | **Accepted with changes**; T1 Companion Conversation complete | Stage 2 gate — await Owner re-review |
| productionReadyToday / releaseReady | false | Unchanged |

Playbook authority for post-hold sequence: `docs/production/A2_F2_RELEASE_EXECUTION_PLAYBOOK.md`.

---

## 4. Engineering resume trigger table

**Only** these events may resume engineering. No other program may start by default.

| ID | Trigger | Resume action |
| --- | --- | --- |
| **T1** | Owner Product Proof outcome: **Accepted with changes** | One bounded engineering slice addressing Owner changes only — **fired 2026-08-08**; report: `docs/capability-runtime/product-proof/T1_COMPANION_CONVERSATION_BEHAVIOR.md` |
| **T2** | Owner Product Proof **Rejected** | One scoped Product Proof audit (identify single highest-impact issue) — **fired 2026-08-08**; audit: `docs/capability-runtime/product-proof/T2_PRODUCT_PROOF_REJECTION_AUDIT.md` |
| **T3** | Authenticode certificate available (+ Owner authorize A2) | Begin **A2** per playbook |
| **T4** | Verified **release blocker** (ship-blocking defect) | One bounded release fix |
| **T5** | Post-release product evolution **explicitly approved** by Owner | Begin next capability program under Spec/EES |

**Explicit non-triggers (do not resume):**

- Desire for more UX polish  
- Canonical `nextReadyNow` = D1 alone  
- Curiosity / roadmap momentum  
- Partial gate completeness without Owner/cert  

---

## 5. Consistency map

| Document | Must reflect |
| --- | --- |
| `ENGINEERING_HANDOFF.md` | Release Hold · Stage 2 · next A2 after Accept+cert |
| `engineering-milestone-report.md` | R2 Release Hold latest |
| `project-health.json` (via sync) | Hold note; PP pending |
| `RELEASE_PIPELINE.md` | F1 Complete; hold until A2 trigger |
| `A2_F2_RELEASE_EXECUTION_PLAYBOOK.md` | Execution after cert — not active impl |
| `production-gates-dependency.json` | F1 Complete; A2 BlockedExternal |
| `production-readiness.json` | productionReadyToday false; F1 in completedGates |

Verifier: `pnpm verify:release-hold` / `scripts/verify-release-hold.mjs`.

---

## 6. Files modified (this program)

| File | Change |
| --- | --- |
| `docs/production/R2_RELEASE_HOLD.md` | This authority |
| `docs/project/ENGINEERING_HANDOFF.md` | Release Hold fields |
| `docs/engineering-milestone-report.md` | R2 stamped |
| `docs/production/RELEASE_PIPELINE.md` | Hold pointer |
| `scripts/verify-release-hold.mjs` | Consistency verifier |
| `package.json` / `sync-project-health.mjs` | Wire verifier + health note |

**Unchanged:** Runtime, UX, Conversation, Moments, signing, updater, architecture.

---

## 7. Stop

Release Hold is authoritative. **Do not begin the next engineering program** until T1–T5.
