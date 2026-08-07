# Workspace — Engineering Handoff
## Canonical onboarding for future engineering agents

| Field | Value |
| --- | --- |
| **Authority** | This document + machine state in `docs/project-health.json` |
| **Updated** | 2026-08-07 |
| **Branch** | `v2-dev` |
| **Handoff status** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |
| **Latest program** | P16.PI2 Gate C — Single-instance reliability; await Owner before next gate |
| **Production gates** | `docs/production/PRODUCTION_GATES.md` |
| **Production readiness** | `docs/production/PR1_PRODUCTION_READINESS_PROGRAM.md` · `docs/production/production-readiness.json` |
| **Installer** | `docs/production/INSTALLER.md` · `docs/production/P16_PI1_SLICE1_INSTALLER_FOUNDATION.md` |
| **Gate C report** | `docs/production/P16_PI2_GATE_C_SINGLE_INSTANCE.md` |
| **Governance resilience** | `docs/00-Constitution/P16_O3_GOVERNANCE_RESILIENCE_REPORT.md` |
| **Product Proof readiness** | `docs/capability-runtime/product-proof/P16_O1_PRODUCT_PROOF_READINESS_REPORT.md` |
| **Product Proof execution** | `docs/capability-runtime/product-proof/P16_O2_OWNER_PRODUCT_PROOF_EXECUTION_AUTHORITY.md` |
| **Product Proof workbook** | `docs/capability-runtime/product-proof/P16_O2_PRODUCT_PROOF_SESSION_WORKBOOK.md` |
| **Rule** | Spec v2 = sole architectural authority. Execution Standard v1 = how all programs run. Classify work, max layer, lowest-layer invariant, compliance checklist. Do not reopen Spec without a Review Trigger. |

**Milestone:** Workspace operates in **Sustainable Engineering Operations**. Spec v2 and EES v1 are stable constraints. **PR1** records production truth: not public-release ready (installer/signing/updater/tray/diagnostics/IPC). Future effort: Owner Product Proof → Phase 2 Critical production → P17 after P16 Accept. No meta-governance. Architecture stable by default; Review Trigger required for constitutional change.

**Start here before any execution program.**

---

## 1. Repository identity

Workspace is a **Windows-targeted Tauri 2 desktop app**:

| Layer | Location |
| --- | --- |
| Frontend | `app/` — React 18 + Vite |
| Native shell | `app/src-tauri` — Tauri 2 |
| Kernel / domain / OS ports | `packages/*` (Rust) |
| Persistence | Embedded SQLite (no external DB, no microservices, no Docker) |

Product identity: a **Conversational Desktop Operator** — not a chat app, not a capability launcher, not an agent IDE.

Cloud / Linux VM notes: `AGENTS.md`. Full native Voice/OS paths require Windows.

---

## 2. Current architectural pipeline (frozen)

```
Conversation
    → Intent Layer
    → execute_capability_intent
    → Kernel Operator
    → Capability Runtime
    → Providers / Ports
    → Operating System
    → Conversation Response
```

**Laws:**

- Conversation never calls providers directly for desktop effects.
- Providers own operations; providers never call each other.
- Kernel owns composition (Capability Composition Rule).
- Voice is a Conversation **input device** — transcript then follows the same path as typed text.

---

## 3. Accepted constitutional rules

| Authority | Document |
| --- | --- |
| **Workspace Constitutional Specification v2** | `docs/00-Constitution/WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md` — **sole architectural authority** |
| **Engineering Execution Standard v1** | `docs/00-Constitution/WORKSPACE_ENGINEERING_EXECUTION_STANDARD_V1.md` — how programs execute |
| Authority hierarchy | `docs/00-Constitution/ARCHITECTURE_AUTHORITY_HIERARCHY.md` |
| Compliance checklist | `docs/00-Constitution/CONSTITUTIONAL_COMPLIANCE_CHECKLIST.md` |
| Decision record index | `docs/00-Constitution/ARCHITECTURAL_DECISION_RECORD_INDEX.md` — why Workspace works this way |
| Architectural Constitution (subordinate) | `architecture/ARCHITECTURAL_CONSTITUTION_V2.md` — MUST NOT override Spec |
| Product Constitution | `docs/00-Constitution/PRODUCT_CONSTITUTION.md` — experience law; subordinate for architecture |
| Execution protocol | `.cursor/rules/constitutional-execution-protocol.mdc` — governance under Spec |
| Machine state | `docs/project-health.json` (synced to `app/public/project-health.json`) |
| Product Gravity | `docs/ui/PRODUCT_GRAVITY_RULE.md` — principle under Spec |
| Product Proof | `docs/capability-runtime/PRODUCT_PROOF_RULE.md` — governance under Spec |
| User Adaptation Prohibition | Permanent (P16.6) — same Product Proof authority |
| Production Before Expansion | Permanent (P16.15) — same Product Proof authority |
| Evidence Before Modification | Permanent (P16.16) — same Product Proof authority |
| Root Cause Before Rewrite | Permanent (P16.16) — same Product Proof authority |
| Production Quality Includes Repository Quality | Permanent (P16.17) |
| Evidence Before Completion | Permanent (P16.17) |
| Repository Health Before Milestone Closure | Permanent (P16.17) |
| Provider Acceptance Standard | `docs/capability-runtime/PROVIDER_ACCEPTANCE_STANDARD.md` |
| Capability Independence Rule | Recorded in Provider Acceptance Standard + protocol |
| UI Architecture | Accepted / frozen (P8) |

---

## 4. Permanent engineering cadence

```
One execution program
    → Engineering Complete
    → Product Proof (live Owner)
    → Owner acceptance
    → Permanent closure in repository truth
    → Next program
```

- Exactly one active program at a time.
- No speculative redesign of frozen layers.
- No beginning the next program until the current one is permanently closed **by Owner acceptance**.
- Bugfixes only on permanently closed milestones.

---

## 5. Product philosophy

- Desktop operation through Conversation — not chat theatre.
- Ordinary language only in user-facing replies (no Provider / Runtime / WinRT / HRESULT jargon).
- Truthful failure > silent failure.
- Conversation Complete ≠ Product Complete.
- Engineering Complete ≠ permanently closed.

---

## 6. Current roadmap

Authoritative rolling list: `docs/capability-runtime/FIVE_PROGRAM_ROADMAP.md`.

| Now | P16 Voice Input — **P16.39** Operator Intelligence (Owner evidence > eng confidence); Product Proof pending Owner live review; P17 blocked |
| Next (after P16 acceptance) | **P17 File Provider** |
| Then | P18 Terminal · P19 Memory · P20 Automation · P21 Workspace Intelligence |

**Do not begin P17 until P16 is permanently closed.**

---

## 7. Permanently accepted milestones

Do not reopen except bugfixes:

| Program | Title |
| --- | --- |
| P6 | Desktop Operator Shell |
| P8 | UI Architecture & Product Presentation |
| P9 | Capability Runtime Research |
| **P10** | Capability Runtime Foundation + Clipboard |
| **P11** | Application Provider |
| **P12** series | Window Provider + Product Proof + Product Gravity + Kernel Operator |
| **P13** | Notifications Provider |
| **P14** | Browser Provider |
| **P15** | Screenshot Provider |

---

## 8. Current provider / capability inventory

| Domain / surface | Status | Notes |
| --- | --- | --- |
| Clipboard | Closed (P10) | Port + Runtime |
| Application (`app_control`) | Closed (P11) | |
| Window (`window` / window_mgmt) | Closed (P12) | |
| Notifications | Closed (P13) | |
| Browser | Closed (P14) | |
| Screenshots | Closed (P15) | |
| **Voice Input** | **P16.39 engineering complete** (Product Proof open) | Operator activity states via Situation Goals + ≥750 battery; WRAP frozen; Voice not reopened; `VOICE_P16_39_OPERATOR_INTELLIGENCE.md` |
| File | Not started | **P17** (blocked) |
| Terminal / Memory / Automation | Roadmap | P18+ |

Catalogue: `docs/capability-runtime/CAPABILITY_DOMAIN_CATALOGUE.md`.

---

## 9. Current Operator architecture

| Concern | Location |
| --- | --- |
| Conversation shell UI | `app/src/components/operator/` (`OperatorRoot`, mic, composer) |
| Intent bridge | `app/src/lib/intentBridge.ts` |
| Kernel Operator | `docs/operator/` + kernel capability intent path |
| Operator Authority / Kernel Authority / Composition | `docs/operator/*_RULE.md`, policies |

---

## 10. Capability Runtime summary

- Foundation frozen at P10.
- Runtime hosts providers; Intent enters via Kernel Operator.
- Verifiers: `pnpm verify:capability-runtime-foundation`, provider-specific `pnpm verify:*-provider`, `pnpm verify:voice-input`, `pnpm verify:conversation-quality`, `pnpm verify:product-proof-harness`.
- Index: `docs/capability-runtime/00_INDEX.md`.
- **Voice engineering exit:** `docs/capability-runtime/product-proof/VOICE_ENGINEERING_EXIT_AUDIT.md`
- **Voice live Product Proof package (canonical):** `docs/capability-runtime/product-proof/VOICE_LIVE_PRODUCT_PROOF_PACKAGE.md`
- Voice findings traceability: `docs/capability-runtime/product-proof/VOICE_OWNER_FINDINGS_TRACEABILITY.md`
- Voice live instrumentation: `docs/capability-runtime/product-proof/VOICE_LIVE_INSTRUMENTATION.md`

---

## 11. Frozen architectural boundaries

**Canonical:** `docs/00-Constitution/WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md`.

Do not redesign constitutional concepts unless a named review trigger is satisfied. Implementation mapping (non-exhaustive):

1. Constitutional Transformation Chain: Signal → Meaning → Plan → [Authority] → Effect → Fact → Experience  
2. Conversation → Intent → Kernel Operator → Runtime → Providers → OS (current realization)  
3. UI Architecture Spec (P8)  
4. Product Gravity / Presentation purity  
5. Provider non-orchestration (no provider-to-provider calls)  
6. Voice as Input (never owns Effects)  
7. Capability Independence Rule  
8. User Adaptation Prohibition  
9. Production Before Expansion  
10. Evidence Before Modification / Root Cause Before Rewrite  
11. Constitutional Closure — evolution within Spec only 

---

## 12. Known future work

| Item | Notes |
| --- | --- |
| **P16 Owner Product Proof** | Outstanding — must accept before permanent closure |
| **P17 File Provider** | Next program after P16 acceptance |
| Docs authority convergence | Backlog G3 |
| WorkspaceState naming | Backlog G4 |
| AgentToolGate / AuditIntegrity | Phase B |
| Tray / native polish | Track A / PR1 Phase 2 Critical |
| Installer · signing · updater · diagnostics · IPC quarantine | Track A / PR1 Phase 2 Critical |

---

## 13. Outstanding Product Proof items

| Item | Status |
| --- | --- |
| **P16 Voice Input — live Owner Product Proof** | **PENDING FINAL ACCEPTANCE** — execute under P16.O2; record in session workbook |
| Harness / verifiers for Voice + conversation quality | Green (engineering) — do not treat as Owner acceptance |
| P10–P15 Product Proof | Accepted / closed |

P16.28 engineering exit: **no further Voice engineering is justified by current evidence.**  
Product Proof remains open until Owner acceptance.  
**Do not launch Workspace until the Product Owner requests it.**  
For live Product Proof: follow `VOICE_LIVE_PRODUCT_PROOF_PACKAGE.md` with `WORKSPACE_VOICE_PRODUCT_PROOF=1`, under `P16_O2_OWNER_PRODUCT_PROOF_EXECUTION_AUTHORITY.md`, recording evidence in `P16_O2_PRODUCT_PROOF_SESSION_WORKBOOK.md`.  
After Owner closes Workspace: cleanup only — **never relaunch**.

---

## 14. Current branch

`v2-dev` (tracks `origin/v2-dev`)

---

## 15. Latest commit

| Tip | Use `git log -1 --oneline` on `v2-dev` after P16.39 push |
| --- | --- |
| Canonical Product Proof | `docs/capability-runtime/product-proof/VOICE_P16_39_OPERATOR_INTELLIGENCE.md` |

---

## 16. Repository health

| Field | Value |
| --- | --- |
| Machine state | `docs/project-health.json` |
| Handoff status | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |
| Engineering mode | `constitutional-execution` |
| Protocol | v1.19 |
| Constitution | v2.0 |
| Sync | `node scripts/sync-project-health.mjs` |
| Verify | `node scripts/verify-project-health.mjs` |

---

## 17. Next eligible execution program

| Program | P17 File Provider |
| --- | --- |
| When | **Only after** Product Owner permanently closes P16 |
| Standard | Provider Acceptance Standard + Independence Rule |
| Instruction | **Do not begin P17 in this handoff.** |

---

## Agent realignment checklist

1. Read **this document**.  
2. Read `docs/project-health.json`.  
3. Reassess git (`v2-dev`, clean tree).  
4. Ignore previous conversational history.  
5. Treat repository documentation as authoritative.  
6. Continue from **P16 Product Proof Owner acceptance** — or, after that stamp exists, from **P17** when the Owner starts it.  
7. No feature work until the Owner issues an execution program.

---

## Key commands

```bash
pnpm typecheck
pnpm build
pnpm test
cargo check -p workspace-app
pnpm verify:voice-input
pnpm verify:conversation-quality
pnpm verify:project-health
pnpm --filter @workspace/app exec tauri dev   # Windows Product Proof launch (Owner-requested only)
```
