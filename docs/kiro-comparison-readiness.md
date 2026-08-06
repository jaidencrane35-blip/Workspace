# Kiro Comparison Readiness

| Field | Value |
| --- | --- |
| **Purpose** | Prepare a future Workspace-versus-Kiro architecture comparison — **without performing that comparison** |
| **Date** | 2026-08-07 |
| **Constraint** | No claims about Kiro’s internal implementation; classify Workspace systems by comparison role only |
| **Prerequisite reading** | `architecture-knowledge-map.md` and companion maps in `docs/` |

---

## Intent of this section

A later exercise may compare Workspace to **Kiro** (or a similarly positioned agent/IDE/platform product). This document only answers:

1. Which Workspace systems are **comparable in kind** to systems typically found in agent/IDE platforms.
2. Which Workspace systems are **product-unique** (desktop interruption-recovery layer on Windows).
3. What must be compared **first**.
4. What must remain **Workspace-authoritative** regardless of external maturity.
5. What *could* later be evaluated for replacement by a mature external implementation.
6. What must **never** be replaced if Workspace is to remain Workspace.

No replacement decisions are made here.

---

## Classification key

| Label | Meaning |
| --- | --- |
| **Analogue-likely** | Common category in agent/IDE/tooling platforms (permissions, audit, IPC, models, memory, planning) |
| **Unique-to-Workspace** | Tied to Windows desktop capture/restore companion product proof |
| **Compare-first** | High signal for architectural contrast |
| **Must-remain-authoritative** | Ownership must stay inside Workspace for product identity / safety |
| **Replacement-candidate (evaluate later)** | Mature external libs might supply *parts* — only after explicit governance decision |
| **Never-replace** | Replacing would abandon constitutional or Product Proof identity |

---

## Workspace systems — comparison roles

### Unique-to-Workspace (desktop product core)

| System | Why unique | Comparison role |
| --- | --- | --- |
| Save Moment / SavedContext | Explicit consented desktop snapshot as product object | Compare-first *as differentiator*; Never-replace product semantics |
| Resume plan + RestoreExecutor | Same-session window place/focus with honest partial outcomes | Compare-first; Must-remain-authoritative for OS effects |
| `workspace-windows-integration` | Sole Win32 capture/mutate boundary | Must-remain-authoritative; Never-replace with UI-only stack |
| WorkspaceRuntimeState + session recovery fences | Live restore/observation phases + crash dispositions | Compare-first for resilience story |
| Pilot measurement (consented self-report) | Product-proof measurement loop | Unique; keep authoritative |
| Product Proof experience chrome (Home/Save/Continue/Check-in/Guide) | Frozen behavioural contract for interruption recovery | Unique UX; compare as product wedge |
| Ambient-capture prohibition at startup | Hard Product Proof safety posture | Must-remain-authoritative policy |

### Analogue-likely (agent / platform categories)

| System | Typical analogue category | Comparison role |
| --- | --- | --- |
| PermissionGateway + actor types | Tool permission / approval brokers | Compare-first |
| CommandPipeline + audit trail | Command bus + audit log | Compare-first |
| CapabilityBoundPolicy | Capability-based authorization | Compare-first |
| AI planning / orchestration / assistant workflows | Agent planners / multi-step tools | Compare-first (note: Workspace uses stub model providers) |
| AI memory + user preferences | Agent memory / personalization stores | Analogue-likely |
| Model provider registry | LLM provider adapters | Replacement-candidate *evaluate later* (stubs today) |
| Decision Engine / Decision Queue / Recommendation Engine | Suggestion → human decision queues | Compare-first (ownership split is distinctive) |
| Automation contracts + trigger→intent proposals | Automation/rules engines | Analogue-likely; Workspace non-executing by design |
| SQLite local persistence | Local app DB | Analogue-likely |
| EventBus (sync domain events) | In-process eventing | Low priority |
| Plugin actor type (no runtime) | Extension hosts | Compare only as **gap** (not implemented) |
| CSP / WebView shell (Tauri) | Desktop shell security | Analogue-likely |
| Explanation catalog + DisplayReason | User-facing rationale systems | Analogue-likely |
| Frontend experience adaptation pipeline | Theme/personalization layers | Lower priority vs desktop core |

### Cognition `generate_*` projection family

| System | Role |
| --- | --- |
| Activity / continuity / attention / environment / composition / … | Analogue-likely to “context assemblers”; risk of over-engineering vs mounted UI — compare for **scope discipline**, not feature parity |

---

## Suggested comparison order (future exercise)

Compare in this order to maximise architectural signal before feature laundry lists:

1. **Authority model** — PermissionGateway, actors, ApprovalRequired, audit.
2. **OS mutation boundary** — how (or whether) the other product mutates the desktop; Workspace RestoreExecutor honesty constraints.
3. **Observation ethics** — ambient vs explicit capture; Workspace Product Proof zero-ambient startup.
4. **Command/IPC surface governance** — how tools are registered, gated, audited.
5. **Local persistence & session recovery** — SQLite + persistent session fences.
6. **Agent planning stack** — Workspace stub providers + orchestration vs external model runtime.
7. **Human decision overlays** — RE vs DE vs Decision Queue separation.
8. **Automation** — definition vs execution separation.
9. **Presentation shell** — companion chrome vs IDE chrome (product category difference).
10. **Extension/plugin host** — Workspace gap vs mature hosts.

Do **not** start with visual UI polish or cognition generator enumeration.

---

## Must remain Workspace-authoritative

Regardless of external maturity:

| Authority | Reason |
| --- | --- |
| Win32 integration crate boundary | Safety and testability of OS effects |
| PermissionGateway on every mutation | Constitutional AI sequence |
| Explicit Save consent before desktop read | Privacy promise |
| RestoreExecutor outcome honesty (partial success, no silent relaunch) | Trust wedge |
| Audit durability for permission/command decisions | Accountability |
| Product Proof behavioural IPC catalog for mounted chrome | Release evidence continuity |
| Local-first default for Moments/session | DEC-005 / DEC-010 |

---

## Could potentially be replaced by mature implementations (*evaluate later only*)

These are **candidates for study**, not approvals:

| Area | Why it might be replaceable in principle |
| --- | --- |
| LLM/model provider adapters | Currently deterministic stubs; mature SDKs exist elsewhere |
| Generic JSON schema / TS codegen | Contract pipeline not product-differentiating |
| Some in-process EventBus mechanics | Commodity pattern |
| Portions of DEV certification dashboards | Engineering tooling, not product core |
| Icon/motion libraries | Already external (`lucide-react`, `motion`) |

Any replacement would require Decision Log / ADR process; not implied by this list.

---

## Must never be replaced

| System / property | Why |
| --- | --- |
| User-permission-before-automate sequence | Constitution §AI Governance |
| Windows-as-platform (enhance, don’t replace OS) | Constitution §Relationship to Windows |
| Product Proof Save → Plan → Execute honesty model | Mission wedge “跨 interruption recovery” |
| Non-execution of Decision/Recommendation engines | Authority split; prevents silent action |
| Ambient capture off by default under Product Proof | Privacy / PP-B02 lineage |
| Single OS integration crate rule | Prevents UI/OS coupling |

---

## Systems with unclear analogue status (flag for research)

| System | Why unclear |
| --- | --- |
| PersistentWorkspaceSession recovery fences | May or may not exist under another name elsewhere |
| Explanation catalog binding to Experience freeze | Product-specific packaging |
| Pilot measurement consent schema | Study-specific |
| Dual RE/DE overlay persistence model | Distinctive; confirm whether peers conflate them |

---

## Inputs required before the actual Kiro comparison

1. Authoritative Kiro architecture materials (or agreed black-box behaviour suite).
2. Shared vocabulary table (tool ↔ command, approval ↔ PermissionGateway, etc.).
3. Explicit non-goals (e.g. do not compare IDE editing features to desktop restore).
4. Decision on whether comparison is **capability**, **safety model**, or **replaceability** oriented — these yield different matrices.
5. Freeze Workspace evidence set (this doc pack + `architecture/evidence/*` + baseline `32_*`).

---

## Out of scope for this document

- Any claim that Kiro implements or lacks a named Workspace subsystem.
- Any recommendation to adopt Kiro components.
- Any IPC reduction or refactor plan.
- Any product roadmap change.

When the comparison is authorised, start from **Suggested comparison order** and the **Must remain / Never replace** tables above.
