# Kiro Adoption Matrix

| Field | Value |
| --- | --- |
| **Purpose** | Phase 4 — classify what to ADOPT / ADAPT / WRAP / STUDY / IGNORE / REJECT |
| **Constitutional constraints** | Workspace retains interfaces, semantics, governance, permission, audit, desktop authority |
| **Date** | 2026-08-07 |
| **Rule** | Prefer adopting **principles** or **wrapping implementations** behind Workspace interfaces |

**Distinguish:** Adopt implementation · Adopt principle · Adopt neither.

---

## Matrix

| Opportunity | Class | Adopt what? | Why (evidence) | Constitutional fit |
| --- | --- | --- | --- | --- |
| Product identity / agent IDE | **REJECT** | Neither | Different category (`kiro-raw-evidence`, strategic review) | Law I |
| Unattended desktop ambient loops | **REJECT** | Neither | Crew cron/heartbeat unattended by design; conflicts Law XII | Law XII |
| Dual mutation path via kiro-cli for Win32 | **REJECT** | Neither | Would break Law II/III | Laws II–III |
| PreToolUse ceiling outside model | **ADAPT** | Principle | security-deep-dive.md; maps to Gateway for AI tools | Law IV |
| Layered agent-tool defenses (path/command/redaction) | **STUDY** → **ADAPT** | Principle (+ selective impl patterns) | security.py / deep-dive | Strengthens AI subsystem |
| HMAC-chained security event log | **STUDY** → **ADAPT** | Principle; maybe impl later | sel.py | Law V modernise |
| ACP protocol as Workspace desktop bus | **IGNORE** | Neither | Workspace already has Tauri IPC; ACP solves agent runtime | Independent |
| Runtime/orchestrator split for future AI workers | **ADAPT** | Principle | overview.md three-layer; aligns DEC-011 aspiration | Law X / future Expand |
| ModelProvider behind interface (kiro-cli as backend) | **WRAP** | Implementation candidate behind interface | overview LLM via kiro-cli | Law X; Hold until Study |
| Local markdown+FTS memory pattern | **ADAPT** | Principle | memory.py; TENETS #6 | Law VII complementary |
| Lessons-from-corrections loop | **STUDY** | Principle | learn.py | Optional; not PP core |
| Inspectable skills markdown | **STUDY** | Principle | skills system | Extensibility later |
| Cron/TaskRunner/subagent reaper matrix | **STUDY** | Principle | resource-protection.md | For non-desktop workers only |
| cgroup/rlimit spawn hygiene | **STUDY** | Principle | sandbox/resource docs | If multi-process AI Expand |
| MCP tool ecosystem | **WRAP** | Implementation behind Workspace tool interface | mcp_*.py; standards | Only if Expand needs tools |
| Apps SDK / Electron dashboard | **IGNORE** | Neither | Wrong product surface | Law I |
| Messaging channel gateways | **IGNORE** | Neither | Not Workspace companion identity | Law I |
| RFC / ordered tenets process | **ADAPT** | Principle | GOVERNANCE/TENETS | Complements Decision Log |
| Architecture overview + system-specs discipline | **ADAPT** | Principle | docs/architecture quality | Law XI |
| Property-based testing culture | **ADAPT** | Principle | hypothesis in Crew; kiro.dev claims | Testing principle 4.10 |
| computer_use modules | **STUDY** then likely **REJECT** for PP | Neither by default | Unknown overlap; high risk to Law II | Laws II, XII |
| Open-source Crew as dependency of Workspace app | **REJECT** (default) | Neither | Massive Python agent stack ≠ desktop recovery kernel | Identity + complexity |
| Selective vendoring of SEL or deny-list ideas | **STUDY** | Principle / small adapt | security.py, sel.py | Must re-express in Rust ownership |

---

## Summary counts (this pass)

| Class | Count (approx.) |
| --- | --- |
| REJECT | 5 |
| IGNORE | 3 |
| STUDY | 8 |
| ADAPT | 7 |
| WRAP | 2 |
| ADOPT (full implementation as-is) | **0** |

**No full ADOPT of Kiro or Kiro Crew into Workspace** is justified by evidence without violating identity.

---

## Conditions for any future WRAP of kiro-cli / Crew pieces

Must satisfy Constitution Section 5.4:

1. Named Workspace interface  
2. Replaceability WRAP classification  
3. ADR  
4. PP tests unchanged  
5. Vendor ≠ product identity  

Additionally for agent backends:

6. All desktop effects still go through Workspace Gateway + Win32 crate  
7. No ambient capture enablement  
8. Non-human ApprovalRequired preserved  

---

## Recommended near-term focus (no implementation now)

1. **ADAPT** agent-tool authorization-outside-model principle into AI IPC hardening design notes.  
2. **ADAPT** docs/RFC discipline (Law XI).  
3. **STUDY** SEL integrity chain for AuditService modernise options.  
4. **STUDY** resource-protection patterns for a future AI worker process — not for observation ambient.  
5. Keep **REJECT** list as hard gates in PR checklist.
