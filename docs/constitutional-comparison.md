# Constitutional Comparison — Workspace vs Kiro Principles

| Field | Value |
| --- | --- |
| **Purpose** | Phase 3 — compare extracted Kiro principles to Workspace Architectural Constitution V2 |
| **Workspace baseline** | `architecture/ARCHITECTURAL_CONSTITUTION_V2.md`, strategic review, replaceability matrix |
| **Kiro principles** | `docs/kiro-design-philosophy.md` |
| **Date** | 2026-08-07 |

**Classifications:** Equivalent · Complementary · Superior · Inferior · Independent · Unknown

“Superior/Inferior” means **for Workspace’s constitutional goals**, not absolute moral rank.

---

## Principle-by-principle

| ID | Kiro principle | vs Constitution V2 | Classification | Evidence / why |
| --- | --- | --- | --- | --- |
| P1 | Gateway over replacement | Law X interfaces-over-vendors; enhance-not-replace Windows | **Complementary** | Same spirit (don’t replace platforms); different object (dev tools vs OS) |
| P2 | Runtime / orchestrator split | Kernel vs UI vs windows-integration already split | **Complementary** | Workspace splits OS/kernel/UI; Crew splits runtime/orchestrator |
| P3 | Standard protocol boundary | Tauri IPC is Workspace’s boundary; ACP is Crew’s | **Independent** | Both use a hard boundary; protocols differ |
| P4 | Authz ceiling outside model | Law IV PermissionGateway; non-human ApprovalRequired | **Equivalent** (intent) | Both keep authority outside the model |
| P5 | Model as untrusted | Constitution AI policy; debug_assert gaps elsewhere | **Equivalent** intent; Workspace **Inferior** in agent-tool depth | Crew has mature PreToolUse stack for shell agents |
| P6 | Layered defense | CSP + gateway + OS crate; thinner agent stack | Workspace **Inferior** for agent-tool threat; **Superior** for Win32 product boundary clarity | Different threat models |
| P7 | Ask + audit | Laws IV–V | **Equivalent** | Both require permission + audit |
| P8 | Local-first memory | Law VII local-first | **Equivalent** | Both local-default |
| P9 | Cross-session accumulation | Workspace has session recovery + optional AI memory; not Crew-style lessons | **Complementary** / Workspace thinner | Different product needs |
| P10 | Inspectable artefacts | Explanation catalog generated; skills less central | **Complementary** | Workspace could ADAPT markdown lessons later |
| P11 | First-class long-running work | Law XII ambient-off; automation scheduled non-executing | **Independent** / often **REJECT** for desktop ambient | Crew optimizes unattended coding agents; Workspace forbids ambient desktop capture by default |
| P12 | Independent watchdogs | Workspace session fences + restore recovery; no cron reaper matrix | **Complementary** | Resource-protection patterns STUDY for future workers |
| P13 | Operator vs agent asymmetry | StandardPermissionGate asymmetry | **Equivalent** | Same structural idea |
| P14 | Fail closed isolation | Win32 stubs fail closed off-platform; encryption Tier 0 acknowledged | **Complementary** | Crew sandbox fail-closed is stronger for agent spawn |
| P15 | Open standards tools | MCP not central to Workspace product | **Independent** | May WRAP MCP later behind interfaces |
| P16 | Public RFC trail | Decision Log / ADR / Guardian proposed | **Equivalent** intent | Process maturity differs |
| P17 | Ordered tenets | Constitution laws + Project Constitution | **Equivalent** | Both hierarchical law |
| P18 | Additive extensibility | Plugin host absent; Law on fake marketplace | **Independent** | Crew mature; Workspace must not fake DEC-011 |

---

## Constitutional law stress test

| Workspace Law | Kiro Crew tension | Result |
| --- | --- | --- |
| I Product identity (recovery) | Crew identity is agent orchestration | **Independent products** — do not merge identities |
| II One OS authority | computer_use/mcp_computer exist in Crew | **Unknown** detail; any adoption must not bypass Workspace Win32 crate |
| III One mutation pipeline | Crew mutations are tool calls via kiro-cli | **Independent** pipelines — never dual-write Workspace desktop state |
| IV Permission model | PreToolUse + governance ceiling | **Complementary** patterns for AI tools; must not replace Gateway for desktop |
| V Audit trail | SEL HMAC chain | Crew **Superior** as audit implementation reference |
| VI Desktop truth | N/A in Crew docs | Workspace unique |
| VII Local-first | Strong match | **Equivalent** |
| VIII Presentation boundary | Multi-surface gateway | Different UX category |
| IX Honesty | Agent may overclaim; Crew mitigates via gates | Different honesty problem (restore vs tool safety) |
| X Interfaces over vendors | ACP/MCP WRAP culture | **Complementary** — adopt WRAP discipline |
| XI Single authority narrative | Strong OSS docs | Crew **Superior** doc discipline in architecture/ |
| XII Ambient off | Unattended cron/heartbeat **on by design** | **Conflict** if applied to desktop observation — **REJECT** ambient agent loops for PP |

---

## Strategic review alignment

| Strategic claim | Comparison outcome |
| --- | --- |
| Workspace irreplaceable = recovery trust | Unchallenged by Kiro — no Save/Resume analogue found |
| External platforms = WRAP only | Confirmed: ACP orchestrator patterns WRAP-eligible; product identity not |
| NEVER REPLACE Win32 / Gateway / ambient-off | Reinforced: Crew’s unattended model must not override XII |

---

## Summary verdict

Kiro Crew is a **mature reference for agent orchestration security, long-running task hygiene, local memory, and protocol-split architecture**.

It is **not** a constitutional peer for Workspace’s desktop recovery identity. Several of its best ideas are **Equivalent/Complementary**; its unattended-agent default is **constitutionally incompatible** with Law XII if misapplied to desktop sensing/automation.
