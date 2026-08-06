# Architectural Blind Spots & Independent Critique

| Field | Value |
| --- | --- |
| **Purpose** | Phases 5–6 — critique both systems; migration opportunities; unknowns |
| **Date** | 2026-08-07 |
| **Stance** | Neither system assumed superior overall |

---

## 1. Workspace strengths

| Strength | Evidence |
| --- | --- |
| Clear product wedge with honesty constraints | Constitution V2; PP evidence; restore limits |
| Structural permission sequence | PermissionGateway; StandardPermissionGate |
| Hard OS crate boundary | windows-integration |
| Local SQLite product schema for Moments/session | database migrations 041–046 |
| Dense in-crate contract tests | kernel test volume |
| Now has Constitution V2 + knowledge maps | this programme |

---

## 2. Workspace weaknesses

| Weakness | Evidence |
| --- | --- |
| Accidental complexity: large IPC vs small mounted chrome | ipc-surface-map |
| Hand-duplicated TS domain contracts | domain-contract-assessment |
| Agent-tool security thinner than Crew’s PreToolUse stack | governance-map vs security-deep-dive |
| Audit not tamper-evident (no HMAC chain) | AuditService vs sel.py |
| Dual documentation authority historically | repository-audit drift |
| Experimental cognition surface gravity | strategic review |
| Encryption Tier 0 only | NoOpEncryptionProvider |
| Multi-monitor gate FAIL | baseline 32 |
| Plugin host unimplemented but actor type exists | plugins placeholder |
| AI workers / process isolation aspirational (DEC-011) | SYSTEM-OVERVIEW vs implementation |

---

## 3. Kiro / Kiro Crew strengths

| Strength | Evidence |
| --- | --- |
| Mature agent orchestration + multi-session | overview.md |
| Authorization ceiling outside agent config | security-deep-dive |
| Long-running task hygiene | resource-protection.md |
| Local memory + lessons accumulation | memory.py, learn.py |
| Tamper-evident SEL | sel.py |
| Strong architecture/system-spec documentation | docs/architecture |
| Open governance + tenets | GOVERNANCE.md, TENETS.md |
| Protocol split (ACP) | overview.md |

---

## 4. Kiro / Kiro Crew weaknesses / risks

| Weakness | Evidence |
| --- | --- |
| IDE/CLI core not open in `kirodotdev/Kiro` — limits verifiability of full stack | raw-evidence |
| Hard dependency on external kiro-cli binary/runtime | overview.md |
| Unattended autonomy increases blast radius if gates fail | design goal + threat model |
| Very large Python surface area (ops complexity) | src/kiro_crew breadth |
| Sandbox default nuances (`off` vs `auto`) easy to misconfigure | security-deep-dive |
| Trademark/code split may confuse “fully open” marketing | GOVERNANCE trademarks |
| Not a desktop recovery product — no Save/Resume honesty model | absence in architecture docs |
| computer_use expands OS action surface — high stakes | module existence |

---

## 5. Over-engineering risks

| System | Risk |
| --- | --- |
| Workspace | Dozens of `generate_*` projections without mounted consumers |
| Workspace | Parallel V2 certification meta-systems vs product value |
| Kiro Crew | Many messaging surfaces and apps may exceed a minimal trusted agent core |
| Both | Treating the other product’s entire stack as a dependency |

---

## 6. Under-engineering risks

| System | Risk |
| --- | --- |
| Workspace | Agent-path hardening relative to injection threats if AI expands |
| Workspace | Contract codegen still manual |
| Workspace | Release-safe RE/DE invariants (`debug_assert`) |
| Kiro Crew | Closed IDE/CLI reduces “verify every layer” for full product to OSS Crew only |
| Kiro Crew | Windows sandbox parity less documented than Linux/macOS |

---

## 7. Missed opportunities (Workspace)

1. Borrow **PreToolUse-outside-model** discipline for AI/automation IPC.  
2. Borrow **SEL integrity** ideas for audit modernise.  
3. Borrow **RFC/tenet ordering** hygiene for eng process.  
4. Borrow **resource watchdog** patterns if/when AI workers exist.  
5. Borrow **inspectable lessons** pattern for user-visible learning — without ambient desktop capture.

---

## 8. Missed opportunities (if evaluating Kiro-class systems)

1. Workspace’s **honesty-about-limits** could inform agent UX (“cannot restore X”).  
2. Workspace’s **single OS authority crate** is a clean pattern agent platforms often blur.  
3. Workspace’s **ambient-off default** is a strong privacy posture agents rarely match.

---

## 9. Architectural blind spots (unknowns needing evidence)

| ID | Blind spot | Needed evidence |
| --- | --- | --- |
| U1 | kiro-cli source architecture | Locate OSS or accept black-box ACP contract tests |
| U2 | Kiro IDE extension host internals | Closed; treat as UNKNOWN |
| U3 | Exact Windows behaviour of Crew sandbox/rlimits | Read windows-install + platform_compat tests |
| U4 | Depth of `computer_use` vs Win32 window managers | Module deep-dive |
| U5 | Whether App SDK TS lives in-repo or npm-only | Package search |
| U6 | Production AWS-hosted Kiro data flows | Out of Crew OSS scope |
| U7 | Long-term licence/trademark constraints on embedding | NOTICE + legal |
| U8 | Compatibility of Crew memory with Workspace Moments | Must remain separate authorities |

---

## 10. Migration opportunities (interface-backed only)

Recommend **design spikes**, not merges:

| Opp | Interface Workspace would own | External piece | Stage |
| --- | --- | --- | --- |
| M1 | `ModelProvider` | Optional kiro-cli/ACP backend | Modernise/Expand after ADR |
| M2 | `AuditIntegrity` trait | HMAC-chain pattern inspired by SEL | Modernise |
| M3 | `AgentToolGate` policy module | Patterns from PreToolUse/deny lists | Modernise AI |
| M4 | `BackgroundWorkerSupervisor` | Reaper/timeout patterns | Expand workers only |
| M5 | Docs process | RFC folder + ordered tenets | Stabilise/Consolidate |

**Do not migrate:**

- Crew as the Workspace shell  
- Cron-driven desktop observation  
- Messaging-first product identity  
- Bypassing PermissionGateway  

---

## 11. Final independent judgment

Kiro Crew is **exceptionally strong** at **agent orchestration security**, **unattended job hygiene**, **local accumulative memory**, and **protocol-separated runtimes**.

Workspace is **exceptionally strong** at **constitutional desktop recovery authority**, **permission-before-OS-effect**, and **honest restore semantics**.

The correct relationship is **reference architecture + selective principle adaptation**, not convergence into one product.

Workspace must remain recognisably Workspace.
