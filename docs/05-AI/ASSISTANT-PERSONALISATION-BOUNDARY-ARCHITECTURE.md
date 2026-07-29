# Assistant Personalisation Boundary Architecture (Programme IV — Batch 16)

**Status:** Active — implemented  
**Audience:** Architecture, Kernel, Frontend, Product, Governance, Security  
**Depends on:**  
- [Assistant Interaction Intelligence Architecture](./ASSISTANT-INTERACTION-INTELLIGENCE-ARCHITECTURE.md) (Batch 15 — accepted / implemented)  
- [Assistant Explanation Intelligence Architecture](./ASSISTANT-EXPLANATION-INTELLIGENCE-ARCHITECTURE.md) (Batch 14 — accepted / implemented)  
- [Assistant Retrieval Intelligence Architecture](./ASSISTANT-RETRIEVAL-INTELLIGENCE-ARCHITECTURE.md) (Batch 13 — accepted / implemented)  
- [Assistant Context Intelligence Architecture](./ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md) (Batch 12 — accepted / implemented)  
- [Conversational / Assistant Surface Architecture](./CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md) (Batch 11 — accepted / implemented)  
- [AI Personalization Foundation](./AI-PERSONALIZATION-FOUNDATION.md) (existing explicit preference owners)  
- [Workspace Evidence Observational Scaffold](./WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md) (Batch 10 — accepted)  
- [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md)  
- [Programme IV Maintainability Audit](../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md)

## Purpose

Batches 11–15 built a stacked assistant capability:

```text
Surface → Context → Retrieval → Explanation → Interaction
```

each remaining a presentation/coordination layer over existing Programme II–IV intelligence.

Batch 16 asks the next architectural question:

> **"How can the assistant adapt presentation preferences while remaining separate from identity, memory, user modelling, and autonomous behaviour?"**

This charter defines the **Assistant Personalisation Boundary Contract** — packaging and displaying **explicit, user-controlled** presentation preferences for assistant surfaces **without** hidden profiling, personality inference, behavioural prediction, identity conclusions, or autonomous adaptation.


---

## Primary principle

**Adapt presentation from explicit preferences. Never invent who the user is.**

Assistant personalisation:

- **packages** explicit presentation preference settings for assistant display
- **surfaces** user-controlled configuration and explainable adjustments
- **never** becomes a user model, personality engine, memory SoT, identity authority, behavioural predictor, or autonomous adapter

Capability growth must not equal code duplication (Batch 10 direction lock; Batches 11–15 Grade A acceptance). Batch 16 must evaluate reuse of:

1. Existing [AI Personalization Foundation](./AI-PERSONALIZATION-FOUNDATION.md) / `UserPreference` / preference profile owners (explicit preferences only)
2. Batches 11–15 assistant packages (presentation consumers of preference packaging)
3. Batch 10 observational scaffolding (`workspace_evidence_contract`, `evidenceProjectionContract`)
4. Existing workspace settings / working-style evidence only when already recorded and authorised — never as silent profiling

before introducing any new module, migration, or guard family.

---

## Status of this document

| State | Meaning |
|---|---|
| **Active — implemented** | Domain/kernel/database/React implementation accepted |
| **Depends on Batches 11–15 + existing personalization owners** | Composes explicit preference contracts — does not replace them |
| **Governed baseline** | Mutation baseline **88**; history/projection DTO inventory **39** |

---

## Relationship to prior systems

```text
Explicit user preferences / settings (existing owners)
        │
        ▼
Assistant Personalisation (Batch 16)  ← packages presentation prefs only
        │
        ├── Assistant Surface / Context / Retrieval / Explanation / Interaction
        ├── load_snapshot / existing preference read paths only
        ▼
Human-visible configuration & presentation adaptation
Never → hidden profile / inferred personality / autonomous behaviour change
```

| Concern | Prior owner | Batch 16 (this charter) |
|---|---|---|
| Explicit user preferences | AI Personalization Foundation / preference commands | Consumes — does not reimplement preference SoT |
| Assistant presentation layers | Batches 11–15 | May apply packaged prefs for display only |
| Working style observations | Existing Working Style owners | Evidence only if already recorded — never silent profiling |
| Identity | Identity / actor owners | Forbidden |
| Memory | Memory owners | Forbidden as personalisation SoT |
| Intent / decisions / recommendations / execution | Existing authorities | Forbidden |

Batch 16 must **not** fork a user-model engine, preference-inference system, or hidden profile database when existing preference contracts can be composed.

---

## Ownership

### Owner

`WorkspaceAssistantPersonalisationService` — clear folder `packages/domain/src/workspace_assistant_personalisation/` for ownership clarification beside Batches 11–15. Rust spelling uses British `personalisation` for this Programme IV boundary while composing American `ai_personalization` foundation types.

### May own

| Owns | Meaning |
|---|---|
| Presentation preference packaging | Structured package of **explicit** assistant presentation prefs for display |
| Explicit user preference display | Show recorded preference values with provenance / source |
| Interface adaptation metadata | Non-authoritative metadata describing how UI may adapt (density, verbosity, citation detail) from explicit settings |
| Interaction style configuration | User-visible style knobs (e.g. concise vs detailed) only when explicitly set |
| Personalisation diagnostics | Missing prefs / disabled personalization / unavailable preference owner — diagnostic only |

### Must not own

| Does not own | Remains owned by |
|---|---|
| Hidden user modelling | Forbidden |
| Personality inference | Forbidden |
| Identity | Identity / actor owners |
| Durable memory | Memory owners |
| Behavioural prediction | Forbidden |
| Psychological profiling | Forbidden |
| Autonomous adaptation | Forbidden — no silent preference changes or self-tuning behaviour |
| Decision-making | Decision Engine / Queue |
| Preference SoT CRUD authority (unless explicitly routed through existing preference commands) | AI Personalization Foundation / preference command owners |
| Batches 11–15 ownership | Those remain package owners |

---

## Required reuse

### Must compose

| Contract | Role |
|---|---|
| Assistant Surface (Batch 11) | Presentation consumer |
| Assistant Context (Batch 12) | Scope / continuity consumer |
| Assistant Retrieval (Batch 13) | Retrieval presentation consumer |
| Assistant Explanation (Batch 14) | Clarity presentation consumer |
| Assistant Interaction (Batch 15) | Flow consumer of presentation prefs |
| AI Personalization Foundation / explicit preferences | Canonical explicit preference artefacts |
| Batch 10 scaffold | Digest / authority / projection helpers |

### Must not create

| Forbidden creation | Why |
|---|---|
| User model engine | Hidden modelling forbidden |
| Memory replacement | Memory owners remain SoT |
| Preference inference system | No automatic long-term inference (foundation already forbids) |
| Hidden profile database | Would create unauditable identity/memory |
| Autonomous adaptation loop | No self-directed behaviour change |

### Access rules

1. Preference reads: existing preference/query/`load_snapshot` paths only.
2. Preference writes: **only** via existing authorised preference commands owned elsewhere — never silent side effects of packaging.
3. Never invent preferences from behaviour, conversation content, or working-style observations.
4. If personalization is disabled upstream, packages must reflect disabled/neutral state — never override.

---

## Personalisation boundary

### Allowed

| Allowed | Meaning |
|---|---|
| Explicit preferences | User-defined / user-confirmed / imported sources only (per foundation) |
| User-controlled settings | Visible toggles and editable values |
| Visible configuration | Humans can inspect what drives presentation adaptation |
| Explainable adjustments | “Presentation uses preference X (source Y)” with provenance |

### Forbidden

| Forbidden | Why |
|---|---|
| Inferred personal attributes | No personality / demographic / psychological inference |
| Hidden tracking | No silent behavioural telemetry owned here |
| Behavioural manipulation | No dark-pattern adaptation |
| Silent preference changes | No auto-write of prefs from interaction |
| Identity conclusions | No “you are the kind of user who…” |

### Explicit vs inferred

| Explicit (allowed as input) | Inferred (forbidden) |
|---|---|
| User set “prefer concise citations” | Assistant concludes user is impatient |
| User confirmed imported layout preference | Assistant learns “style” from click streams here |
| Personalization disabled flag | Assistant ignores disable and adapts anyway |

---

## Authority boundaries

Assistant personalisation **must never**:

| Forbidden | Why |
|---|---|
| Infer identity or personality | No profiling authority |
| Replace memory | Memory owners |
| Predict behaviour | Forbidden modelling |
| Autonomously adapt without explicit prefs | Autonomous adaptation forbidden |
| Decide / recommend / execute | Existing authorities |
| Bypass PermissionGateway | Sole `require()` authority |
| Influence permissions or capability grants | Foundation permanent separation |
| Silently mutate preference SoT | Existing preference commands only |

**Authority effect:** all assistant personalisation packages use `authority_effect: "none"` and `actionable: false` unless a *separate*, existing preference mutation command is explicitly invoked by the human.

---

## Memory, identity, and preference separation

| Layer | Batch 16 role |
|---|---|
| Explicit preferences (existing SoT) | Read/display with provenance; write only via existing owners |
| Presentation preference package | Temporary/dual-channel evidence packaging only if accepted later |
| Memory artefacts | Never owned; never rewritten as prefs |
| Identity | Never concluded |
| Working-style observations | Evidence display only if already recorded — never converted into inferred prefs here |
| Batches 11–15 packages | May consume packaged prefs for presentation — not redefine them |

Displayed preference ≠ inferred identity ≠ durable memory ≠ Intent.

---

## UI boundary

### Projection-only

- React renders personalisation packages via Batch 10–15 projection helpers (extend; do not invent another parallel contract family without audit justification).
- Preference **edits** must route through existing preference command affordances — clearly labeled as user settings, not assistant agency.
- No hidden invoke from render paths.

### Information vs action

| Information (personalisation-owned) | Action (other owners) |
|---|---|
| “Explicit preference X is set (source user_defined)…” | “Update preference” via existing preference command |
| “Personalization disabled — presentation is neutral” | “Enable personalization” via settings owner |
| “No preference recorded for citation density” | Gap remains gap — no invented default-as-identity |

UI must not imply the assistant has profiled the user.

---

## Governance

### Permissions

- Personalisation package mutation: `work_context.write` (dual-channel assistant evidence only — does **not** write `user_preferences`)
- Personalisation package read / explain: `work_context.read`
- Preference mutations: existing `personalization.write` / settings owners only — **not** `assistant.personalisation.superuser`
- No capability grants issued by assistant personalisation
- Must never influence PermissionGateway decisions

### Audit expectations

- Observational events for preference packaging (e.g. `workspace.assistant.personalisation.packaged`) with workspace id, preference refs, `authority_effect: none`
- Never audit packages as identity conclusions, memory writes, or autonomous adaptations
- Append-only evidence only

### Provenance requirements

- Every applied presentation adjustment must cite an explicit preference id/source or state that no preference is recorded
- Disabled personalization remains visible
- Missing prefs produce diagnostics — never silent inference

---

## Maintainability requirements (before implementation)

Implementation (when unblocked) must document in the maintainability audit:

1. **Reusable contracts first**
   - Compose AI Personalization Foundation + Batches 11–15
   - Identify existing configuration/preferences contracts before new tables
   - Reuse `workspace_evidence_contract` and thin React wrappers
2. **Avoid subsystem clone**
   - No user-model engine / hidden profile DB / inference system
   - One governance guard entry via `EVIDENCE_ENGINE_GUARD_SPECS` if a new service file appears
   - No migration that duplicates `user_preferences` as “assistant profile memory”
3. **Folder ownership**
   - Clear naming (`assistant_personalisation` / `assistant_personalization` with documented alias)
   - Docs linked from Programme IV indexes
   - Minimal abstractions; composition over expansion
4. **LOC honesty**
   - Justify any new files against “capability ≠ duplication”
   - Prefer thin packaging over re-implementing preference CRUD

---

## Forbidden behaviour (explicit)

1. **Hidden user modelling** — no private psychographic/behavioural profile.
2. **Personality inference** — no trait conclusions from conversation or clicks.
3. **Silent preference writes** — no auto-save of inferred prefs.
4. **Identity conclusions** — no labelling who the user “is.”
5. **Autonomous adaptation** — no self-tuning presentation without explicit prefs.
6. **Permission influence** — preferences never grant capabilities.
7. **Clone-wave architecture** — no parallel personalization foundation wrapped as “assistant profile.”

---

## Out of scope for Batch 16

- Model provider / prompt engineering details
- Product UX layouts beyond preference/information distinction
- Resolving case5 / case11 (non–Programme IV debt)
- Replacing AI Personalization Foundation
- Replacing Batches 11–15
- Building recommendation or decision personalization beyond explicit prefs

---

## Acceptance criteria

- [x] Ownership / non-ownership tables are unambiguous
- [x] Reuse of AI Personalization Foundation + Batches 11–15 is explicit
- [x] Explicit vs inferred rules forbid hidden modelling and silent adaptation
- [x] Preference write path remains existing authorised owners only (`personalization.write` commands — not packaging)
- [x] Maintainability reuse path vs Batches 10–15 is explicit
- [x] Mutation baseline **88** / DTO inventory **39** for `PackageWorkspaceAssistantPersonalisation`
- [x] Status is **Active — implemented**

---

## Related documents

- [Assistant Interaction Intelligence Architecture](./ASSISTANT-INTERACTION-INTELLIGENCE-ARCHITECTURE.md)
- [Assistant Explanation Intelligence Architecture](./ASSISTANT-EXPLANATION-INTELLIGENCE-ARCHITECTURE.md)
- [Assistant Retrieval Intelligence Architecture](./ASSISTANT-RETRIEVAL-INTELLIGENCE-ARCHITECTURE.md)
- [Assistant Context Intelligence Architecture](./ASSISTANT-CONTEXT-INTELLIGENCE-ARCHITECTURE.md)
- [Conversational / Assistant Surface Architecture](./CONVERSATIONAL-ASSISTANT-SURFACE-ARCHITECTURE.md)
- [AI Personalization Foundation](./AI-PERSONALIZATION-FOUNDATION.md)
- [Workspace Evidence Observational Scaffold](./WORKSPACE-EVIDENCE-OBSERVATIONAL-SCAFFOLD-ARCHITECTURE.md)
- [Programme IV Interaction Runtime](./PROGRAMME-IV-INTERACTION-RUNTIME.md)
- [Workspace Vocabulary](./WORKSPACE-VOCABULARY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Programme IV Maintainability Audit](../03-Engineering/PROGRAMME-IV-MAINTAINABILITY-AUDIT.md)
- [Threat Model](../07-Security/THREAT-MODEL.md)

---

## Explicit confirmation

> Assistant Personalisation packages explicit, user-controlled presentation preferences for Batches 11–15.
> It never infers personality or identity, never creates hidden user models, never predicts behaviour,
> never autonomously adapts, never replaces memory or preference SoT owners,
> never influences permissions, never decides or executes, and never silently mutates preferences.
> Status is Active — implemented.
