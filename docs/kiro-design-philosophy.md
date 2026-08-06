# Kiro Design Philosophy (Extracted Principles)

| Field | Value |
| --- | --- |
| **Purpose** | Phase 2 — reusable engineering principles only (no branding/UI/UX/terminology fetish) |
| **Source** | Kiro Crew implementation + architecture docs; Kiro product docs only where they state platform principles |
| **Date** | 2026-08-07 |

Principles are stated abstractly so they can be evaluated against Workspace Constitution without importing product names as requirements.

---

## P1 — Gateway over replacement

**Principle:** Prefer a multiplexing gateway that connects existing tools to an orchestrated runtime, rather than forcing users to abandon their toolchain.

**Evidence:** TENETS #4; overview “gateway, not the replacement.”

---

## P2 — Runtime / orchestrator split

**Principle:** Separate the **agent runtime** (model I/O, tool exec, session compaction) from the **orchestrator** (multi-session, scheduling, shared memory, surface adapters).

**Evidence:** overview.md three-layer model (kiro-cli vs Crew).

---

## P3 — Standard protocol boundary

**Principle:** Drive the runtime through a stable protocol (here: ACP JSON-RPC stdio) so orchestrators and clients remain swappable.

**Evidence:** overview.md ACP-only; website ACP compatibility claims.

---

## P4 — Authorization ceiling outside the model

**Principle:** Security ceilings that the agent must not weaken live in the orchestrator/gateway, not in model-editable config alone.

**Evidence:** security-deep-dive.md PreToolUse gate independent of agent JSON.

---

## P5 — Model as untrusted input

**Principle:** Treat model-chosen tool arguments as attacker-controllable under injection; enforce on ground-truth paths/commands.

**Evidence:** security-deep-dive.md threat model.

---

## P6 — Layered defense with uncorrelated failure modes

**Principle:** Stack sandbox, path gates, command denies, validation, output redaction, and audit so compromise of one layer does not equal total failure.

**Evidence:** security-deep-dive.md layer composition.

---

## P7 — Ask when irreversible; audit always

**Principle:** Gate dangerous actions; prefer reversibility; require explicit operator trust expansion; record security-relevant events in a tamper-evident log.

**Evidence:** TENETS #1; sel.py HMAC chain.

---

## P8 — Local-first memory with explicit sharing

**Principle:** Persist learning locally by default; sharing is explicit, controlled, auditable; support right-to-forget.

**Evidence:** TENETS #6; memory.py local paths.

---

## P9 — Cross-session accumulation

**Principle:** Sessions should feed shared memory/lessons so later work starts smarter than cold chat.

**Evidence:** overview.md capability table; learn.py.

---

## P10 — Inspectable behavioural artifacts

**Principle:** Skills/lessons/steering as human-readable files the operator can edit or delete.

**Evidence:** website crew; skills/memory markdown design; TENETS.

---

## P11 — First-class long-running work

**Principle:** Design for timeouts, reapers, checkpoints, retries, and resource ceilings for unattended jobs — not only interactive turns.

**Evidence:** resource-protection.md; cron/taskrunner/subagent.

---

## P12 — Independent watchdogs

**Principle:** Do not rely on a single timeout path; pair primary timeouts with independent reaper/watchdog loops and startup orphan cleanup.

**Evidence:** resource-protection.md.

---

## P13 — Operator vs agent asymmetry

**Principle:** Operators may widen posture; agents must not widen it for them (keystone files / sensitive paths).

**Evidence:** security-deep-dive.md.

---

## P14 — Fail closed on isolation when required

**Principle:** If a required sandbox backend is unavailable, refuse spawn rather than silently run unconfined (unless explicit opt-in).

**Evidence:** security-deep-dive.md Layer 0.

---

## P15 — Open standards for tools and agents

**Principle:** Prefer MCP / AGENTS.md / skills formats for interoperability.

**Evidence:** kiro.dev; Crew MCP modules.

---

## P16 — Public architectural decision trail

**Principle:** Expensive-to-reverse interface changes get RFCs with written reasoning before code.

**Evidence:** GOVERNANCE.md request-for-change process.

---

## P17 — Ordered tenets with recorded trade-offs

**Principle:** When principles conflict, earlier tenets win and the trade-off is written down.

**Evidence:** TENETS.md preamble.

---

## P18 — Additive extensibility

**Principle:** Extensions (apps/skills/MCP) compose without requiring users to rewrite their existing agents/tools.

**Evidence:** TENETS #4–5; website “additive, not a replacement.”

---

## Non-principles (explicitly not extracted)

- Spec-driven IDE UX flows  
- Electron dashboard aesthetics  
- Slack/Discord as product identity  
- AWS billing/IAM enterprise packaging  
- Character “Agent Worlds” gamification  

These may be good products; they are not reusable engineering laws for Workspace.
