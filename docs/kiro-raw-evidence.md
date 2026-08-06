# Kiro Raw Evidence

| Field | Value |
| --- | --- |
| **Purpose** | Phase 0 evidence collection only — no architectural conclusions |
| **Collected** | 2026-08-07 |
| **Evidence hierarchy** | Source > repo structure > architecture docs > website |
| **Local clone** | `%TEMP%\KiroCrew-ev2` (shallow clone of `kirodotdev/KiroCrew`) |

**Legend:** **FACT** · **INFERENCE** · **UNKNOWN**

---

## 0. Repository identification

### 0.1 Official websites

| URL | Content observed | Class |
| --- | --- | --- |
| https://kiro.dev/ | Markets “agentic engineering”, spec-driven development, IDE + CLI, ACP/MCP/AGENTS.md, AWS enterprise claims | **FACT** (marketing site) |
| https://kiro.dev/crew/ | Markets Kiro Crew: persistent agents, cron/webhooks/heartbeat, “7 layers of defense”, open source, “View on GitHub”, built on Kiro CLI / ACP | **FACT** (marketing + product claims) |

### 0.2 GitHub organization

| Item | Value | Class |
| --- | --- | --- |
| Org | `kirodotdev` (GitHub API / org page) | **FACT** |
| Homepage | https://kiro.dev | **FACT** |

### 0.3 Candidate repositories verified

#### A. `https://github.com/kirodotdev/Kiro`

| Observation | Class | Source |
| --- | --- | --- |
| GitHub description: “Kiro is an agentic IDE…”; language reported TypeScript; size ~1564 | **FACT** | GitHub API |
| Root contents: `.github`, `.gitignore`, `.kiro`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `README.md`, `assets`, `docs`, `scripts` | **FACT** | GitHub API `/contents` |
| README directs IDE download from kiro.dev and CLI install from docs; no application `src/` tree present in root listing | **FACT** | README + contents listing |
| License field on API returned `None` | **FACT** | GitHub API |
| This repository does **not** contain the full Kiro IDE/CLI runtime implementation | **FACT** (from structure) | Root listing lacks source package tree |
| Purpose appears to be docs, assets, community/issues | **INFERENCE** | Structure + README support links |

#### B. `https://github.com/kirodotdev/KiroCrew`

| Observation | Class | Source |
| --- | --- | --- |
| Description: “A persistent workspace for development work that self-improves and continues beyond one session.” | **FACT** | GitHub API |
| License: Apache-2.0 | **FACT** | `LICENSE`, GitHub API |
| Language: Python; size ~258356 (API units) | **FACT** | GitHub API |
| Root contains `src/`, `packages/`, `docs/`, `tests`/`test`, `website/`, `Makefile`, `pyproject.toml`, `GOVERNANCE.md`, `TENETS.md`, `SECURITY.md` | **FACT** | Clone listing |
| Package name `kirocrew` version `0.2.0`, console script `kirocrew = kiro_crew._bootstrap:main` | **FACT** | `pyproject.toml` |
| Architecture docs state Kiro Crew is a gateway over **kiro-cli** via ACP; kiro-cli is a hard requirement | **FACT** | `docs/architecture/overview.md` |
| kiro-cli source is **not** vendored as the primary implementation inside this clone’s described layout; it is an external runtime | **FACT** (docs) / **UNKNOWN** (where kiro-cli source lives) | overview.md |

**Phase-0 selection for source reverse engineering:** `kirodotdev/KiroCrew` is the verified open-source implementation repository for **Kiro Crew**. The product marketed as **Kiro IDE/CLI** is not fully open-sourced in `kirodotdev/Kiro`.

---

## 1. Repository structure (KiroCrew)

### 1.1 Top-level layout

| Path | Purpose (from docs/README) | Class |
| --- | --- | --- |
| `src/kiro_crew/` | Python backend | **FACT** | overview.md |
| `website/` | React + TypeScript + Tailwind dashboard (Vite) | **FACT** | overview.md |
| `website/electron/` | Electron desktop shell | **FACT** | overview.md |
| `packages/kirocrew-client-py` | Standalone SDK client package | **FACT** | packages listing |
| `skills/` | Checkout-only reference skills | **FACT** | overview.md |
| `docs/` | architecture, guides, system-specs, RFCs | **FACT** | clone |
| `docker/`, `packaging/` | Container / desktop packaging | **FACT** | clone |
| `test/` / `tests/` | pytest suite | **FACT** | clone |
| `AGENTS.md`, `CLAUDE.md` | Agent contributor guidance files | **FACT** | clone |
| `TENETS.md` | Ordered design principles | **FACT** | file |
| `GOVERNANCE.md` | Maintainer/RFC governance | **FACT** | file |

### 1.2 Major Python modules under `src/kiro_crew/` (non-exhaustive)

Observed directories/files include (clone listing):

`acp/`, `apps/`, `browser/`, `builtin_skills/`, `cloud/`, `computer_use/`, `config/`, `connections/`, `dashboard/`, `data/`, `deploy/`, `discord/`, `knowledge/`, `mcp_gateway/`, `mcp_providers/`, `messaging/`, `metrics/`, `platform/`, `providers/`, `sandbox.py`, `security.py`, `sel.py`, `session.py`, `memory.py`, `vector_memory.py`, `cron.py`, `taskrunner.py`, `subagent.py`, `heartbeat.py`, `hooks.py`, `webhooks.py`, `workflows/`, `slack/`, `telegram/`, `tools/`, `_vendor/`, plus many channel integrations (`teams`, `webex`, `wecom`, `weixin`, …).

**Class:** **FACT** (directory/file names present).

---

## 2. Architecture documentation present

| Document | Topic | Class |
| --- | --- | --- |
| `docs/architecture/overview.md` | Gateway vs kiro-cli vs agent configs; message flow; component map | **FACT** |
| `docs/architecture/security-deep-dive.md` | Threat model, trust boundaries, layered defenses | **FACT** |
| `docs/architecture/resource-protection.md` | Timeouts, reapers, cgroups, rlimits | **FACT** |
| `docs/architecture/mcp.md` | MCP architecture | **FACT** (file exists; not fully quoted here) |
| `docs/architecture/resource-protection.md` | Resource ceilings | **FACT** |
| `docs/system-specs/modules/*` | Module specs referenced extensively | **FACT** (referenced; full inventory not exhaustively listed in this note) |
| `docs/request-for-change/` | RFC process for architectural changes | **FACT** | GOVERNANCE.md |

---

## 3. Build system

| Fact | Source | Class |
| --- | --- | --- |
| Python packaging via setuptools (`pyproject.toml`, `setup.py`/`setup.cfg`) | pyproject.toml | **FACT** |
| `Makefile` with `make build` referenced in README | README / Makefile present | **FACT** |
| Dev tools pinned: pytest, hypothesis, mypy, black, coverage, etc. | pyproject.toml dependency-groups | **FACT** |
| Frontend: Vite React dashboard under `website/` | overview.md | **FACT** |
| Electron packaging under `website/electron/` | overview.md | **FACT** |
| Docker support under `docker/` | clone | **FACT** |

---

## 4. Runtime model (documented)

| Claim | Source | Class |
| --- | --- | --- |
| Kiro Crew = single asyncio gateway process (Python / aiohttp) | overview.md | **FACT** |
| Agent backend = `kiro-cli` over ACP (JSON-RPC 2.0 stdio) | overview.md | **FACT** |
| `agent.provider` fixed to `acp`; kiro-cli hard requirement | overview.md | **FACT** |
| Surfaces: CLI, web dashboard, Electron, messaging channels | overview.md | **FACT** |
| Session pool with warm pool of kiro-cli processes | overview.md | **FACT** |
| Message flow: Surface → Hooks → Session → Context → ACP → stream back → JSONL log → async memory consolidation | overview.md sequence diagram | **FACT** (documented design) |
| Tool calls evaluated at Kiro Crew PreToolUse gate before execution | overview.md / security-deep-dive.md | **FACT** |

---

## 5. State & persistence

| Claim | Source | Class |
| --- | --- | --- |
| Memory files under `~/.kiro/crew/workspace/memory/` (`preferences.md`, `projects.md`, `history/YYYY-MM-DD.md`) | `memory.py` docstring | **FACT** |
| FTS5 index at `~/.kiro/crew/memory_index.db` | `memory.py` docstring | **FACT** |
| Vector/semantic memory module `vector_memory.py` exists | file present + overview map | **FACT** |
| Conversation log JSONL | overview.md | **FACT** (documented) |
| Config under `~/.kiro/crew/config.json` (referenced) | sandbox.py docstring / security docs | **FACT** |
| Agent configs under `~/.kiro/agents/` including generated `kirocrew.json` | overview.md | **FACT** |
| SEL storage: `<config_dir>/security_events.jsonl` append-only | `sel.py` docstring | **FACT** |
| Exact full schema of all persisted stores beyond above | — | **UNKNOWN** (not fully inventoried in Phase 0) |

---

## 6. Permission / security model

| Claim | Source | Class |
| --- | --- | --- |
| Dominant threat: prompt injection → credential theft / destructive action | security-deep-dive.md | **FACT** (documented threat model) |
| Model treated as untrusted input | security-deep-dive.md | **FACT** |
| Operator trusted; agent must not widen posture | security-deep-dive.md | **FACT** |
| PreToolUse gate is Kiro Crew’s gate, not the agent’s; deny list not writable into agent JSON to weaken ceiling | security-deep-dive.md | **FACT** |
| Layers: OS sandbox, filesystem path gate, command deny, validation, output redaction, SEL audit | security-deep-dive.md | **FACT** |
| OS sandbox Linux namespaces / macOS Seatbelt; `agent.sandbox` enum `auto`\|`off` | sandbox.py + security-deep-dive.md | **FACT** |
| Large built-in denied-command regex catalog in `security.py` | security.py | **FACT** |
| Governance: Policy ∩ Profile two-level model referenced | security-deep-dive.md → system-specs/governance.md | **FACT** (existence of model) |
| SEL: HMAC-SHA256 chained JSONL events | sel.py | **FACT** |
| Website claim “7 layers of defense” | kiro.dev/crew | **FACT** (marketing count); maps loosely to documented layers |

---

## 7. Memory / learning

| Claim | Source | Class |
| --- | --- | --- |
| Structured markdown memory + FTS5 | memory.py | **FACT** |
| Lessons extracted from corrections (`learn.py`) | overview.md component map | **FACT** (module + docs) |
| Skills system (`skills.py`, builtin_skills, Skills.md compatibility claimed on site) | code + website | **FACT** / site **FACT** |
| Knowledge graph with embeddings claimed on website | kiro.dev/crew | **FACT** (marketing); backed by `knowledge/`, `embeddings.py`, `vector_memory.py` presence | **FACT** (modules exist) |
| Exact retrieval ranking algorithm | — | **UNKNOWN** |

---

## 8. Scheduling & background work

| Claim | Source | Class |
| --- | --- | --- |
| Cron jobs with timeouts/reapers | cron.py + resource-protection.md | **FACT** |
| TaskRunner autonomous tasks with checkpoint/retry | taskrunner.py + overview.md | **FACT** |
| Subagents with timeouts/reapers | subagent.py + resource-protection.md | **FACT** |
| Heartbeat maintenance | heartbeat.py + overview.md | **FACT** |
| Webhooks module | webhooks.py | **FACT** |
| Website claims timezone-aware cron, jitter, skip dates | kiro.dev/crew | **FACT** (marketing); code details | **UNKNOWN** without deeper cron.py read |

---

## 9. Plugin / extension model

| Claim | Source | Class |
| --- | --- | --- |
| Apps SDK marketed (`@kirocrew/app-sdk`, TypeScript/Python) | kiro.dev/crew | **FACT** (marketing) |
| `apps/` package under `src/kiro_crew/apps` | clone listing | **FACT** |
| MCP servers for tools (`mcp_*.py`, mcp_gateway) | overview.md | **FACT** |
| Skills as markdown files inspectable/editable | TENETS + website | **FACT** (philosophy + marketing) |
| Open VSX / VS Code settings apply to **Kiro IDE**, not necessarily Crew | kiro.dev | **FACT** (IDE claim); Crew relationship | **INFERENCE** (separate product surface) |

---

## 10. IPC / messaging

| Claim | Source | Class |
| --- | --- | --- |
| ACP JSON-RPC 2.0 stdio to kiro-cli | overview.md | **FACT** |
| Dashboard: aiohttp + WebSocket | overview.md | **FACT** |
| Channel transports for Slack/Discord/Telegram/etc. | overview.md + module dirs | **FACT** |
| MCP for tool servers | overview.md | **FACT** |

---

## 11. AI / agent orchestration philosophy (documented)

| Claim | Source | Class |
| --- | --- | --- |
| Unattended work is an explicit design goal (cron, TaskRunner, subagents) | overview.md “Why orchestrate” | **FACT** |
| Specialization via multiple agent configs | overview.md | **FACT** |
| Accumulation via shared memory/lessons | overview.md | **FACT** |
| LLM connection owned by kiro-cli, not Crew gateway | overview.md | **FACT** |
| Model registry file `model_registry.json` present | clone | **FACT** |

---

## 12. Governance (project)

| Claim | Source | Class |
| --- | --- | --- |
| Tenets ordered; Safety first; Build in the open; Easy to use; Gateway not replacement; Community; Knowledge with boundaries; Teammates not tools | TENETS.md | **FACT** |
| Maintainers decide; RFCs in `docs/request-for-change/` for architectural changes | GOVERNANCE.md | **FACT** |
| Trademarks not open-sourced; code is | GOVERNANCE.md / NOTICE | **FACT** |
| “No rug pulls” openness constraint | GOVERNANCE.md | **FACT** |

---

## 13. Testing

| Claim | Source | Class |
| --- | --- | --- |
| pytest, hypothesis, coverage, mypy, flake8, black in tooling | pyproject.toml | **FACT** |
| Dedicated `test/` and/or `tests/` trees | clone | **FACT** |
| Property-based testing marketed for Kiro product broadly | kiro.dev | **FACT** (product marketing); Crew uses hypothesis in deps | **FACT** |

---

## 14. Computer use / desktop OS control

| Claim | Source | Class |
| --- | --- | --- |
| `computer_use/` module and `mcp_computer.py` exist | clone listing | **FACT** |
| Scope of computer-use relative to Windows window management | — | **UNKNOWN** without module deep-read |
| No evidence in Crew docs that Crew implements Workspace-style Save Moment / Win32 place-focus product | searched architecture overview | **FACT** (not described there) |

---

## 15. Explicit unknowns (Phase 0)

1. Full source of **kiro-cli** / Kiro IDE agent runtime (not in `kirodotdev/Kiro` tree).  
2. Complete dependency graph of runtime Python packages (setup.cfg dynamic deps not fully extracted).  
3. Exact Windows sandbox behaviour (sandbox docs emphasize Linux/macOS; Windows install guide exists).  
4. Full App SDK TypeScript package location (marketing references `@kirocrew/app-sdk`; may live under `website/` or separate publish — **UNKNOWN** in this pass).  
5. Production deployment topology at AWS for hosted Kiro (out of Crew OSS scope).

---

## 16. Evidence integrity note

Statements labeled **FACT** are tied to cited files or API observations from this collection pass.  
Marketing claims from kiro.dev are **FACT as claims**, not **FACT as verified runtime behaviour**, unless corroborated by source/docs in the clone.
