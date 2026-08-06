# Architecture Consolidation Plan

| Field | Value |
| --- | --- |
| **Purpose** | Simplify Workspace architecture without reducing Product Proof capability |
| **Parent** | `docs/workspace-strategic-review.md` |
| **Date** | 2026-08-07 |
| **Constraint** | Plan only — no refactors in this task |

---

## Goal

Preserve **Save → Plan → Execute** trust and the **PermissionGateway** platform while removing **accidental complexity**: duplicate ownership, unused pathways that create gravity, and documentation that fights implementation.

---

## Necessary vs accidental complexity

### Necessary complexity (keep)

| Complexity | Why necessary |
| --- | --- |
| CommandPipeline → PermissionGateway → Audit | Constitutional safety structure |
| Separate `windows-integration` crate | Testability + sole OS authority |
| Explicit capture admission + single-flight | Privacy + concurrency safety |
| Derived WorkspaceState vs durable observations | Read models must not become write authorities |
| RE vs DE persistence split (if product keeps both) | Prevents suggestion engines from executing |
| Experience catalog freeze for mounted chrome | Behavioural contract / evidence continuity |
| Session recovery fences | Crash honesty |

### Accidental complexity (reduce)

| Complexity | Why accidental | Consolidation move |
| --- | --- | --- |
| Dual docs authority (`docs/` Phase 1 vs `architecture/32_*` / V2) | Historical layering | Single status page; phase language update |
| ~197 IPC without product tier labels | Growth without retirement policy | Label tiers A/B/C in living IPC map; quarantine policy |
| Unmounted OperatorConsole / Intelligence / Assistant | Second product gravity | DEV-only gate or archive policy |
| CanvasShell retained | Superseded by WorkspaceShell | Mark LEGACY; eventual delete after zero refs |
| Cognition `generate_*` explosion vs mounted UI | Engineering ahead of product mount | Freeze new generators; require mount brief |
| Hand `domain.ts` | No codegen | Contract pipeline (MODERNISE) |
| Kernel vs domain `WorkspaceState` name | Education tax | Document qualification; rename only in later MODERNISE ADR |
| Dormant ambient scheduler + unwired startup trigger | Retained “just in case” | Keep code but document DORMANT; no new ambient callers |
| Kernel README exclusions vs AI services | Doc lag | Align README with runtime-service-map |
| V1 freeze vs V2 refoundation narrative | Concurrent programmes | Keep both tracks; forbid conflation in briefs |

---

## Duplicate responsibilities

| Overlap | Owners | Consolidation stance |
| --- | --- | --- |
| Desktop truth | Observations/SQLite vs UI state | UI never authoritative — already true; enforce in reviews |
| Session concepts | `WorkspaceSessionStore` vs `generate_workspace_session` | Rename-in-docs now; API rename only via ADR |
| Human decisions | Recommendation / Decision Engine / Decision Queue | Preserve ownership split; forbid new fourth overlay |
| Layout | `layoutPersistence` (unmounted Canvas path) vs PP chrome | Layout IPC remains platform; Canvas path LEGACY |
| Explanations | Kernel catalog vs UI DisplayReason | Keep single catalog SoT (already) |
| Adaptation | Kernel `WorkspaceAdaptationService` vs `app/src/experience` adaptation | Different layers (proposal vs presentation); document boundary |

---

## Overlapping ownership (resolve in policy, not code yet)

| Asset | Current writers | Policy |
| --- | --- | --- |
| Mounted product behaviour | Pilot panels + experience catalog | Only PP-authorised briefs may change |
| Diagnostic behaviour | Unmounted panels | May call Tier B IPC; must not ship as default chrome |
| Presentation adaptation | `experience/*` localStorage | Must not alter Save/Resume IPC semantics |
| Evidence JSON | Windows/kernel tests | Regenerate intentionally; don’t mix with doc commits |

---

## Unused pathways

| Pathway | Status | Consolidation |
| --- | --- | --- |
| ObservationScheduler enabled ambient | Disabled at init | Leave disabled; no product enablement without ADR |
| ObservationStartupTrigger | Unwired | Leave dormant; document |
| Automation `Scheduled` auto-fire | Never auto-fires | Keep non-executing |
| Plugin runtime | Absent | Do not scaffold until host design |
| Quarantined IPC (Tier C) | Registered | Keep for parity until delete ADR; do not build UI on them |
| ElevatedCard (removed) | Gone | No action |

---

## Over-engineering without product value

| Surface | Symptom | Plan |
| --- | --- | --- |
| Cognition generator matrix | High maint. cost, unmounted | Moratorium on new `generate_*` without mounted consumer brief |
| AI orchestration in-memory stores | Diagnostic depth | Cap expansion; WRAP model SDK later if needed |
| DEV certification proliferation | V2 sprints 72–74 already shifted to confidence | No new certification systems (handoff constraint) |
| Experience screenshot archive growth | ~137 MB research assets | Keep as evidence; don’t treat as cache; LFS decision later |

---

## Consolidation workstreams (no implementation now)

### C1 — Authority narrative

- Update `docs/README.md` current phase to match V1 baseline + V2 tip.
- Point contributors to knowledge maps for runtime truth.
- Accept Architecture Guardian only if owner wants process gate (`architecture/14_*` proposed).

### C2 — IPC tier policy

- Publish Tier A (Experience catalog), Tier B (diagnostic), Tier C (parity) as governance.
- Rule: new commands require tier + mounted consumer or explicit experimental label.
- Do **not** delete commands in Stabilise.

### C3 — Legacy UI gravity

- Policy: OperatorConsole / Intelligence / Assistant / CanvasShell are LEGACY/DEV.
- Success: default App mount tree unchanged; docs stop calling Canvas primary.

### C4 — Contract consolidation

- Follow `domain-contract-assessment.md` pipeline design in Modernise.
- Until then: treat Rust domain as SoT; TS drift is known risk.

### C5 — Experimental freeze

- Align with V2 handoff: no new architectural layers.
- Experimental services may bugfix but not expand surface without brief.

---

## What consolidation must not do

- Must not weaken PermissionGateway.
- Must not enable ambient capture “for convenience.”
- Must not merge RE into DE.
- Must not move Win32 calls into UI or kernel-without-boundary.
- Must not replace Product Proof chrome with diagnostic consoles.
- Must not adopt external platforms as authority owners.

---

## Success criteria for consolidation

| Criterion | Measure |
| --- | --- |
| Single eng status story | `docs/README` phase matches baseline/V2 tip |
| IPC tiers documented | Living map + contributor rule |
| Legacy UI labelled | Knowledge map + strategic review agree |
| No PP behaviour change | Experience E2E evidence still meaningful |
| No new generators | PR policy / handoff compliance |
| Contract path chosen | ADR or Decision Log entry for codegen approach |

---

## Dependency on roadmap stages

| Workstream | Stage |
| --- | --- |
| C1, C2, C3 | Stage 1 Stabilise |
| C4 start, C5 enforce | Stage 2 Consolidate |
| Codegen implement, invariant harden | Stage 3 Modernise |
| Selective experimental promotion | Stage 4 Expand |
