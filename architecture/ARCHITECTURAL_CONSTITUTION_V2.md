# Workspace Architectural Constitution V2

| Field | Value |
| --- | --- |
| **Status** | Binding for engineering on and after acceptance by Project Owner |
| **Version** | 2.0 |
| **Date** | 2026-08-07 |
| **Authority rank** | Highest **engineering architecture** authority for implementation decisions |
| **Supersedes** | Conflicting *engineering* guidance in stale phase language, package README exclusions, and informal chat — where they disagree with implementation truth or this Constitution |
| **Does not repeal** | `docs/00-Constitution/PROJECT-CONSTITUTION.md` product values; accepted ADRs (0001–0008+) and Decision Log entries unless explicitly amended through their processes |
| **Evidence base** | Implementation; `docs/architecture-knowledge-map.md` and companion maps; `docs/workspace-strategic-review.md`; `docs/replaceability-matrix.md`; `docs/governance-map.md`; V1 baseline `architecture/32_*` |
| **Amendment** | Requires Project Owner approval, Decision Log / ADR entry, and update of this document in the same change set |

---

## Preamble

This Constitution exists so that Workspace cannot lose itself to feature accumulation, tool-first development, external platforms, or multiple truths.

**Implementation is the highest authority for what the system currently does.**  
**This Constitution is the highest authority for what the system is allowed to become.**

When documentation conflicts with implementation, record the discrepancy and correct documentation — do not silently invent a third truth.

When a proposal conflicts with this Constitution, the proposal loses unless this Constitution is formally amended.

---

# Section 1 — Product Identity

## 1.1 What Workspace is

Workspace is a **Windows companion desktop layer** that restores interrupted work through governed, honest desktop recovery — without replacing Windows and without acting without permission.

**Product identity (engineering binding):**

Trusted interruption recovery:

1. The user **explicitly consents** before desktop observation for a Save.
2. The user **names and keeps** a bounded Saved Moment.
3. The user **reviews a restore plan** before mutation.
4. The system **executes only approved place/focus effects** on still-living windows.
5. The system **reports honest partial outcomes** when it cannot fully restore.
6. Non-human actors **cannot** bypass the permission model.

**Long-horizon mission** (from Project Constitution; aspirational, not a licence to sprawl):

> One workspace that brings your PC, phone, audio, and apps together.

Long-horizon domains may be pursued only through the laws in this document. They do not override Product Proof identity.

**Irreplaceable value:**

> I can interrupt my work, save what mattered with consent, and return through a plan I approve — with the system telling me the truth when it cannot fully restore.

## 1.2 What Workspace is NOT

Workspace is **not**:

| Forbidden identity | Why |
| --- | --- |
| A Windows replacement or shell lockdown | Constitution: enhance Windows |
| An autonomous agent | Permission sequence is mandatory |
| An IDE or cloud workspace suite | Wrong product category |
| A plugin marketplace by default | Host not implemented; must not fake one |
| Ambient desktop surveillance | Ambient capture off under Product Proof law |
| A pile of IPC features | Mounted product ≠ registered command count |
| Whatever an external platform’s brand is | External tech is never product identity |

## 1.3 Mission (one page)

**Mission:** Make interruption recovery on Windows trustworthy enough that professionals keep Workspace because it tells the truth and asks before it acts.

**How:**

- Present a companion Experience (Home / Save / Continue / Check-in / Guide) that exercises the recovery contract.
- Own a local Platform Kernel that gates every meaningful mutation.
- Isolate OS power in one integration boundary.
- Persist Moments, session integrity, preferences, and audit locally by default.
- Allow intelligence, automation, and adaptation only as **permissioned suggestions and approved actions**, never as silent desktop control.

**Success feeling (user):**

- “I got back to my work.”
- “It didn’t do anything I didn’t approve.”
- “It admitted what it couldn’t restore.”

**Success feeling (engineering):**

- One product identity.
- One mutation pipeline.
- One permission model.
- One OS authority.
- One desktop truth.
- Evidence for claimed behaviour.

---

# Section 2 — Architectural Laws

These laws are immutable without formal amendment.

## Law I — One Product Identity

The mounted product is trusted interruption recovery. Experimental, diagnostic, and future domains must not redefine or outrank that identity in default UX, release claims, or contributor onboarding.

## Law II — One OS Authority

Only `workspace-windows-integration` (or its formally designated successor crate under ADR) may perform Win32 / OS window and process effects. UI, TypeScript, and unrelated crates must not call OS mutation APIs.

## Law III — One Mutation Pipeline

All durable state changes and OS-effecting actions enter through `CommandPipeline` (or a formally designated successor). Ad-hoc service calls that bypass the pipeline for mutations are unconstitutional.

## Law IV — One Permission Model

All meaningful actions are authorised through `PermissionGateway` with actor, capability, and decision audit. The sequence **Observe → Learn → Suggest → Receive Permission → Automate** may not be skipped. Non-human actors default to ApprovalRequired unless a recorded grant applies.

## Law V — One Audit Trail

Permission decisions and command executions that affect authority or durability must leave durable audit records. Silent privileged paths are forbidden.

## Law VI — One Desktop Truth

Authoritative desktop observation lives in kernel-owned observation persistence and derived domain desktop state. The UI may display, never author, desktop truth. localStorage presentation state is not desktop truth.

## Law VII — One Local-First Default

Core recovery, session, preferences, and audit function locally (ADR-0001). Cloud or network services are optional enhancements and must not be required for Product Proof recovery.

## Law VIII — One Presentation Boundary

The React shell presents; the Rust kernel decides. The Experience IPC catalog defines the mounted Product Proof contract. Diagnostic surfaces must not become the default product.

## Law IX — Honesty Over Completeness

Restore and observation claims must match evidence. Known limits (no ambient startup capture, no relaunch of closed apps, monitor topology constraints, partial restore) must not be papered over by UX fiction.

## Law X — Interfaces Over Vendors

Workspace owns interfaces and product semantics. Commodity engines, SDKs, hosts, and external platforms live behind those interfaces. No external project may become Workspace’s product identity.

## Law XI — Single Authority Narrative

Engineering status and readiness are stated from implementation truth and accepted architecture baselines. Stale phase language must be corrected, not used to justify re-scaffolding finished platforms.

## Law XII — No Ambient Capture by Default

Under Product Proof law, ambient observation at startup remains off. Enabling ambient capture requires Product Owner decision, Decision Log / ADR, explicit user control model, and evidence — not convenience.

---

# Section 3 — Product Scope

## 3.1 Workspace MUST

| # | Obligation |
| --- | --- |
| M1 | Provide explicit Save Moment with consent-before-capture |
| M2 | Provide restore plan preview before execute |
| M3 | Execute restore only through permissioned OS boundary with honest outcomes |
| M4 | Keep LocalUser / System vs non-human actor distinction at the gate |
| M5 | Persist Moments, settings, audit, and session recovery data locally by default |
| M6 | Keep UI free of direct database and Win32 authority |
| M7 | Document product limits honestly in release/baseline materials |
| M8 | Gate new desktop-affecting behaviour behind tests and evidence |

## 3.2 Workspace SHOULD

| # | Guidance |
| --- | --- |
| S1 | Improve multi-monitor honesty when evidence supports claims |
| S2 | Generate or verify cross-language domain contracts |
| S3 | Keep experimental cognition/AI/automation behind clear tiers |
| S4 | Converge documentation authority with implementation |
| S5 | Prefer release-safe enforcement of architectural invariants over debug-only asserts |
| S6 | Maintain dense contract tests for recovery and permission paths |
| S7 | Label legacy/unmounted UI so it cannot be mistaken for the product |

## 3.3 Workspace MAY

| # | Permission |
| --- | --- |
| A1 | Offer suggestion, decision, recommendation, and automation **definition** surfaces that never self-execute |
| A2 | Integrate model SDKs behind `ModelProvider` (or successor) interfaces |
| A3 | Add diagnostic IPC for engineering, clearly tiered and non-default |
| A4 | Evolve Experience presentation without changing Product Proof behavioural contracts unless versioned |
| A5 | Design a plugin host later under ApprovalRequired actors and sandbox law |
| A6 | Use external commodity libraries for UI, logging, codegen, and host shells under Law X |

## 3.4 Workspace MUST NEVER become

| # | Prohibition |
| --- | --- |
| N1 | An OS replacement or forced full-screen prison |
| N2 | An agent that mutates the desktop without permission |
| N3 | A silent ambient recorder by default |
| N4 | A second product that is “the diagnostic console” |
| N5 | A thin skin over an external platform’s identity |
| N6 | A cloud-required recovery tool |
| N7 | A system with multiple competing mutation paths |
| N8 | A system where suggestion engines execute actions |
| N9 | A dumping ground for unbounded `generate_*` surfaces without mounted purpose |

---

# Section 4 — Engineering Principles

## 4.1 Complexity

- Prefer **necessary complexity** that protects trust (gateway, OS boundary, honest restore).
- Reject **accidental complexity** (duplicate truths, unmounted second products, undocumented tiers).
- New abstraction requires a named ownership problem it uniquely solves.

## 4.2 Dependencies

- Dependencies follow Dependency Policy and Decision Log norms.
- Prefer MIT/compatible licences consistent with project licence decisions.
- No dependency may own product semantics (Moments, restore honesty, permission meaning).

## 4.3 Abstractions

- Domain models stay pure (no I/O) in `workspace-domain`.
- Services coordinate; repositories persist; UI presents.
- Do not introduce a fourth human-decision overlay without ADR (RE / DE / Queue already exist).

## 4.4 Generated code

- Generated artifacts must be marked GENERATED and verified in CI when adopted.
- Hand-maintained duplicates of generated contracts are technical debt and must be scheduled for elimination once a pipeline exists.
- Explanation catalog pattern is the reference for domain contract modernisation.

## 4.5 Ownership

Every durable concept has one write authority (see state authority map). Dual writers require ADR.

## 4.6 State

- Durable product state: SQLite via repositories.
- Process-local runtime state: explicitly owned (e.g. runtime state service).
- Presentation state: may use localStorage; never elevates to desktop truth.
- Name collisions (e.g. two `WorkspaceState` types) must be documented until MODERNISE renames land via ADR.

## 4.7 Contracts

- Rust domain (or designated IPC DTO layer) is source of truth for wire shapes.
- TypeScript mirrors must not invent fields the kernel does not provide.
- Breaking mounted Experience catalog contracts requires explicit versioning and evidence updates.

## 4.8 IPC

- All UI↔kernel calls go through the single invoke boundary.
- Commands are tiered:
  - **Tier A** — Experience / Product Proof catalog
  - **Tier B** — Diagnostic / experimental (non-default)
  - **Tier C** — Parity / unused registered
- New commands require a tier, an owner, and either a mounted consumer or an explicit experimental brief.
- IPC count is not a product metric. Do not grow Tier A casually.

## 4.9 Documentation

- Document before implementing production architecture (Project Constitution).
- Correct stale docs when implementation advances.
- Strategic and knowledge maps guide engineering; chat is never authority (ADR-0008 spirit).

## 4.10 Testing

- Recovery, permission, and observation ethics paths require automated tests.
- Evidence JSON for claimed Windows behaviour must remain regenerable on the proof host.
- CI green is necessary, not sufficient — behavioural honesty still binds.

## 4.11 Security

- Least privilege at gate and WebView CSP.
- Secrets never in source.
- Encryption tiers advance only by Decision Log; Tier 0 is acknowledged, not pretended to be Tier 2.
- Non-human automation remains ApprovalRequired by default.

---

# Section 5 — External Technology Policy

## 5.1 Rule of interfaces

**Workspace owns interfaces. Commodity software lives behind Workspace interfaces.**  
**Never allow external software to become product identity.**

## 5.2 Categories

| Category | Policy |
| --- | --- |
| Desktop host (e.g. Tauri) | WRAP — host is commodity; product semantics remain Workspace |
| Embedded DB engine | WRAP — schema and repositories remain Workspace |
| UI libraries | WRAP |
| Model / AI SDKs | WRAP behind provider interfaces; gateway still mandatory on actions |
| Scheduling engines | WRAP or reject if they imply ambient capture without Law XII compliance |
| Plugin systems | May study mature hosts; Workspace policy and ApprovalRequired remain; no fake marketplace |
| Open-source projects generally | Integrate Behind Interface; licence and Decision Log as required |
| External “workspace platforms” / agent IDEs | Evaluate as advisor or model backend only (integration strategy Hold/Study); never as OS authority |

## 5.3 Forbidden externalisations

External code must not:

- Call Win32 outside the OS authority crate
- Write `workspace.db` except through kernel repositories
- Bypass `CommandPipeline` / `PermissionGateway`
- Define Save/Resume semantics
- Enable ambient capture without constitutional amendment
- Rebrand Workspace as the external product

## 5.4 Adoption bar

Any external adoption requires:

1. Named Workspace interface it implements  
2. Replaceability classification (WRAP/REPLACE)  
3. ADR or Decision Log entry  
4. Tests proving Product Proof paths unchanged  
5. Explicit non-identity statement (vendor ≠ product)

---

# Section 6 — Architecture Evolution

Evolution follows replaceability vocabulary:

| Action | When allowed |
| --- | --- |
| **ADD** | Solves a named ownership gap; has tier/owner; passes Section 7 checklist; does not violate Laws I–XII |
| **REJECT** | Creates second mutation path, second desktop truth, ambient-by-default, external identity takeover, or unmounted feature gravity without brief |
| **WRAP** | Commodity behind stable Workspace interface; semantics unchanged |
| **REPLACE** | Only after ADR; interface preserved; evidence regenerated; NEVER REPLACE set untouched |
| **MODERNISE** | Same ownership; better safety/contracts/docs |
| **DELETE** | After quarantine, zero required consumers, ADR, and green recovery/permission tests |
| **PRESERVE** | Default for CORE PRODUCT and CORE PLATFORM |

### NEVER REPLACE set (Band 0)

- Save Moment / SavedContext semantics  
- Resume plan honesty / RestoreExecutor semantics  
- Permission-before-automate sequence  
- Ambient capture off by default (Product Proof)  
- Win32 sole-crate rule  
- Non-execution of suggestion/decision engines  
- Enhance-not-replace Windows  

### Growth moratorium defaults

Until an Expand-stage brief authorises otherwise:

- No new architectural layers for convenience  
- No new certification/governance meta-systems  
- No new cognition `generate_*` families without mounted consumer  
- No plugin runtime scaffolding that pretends DEC-011 is done  

### Roadmap alignment

Stabilise → Consolidate → Modernise → Expand (`docs/strategic-roadmap.md`).  
Expand requires explicit brief. V2 confidence track must not silently become product expansion.

---

# Section 7 — Architectural Review Checklist

Every Pull Request, Cursor session that changes repository meaning, and architecture proposal **must** satisfy:

### Identity & scope

- [ ] Does not redefine product identity away from trusted interruption recovery  
- [ ] Does not mount diagnostic/experimental surfaces as default product without brief  
- [ ] States MUST / SHOULD / MAY / NEVER alignment  

### Laws

- [ ] No new OS mutation path outside OS authority crate  
- [ ] Mutations use CommandPipeline  
- [ ] PermissionGateway applies; non-human path cannot silently Allow  
- [ ] Audit implications considered  
- [ ] UI does not become desktop truth  
- [ ] Local-first default preserved for core recovery  
- [ ] No ambient capture enablement without Product Owner + ADR  
- [ ] External tech (if any) is WRAP’d; vendor ≠ identity  

### Ownership & contracts

- [ ] Single write authority for any new durable state  
- [ ] IPC tier labelled (A/B/C) with owner  
- [ ] Domain/TS contract impact acknowledged  
- [ ] No duplicate abstraction without unique ownership problem  

### Evidence & quality

- [ ] Tests for changed authority paths  
- [ ] Product Proof behaviour unchanged **or** deliberately versioned with evidence updates  
- [ ] Docs updated when meaning/status changes  
- [ ] Complexity justified as necessary, not accidental  

### Process

- [ ] Decision Log / ADR filed when Type A/B or precedent-setting  
- [ ] Open Questions updated when guessing was avoided by stopping  
- [ ] No “chat said so” authority  

**Fail any box that applies ⇒ do not merge / do not declare complete.**

---

# Section 8 — Future AI Development Policy

## 8.1 AI is a subsystem, not the product

AI exists to observe, learn, suggest, and — only after permission — automate.  
Workspace’s product remains recovery trust on Windows.

## 8.2 Mandatory sequence

```
Observe → Learn → Suggest → Receive Permission → Automate
```

No stage may be skipped. “The model is confident” is not permission.

## 8.3 Introduction requirements for any AI capability

| Requirement | Meaning |
| --- | --- |
| **Evidence** | Behavioural tests; no ambient surprise |
| **Governance** | Actor type, capabilities, ApprovalRequired defaults |
| **Permission** | Gateway path for any desktop-affecting or durable privileged effect |
| **Testability** | Deterministic fakes/stubs acceptable; live model optional behind interface |
| **Clear ownership** | One service owns writes; RE/DE/automation non-execution preserved |
| **Interface** | Models behind provider interface (Law X) |
| **Tier** | Diagnostic vs mounted product explicitly labelled |
| **Honesty** | Cannot claim restore/automation fidelity the system does not have |

## 8.4 Prevent

| Failure mode | Constitutional response |
| --- | --- |
| Architecture drift | Checklist + Laws I–XII |
| Tool-first development | Product identity first; tools WRAP |
| Feature accumulation | IPC tiers + mounted consumer rule |
| Duplicate abstractions | Ownership test before new engine |
| Multiple truths | Implementation + this Constitution; fix docs |
| Agent identity takeover | External platforms cannot own OS/permission/Save semantics |
| Ambient “for better AI” | Law XII |

## 8.5 Automation & decision engines

- Automation **definitions** and trigger→intent proposals may exist.  
- Decision / Recommendation engines **must not execute**.  
- Execution remains intent → gateway → audited command.  
- Scheduled ambient automation is forbidden without constitutional-level product change.

## 8.6 Data ethics for AI

- Prefer local context the user has consented to expose.  
- Pilot/measurement data remains consented and purpose-limited.  
- Egress to external model APIs requires explicit policy decision (privacy, local-first).  

---

# Authority & conflict resolution

| Conflict | Resolution |
| --- | --- |
| This Constitution vs stale engineering docs | This Constitution + implementation truth win; update stale docs |
| This Constitution vs Project Constitution values | Harmonise; do not weaken user control or AI sequence; amend via Project Constitution process if needed |
| This Constitution vs accepted ADR | ADR stands until superseded by new ADR that also amends this Constitution if laws change |
| Chat / agent suggestion vs this Constitution | Constitution wins |
| Speed vs law | Law wins |

---

# Closing

Workspace earns the right to grow only by remaining trustworthy.

Build the companion that restores work honestly.  
Gate every power.  
Own the interfaces.  
Never become the thing we refused to be.

---

## Related documents

| Document | Role |
| --- | --- |
| `docs/00-Constitution/PROJECT-CONSTITUTION.md` | Product values & founder authority |
| `docs/workspace-strategic-review.md` | Strategic decisions informing V2 |
| `docs/replaceability-matrix.md` | NEVER REPLACE / WRAP / DELETE vocabulary |
| `docs/governance-map.md` | Implemented governance mechanisms |
| `docs/state-authority-map.md` | State ownership |
| `docs/kiro-integration-strategy.md` | External platform posture |
| `docs/strategic-roadmap.md` | Stabilise → Expand stages |
| `architecture/32_Version_1_Baseline.md` | V1 engineering baseline facts |
| `architecture/V2_AGENT_HANDOFF.md` | V2 confidence constraints |
| `architecture/decisions/ADR-*` | Accepted architectural decisions |
