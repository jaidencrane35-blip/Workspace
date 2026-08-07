# Production Dependency Authority

| Field | Value |
| --- | --- |
| **Kind** | Canonical execution authority for Track A production engineering |
| **Status** | Binding for production programs under Sustainable Engineering Operations |
| **Program** | P16.PI3 (order) · P16.PI4 (gate schema) |
| **Date** | 2026-08-07 |
| **Supersedes as sequencer** | Ad-hoc slice lists; informal “do updater next” |
| **Does not supersede** | Spec v2 · EES v1 · Product Proof Rule · Owner Product Proof gate |
| **Gate schema (mandatory)** | `docs/production/PRODUCTION_GATE_SPECIFICATION.md` |
| **Machine index** | `docs/production/production-gates-dependency.json` |
| **Human catalog** | `docs/production/PRODUCTION_GATE_CATALOG.md` (generated) |
| **Readiness facts** | `docs/production/production-readiness.json` |

**Rule:** All remaining Track A production work MUST select the next unit from this authority’s **canonical execution order**, among units classified **ReadyNow**, unless the Product Owner explicitly directs otherwise with evidence.

**Schema rule (PI4):** A unit MUST NOT be implemented until its catalog entry satisfies `PRODUCTION_GATE_SPECIFICATION.md` (including Operational Acceptance). Canonical **order is not redesigned** by the Gate Spec.

---

## 1. Three readiness levels (mandatory distinction)

| Level | Meaning | Green CI implies? |
| --- | --- | --- |
| **Engineering Complete** | Code exists; verifiers/tests pass for the unit | Yes — local/CI verification |
| **Production Ready** | Operational Acceptance satisfied for the unit’s scope | **No** |
| **Release Ready** | May be distributed to end users (signed, updateable, supportable at scale) | **No** |

Workspace today (Verified):

| Level | Status |
| --- | --- |
| Engineering Complete (many domains) | Often true |
| Production Ready | **False** — signing/tray/IPC still open; Operational Acceptance pending Owner |
| Release Ready | **False** — unsigned; no updater; incomplete release automation |

Never treat Engineering Complete as Release Ready.  
Never treat verifier green as Operational Acceptance.

---

## 2. Completed units (do not reopen except defects)

| ID | Evidence |
| --- | --- |
| A0-installer-foundation | PI1; `verify:installer-foundation` |
| C0-single-instance | PI2; `verify:single-instance` |
| B1-diagnostics-support-bundle | PI3; `verify:support-bundle` |

Full schema (including Operational Acceptance checklists): catalog.

---

## 3. Gate units (normalized index)

**Identical schema for every unit:** `PRODUCTION_GATE_CATALOG.md`  
**Machine source:** `production-gates-dependency.json` (schemaVersion 2)

| Class | Meaning |
| --- | --- |
| **ReadyNow** | No external blocker; prerequisites Complete |
| **BlockedExternal** | Needs Owner/org artifact (e.g. certificate) |
| **BlockedByGate** | Prerequisite unit not closed |
| **OptionalPolish** | Not on critical readiness path |
| **ReleaseOnly** | Distribution pipeline |
| **Complete** | Unit closed |

| ID | Class | Purpose (one line) |
| --- | --- | --- |
| A0 | Complete | NSIS installer foundation |
| C0 | Complete | Single-instance process integrity |
| B1 | Complete | Diagnostics + support bundle |
| A1 | Complete | Artifact checksums |
| D1 | ReadyNow | IPC quarantine |
| E1 | ReadyNow | Tray lifecycle |
| C1 | ReadyNow | Config/DB validation UX |
| F1 | ReleaseOnly | CI automation |
| A2 | BlockedExternal | Code signing |
| B2 | BlockedByGate | Auto-updater |
| F2 | BlockedByGate | Signed release pipeline |
| C2 | BlockedByGate | Shutdown policy |
| D2 | BlockedByGate | Permission audit |
| B3 | OptionalPolish | Local crash capture |
| E2 | OptionalPolish | UX polish |
| A3 | BlockedByGate | Install rollback |

---

## 4. Dependency graph (summary)

```
A0-installer ✓
C0-single-instance ✓
B1-diagnostics ✓
        │
        ├─► A1-artifact-checksums   [Complete]
        ├─► D1-ipc-quarantine       [ReadyNow]  ← next
        ├─► E1-tray                 [ReadyNow]
        ├─► C1-config-db-ux         [ReadyNow]
        └─► F1-ci-automation        [ReleaseOnly]

A2-signing ──(external cert)──► B2-updater ► A3-rollback
                 └─────────────► F2-signed-release

E1-tray ► C2-shutdown-policy
D1-ipc ► D2-permission-audit
B1 ► B3-crash-local (optional)
```

---

## 5. Canonical execution order

| Order | Unit | Class |
| --- | --- | --- |
| 1 | B1 — Diagnostics & support bundle | Complete |
| 2 | A1 — Artifact checksums | Complete |
| 3 | D1 — IPC quarantine | ReadyNow |
| 4 | E1 — Tray lifecycle | ReadyNow |
| 5 | C1 — Config/DB validation UX | ReadyNow |
| 6 | F1 — CI automation | ReleaseOnly |
| 7 | A2 — Signing | BlockedExternal |
| 8 | B2 — Updater | BlockedByGate (A2) |
| 9 | F2 — Signed release | BlockedByGate (A2) |
| 10 | C2 / B3 / E2 / A3 | As deps clear |

**Do not** jump to B2 updater while A2 is blocked.  
**Do not** treat F1 green as Release Ready.

---

## 6. Product Proof relationship

| Item | Required before Owner Product Proof? |
| --- | --- |
| Owner live PP | **Yes** — product gate |
| A2 signing | **No** for eng-assisted PP |
| B2 updater | **No** for PP |
| Operational Acceptance on production units | Strengthens trust; does not replace Product Proof |

---

## 7. Program decisions

| Program | Result |
| --- | --- |
| PI3 | Authority + B1 implemented |
| PI4 | Gate Specification + full schema normalize; **no gate implemented** |
| PF1 | Premium Finish decision: production trust > UX polish; **A1 implemented** |
| Next implementation | D1 — IPC quarantine (await Owner) |

---

## 8. Forbidden drifts

- Redesigning Spec or governance to “enable” production  
- Beginning P17  
- Implementing updater before signing  
- Claiming Release Ready from Engineering Complete  
- Claiming Production Ready from verifier green without Operational Acceptance  
- Batching multiple gate units without Owner approval  
- Inventing further meta-frameworks beyond Gate Spec + this Authority  
