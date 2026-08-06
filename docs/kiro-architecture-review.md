# Kiro Architecture Review

| Field | Value |
| --- | --- |
| **Purpose** | Phase 1 reverse engineering conclusions from collected evidence |
| **Evidence base** | `docs/kiro-raw-evidence.md` |
| **Scope** | Kiro product family as marketed; deep implementation analysis focused on open-source **Kiro Crew** |
| **Date** | 2026-08-07 |

---

## 1. Product identity

### Conclusion

**Kiro** (IDE/CLI) is an **agentic software-engineering product**: spec-driven development, coding agents, hooks, MCP/powers, running as IDE and CLI — operated by AWS, largely **not open-sourced** in `kirodotdev/Kiro`.

**Kiro Crew** is an **open-source persistent agent gateway/orchestrator**: multiplexes many sessions/surfaces onto `kiro-cli` via ACP, adds memory, scheduling, autonomous tasks, multi-channel access, and an independent tool-security gate.

### Why (evidence)

- kiro.dev markets agentic engineering / specs / parallel agents (**website FACT**).
- `kirodotdev/Kiro` root has README/docs/assets — no application source tree (**raw evidence FACT**).
- `docs/architecture/overview.md` defines three layers: kiro-cli runtime, agent configs, Kiro Crew gateway (**FACT**).
- Crew README/TENETS describe persistent local agent workspace (**FACT**).

---

## 2. Architectural philosophy

### Conclusion

Kiro Crew’s philosophy is **“gateway, not replacement”** (TENETS #4): connect tools you already use; route; remember across them; add orchestration the runtime deliberately omits.

Safety is ordered first (TENETS #1): gated, auditable, reversible where possible; ask first where not.

### Why

- TENETS.md ordered list with conflict rule “earlier wins” (**FACT**).
- overview.md capability table: kiro-cli alone vs with Crew (**FACT**).
- security-deep-dive.md: model untrusted; operator trusted; multi-layer defense (**FACT**).

---

## 3. Core abstractions

| Abstraction | Role | Evidence |
| --- | --- | --- |
| **Gateway** | Single asyncio process owning surfaces → sessions | overview.md |
| **ACP client** | JSON-RPC stdio driver for kiro-cli | overview.md, `acp/` |
| **Agent config** | Behaviour profile for a kiro-cli `--agent` | overview.md |
| **Session** | Independent ACP connection + transcript key | overview.md |
| **PreToolUse gate** | Crew-owned tool authorization chokepoint | security-deep-dive.md, hooks.py |
| **Memory / lessons / skills** | Cross-session learning artifacts | memory.py, learn.py, skills.py |
| **Cron / TaskRunner / Subagent / Heartbeat** | Unattended orchestration | overview.md, resource-protection.md |
| **SEL** | Tamper-evident security event log | sel.py |
| **Apps / MCP** | Extension surfaces | apps/, mcp_*.py |

---

## 4. State ownership

### Conclusion

- **Conversation state:** per-session transcripts / JSONL (gateway + kiro-cli session persistence).
- **Long-term memory:** Crew-owned local files + SQLite FTS (+ vector store module).
- **Security posture / config:** Crew data home under `~/.kiro/crew/` (documented paths).
- **LLM auth / tool execution internals:** owned by kiro-cli (documented boundary).

### Why

memory.py paths; overview layer table; sel.py storage path (**FACT**).

---

## 5. Persistence philosophy

### Conclusion

**Local-first structured memory** (markdown + FTS5 index), append-only security log, JSONL history; designed to survive restarts and feed later sessions.

### Why

memory.py docstring; TENETS #6 “Memory is local-first and private by default”; website crew claims corroborated by modules (**FACT**).

---

## 6. Security philosophy

### Conclusion

Defense-in-depth against **prompt injection with shell/filesystem power**. Critical property: **authorization ceiling lives in the gateway**, not in the model-writable agent config. OS sandbox optional/additive; path/command/output layers always on (per docs).

### Why

security-deep-dive.md trust boundaries and layer composition; security.py deny catalog; sandbox.py (**FACT**).

---

## 7. Governance philosophy

### Conclusion

Open maintainer governance with public RFCs for architectural change; trademarks held separately; “no rug pulls” openness constraint.

### Why

GOVERNANCE.md (**FACT**). Distinct from runtime “governance Policy ∩ Profile” for agent ceilings (also documented).

---

## 8. AI philosophy

### Conclusion

AI agents are **teammates with tools**, expected to run **attended and unattended**, accumulate lessons, and specialize via configs. The model is powerful but **untrusted**; tool use is gated.

### Why

TENETS #7; overview “Why orchestrate”; security threat model (**FACT**).

---

## 9. Scheduling philosophy

### Conclusion

First-class background work: cron, webhooks, heartbeats, task runners, with timeouts/reapers/resource limits — **productivity while away** is intentional.

### Why

overview.md; resource-protection.md mechanism table; website crew (**FACT**).

---

## 10. Plugin / extension philosophy

### Conclusion

Extend via **skills (markdown)**, **MCP tools**, **Apps** (UI + skills + schedules), and messaging surfaces — additive interoperability with open standards (ACP, MCP, AGENTS.md) claimed.

### Why

overview.md; apps/; website; TENETS #4–5 (**FACT** / marketing corroborated by modules).

---

## 11. Evolution strategy

### Conclusion

Public RFCs for expensive-to-reverse interface changes; tenets ordered; community contribution; Apache-2.0 code.

### Why

GOVERNANCE.md + TENETS.md (**FACT**).

---

## 12. What was *not* reverse-engineered from source

| Item | Status |
| --- | --- |
| Kiro IDE binary internals | Closed; only marketing + issues repo |
| kiro-cli ACP server implementation | External dependency; not in Crew tree as SoT |
| Full Windows sandbox parity | Docs emphasize Linux/macOS mechanisms |

These gaps are recorded so conclusions about “Kiro” as a whole do not over-claim IDE internals.
