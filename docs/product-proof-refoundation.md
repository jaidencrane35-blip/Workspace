# Workspace Product Definition
## Product Proof Refoundation — Experience Before Features

| Field | Value |
| --- | --- |
| **Status** | Product definition for Project Owner review — **not implemented** |
| **Date** | 2026-08-07 |
| **Constraint** | Does **not** amend Architectural Constitution V2 or the Constitutional Execution Protocol |
| **Input** | Owner review conclusion: engineering progressing; experience not yet compellingly useful |
| **Authority for identity** | `architecture/ARCHITECTURAL_CONSTITUTION_V2.md` §1 (trusted interruption recovery) |
| **Reference study** | Kiro / Kiro Crew — **principles only** (REJECT product identity, branding, layout, wording) |
| **Next step** | Owner review of this definition → then select one experience execution program |

---

# Product questions (answered)

### Who is the first-time user?

A **Windows professional who context-switches under interruption** — consultant, engineer, analyst, founder, support lead — not an IDE power-user and not someone shopping for an AI chat app.

They already have windows, apps, and rituals. They do not want a new desktop religion. They want one trustworthy move when work breaks: *put this down cleanly and pick it up later.*

### What problem are they trying to solve?

**Interrupted return cost.** After a call, a Slack fire, a meeting, or a kids-at-the-door moment, reconstructing “where I was” is slow, stressful, and incomplete. Notes help a little; Windows does not. Other tools either watch too much or do too much.

Workspace’s constitutional problem statement:

> Interrupt → consent → name what mattered → return through a plan I approve → hear the truth when restore is partial.

### What emotional state are they likely in?

| Moment | Emotion |
| --- | --- |
| First open | Skeptical, time-poor, allergic to setup wizards and AI theatre |
| Mid-interruption (Save) | Rushed, mildly anxious, wants *control* more than cleverness |
| Return (Continue) | Hopeful but braced for disappointment; hates false confidence |
| After partial restore | Needs honesty, not apology spam |

Design must lower cognitive load and never add “another system to manage” feeling.

### What is the first thing they should see?

**One place that already knows its job:** a calm Home stage that answers three questions without a tutorial:

1. **What is this?** — interruption recovery companion (not IDE, not agent cockpit).  
2. **What do I do now?** — one primary action: **Save this moment** *or* **Continue a moment** (whichever is more relevant).  
3. **What do I already have?** — their Moments as memory in space, not a CRUD admin table.

Brand/product name is hero-level. No dashboard of equal modules. No Check-in/pilot as the emotional centre of first open.

### What should they accomplish in 30 seconds?

- Understand Workspace in one glance.  
- Either **start a Save** or **open an existing Moment to Continue**.  
- Never be blocked by account, cloud, model picker, or diagnostic chrome.

### Within two minutes?

Complete a **first Save**: name the Moment, read/consent to a short honest scope, write a one-line handoff (optional but encouraged), keep the Moment.

They should feel: *“I put work down on purpose.”*

### Within five minutes?

Complete a **Continue loop** on that Moment (or a demo Moment in pilot): see the restore plan, approve, see place/focus happen or an honest skip, land with handoff note visible.

They should feel: *“I got back — and it didn’t lie.”*

That five-minute loop is the Product Proof of usefulness. Everything else is secondary until this feeling is reliable and beautiful.

### When should AI first appear?

**Not in the first five minutes** of a cold first-time session.

AI may appear only after the user has completed at least one Save→Continue cycle *or* has explicitly opened an intelligence surface — and then only as:

- **explanation** of a restore plan (“why this window will be skipped”), or  
- **suggestion** that still requires permission (never silent desktop action).

Constitutional sequence remains: Observe → Learn → Suggest → Receive Permission → Automate. Product Proof default UX must not lead with chat.

### What should AI know?

| May know (with consent / evidence) | Source |
| --- | --- |
| What the user saved in a Moment (titles, layout facts under approved scope) | Save consent |
| What the restore plan will attempt / skip and why | Kernel plan + honesty law |
| User-authored handoff text | User |
| Explicit preferences the user set | Preferences |
| Pilot answers the user typed (pilot only) | Check-in consent |

### What should AI NOT pretend to know?

- Why the user was working (inferred goals, psychology, “you seem stressed”).  
- Contents inside documents, messages, or pages (out of capture scope).  
- Closed apps it cannot relaunch as if they were restored.  
- Ambient work patterns (ambient capture off — Law XII).  
- Authority to act because a model is “confident.”

### When should Workspace ask questions?

| Ask | When |
| --- | --- |
| Capture consent / scope | Before every Save observation |
| Moment name + handoff | During Save (handoff can be skippable but prompted) |
| Restore approval | Before Continue executes place/focus |
| Pilot consent | Only when user enters Check-in |
| Permission for any automation / AI effect | Always, before effect |

### When should Workspace stay silent?

- While the user is mid-flow on Save or Continue (no unsolicited tips).  
- Between Moments (no ambient “I noticed…”).  
- When it has nothing evidence-backed to say (prefer empty structure over fake activity).  
- About features that are not Product Proof (no upselling Canvas/Operator/Assistant).

### What actions should feel magical?

- Windows returning to place/focus **as if you never left** — when identity matches.  
- Seeing your own handoff sentence the moment you Continue.  
- Moments that feel like *places in memory*, not database rows.  
- Partial restore that still looks intentional (skipped items explained, not hidden).

Magic = **desktop physics that respect consent**, not chat fireworks.

### What actions should feel predictable?

- Consent → capture → named Moment.  
- Plan preview → approve → execute.  
- Same dock destinations every session.  
- Same permission sequence every time AI or automation appears.  
- Same honesty about limits (session-bound restore, no ambient watch).

---

# Kiro reference study — reusable principles only

Studied via existing Workspace evidence (`docs/kiro-design-philosophy.md`, adoption matrix, Crew architecture notes) and public Kiro positioning. **Zero ADOPT of Kiro product identity.**

| Reusable principle | Workspace translation |
| --- | --- |
| Gateway over replacement | Enhance Windows; do not replace shell or toolchain |
| Authorization ceiling outside the model | PermissionGateway / future AgentToolGate — never “AI decided” |
| Ask when irreversible; audit always | Approve restore; consent capture; durable audit |
| Local-first with explicit sharing | Moments & session local; no cloud required for PP |
| Inspectable artefacts | Moments, plans, scopes, handoff — human-readable, deletable |
| Additive extensibility | Future capabilities orbit recovery; never rewrite the core loop |
| Progressive capability exposure | Don’t dump Operator/AI surfaces on day one |
| Operator vs agent asymmetry | User widens trust; software never widens it for them |
| Fail closed | Refuse silent ambient / refuse fake restore completeness |

**Explicitly not copied:** IDE chrome, agent worlds, chat-first IA, branding, icon language, layout systems, Kiro wording.

---

# 1. Product Philosophy

**Workspace exists so interruption is not a small tragedy.**

It is a **trusted companion for putting work down and picking it up** — governed, local-first, honest when incomplete.

### Philosophical pillars

1. **Usefulness is the Save→Continue loop.** Architecture without that feeling is unfinished product.  
2. **Consent is the brand.** Asking is not friction; asking is the product.  
3. **Honesty beats completeness.** Partial restore told well beats full restore faked.  
4. **Memory over menu.** Moments are the user’s property in space; navigation serves memory.  
5. **Silence is a feature.** No ambient theatre, no AI host until invited.  
6. **Companion, not cockpit.** Five destinations max for Product Proof; diagnostics stay backstage.

### Why this improves the product

It replaces “demonstrate architecture” with “deliver one irreplaceable feeling in five minutes.”

### Constitutional alignment

Law I identity; Laws IV/IX/XII consent, honesty, no ambient; Law VIII mounted Experience ≠ IPC sprawl.

### Timing vs engineering backlog

**Before** most Phase B/C modernisation. Experience refoundation of the PP loop outranks AgentToolGate / ModelProvider / RFC for *product* progress. Engineering backlog items that **protect** the loop (contracts, docs truth) may proceed in parallel after owner selects the next program; they must not dilute the loop.

---

# 2. First-Time User Journey

**Goal feeling at T+5 min:** *I interrupted on purpose and came back without being lied to.*

| Time | Stage | User sees / does | System does |
| --- | --- | --- | --- |
| 0–15s | Arrive | Place identity + one clear primary CTA | No wizard; no AI; no pilot pressure |
| 15–30s | Orient | Empty or sparse Moments; secondary CTA Continue if any exist | Honest empty state with structure |
| 30s–2m | First Save | Name → short scope → consent → optional handoff → Keep | Capture only after consent; persist Moment |
| 2–5m | First Continue | Select Moment → plan cinema → approve → result + handoff | Place/focus or honest skips; no relaunch fiction |
| Optional later | Guide | 3–5 screens: what Workspace is / isn’t / limits | No IPC required |
| Not yet | Check-in | Discoverable but not first-run centre | Pilot remains opt-in |

### Why it improves the product

Forces the first session to prove usefulness, not explain architecture.

### Constitutional alignment

Exercise of the recovery contract (Constitution §1.3); presentation boundary; honesty.

### Timing vs backlog

**Immediate product priority** — experience program after this definition is approved. Engineering debt (G3/G4) can wait unless it blocks the loop.

---

# 3. Returning User Journey

**Goal feeling:** *My Moments are waiting; I know whether to Save or Continue.*

| Situation | Default Home behaviour |
| --- | --- |
| Has Moments, no active crisis | Hero = most relevant Moment to Continue; Save secondary |
| User is about to leave | Hero = Save; Moments as memory satellites |
| Last Continue was partial | Surface honest residue (“3 placed, 1 skipped”) once, then quiet |
| User opens Check-in | Only then pilot instrumentation |

Returning chrome stays the same five destinations. No new primary tabs for AI/Canvas.

### Why it improves the product

Returns are the habit; first-run is only the onboarding of the habit.

### Constitutional alignment

Law I continuity; Law IX honesty on residual state; consistent navigation (Project Constitution §4.4).

### Timing vs backlog

Same experience program as first-run; returning heuristics are UX logic, not new platform services.

---

# 4. AI Introduction Strategy

### Phases

| Phase | When | AI role | Surface |
| --- | --- | --- | --- |
| **A — Absent** | First session / until one Continue completes | None in chrome | — |
| **B — Explainer** | During Continue preview | Plain-language *why skip / why attempt* from plan facts | Inline on plan items — not a chat panel |
| **C — Suggest** | After repeated Moments or explicit invite | “Want a reminder handoff template?” / layout tip | Permissioned suggestion card |
| **D — Assist** | User opens Assistant (unmounted today → future) | Goal → plan preview only | Behind permission; never default dock item until product-ready |

### Rules

- No chat dock icon in Product Proof until Phase C+ is earned.  
- No model vendor in the hero.  
- No inferred personality.  
- AI copy must cite evidence (“because restore identity missing”) or not speak.

### Why it improves the product

Stops AI from stealing the usefulness story; introduces it as trust amplifier.

### Constitutional alignment

AI governance sequence; Laws IV/X/XII; not autonomous agent (Constitution §1.2).

### Timing vs backlog

Phase B explainer can ship in experience work **before** ModelProvider (Phase C engineering). AgentToolGate / ModelProvider **before** any Phase D chat. Do not enable ambient “learning” to feed AI.

---

# 5. Workspace Capability Map

Capabilities classified for product (not engineering ownership matrix):

| Capability | Product role | PP visibility |
| --- | --- | --- |
| Save Moment | Core | Primary |
| Continue / restore plan + execute | Core | Primary |
| Handoff note | Core | Primary |
| Honest limits / partial outcomes | Core | Primary |
| Home memory of Moments | Core | Primary |
| Guide | Core support | Primary dock |
| Check-in / pilot measurement | Evaluation | Secondary (not first-run hero) |
| Capture scope consent | Core trust | Inside Save |
| Desktop observation (consented) | Core mechanism | Invisible except scope copy |
| PermissionGateway | Core trust | Felt as asks, not a settings religion |
| Audit | Core trust | Inspect later; not hero |
| Preferences | Support | Quiet |
| AI explain / suggest | Enhancer | Progressive |
| Automation contracts | Future | Hidden until earned |
| Operator / Canvas / Intelligence | Engineering | Never default PP |
| Ambient scheduler | Rejected by default | Off |
| Plugins marketplace | Rejected until real | Hidden |

### Why it improves the product

Makes capability exposure match identity instead of IPC inventory.

### Constitutional alignment

Law I / VIII mounted product; Law XII ambient off.

### Timing vs backlog

Map guides experience sequencing **now**. Engineering backlog continues to harden invisible trust layers (audit integrity, tool gate) **after** or **beside** experience, never as substitute for the loop.

---

# 6. Information Architecture

```
Workspace (one place)
├── Home          — memory stage + situational primary CTA
├── Save          — interrupt ritual (consent → name → handoff → keep)
├── Continue      — return ritual (choose → plan → approve → result)
├── Check-in      — pilot measurement (opt-in evaluation)
└── Guide         — what / isn’t / limits (static)
```

**Off-stage (not in IA for PP):** Operator, Canvas, Assistant, Intelligence, Settings dumps, model galleries.

**Objects over pages:** Moment, Scope, Plan, Outcome, Handoff — these are the nouns users learn.

### Why it improves the product

Reduces “five tabs of equal boredom” to a spatial story with clear nouns.

### Constitutional alignment

Existing PP destinations; Law VIII; Experience Refoundation place-before-page (presentation, not architecture change).

### Timing vs backlog

Experience program — before docs-convergence only if owner prioritises product; docs convergence still valuable for engineering truth but does not create usefulness.

---

# 7. Screen Hierarchy

### Priority stack (what may dominate a viewport)

1. **Place identity** (Workspace / product)  
2. **Anchor object** (primary Moment, Save write surface, Continue plan)  
3. **One primary action**  
4. **Satellites** (other Moments, secondary facts)  
5. **Quiet chrome** (dock)  
6. **Inspect / detail** (opt-in)  
7. **System messages** (errors, honesty) — calm, not banners of shame  

### Anti-hierarchy (forbidden in PP hero)

- Equal card grids  
- Pilot metrics as Home hero  
- AI chat as Home hero  
- Engineering IDs, digest strings, capability catalogues  
- Multi-CTA toolbars competing with Save/Continue  

### Why it improves the product

Creates one composition users can feel in under 30 seconds.

### Constitutional alignment

Law I success feelings; honesty without compliance wallpaper.

### Timing vs backlog

Immediate experience work; no kernel redesign required.

---

# 8. Navigation Philosophy

1. **Five destinations, stable forever for PP** — Home / Save / Continue / Check-in / Guide.  
2. **Dock is wayfinding, not a feature menu** — icons + accessible names; not a strip of products.  
3. **Deep links are situational CTAs**, not new tabs (e.g. Home → Continue this Moment).  
4. **Back is spatial**, not browser history theatre.  
5. **Discoverability of advanced power is progressive** — long-press / Guide / future Command surface — never day-one dump.  
6. **Developer surfaces are unreachable from PP chrome** without an explicit developer mode (not part of this definition’s default).

### Borrowed from Kiro (principle only)

Progressive exposure; additive power; operator chooses when to widen — translated to “user chooses when to meet AI / diagnostics.”

### Why it improves the product

Predictable navigation builds trust faster than novelty navigation.

### Constitutional alignment

Project Constitution navigation consistency; Law I anti-cockpit.

### Timing vs backlog

Experience program; no conflict with IPC tiering already done.

---

# 9. Capability Roadmap

Ordered by **product usefulness**, not engineering curiosity.

| Stage | Capability experience | Depends on eng? | Before/after remaining eng backlog |
| --- | --- | --- | --- |
| **P0** | Irresistible Save→Continue five-minute loop; Home as memory place | Mostly UX + existing IPC | **Before** Phase B/C; may precede docs/naming debt |
| **P1** | Returning-user situational Home; residual honesty after partial restore | UX | Before Phase B/C |
| **P2** | Plan explainer language (rules/templates OK; model optional) | Light copy / optional AI explain | Before ModelProvider |
| **P3** | Guide as sharp identity story (what we won’t do) | Content | Anytime; cheap |
| **P4** | Check-in demoted emotionally; still available for pilots | UX IA | Before treating pilot as product core |
| **P5** | Permissioned suggestions (non-desktop) | Suggestion + Gateway | After AgentToolGate design if AI-sourced |
| **P6** | Optional Assist surface | ModelProvider + ToolGate | **After** Phase B items 4–7 |
| **P7** | Automation / long-horizon domains | Platform expand | Only under Expand brief; never PP identity |

### Why it improves the product

Stops feature accumulation from outrunning the only loop that matters.

### Constitutional alignment

Stabilise → consolidate experience before expand; ambient stays off.

---

# 10. Product Refoundation Plan

### Diagnosis (owner-aligned)

| Layer | State |
| --- | --- |
| Architecture / kernel | Strong enough to demonstrate trust mechanics |
| Product Proof chrome | Exists but feels like a proof harness, not a companion |
| Emotional usefulness | Not yet earned in five minutes |

### Refoundation intent

Rebuild the **Experience expression** of the unchanged constitutional contract until the five-minute loop feels inevitable and calm.

### Recommended workstreams (definition only — not started)

1. **Loop cinema** — Save and Continue as one-composition rituals (anchor + one action).  
2. **Home as memory** — Moments with weight; situational primary CTA.  
3. **Honesty craft** — limits and skips as first-class UX, not footnotes.  
4. **AI deferral** — remove AI gravity from first session; add explainer later.  
5. **Pilot humility** — Check-in available, never the point of the product.  
6. **Guide as doctrine** — short, sharp, anti-surveillance / anti-agent positioning.

### Selection rule for the next execution program

After owner approval of this document, pick **exactly one**:

- **Preferred product program:** P0 Experience loop refoundation (Save→Continue→Home), or  
- **Preferred engineering program:** only if owner explicitly prioritises debt (e.g. docs convergence) over usefulness.

Do not run both as one program. Do not amend Constitution or Execution Protocol to “make room” for features.

### Success criteria for refoundation (product)

- First-time user can explain Workspace in one sentence after 30s.  
- First Save completed in ≤2 minutes without confusion about surveillance.  
- First Continue produces either restored place/focus or believed honesty.  
- No AI chat required to feel value.  
- Owner would keep the app for the loop alone.

---

# Recommendation ledger (every major recommendation)

| Recommendation | Improves product because… | Aligns with Constitution because… | Before / after remaining eng backlog |
| --- | --- | --- | --- |
| Centre usefulness on Save→Continue in 5 minutes | Proves irreplaceable value | Law I recovery contract | **Before** Phase B/C |
| Home as memory place + situational CTA | Removes proof-harness feeling | Law I / VIII presentation | **Before** |
| Defer AI until after first loop | Stops wrong category perception | Not an agent; AI sequence | **Before** ModelProvider |
| Keep Check-in secondary | Prevents pilot-as-product | PP identity ≠ evaluation tooling | **Before** |
| Progressive capability exposure | Reduces overwhelm | Law I anti-feature-pile | **Before** Expand |
| Plan explainer without chat | Trust without theatre | Law IX honesty | **Before** AgentToolGate |
| Stable five-destination nav | Predictability | Consistent navigation principle | Now |
| Hide Operator/Canvas/Assistant from PP | Clarity | Law VIII | Now (already mostly true; enforce in UX) |
| Docs convergence / WorkspaceState naming | Engineering clarity | Law XI | **After or parallel** — not a substitute for P0 |
| AgentToolGate / AuditIntegrity / ModelProvider | Future-safe AI | Laws IV/V/X | **After** P0–P2 experience, before AI Assist |

---

# STOP

This document is **product definition only**.

- Constitution V2 — untouched  
- Execution Protocol — untouched  
- No code, no architecture redesign  

**Handoff status for engineering:** wait for Project Owner review of this Product Proof Refoundation. The next execution program is chosen only after that review.
