# Workspace Strategic Architecture Review

| Field | Value |
| --- | --- |
| **Purpose** | Decide Workspace’s future direction from reconstructed architecture evidence |
| **Date** | 2026-08-07 |
| **Branch** | `v2-dev` |
| **Authority** | Implementation > generated knowledge maps > older aspirational docs |
| **Inputs** | `repository-audit.md`, `architecture-knowledge-map.md`, `runtime-service-map.md`, `ipc-surface-map.md`, `state-authority-map.md`, `domain-contract-assessment.md`, `governance-map.md`, `kiro-comparison-readiness.md` |
| **Companion outputs** | `architecture-consolidation-plan.md`, `replaceability-matrix.md`, `kiro-integration-strategy.md`, `strategic-roadmap.md`, `source-control-readiness.md` |
| **Constraint** | Strategic decisions only — no implementation, refactor, or commit |

---

## Executive decision

**Workspace’s irreplaceable product is trusted interruption recovery on Windows** — explicit Save → honest resume plan → permission-gated place/focus — owned by a local kernel with a hard OS boundary and a non-negotiable permission sequence.

Everything else is either:

- **platform necessary to keep that product safe and testable**, or
- **commodity / experimental surface that must not redefine the product**.

Future direction: **stabilise Product Proof ownership and documentation authority, consolidate accidental complexity, then modernise commodity seams behind Workspace interfaces** — without adopting external platforms as product identity.

---

## Phase 1 — Workspace identity

### What is Workspace?

Workspace is a **Windows companion desktop layer** that helps an interruption-heavy professional return to meaningful work faster than unmanaged window juggling — without replacing Windows and without acting without permission.

In one page of product truth:

1. **Companion, not OS.** It enhances Windows; it does not become a shell replacement or locked desktop.
2. **Moment-centric recovery.** The user names and saves a bounded desktop context (Save Moment), reviews a restore plan, and executes place/focus on still-living windows with honest partial outcomes.
3. **Permission before power.** Observe → Learn → Suggest → Receive Permission → Automate is structural (PermissionGateway), not a UI slogan.
4. **Local-first trust.** Moments, sessions, audit, and preferences default to embedded SQLite on the machine.
5. **Evidence-backed honesty.** Product Proof limits are explicit: no ambient capture at startup, no relaunch of closed apps, single-monitor claim, partial restore retained.
6. **Kernel-owned authority.** React presents; Rust kernel gates; Win32 lives in one crate; UI never owns desktop truth.

That is the product. That is what must remain true if every experimental generator and diagnostic panel disappeared tomorrow.

### What is Workspace NOT?

| Not this | Evidence |
| --- | --- |
| Not a Windows replacement | Constitution; architecture principles; Win32 place/focus only |
| Not an autonomous agent | Non-human actors → ApprovalRequired; AI stubs; no silent automate |
| Not an IDE | No editing/host language runtime product surface |
| Not a cloud workspace suite | Local SQLite; no cloud-required path |
| Not a plugin marketplace (today) | `plugins/` placeholder; DEC-011 unimplemented |
| Not a full ambient desktop AI | Ambient capture gate closed; scheduler disabled at init |
| Not “197 features” | Mounted chrome uses ~21 IPC commands; rest is platform/diagnostic |
| Not documentation theatre | Implementation is authority when `docs/` phase language lags V1/V2 reality |

### Product identity vs engineering infrastructure

| Product identity (user-valuable) | Engineering infrastructure (necessary but not the brand) |
| --- | --- |
| Save / Continue / Check-in / Guide chrome | Tauri + Vite + React shell mechanics |
| SavedContext semantics | SQLite + migrations runner |
| Restore plan honesty | CommandPipeline plumbing |
| Explicit consent before capture | CapabilityBoundPolicy implementation detail |
| Pilot measurement loop | Vitest/Cargo CI |
| Trust that AI/automation cannot bypass gate | EventBus, ServiceRegistry health names |

### Irreplaceable value

**The irreplaceable value is the governed desktop recovery contract:**

> I can interrupt my work, save what mattered with consent, and return through a plan I approve — with the system telling me the truth when it cannot fully restore.

No amount of cognition generators, model adapters, or IDE-like agents substitutes for that contract on Windows.

---

## Phase 2 — Core architecture classification

Exactly one category per major subsystem.

| Subsystem | Category | Why |
| --- | --- | --- |
| Save Moment / SavedContext | **CORE PRODUCT** | Primary user-valuable object |
| Resume plan + RestoreExecutor | **CORE PRODUCT** | Delivers recovery outcomes |
| Product Proof chrome (5 destinations) | **CORE PRODUCT** | Mounted contract surface |
| Pilot measurement | **CORE PRODUCT** | Proves/trusts the wedge |
| Zero-ambient capture policy | **CORE PRODUCT** | Product safety identity |
| PermissionGateway + actors + grants | **CORE PLATFORM** | Structural authority; enables all safe expansion |
| CommandPipeline + audit | **CORE PLATFORM** | Sole mutation path |
| `workspace-windows-integration` | **CORE PLATFORM** | Sole OS boundary |
| Observation stack (explicit Manual) | **CORE PLATFORM** | Feeds Save; dormant ambient is policy, not product |
| WorkspaceRuntimeState + session store | **CORE PLATFORM** | Recovery integrity |
| `workspace-domain` pure models | **CORE PLATFORM** | Shared contract language |
| SQLite + repositories + migrations | **CORE PLATFORM** | Local-first durability |
| Thin Tauri IPC shell | **CORE PLATFORM** | Process boundary |
| Experience IPC catalog + demo adapter | **CORE PLATFORM** | Contract freeze + DEV fidelity |
| CapabilityBoundPolicy / StandardPermissionGate | **CORE PLATFORM** | Production authz |
| CSP + capability `core:default` | **CORE PLATFORM** | WebView security |
| SQLite engine / rusqlite bundling | **COMMODITY INFRASTRUCTURE** | Commodity DB behind Workspace schema |
| Tauri / WebView / Vite | **COMMODITY INFRASTRUCTURE** | Desktop host commodity |
| React / motion / lucide | **COMMODITY INFRASTRUCTURE** | UI stack commodity |
| Sync EventBus mechanics | **COMMODITY INFRASTRUCTURE** | Standard in-process pattern |
| `log` + `env_logger` | **COMMODITY INFRASTRUCTURE** | Logging commodity |
| Model provider stubs registry | **COMMODITY INFRASTRUCTURE** | Adapter seam (not differentiator today) |
| Vitest / Cargo test / CI | **DEVELOPER TOOLING** | Quality gates |
| scripts verifiers (CSP, catalog, boundary) | **DEVELOPER TOOLING** | Contract enforcement tools |
| `app/src/dev/*` evidence/cert dashboards | **DEVELOPER TOOLING** | Engineering confidence |
| `tools/`, `plugins/` placeholders | **DEVELOPER TOOLING** | Future hooks only |
| Cognition `generate_*` family (unmounted) | **EXPERIMENTAL** | Implemented, not Product Proof chrome |
| AI orchestration / assistant (stubs) | **EXPERIMENTAL** | Governed but not product wedge |
| Automation contracts / triggers (non-executing) | **EXPERIMENTAL** | Platform for future; not mounted recovery |
| Decision Engine / Queue / Recommendation | **EXPERIMENTAL** | Distinctive ownership, not mounted PP |
| V2 experience adaptation pipeline | **EXPERIMENTAL** | Presentation evolution track |
| Unmounted CanvasShell / OperatorConsole / Intelligence / Assistant | **LEGACY** | Present, superseded or diagnostic-only |
| ObservationStartupTrigger unwired + disabled scheduler thread | **LEGACY** | Retained dormant ambient path |
| Dual docs phase language (`docs/README` vs `architecture/32_*`) | **LEGACY** | Authority drift, not runtime |
| Hand-maintained `domain.ts` duplication | **LEGACY** *(process debt)* | Accidental sync burden |

---

## Phase 3 — Architectural value analysis

Ratings: **H** high · **M** medium · **L** low · **—** not applicable

| Subsystem | Competitive advantage | Commodity? | Differentiation | Tech debt | Maint. cost | Arch. importance | Future importance |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Save/Resume honesty | H | No | H | L–M (limits known) | M | H | H |
| PermissionGateway | H (trust) | Pattern yes, semantics no | H | M (`debug_assert` gaps) | M | H | H |
| Win32 boundary crate | H | OS APIs commodity; boundary not | H | M (multi-monitor FAIL) | M | H | H |
| Session recovery | M–H | Partial | M–H | M | M | H | H |
| Product Proof chrome | H (wedge UX) | No | H | L (frozen) | L–M | H | H |
| Kernel pipeline | M–H | Pattern | M | L–M | M | H | H |
| SQLite schema | M | Engine yes | Schema/semantics | M (45 migrations) | M | H | H |
| Cognition generators | L today | Assembler pattern | L–M | H (surface area) | H | M | Unknown / defer |
| AI stubs / assistant | L today | Adapters commodity | L | M | M | M | Conditional |
| Automation defs | L today | Rules engines | M (non-exec design) | M | M | M | Conditional |
| RE/DE/DQ overlays | M (split) | Decision queues | M–H split | M–H | H | M | Conditional |
| Unmounted diagnostic UI | — | — | — | H (IPC gravity) | H | L | Reduce |
| V2 presentation adaptation | L–M craft | Personalization | M | M | M | L–M | After PP stable |
| DEV certification | — | Tooling | — | L | M | L | Keep lean |
| Plugin runtime | — | Hosts exist | — | Gap | — | L today | Later |

---

## Phase 4 — Replaceability (summary)

Full matrix: `docs/replaceability-matrix.md`.

Strategic posture:

- **NEVER REPLACE** product recovery semantics, permission-before-automate, Win32 crate rule, ambient-off Product Proof posture.
- **PRESERVE** kernel pipeline, domain purity, SQLite ownership of Moments/session/audit, experience catalog.
- **MODERNISE** domain TS contracts, release-safe RE/DE invariants, documentation authority, multi-monitor gate.
- **WRAP** commodity hosts (Tauri, SQLite engine, model SDKs) behind Workspace interfaces.
- **REPLACE** only after ADR: stub model adapters, possibly EventBus internals, DEV dashboard chunks — never product semantics.
- **DELETE** (eventually, after quarantine policy): unused pathways that create gravity without mounted value — decisions deferred to consolidation plan; no deletes in this review.

---

## Phase 5 — Kiro integration strategy (summary)

Full strategy: `docs/kiro-integration-strategy.md`.

Posture: **evaluate mature platforms as optional infrastructure behind Workspace interfaces — never as product owners.**

- Likely-solved-elsewhere: model adapters, generic agent planners, extension hosts, some IDE chrome patterns.
- Uniquely Workspace: Save/Resume honesty on Win32, ambient-off ethics, recovery fences, PP chrome contract.
- Never leave Workspace: OS mutation authority, PermissionGateway meaning, consent-before-capture, audit of desktop mutations.
- Safe as implementation detail candidates: LLM SDK, codegen, logging, icon/motion — behind stable interfaces.

**No adoption recommended in this review.**

---

## Phase 6 — Simplification thesis

Full plan: `docs/architecture-consolidation-plan.md`.

| Necessary complexity | Accidental complexity |
| --- | --- |
| PermissionGateway + audit | 197 IPC vs 21 catalog without tier labels in product docs |
| Win32 isolation crate | Unmounted OperatorConsole as de facto second product |
| Explicit capture admission | Dozens of `generate_*` projections unused by mounted chrome |
| RE vs DE ownership split (if kept) | Hand-copied `domain.ts` |
| Honest restore limitations | Dual documentation authority tracks |
| Session recovery fences | Kernel vs domain `WorkspaceState` name collision (docs/education cost) |

Simplification rule: **reduce pathways that do not serve Save/Resume trust or the permission platform**, without deleting experimental code until ownership is labelled and gated.

---

## Phase 7 — Roadmap thesis

Full roadmap: `docs/strategic-roadmap.md`.

1. **Stabilise** — single authority story; protect PP behaviour; label IPC tiers.
2. **Consolidate** — contract generation path; quarantine legacy UI gravity; documentation sync.
3. **Modernise** — release-safe invariants; multi-monitor; encryption tier decision; optional model adapter wrap.
4. **Expand** — only behind gateway; automation/AI productisation after PP metrics; plugins only with host design.

---

## Phase 8 — Source control (summary)

Full readiness: `docs/source-control-readiness.md`.

**Recommendation: Further review before a single bulk commit.**  
Prefer a **documentation-only commit** of the knowledge + strategic packs after excluding unrelated architecture research WIP and evidence JSON churn — or split commits by concern. **Do not commit** until the owner chooses the split. **Do not push.**

---

## Binding strategic principles (decisions)

1. **Product identity = trusted interruption recovery on Windows.**
2. **Platform identity = permissioned local kernel with one OS boundary.**
3. **Experimental surfaces must not redefine the mounted product.**
4. **External maturity is a WRAP opportunity, never a silent REPLACE of authority.**
5. **Documentation authority must converge on implementation truth.**
6. **No new architectural layers without an explicit brief** (aligns with V2 handoff).

---

## Decision log candidates (for Project Owner)

These are strategic recommendations requiring owner acceptance before engineering:

| ID | Decision |
| --- | --- |
| S-001 | Declare Product Proof Save/Resume/Pilot the sole v1 product identity |
| S-002 | Classify cognition/AI/automation IPC as Experimental Platform until mounted |
| S-003 | Adopt documentation authority: `architecture/23–32` + V2 tip for eng; update `docs/README` phase language |
| S-004 | Pursue domain contract generation (design in domain-contract-assessment) as Modernise item |
| S-005 | Kiro/external platforms: evaluation only behind WRAP; no adoption this cycle |
