# Engineering Capability Ownership Matrix

| Field | Value |
| --- | --- |
| **Purpose** | Constitutional ownership of engineering capabilities |
| **Authority** | `architecture/ARCHITECTURAL_CONSTITUTION_V2.md`, replaceability matrix, knowledge maps |
| **Date** | 2026-08-07 |

**Classes:** Owns · Wraps · Adapts · Studies · Rejects

---

## Matrix

| Capability | Class | Owner / interface | Why | Replaceability | Constitutional justification |
| --- | --- | --- | --- | --- | --- |
| Save Moment semantics | **Owns** | SavedContextService / domain | Product identity | NEVER REPLACE | Law I, MUST M1 |
| Restore semantics | **Owns** | RestoreExecutor / DesktopAction | Trust wedge | NEVER REPLACE | Law I, IX |
| Permission Gateway | **Owns** | PermissionGateway | Sole authz | PRESERVE | Law IV |
| Audit semantics | **Owns** | AuditService contracts | Accountability | PRESERVE | Law V |
| Audit implementation (storage/integrity) | **Owns** (+ **Studies** integrity chain) | SQLite audit; SEL ideas | Own semantics; may adapt integrity | MODERNISE | Law V, X |
| Desktop authority / truth | **Owns** | Observation + domain WorkspaceState | One desktop truth | NEVER REPLACE | Law VI |
| Win32 integration | **Owns** | windows-integration crate | Sole OS boundary | NEVER REPLACE | Law II |
| Command Pipeline | **Owns** | CommandPipeline | Sole mutation path | PRESERVE | Law III |
| Kernel lifecycle state | **Owns** | kernel::state::WorkspaceState | Process lifecycle | PRESERVE | — |
| Domain desktop WorkspaceState | **Owns** | domain::WorkspaceState | Derived desktop projection | PRESERVE | Law VI |
| SQLite schema (product) | **Owns** | database migrations/repos | Local-first product data | PRESERVE | Law VII |
| Domain contracts (SoT) | **Owns** | workspace-domain | Wire truth | PRESERVE / MODERNISE codegen | §4.7 |
| TS domain mirrors | **Owns** (must generate) | app types / generated | Drift risk today | MODERNISE | §4.4, §4.7 |
| IPC architecture | **Owns** | Tauri + invokeIpc | Process boundary | PRESERVE | Law VIII, §4.8 |
| IPC tier governance | **Owns** | Tier A/B/C policy | Accidental surface | MODERNISE | §4.8 |
| Agent tool authorization | **Owns** interface; **Adapts** principle | Future AgentToolGate + Gateway | Outside-model ceiling | ADAPT from Kiro | Law IV; kiro-adoption |
| Background workers | **Owns** policy; **Studies** supervisors | ObservationScheduler (dormant); future supervisor | No ambient desktop | STUDY patterns; REJECT ambient | Law XII |
| Scheduling (desktop ambient) | **Rejects** by default | — | Product Proof | REJECT | Law XII |
| Scheduling (non-desktop AI jobs) | **Studies** | Future supervisor | Expand only | STUDY | Expand brief |
| Memory engine (Moments/session) | **Owns** | SQLite + session store | Product memory | PRESERVE | Law VII |
| Learning / lessons engine | **Studies** | Optional later | Not PP core | STUDY | kiro-adoption |
| Plugin runtime | **Rejects** fake host; **Studies** design | Placeholder | No marketplace pretence | PRESERVE placeholder | N5, DEC-011 |
| Model providers | **Owns** interface; **Wraps** backends | ModelProvider | Vendor ≠ identity | WRAP | Law X |
| LLM adapters | **Wraps** | Behind ModelProvider | Commodity | WRAP/REPLACE eval | Law X |
| AI orchestration | **Owns** governance; experimental impl | Kernel services | Non-executing plans | EXPERIMENTAL | AI policy §8 |
| Security hardening (CSP, gate) | **Owns** | tauri CSP, Gateway | Baseline | PRESERVE | §4.11 |
| Documentation governance | **Owns** | Constitution, ADRs, Decision Log | Single narrative | MODERNISE converge | Law XI |
| RFC process | **Adapts** | Decision Log + future RFC dir | Process | ADAPT | Law XI |
| Testing infrastructure | **Owns** | Vitest, Cargo, verifiers | Evidence | PRESERVE | §4.10 |
| Developer tooling | **Owns** scripts; **Wraps** formatters | scripts/, CI | Commodity tools | WRAP | — |
| Build tooling | **Wraps** | pnpm, Cargo, Vite, Tauri | Commodity hosts | WRAP | Law X |
| Dependency management | **Owns** policy | lockfiles, DEC-012 | — | PRESERVE | §4.2 |
| Code generation | **Owns** pipeline | explanation catalog; domain pending | Prefer generated | MODERNISE | §4.4 |
| Kiro Crew as product shell | **Rejects** | — | Wrong identity | REJECT | Law I |
| Unattended desktop automation | **Rejects** | — | Ambient risk | REJECT | Law XII |

---

## Ownership rules (binding)

1. **Owns** = Workspace writes the authority and cannot outsource semantics.  
2. **Wraps** = Workspace interface; commodity behind it.  
3. **Adapts** = Principle from reference; re-expressed under Workspace ownership.  
4. **Studies** = No implementation commitment.  
5. **Rejects** = Constitutionally incompatible or identity-destroying.
