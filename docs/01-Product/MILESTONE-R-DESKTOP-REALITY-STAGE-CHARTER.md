# Milestone R — Desktop Reality Stage Charter

| Field | Value |
|-------|-------|
| **Status** | **Charter approved** — Slice 1 implemented on `cursor/milestone-r-desktop-reality-stage-34a5`; **stop for human visual review** before further slices |
| **Date** | 2026-07-30 |
| **Branch (docs)** | `cursor/product-reality-alignment-audit-34a5` |
| **Milestone** | R — Desktop Reality Stage |
| **Authority** | [WORKSPACE-REFERENCE-INTERPRETATION.md](WORKSPACE-REFERENCE-INTERPRETATION.md), [WORKSPACE-PRODUCT-REALITY-ALIGNMENT-AUDIT.md](WORKSPACE-PRODUCT-REALITY-ALIGNMENT-AUDIT.md), [WORKSPACE-VISUAL-DIRECTION.md](WORKSPACE-VISUAL-DIRECTION.md) |
| **References** | [`references/workspace-concept-01.png`](references/workspace-concept-01.png), [`references/workspace-concept-02.png`](references/workspace-concept-02.png) |
| **Roadmap** | [WORKSPACE-PRODUCT-DELIVERY-ROADMAP.md](WORKSPACE-PRODUCT-DELIVERY-ROADMAP.md) — ship order **R → F → G → E → H → I** |
| **Audience** | Product, engineering, human reviewers |

**Do not modify application code from this document.**  
**Do not create new AI systems or engines.**  
**Do not start Milestone F, G, E, H, or I.**  
**Do not treat concept art as pixel requirements.**

---

## 0. Binding product hierarchy

```text
1. Desktop reality
   - observed applications
   - window identity
   - spatial representation

2. Workspace controls
   - arrangements
   - grouping (later — Milestone E)
   - modes (chrome now; OS apply — Milestone G)
   - restore

3. Assistant companion
   - explanation
   - questions
   - optional suggestions
```

Assistant remains **secondary**. Workspace is **not** an AI dashboard. The user does **not** create a workspace as the primary act; the app **represents** their existing computing environment.

---

## 1. User problem

A person already has a real desktop: running apps, windows with identity and geometry, monitors, and habits of arrangement.

They need a product that:

1. **Shows** that environment clearly (representation)
2. Lets them **organise and control** it without rebuilding it from forms
3. Optionally **remembers** setups and switches density (Flow / Focus)
4. Offers **help** without becoming the product

Today they get a management shell that asks them to invent named workspaces and register apps before the Stage becomes meaningful — which solves the wrong problem.

---

## 2. Current mismatch

| Expected (references + interpretation) | Current (post–Milestone D shell) |
|----------------------------------------|----------------------------------|
| Open app → see desktop reality | Open app → empty Stage gated on “create / choose workspace” |
| Observed windows + identity on stage | Registry tiles / placeholders as Stage hero |
| Controls act on live desktop | Controls and arrangements buried behind workspace entity gates |
| Assistant beside a full stage | Assistant companion correct in placement; content still “no workspace selected” |
| Identity: desktop control layer | Identity: workspace admin / setup dashboard |

Milestone D correctly demoted AI and made Stage the default tab. It did **not** deliver desktop reality representation. That is the gap Milestone R closes.

---

## 3. Target experience

### First session (success criteria)

A user opens Workspace and, without creating a named workspace:

1. Lands on **Stage**
2. Sees a **spatial representation** of observed applications / windows (or an honest “no windows observed yet” state of *that* reality — not “Go create a workspace”)
3. Understands window **identity** (what is running) sufficiently to act
4. Can reach **workspace controls** (arrangements remember/restore, launch/focus where already permissioned) without leaving the reality frame
5. Sees **Assistant** only as companion chrome — hideable; product remains intact without it

### One-line product promise

> “This is my desktop, represented and controllable — not a blank workspace I have to design.”

### Hierarchy in the UI (target)

```text
Stage (primary)
 ├── Desktop reality map / observed set     ← hero
 ├── Workspace controls (arrangements, …)   ← secondary rail / section
 └── Companion canvas (optional practice)  ← tertiary, clearly labelled

Shell chrome
 ├── Flow / Focus (presentation; OS apply = G)
 └── Assistant companion (optional)
```

Named Workspaces / profiles remain available as **labels over reality**, not admission tickets.

---

## 4. Existing systems reused

**No new engines. No new AI. Extend surfaces that already exist.**

| Concern | System | Use in Milestone R |
|---------|--------|--------------------|
| Observe windows / monitors | `packages/windows-integration` capture + identity; observation services; workspace state IPC (`get_workspace_state` and related) | **Primary Stage data source** |
| Window identity | HWND / identity model from DAF-1b | Stable keys for stage tiles / map nodes |
| Control | `WindowController` (`set_bounds`, `focus`) via CommandPipeline → Permission Gateway | Focus / controlled actions exposed from Stage where already safe |
| Remember / restore | `DesktopArrangement` + capture/restore IPC + `DesktopArrangementPanel` | Reframe as “remember this desktop”; remove workspace-first gate on *viewing* where possible without breaking persistence ownership |
| Launch | Application registry + `launch_application` + `applicationLaunch.ts` | Supporting action, not Stage sole content |
| Stage shell | `WorkspaceApplicationStage`, Focus helpers, Stage-first nav in `App.tsx` | Replace registry-first hero with observation-first hero |
| Running apps UI | `ActiveApplicationsView` / Applications “Running on the desktop” | Promote patterns into Stage; keep Applications tab as library + detail |
| Modes chrome | Flow / Focus (`workMode`, `WorkModeSwitch`) | Preserve; do **not** implement OS geometry apply (G) |
| Assistant | `AssistantCompanionRail` + existing read/compose panels | Preserve companion; adjust empty copy to desktop reality |
| Governance | Permission Gateway, IPC contracts, audit | All control remains gated |

### Explicit reuse rule

If a capability seems missing, first search observation → workspace state → arrangement → WindowController → existing UI.  
**Do not** add a parallel “reality engine,” “desktop AI,” or duplicate window stack.

---

## 5. Components changed (planned — not yet implemented)

Frontend product shell only unless a thin IPC read wiring gap is proven. Prefer extend over replace.

| Area | Likely change |
|------|----------------|
| `WorkspaceApplicationStage.tsx` + `layoutsStageUi.ts` | Hero = observed set; empty = no observation; demote “create workspace” CTA |
| `App.tsx` Stage empty / runtime banners | Copy and routing aligned to reality-first |
| `WorkspaceHome.tsx` + home copy helpers | Demote setup briefing; optional orientation; no create-first identity |
| `ApplicationsPanel.tsx` / `applicationsUi.ts` | “Choose a workspace first” demoted; observation not blocked for awareness |
| `DesktopArrangementPanel.tsx` / `desktopArrangementUi.ts` | Framing: remember live desktop; reduce gatekeeping for *understanding* |
| `WorkspaceSwitcher.tsx` / switcher copy | Profiles over reality — secondary IA |
| `AssistantCompanionRail` empty intel copy | “No windows / desktop context” rather than only “no workspace selected” |
| Stage presentation helpers (new small pure modules if needed) | Map observation DTOs → stage view models — **no new domain engines** |

Backend: **read/compose only** if Stage cannot yet consume observation payloads cleanly. No new services named as engines. No schema deletion.

---

## 6. Components preserved

| Preserve | Why |
|----------|-----|
| Stage-first default view and nav order (Milestone D shell) | Correct slot for the product |
| Assistant companion rail placement | Hierarchy layer 3 |
| Diagnostics / Developer tools group | Non-product |
| DAF WindowController + observation + arrangements stack | Layer 1–2 substrate |
| Permission Gateway + CommandPipeline | Non-negotiable control path |
| Flow / Focus chrome density behaviour | Mode affordance; G owns OS apply |
| Companion canvas codepaths | Tertiary practice — keep, don’t delete |
| Workspace entity / SQLite / switcher | Profile utility — demote IA, keep data |
| Application registry | Library / launch enrichment |
| Programme / projection infrastructure | Frozen expansion; not deleted |

---

## 7. Components demoted

| Demote | How (IA / copy / default path — not delete) |
|--------|-----------------------------------------------|
| Create-workspace as Stage/Home primary CTA | Secondary / profile naming |
| Named Workspaces as product centrepiece | Utility over live desktop |
| Registry tiles as sole Stage hero | Fallback when observation empty |
| “Choose a workspace first” as universal gate | Only where persistence truly requires workspace id — document why |
| Companion canvas peer positioning | Explicit tertiary under reality stage |
| Admin verb cluster (create, register, design) | Recovery / power-user language |
| AI / Assistant as identity | Companion only |

---

## 8. Non-goals (Milestone R)

- Pixel-perfect concept art (wallpaper, brands, glass dock, nine layouts)
- Milestone **F** arrangement editor craftsmanship
- Milestone **G** Flow/Focus OS geometry apply
- Milestone **E** window grouping / lock-together
- Milestone **H** audio mixer
- Live window **thumbnails** if OS capture is unavailable — use honest placeholders / metadata maps instead of fakes
- New AI, evidence, recommendation, or “desktop intelligence” engines
- Assistant-owned window movement
- Autonomous layout load without user intent + permissions
- Deleting workspace entities, canvas, or Assistant projections
- Replacing WindowController or inventing a second control stack
- Installed-app OS discovery product (may remain later; R uses **observation of running** reality first)
- Multi-monitor advanced policy product

---

## 9. Risks

| Risk | Mitigation |
|------|------------|
| Browser/Linux preview cannot show real HWND observation | Honest desktop-preview messaging; Windows human verification required for “done” |
| Observation IPC incomplete or Operator-only | Charter implementation batch must inventory existing commands first; thin read wiring only |
| Stage becomes a flat list (still “dashboard”) | Require **spatial** presentation (map by bounds/monitor or explicit layout of identities) — not only a table |
| Fake thumbnails erode trust | Ban fake live pixels; prefer identity + bounds + title |
| Workspace id still required for arrangement persist | Allow **viewing** reality without create; keep persist path honest when profile id is needed |
| Scope creep into F/G/E | Hard non-goals; reviewers reject arrangement-editor and mode-apply work in R PRs |
| Copy-only change without data wiring | Charter success needs observation-backed Stage, not slogans alone |
| Maintainability regression (god Stage component) | Extend with single-responsibility view-model helpers; avoid duplicate observation state |

---

## 10. Human review points

Review against [WORKSPACE-REFERENCE-INTERPRETATION.md](WORKSPACE-REFERENCE-INTERPRETATION.md) hierarchy — not pixel match.

| # | Checkpoint | Pass if |
|---|------------|---------|
| H1 | First paint on Stage | Desktop reality (or honest no-windows-observed) — **not** create-workspace void |
| H2 | Spatial representation | Observed apps/windows readable as a set in space (map or spatial tiles), not only admin forms |
| H3 | Identity | User can tell *which* real apps/windows are represented |
| H4 | Controls secondary | Arrangements / modes / restore visibly serve the reality stage |
| H5 | Assistant tertiary | Hide Assistant → product still makes sense |
| H6 | No AI dashboard | No programme/chat-first identity |
| H7 | Profiles demoted | Workspaces create/switch is optional, not the onboarding funnel |
| H8 | Honesty | No fake thumbnails; runtime limits clear |
| H9 | Windows smoke | On Windows Tauri build, observation populates Stage (separate from Vite preview) |

**Media:** screenshots preferred; no review videos unless debugging.  
**Policy:** [HUMAN-REVIEW-POLICY.md](../03-Engineering/HUMAN-REVIEW-POLICY.md).

---

## 11. Maintainability considerations

| Rule | Application to R |
|------|------------------|
| Single responsibility | Stage presents reality; observation fetch stays in existing IPC/hooks; copy in `*Ui.ts` helpers; no mega-component rewrite |
| Extend, don’t replace | Evolve `WorkspaceApplicationStage` rather than a parallel RealityApp |
| No duplicate state | One observation/workspace-state source feeding Stage; don’t fork a second “running apps” store |
| No temporary abstractions | Avoid `IRealityProvider` layers; map DTOs in small pure functions |
| No new engines | No `DesktopRealityEngine`, no Assistant layout planner |
| Document decisions | Completion report must cite this charter + interpretation hierarchy |
| Architecture governance | Pass existing `verify:architecture-governance` / IPC / UI-boundary checks |
| Demote ≠ delete | Workspace/canvas/registry remain for later milestones and recovery |

---

## 12. Suggested implementation slices (for later authorisation)

Ordered for a future coding batch — **not authorised yet**:

1. **R0 — Framing** — Copy/IA demotion of create-workspace-first on Stage/Home/Assistant empty states.  
2. **R1 — Observation on Stage** — Wire Stage hero to existing observation / workspace state; spatial view model.  
3. **R2 — Control entry points** — Expose existing permissioned focus/launch/remember from Stage without new APIs where possible.  
4. **R3 — Gate audit** — Remove unnecessary workspace gates; document any remaining persistence requirement.  
5. **R4 — Validation + human review package** — typecheck/tests/governance; Windows smoke; screenshots.

Each slice: one PR concern, reuse systems, no Milestone F/G scope.

---

## 13. Success definition (charter acceptance → later implementation Done)

**Charter accepted when:** humans agree this plan matches hierarchy and constraints.

**Implementation Done (future) when:**

- [ ] Stage default content is observation-backed desktop reality (or honest empty observation)
- [ ] Create-workspace is not the primary empty-state story
- [ ] Assistant remains companion-only
- [ ] No new AI/engines; existing control/observation/arrangements reused
- [ ] Non-goals respected (no F/G/E/H)
- [ ] Automated validation green; Windows review for observation claims
- [ ] Human review H1–H8 pass

---

## 14. Stop

Milestone R **charter complete**.  

**Await human approval of this charter before any application code changes.**  
Do not implement, redesign UI, or start later milestones from this file alone.
