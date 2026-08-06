# Kiro Integration Strategy

| Field | Value |
| --- | --- |
| **Purpose** | Identify integration *opportunities* with mature workspace/agent platforms (including Kiro-class systems) without recommending adoption |
| **Parent** | `docs/workspace-strategic-review.md`, `docs/kiro-comparison-readiness.md` |
| **Date** | 2026-08-07 |
| **Constraint** | No assumptions about Kiro’s internals; no adoption recommendation |

---

## Strategic posture

**Workspace remains the authority for desktop recovery trust and permissioned OS mutation.**

Mature external platforms (IDE agents, tool hosts, model runtimes — “Kiro-class”) may later supply **commodity infrastructure behind Workspace interfaces**. They must not become owners of:

- Save/Resume semantics
- Win32 mutation
- PermissionGateway meaning
- Ambient-capture ethics
- Audit of desktop-affecting decisions

This document maps **responsibilities**, not products to install.

---

## Responsibility partition

### Likely solved by mature workspace / agent platforms

These are **categories** commonly matured elsewhere. Opportunity = WRAP evaluation later.

| Responsibility | Workspace today | Opportunity shape |
| --- | --- | --- |
| LLM invocation / streaming | Deterministic stub providers | WRAP behind `ModelProvider` |
| Generic multi-step tool planners | AiOrchestration / assistant (diagnostic) | WRAP planner engine; keep gateway on each step |
| Extension / plugin hosts | Not implemented | Future host design may learn from mature hosts — still Workspace policy |
| IDE-centric UX chrome | Not Workspace’s product | Non-goal to integrate as shell |
| Cloud sync of editor state | Not present | Non-goal for PP |
| Rich markdown/chat UX patterns | Partial assistant panel (unmounted) | Commodity UI patterns only |
| Telemetry for agent traces | Audit exists but agent-trace UX immature | Optional tooling WRAP |

### Uniquely Workspace

| Responsibility | Why unique |
| --- | --- |
| Consented desktop Moment capture | Product object + privacy gate |
| Same-session restore with honest partial outcomes | OS truthfulness |
| Windows integration crate as sole mutator | Architecture law |
| Product Proof chrome contract (21 IPC) | Behavioural freeze + evidence |
| Ambient-off startup law | PP safety |
| Persistent session recovery fences for restore | Desktop resilience |
| Pilot measurement consent loop | Product proof science |

### Must never leave Workspace

| Responsibility | If externalised, what breaks |
| --- | --- |
| PermissionGateway decisions for OS/desktop mutations | Constitutional sequence |
| Final allow/deny of place/focus/launch | Silent automation risk |
| Definition of SavedContext / restore plan honesty | Product identity |
| Audit durability of permission + command outcomes | Accountability |
| Local-first storage of Moments/session (default) | Trust / DEC-005 |
| Observation admission policy | Privacy |

### May safely become implementation details behind Workspace interfaces

| Interface owned by Workspace | Impl detail candidates |
| --- | --- |
| `ModelProvider` trait | External LLM SDKs |
| Domain/IPC DTO codegen | typeshare/ts-rs/specta toolchains |
| `EventBus` transport | Alternate sync bus crates |
| Logging facade | tracing/opentelemetry exporters |
| DEV certification runners | External dashboards |
| Icon/motion rendering | Already external libs |
| SQLite engine | Alternate embedded engines **only if** migrations/repos preserved |

Every WRAP requires: stable Workspace interface, gateway still mandatory, ADR, tests proving PP unchanged.

---

## Integration patterns (evaluation options — not selections)

### Pattern A — Out-of-process advisor (safest conceptually)

External agent suggests; Workspace remains executor.

```
External platform → proposal/intent DTO
        → Workspace IPC / CommandPipeline
        → PermissionGateway
        → OS effects only if Allow
```

**Fits:** AIAssistant / Automation actor types already ApprovalRequired.  
**Does not grant:** ambient capture, silent restore.

### Pattern B — Model backend only

Replace stub `ModelProvider` implementations; keep planning services and gateway.

**Fits:** commodity LLM access.  
**Does not grant:** desktop authority.

### Pattern C — Shared schema / toolchain

Use external codegen or schema registries for TS/Rust contracts.

**Fits:** domain-contract-assessment pipeline.  
**Does not grant:** product behaviour.

### Pattern D — Rejected by default

External platform directly calling Win32, writing `workspace.db`, or bypassing IPC.

**Status:** incompatible with architecture.

---

## Opportunity backlog (evaluate later)

| ID | Opportunity | Stage hint | Risk |
| --- | --- | --- | --- |
| K-01 | ModelProvider adapter evaluation | Modernise | Prompt injection / data egress — needs policy |
| K-02 | Advisor-only intent bridge (Pattern A) | Expand | UI confusion with PP chrome |
| K-03 | Contract codegen toolchain | Consolidate/Modernise | Serde nullability drift |
| K-04 | Learn from plugin host designs | Expand (late) | Scope explosion |
| K-05 | Trace/UX patterns for explanations | Modernise | Must use explanation catalog SoT |

---

## Explicit non-recommendations

This review does **not**:

- Recommend adopting Kiro
- Recommend replacing Tauri, the kernel, or PP chrome
- Recommend merging Workspace into an IDE product category
- Authorise ambient observation to “feed an agent”
- Authorise DE/RE to auto-execute because an external planner is mature

---

## Comparison readiness checklist (before any integration spike)

From `kiro-comparison-readiness.md`, still required:

1. Authoritative external architecture materials or black-box suite  
2. Shared vocabulary (tool ↔ command, approval ↔ gateway)  
3. Non-goals (no IDE feature horse-race)  
4. Chosen lens: safety model vs replaceability vs capability  
5. Frozen Workspace evidence set  

Until then: **opportunities only; no integration engineering.**

---

## Decision for Project Owner

| Option | Meaning |
| --- | --- |
| **Hold** (default) | No external platform work; execute Stabilise/Consolidate |
| **Study** | Authorise comparison exercise using readiness order |
| **Spike WRAP** | Time-boxed ModelProvider or codegen spike behind interfaces |
| **Integrate** | Not available until Study + ADR |

**Strategic recommendation of this document:** **Hold**, with optional **Study** when materials exist. No Integrate.
