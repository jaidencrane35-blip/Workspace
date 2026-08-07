# Workspace — Engineering Handoff
## Canonical onboarding for future engineering agents

| Field | Value |
| --- | --- |
| **Authority** | This document + machine state in `docs/project-health.json` |
| **Updated** | 2026-08-07 |
| **Branch** | `v2-dev` |
| **Handoff status** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |
| **Latest program** | P16.15 Product Completion, Production Hardening & Final Technology Validation (awaiting Owner) |
| **Rule** | Ignore prior chat history. Reassess repository truth before any implementation. |

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
| Architectural Constitution | `architecture/ARCHITECTURAL_CONSTITUTION_V2.md` (**v2.0**) |
| Product Constitution | `docs/00-Constitution/PRODUCT_CONSTITUTION.md` |
| Execution protocol | `.cursor/rules/constitutional-execution-protocol.mdc` (**v1.12**) |
| Machine state | `docs/project-health.json` (synced to `app/public/project-health.json`) |
| Product Gravity | `docs/ui/PRODUCT_GRAVITY_RULE.md` |
| Product Proof | `docs/capability-runtime/PRODUCT_PROOF_RULE.md` |
| User Adaptation Prohibition | Permanent (P16.6) — same Product Proof authority |
| Production Before Expansion | Permanent (P16.15) — same Product Proof authority |
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

| Now | P16 Voice Input — Engineering Complete (through **P16.15** Product Completion); Product Proof pending Owner |
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
| **Voice Input** | **Engineering Complete** (P16.15) | Conversation input device — **not** a Runtime desktop provider; **Product Proof pending Owner** |
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

---

## 11. Frozen architectural boundaries

Do not redesign without a new constitutional program:

1. Conversation → Intent → Kernel Operator → Runtime → Providers → OS  
2. UI Architecture Spec (P8)  
3. Product Gravity / Presentation purity  
4. Provider non-orchestration (no provider-to-provider calls)  
5. Voice as input device (never owns desktop orchestration)  
6. Capability Independence Rule  
7. User Adaptation Prohibition  
8. Production Before Expansion  

---

## 12. Known future work

| Item | Notes |
| --- | --- |
| **P16 Owner Product Proof** | Outstanding — must accept before permanent closure |
| **P17 File Provider** | Next program after P16 acceptance |
| Docs authority convergence | Backlog G3 |
| WorkspaceState naming | Backlog G4 |
| AgentToolGate / AuditIntegrity | Phase B |
| Tray / native polish | Track A debt |

---

## 13. Outstanding Product Proof items

| Item | Status |
| --- | --- |
| **P16 Voice Input — live Owner Product Proof** | **PENDING FINAL ACCEPTANCE** |
| Harness / verifiers for Voice + conversation quality | Green (engineering) — do not treat as Owner acceptance |
| P10–P15 Product Proof | Accepted / closed |

Voice remediations through P16.15 (Product Completion / production hardening) do **not** equal permanent closure.  
After Owner closes Workspace: cleanup only — **never relaunch**.

---

## 14. Current branch

`v2-dev` (tracks `origin/v2-dev`)

---

## 15. Latest commit

| Tip | Use `git log -1 --oneline` on `v2-dev` after P16.15 push |
| --- | --- |
| Prior Voice engineering | `77b3b4d` (P16.14) · `bb8ab78` (P16.13) |

---

## 16. Repository health

| Field | Value |
| --- | --- |
| Machine state | `docs/project-health.json` |
| Handoff status | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |
| Engineering mode | `constitutional-execution` |
| Protocol | v1.12 |
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
pnpm --filter @workspace/app exec tauri dev   # Windows Product Proof launch
```

---

**STOP condition for this handoff:** await Product Owner P16 acceptance. No P17. No further feature implementation from this document alone.
