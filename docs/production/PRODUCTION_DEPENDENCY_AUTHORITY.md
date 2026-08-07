# Production Dependency Authority

| Field | Value |
| --- | --- |
| **Kind** | Canonical execution authority for Track A production engineering |
| **Status** | Binding for production programs under Sustainable Engineering Operations |
| **Program** | P16.PI3 |
| **Date** | 2026-08-07 |
| **Supersedes as sequencer** | Ad-hoc slice lists; informal “do updater next” |
| **Does not supersede** | Spec v2 · EES v1 · Product Proof Rule · Owner Product Proof gate |
| **Machine index** | `docs/production/production-gates-dependency.json` |
| **Readiness facts** | `docs/production/production-readiness.json` |

**Rule:** All remaining Track A production work MUST select the next unit from this authority’s **canonical execution order**, among units classified **Ready Now**, unless the Product Owner explicitly directs otherwise with evidence.

---

## 1. Three readiness levels (mandatory distinction)

| Level | Meaning | Green CI implies? |
| --- | --- | --- |
| **Engineering Complete** | Code exists; verifiers/tests pass for the unit | Yes — local/CI verification |
| **Production Ready** | Operationally complete for real operators (recover, diagnose, trust behaviour) | **No** |
| **Release Ready** | May be distributed to end users (signed, updateable, supportable at scale) | **No** |

Workspace today (Verified):

| Level | Status |
| --- | --- |
| Engineering Complete (many domains) | Often true |
| Production Ready | **False** — diagnostics/signing/tray/IPC still open |
| Release Ready | **False** — unsigned; no updater; incomplete release automation |

Never treat Engineering Complete as Release Ready.

---

## 2. Completed units (do not reopen except defects)

| ID | Gate | Status | Evidence |
| --- | --- | --- | --- |
| A0-installer-foundation | A | Engineering Complete | PI1; NSIS hooks; `verify:installer-foundation` |
| C0-single-instance | C | Engineering Complete → Production Ready (process) | PI2; `verify:single-instance` |

---

## 3. Remaining gate units — dependency graph

### Legend — actionability

| Class | Meaning |
| --- | --- |
| **Ready Now** | No external blocker; no unfinished prerequisite gate unit |
| **Blocked by external dependency** | Needs Owner/org artifact (e.g. code-signing certificate) |
| **Blocked by another production gate** | Prerequisite unit not closed |
| **Optional polish** | Improves feel; not on critical readiness path |
| **Release-only** | Needed for distribution pipeline; not required for Owner live Product Proof |

---

### A — Distribution

#### A1 — Artifact verification (checksums)

| Field | Value |
| --- | --- |
| Prerequisites | A0 |
| Dependents | F1, F2 (soft) |
| Blocking conditions | None |
| Exit criteria | SHA-256 sidecars for setup.exe; `verify:artifact-checksums` |
| Constitutional | Integrity / Observability of release artifacts |
| Engineering | Supply-chain hygiene; low risk |
| User benefit | Can verify download integrity |
| Readiness impact | Engineering → toward Release Ready |
| Class | **Ready Now** |
| Levels after close | Engineering Complete; not Release Ready alone |

#### A2 — Code signing (Authenticode)

| Field | Value |
| --- | --- |
| Prerequisites | A0; Owner certificate / thumbprint |
| Dependents | B2-updater, F2-signed-release, Release Ready |
| Blocking conditions | **External:** signing certificate |
| Exit criteria | Signed setup.exe; SmartScreen-trusted path documented; verifier detects signature when `WORKSPACE_REQUIRE_SIGNED=1` |
| Constitutional | Trust / No Hidden Authority over what runs |
| Engineering | Tauri windows signing config + CI secret |
| User benefit | “Windows trusts this install” |
| Readiness impact | Unlocks Release Ready |
| Class | **Blocked by external dependency** |

#### A3 — Install rollback

| Field | Value |
| --- | --- |
| Prerequisites | B2 (update rollback) or OS previous-version restore |
| Dependents | — |
| Class | **Blocked by another production gate** (B2) / FutureScale |
| Exit criteria | Documented + tested rollback of failed upgrade |

---

### B — Operations

#### B1 — Diagnostics & support bundle

| Field | Value |
| --- | --- |
| Prerequisites | None (A0/C0 helpful but not required) |
| Dependents | Owner support; soft dep for F release notes quality |
| Blocking conditions | None |
| Exit criteria | File logs + rotation; privacy-preserving support export; Conversation path; verifier |
| Constitutional | Observability; Evidence; privacy (no ambient telemetry) |
| Engineering | Production/Presentation; no new Information Owner |
| User benefit | “I can get help when something breaks without a developer” |
| Readiness impact | **Large** toward Production Ready |
| Class | **Ready Now** |

#### B2 — Auto-updater

| Field | Value |
| --- | --- |
| Prerequisites | **A2 signing** (integrity) |
| Dependents | A3 rollback; Release Ready |
| Blocking conditions | A2 |
| Exit criteria | Channel + signed manifest + update UX + failure recovery |
| Class | **Blocked by another production gate** (A2) |

#### B3 — Crash reporting (local)

| Field | Value |
| --- | --- |
| Prerequisites | B1 (bundle can include dumps) |
| Dependents | — |
| Class | **Optional polish** / FutureScale until B1 ships; no network telemetry by default |
| Exit criteria | Local minidump or WER opt-in; included in support bundle |

---

### C — Reliability (remainder)

#### C1 — Config & database validation UX

| Field | Value |
| --- | --- |
| Prerequisites | C0; existing checksum recovery (kernel) |
| Dependents | — |
| Class | **Ready Now** (lower priority than B1) |
| Exit criteria | Honest Conversation/UI when session/DB corrupt; no silent empty without notice |

#### C2 — Shutdown policy (tray-aware)

| Field | Value |
| --- | --- |
| Prerequisites | **E1 tray** for minimize-to-tray semantics |
| Class | **Blocked by another production gate** (E1) for full policy; explicit Exit already exists |

---

### D — Security

#### D1 — IPC quarantine

| Field | Value |
| --- | --- |
| Prerequisites | None |
| Dependents | D2; Release hardening |
| Blocking conditions | Regression risk on experimental surfaces |
| Exit criteria | Registered IPC reduced or hard-quarantined; product path green; surface-size risk lowered |
| Class | **Ready Now** (higher blast radius — after B1) |
| Constitutional | Least privilege / Singular entry |

#### D2 — Permission audit

| Field | Value |
| --- | --- |
| Prerequisites | D1 |
| Class | **Blocked by another production gate** (D1) |

---

### E — Experience

#### E1 — System tray lifecycle

| Field | Value |
| --- | --- |
| Prerequisites | C0 (done) |
| Dependents | C2 shutdown policy |
| Class | **Ready Now** |
| Exit criteria | Tray icon; Show Conversation; Exit; restore; no second product chrome |
| Note | Product Gravity — tray supports Conversation only |

#### E2 — Animations / a11y polish

| Field | Value |
| --- | --- |
| Class | **Optional polish** |
| Prerequisites | E1 helpful for focus states |

---

### F — Release

#### F1 — CI & artifact verification automation

| Field | Value |
| --- | --- |
| Prerequisites | A1 soft; A0 |
| Class | **Release-only** / Ready Now for CI branch coverage |
| Exit criteria | `v2-dev` CI; checksum job; verify:release in pipeline |

#### F2 — Signed release pipeline

| Field | Value |
| --- | --- |
| Prerequisites | **A2** |
| Class | **Blocked by another production gate** (A2) + Release-only |

---

## 4. Dependency graph (summary)

```
A0-installer ✓
C0-single-instance ✓
        │
        ├─► B1-diagnostics          [Ready Now]  ← highest unblocked Production Ready lift
        ├─► A1-artifact-checksums   [Ready Now]
        ├─► D1-ipc-quarantine       [Ready Now — careful]
        ├─► E1-tray                 [Ready Now]
        ├─► C1-config-db-ux         [Ready Now — lower]
        └─► F1-ci-automation        [Release-only]

A2-signing ──(external cert)──► B2-updater ► A3-rollback
                 └─────────────► F2-signed-release

E1-tray ► C2-shutdown-policy
D1-ipc ► D2-permission-audit
B1 ► B3-crash-local (optional)
```

---

## 5. Canonical execution order

Order minimizes risk, maximizes Production Ready before Release Ready:

| Order | Unit | Class | Why this order |
| --- | --- | --- | --- |
| 1 | **B1 — Diagnostics & support bundle** | Ready Now | Unblocked; largest Production Ready lift for trust/recoverability; enables Owner support during Product Proof |
| 2 | A1 — Artifact checksums | Ready Now | Cheap integrity; feeds F1 |
| 3 | D1 — IPC quarantine | Ready Now | Security before wide install; after B1 so support can diagnose fallout |
| 4 | E1 — Tray lifecycle | Ready Now | Presence/trust; unlocks C2 |
| 5 | C1 — Config/DB validation UX | Ready Now | Polish reliability honesty |
| 6 | F1 — CI automation | Release-only | Pipeline hygiene |
| 7 | **A2 — Signing** | External | When Owner cert available — unlocks Release Ready path |
| 8 | B2 — Updater | Blocked→Ready after A2 | |
| 9 | F2 — Signed release | After A2 | |
| 10 | C2 / B3 / E2 / A3 | As deps clear | Polish / FutureScale |

**Do not** jump to B2 updater while A2 is blocked.  
**Do not** treat F1 green as Release Ready.

---

## 6. Product Proof relationship

| Gate | Required before Owner Product Proof? |
| --- | --- |
| Owner live PP | **Yes** — product gate (unchanged) |
| B1 diagnostics | Strongly helpful; not a substitute for PP |
| A2 signing | **No** for eng-assisted PP (`tauri dev`) |
| B2 updater | **No** for PP |

---

## 7. PI3 execution decision

| Question | Result |
| --- | --- |
| Immediately actionable unblocked unit? | **Yes — B1 Diagnostics & support bundle** |
| Implement in PI3 after authority? | **Yes — B1 only** (see `P16_PI3_GATE_B1_DIAGNOSTICS.md`) |
| Stop after B1? | **Yes** — await Owner review; next Ready Now = **A1** |

---

## 8. Forbidden drifts

- Redesigning Spec or governance to “enable” production  
- Beginning P17  
- Implementing updater before signing  
- Claiming Release Ready from Engineering Complete  
- Batching multiple gate units in one program without Owner approval  
