# Workspace vs Kiro — Architecture Comparison

| Field | Value |
| --- | --- |
| **Purpose** | Side-by-side architecture comparison (authority & ownership — not features) |
| **Date** | 2026-08-07 |
| **Caveat** | “Kiro” below means the product family; deep source evidence is primarily **Kiro Crew**. IDE/CLI internals remain partially UNKNOWN. |

---

## 1. Category difference (first-order)

| Dimension | Workspace | Kiro / Kiro Crew |
| --- | --- | --- |
| Primary job | Trusted interruption recovery on Windows desktop | Agentic software engineering + persistent agent orchestration |
| User object | Saved Moment / restore plan | Specs, code changes, agent sessions, skills |
| OS role | Companion mutating window placement under consent | Agent runtime with shell/filesystem (and optional computer-use) |
| Default autonomy | Explicit, ambient-off | Interactive + unattended by design (Crew) |
| Open source core | Full product monorepo (this Workspace repo) | Crew OSS; IDE/CLI largely closed (`kirodotdev/Kiro` docs/issues) |

**Conclusion:** Different product categories. Comparison is for **engineering transfer**, not substitution.

---

## 2. Authority & ownership

| Concern | Workspace owner | Kiro Crew owner | Notes |
| --- | --- | --- | --- |
| Product semantics | SavedContext / RestoreExecutor | Agent configs, tasks, skills | Non-overlapping |
| Mutation path | CommandPipeline | PreToolUse → kiro-cli tools | Must never dual-write |
| Permission | PermissionGateway | Hooks + governance Policy∩Profile | Patterns transferable |
| Audit | AuditService SQLite | SEL HMAC JSONL | Crew richer integrity chain |
| Desktop truth | Observation SQLite | N/A (code/workspace files) | Workspace unique |
| Model I/O | Stub ModelProvider in-kernel | kiro-cli owns LLM | Crew WRAP reference |
| UI | React Tauri WebView | Dashboard/Electron/channels | Commodity hosts both |
| Persistence home | app_data `workspace.db` | `~/.kiro/crew/` files + sqlite indexes | Both local-first |

---

## 3. Runtime topology

```
Workspace:
  React → Tauri IPC → WorkspaceKernel → Pipeline → Gateway → Services
       → SQLite
       → windows-integration (Win32)

Kiro Crew:
  Surfaces → Gateway (asyncio) → Session pool → ACP stdio → kiro-cli
       → LLM / MCP tools
       → PreToolUse gate (Crew)
       → Memory / Cron / TaskRunner
```

| Property | Workspace | Kiro Crew |
| --- | --- | --- |
| Process model | Single Tauri app process (+ WebView) | Gateway + many kiro-cli child processes |
| Concurrency | Mutex kernel; disabled obs scheduler | Warm pool, subagents, cron, tasks |
| Background work | Intentionally dormant ambient | First-class |

---

## 4. Security architecture contrast

| Topic | Workspace | Kiro Crew |
| --- | --- | --- |
| Primary threat | Unauthorized desktop mutation; privacy surprise | Prompt injection → shell/credential damage |
| Hard boundary | Win32 crate + Gateway | PreToolUse + sandbox + path/command gates |
| Audit integrity | Durable SQLite events | HMAC-chained JSONL |
| Sandbox | OS file protection / Tier-0 encryption | Optional OS namespaces/Seatbelt + rlimits/cgroups |
| Non-human actors | ApprovalRequired | Agent untrusted vs operator |

---

## 5. Memory & learning

| Topic | Workspace | Kiro Crew |
| --- | --- | --- |
| Durable product memory | Moments, session fences, prefs, AI memory tables | Markdown memory + FTS + vectors + lessons |
| Learning loop | Pilot self-report; suggestion engines experimental | Corrections → lessons → later sessions |
| Inspectability | DB + UI; explanation catalog | Editable markdown skills/lessons |

---

## 6. Extensibility

| Topic | Workspace | Kiro Crew |
| --- | --- | --- |
| Plugins | Placeholder; Constitution forbids fake host | Apps + MCP + skills mature |
| Standards | Custom Tauri IPC | ACP, MCP, AGENTS.md/skills |
| Messaging surfaces | None (desktop companion) | Slack/Discord/Telegram/… |

---

## 7. Documentation & evolution

| Topic | Workspace | Kiro Crew |
| --- | --- | --- |
| Architecture docs | Dual tracks historically; Constitution V2 + knowledge maps | Coherent `docs/architecture` + system-specs |
| Decisions | Decision Log + ADRs | RFCs in request-for-change |
| Openness | MIT product repo | Apache-2.0 Crew; trademarks separate |

---

## 8. Where each is stronger (architecture)

### Workspace strengths

- Clear **desktop recovery** ownership and honesty model  
- Strict **one OS authority crate**  
- Product Proof **ambient-off** ethics  
- Domain purity + dense kernel contract tests  
- Constitutional clarity after V2  

### Kiro Crew strengths

- **Agent-tool security** depth (PreToolUse + layered controls)  
- **Long-running task** hygiene (timeouts/reapers/cgroups)  
- **Protocol split** (ACP) and multi-session orchestration  
- **Local memory/lessons** design for accumulation  
- **Tamper-evident audit** (SEL)  
- Public tenets/governance/RFC discipline  

---

## 9. What must not be confused

| Confusion | Correction |
| --- | --- |
| “Kiro is open source” | **Crew** is; **IDE/CLI** largely not in `kirodotdev/Kiro` |
| “Adopt Kiro = adopt recovery” | No — different jobs |
| “Crew cron = Workspace observation scheduler” | Cron is agent jobs; Workspace scheduler is desktop capture (disabled) |
| “More agents = better Workspace” | Violates identity if it weakens permission/honesty |
