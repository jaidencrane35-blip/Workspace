# Programme IV Maintainability & Architectural Integrity Audit

**Status:** Complete — gate before Batch 10  
**Date:** 2026-07-29  
**Baseline tip audited:** `cursor/programme-iv-evidence-reliability-34a5` @ `96d57ee`  
**Audience:** Engineering leads, maintainers, commercial due-diligence

---

## Verdict

Programme IV Batches 1–9 are **architecturally sound on ownership** (observational evidence layers; `load_snapshot` only; no SoT elevation found in Batches 7–9). They are **commercially maintainable with caveats**: clear folder ownership, strong contract docs, but **high copy-paste inflation** across domain/repo/service/governance/React surfaces.

**Batch 10 may proceed** after accepting the actions classified below. No Programme IV ownership breach requires a blocking fix. Two kernel failures are **pre-existing, unrelated to Programme IV**, and must remain documented debt (not silenced by architecture changes).

---

## A. Technical debt findings

### A1. `case5_timeline_deterministic` (Activity Graph)

| Field | Finding |
|---|---|
| Location | `packages/kernel/src/commands/workspace_activity_tests.rs` |
| Service | `WorkspaceActivityService` (`packages/kernel/src/services/workspace_activity.rs`) |
| Classification | **Unstable / outdated determinism expectation** (self-affecting inputs) |
| Programme IV related? | **No** |
| Architectural drift? | Local to Activity Graph audit gap-fill — not Programme IV |

**Root cause**

1. Timeline construction calls `adapt_audit_gap_fill`, which reads `AuditService::list_recent(db, 40)`.
2. `generate` then writes four audit events (`workspace.timeline.generated`, `workspace.relationship.generated`, etc.).
3. Those writes are filtered from gap-fill by `covered_event_types`, but they still **consume slots in the fixed 40-event window**.
4. On a second `generate`, an older seed audit that appeared in the first timeline can fall out of `list_recent(40)`, so timeline IDs differ (extra `activity:audit_signal:*` on first run).

**Evidence:** failure shows first timeline with two audit signals vs second with one; durable work IDs (project/task/contract/proposal) are identical.

**Recommended remediation** (do not change Programme IV)

1. Prefer: assert determinism on non-audit activities only, **or**
2. Snapshot audit input set before generate and reuse it for the test, **or**
3. Raise/fix the audit window contract and document that gap-fill is “recent operational signals,” not a closed deterministic set.

**Action:** Accept as documented debt · Schedule later (Activity Graph hardening sprint)

---

### A2. `case11_evaluation_does_not_contaminate_its_own_inputs` (Intelligence / Recommendation)

| Field | Finding |
|---|---|
| Location | `packages/kernel/src/commands/workspace_evolution_tests.rs` |
| Surfaces | Workspace Intelligence → Attention + Recommendation Engine |
| Classification | **Incomplete historical fix / residual feedback non-determinism** |
| Programme IV related? | **No** |
| Architectural drift? | Cognition stack (Recommendation/Attention), not evidence engines |

**Root cause**

- Sprint 128 (`a4a098a`) aimed to stop evaluation from feeding itself by filtering telemetry before windowing.
- Test still fails on **attention `top_items`**, not evolution insights: consecutive `generate_workspace_intelligence` calls with unchanged work produce different recommendation-derived attention IDs.
- Observed delta: first cycle surfaces `recommendation:from_readiness:*` items; second surfaces `recommendation:restore_context:*` / `complete_task:*` (and different lower-score set).
- Recommendation generation (`workspace_recommendation.rs`) merges Continuity restore-context candidates and Readiness gap enrichment (`enrich_with_readiness`). Ordering/top-N selection remains sensitive to intermediate state written by the previous intelligence cycle (audit volume, recommendation overlays, or readiness-derived IDs), despite Sprint 128 filters.

**Recommended remediation**

1. Trace which inputs differ between cycle 1 and 2 (audit, persisted recommendation lifecycle, readiness gap IDs).
2. Extend Sprint 128 filtering so Attention/Recommendation top-N is closed over durable work evidence only.
3. Keep the test — it is a valuable anti-contamination contract; fix the product path, not the assertion.

**Action:** Accept as documented debt · Schedule later (cognition integrity sprint). **Do not** weaken Programme IV contracts to compensate.

---

### A3. Other maintainability risks

| Risk | Severity | Notes |
|---|---|---|
| Programme IV clone inflation (domain ~1.4–1.7k LOC × 9 modules; ~25k LOC evidence family) | **High** | Envelope/status/history/projection duplicated; classification differs |
| Repository clones (~330–350 LOC × 8) | **High** | Identical persist/supersede/history API |
| Kernel service scaffolds + long `compose_view` chains | **High** | Growing O(n) upstream fan-out per batch |
| Governance `evidence*Guards` × 8 near-copies | **High** | Should be data-driven |
| React `evidence*Projection.ts` helpers | **Med** | Used almost only by `tests/projection-integrity.test.ts` — ceremonial for product UI today |
| `packages/domain/src/lib.rs` flat export surface | **Med** | Hard to discover; every DTO re-exported at crate root |
| `docs/05-AI/README.md` incomplete Programme IV index | **Med** | Lists Reliability + Programme IV only; omits Batches 1–8 doc links |
| `main` may not compile (`resilience_validation.rs` DecisionCandidateProgression / score) | **High (platform)** | Documented in `AGENTS.md`; blocks green `main` CI independent of Programme IV |
| Linux `workspace-database` TempDir `READONLY_DBMOVED` | **Low (platform)** | Documented; Windows CI semantics differ |

---

## B. Architecture health

### Ownership boundaries (Batches 7–9)

| Batch | Service | Integrity |
|---|---|---|
| 7 Freshness | `WorkspaceEvidenceFreshnessService` | **Pass** — `load_snapshot` only; negative refresh/schedule guards |
| 8 Completeness | `WorkspaceEvidenceCompletenessService` | **Pass** — observes omissions; no fill/repair |
| 9 Reliability | `WorkspaceEvidenceReliabilityService` | **Pass / watch** — observational only; risk is *conceptual misreading* as trust authority if UI/docs weaken |

No duplicated lifecycle systems introduced. No accidental replacement of Decision/Recommendation/Execution authority. Repositories remain persistence-only. Commands remain CommandPipeline + PermissionGateway gated.

### Duplication risks

- **Pattern:** each batch adds nearly isomorphic domain + migration + repository + service + 4 commands + tests + governance guard + React helper + architecture doc.
- **Justification:** deliberate contract isolation (each answer is a separate observational question).
- **Cost:** Batch 10+ will worsen fan-out (each new layer loads all prior `load_snapshot`s) unless shared scaffolding is extracted **without** collapsing ownership.

### Complexity assessment

- Conceptual model: **clear** (Programme IV roadmap answers distinct questions).
- Implementation cost per batch: **high boilerplate, low novel logic**.
- Runtime coupling: **increasing** (Reliability already loads 15 upstream surfaces).
- Extensibility without consolidation: **poor beyond ~Batch 12**.

---

## C. Human maintainability score

| Dimension | Score (1–10) | Rationale |
|---|---|---|
| Code organisation | **8** | Domain / kernel / database / app / docs boundaries clear and consistent |
| Documentation | **7** | Per-batch architecture docs strong; AI index incomplete; weak code-path pointers |
| Naming | **8** | Services/commands/migrations match ownership; vocabulary table helps |
| Extensibility | **5** | New batch is copy-paste + longer upstream list; no shared evidence artefact kit |
| Complexity control | **5** | Ownership simple; implementation volume and fan-out growing faster than insight |

**Overall commercial readiness:** **6.5 / 10** — sellable with an engineering playbook and a consolidation sprint; not yet “thin and elegant.”

Could a new senior engineer find where a feature belongs? **Yes** via Programme IV map → architecture doc → `workspace_evidence_*` folders.  
Would another company maintain this? **Yes, if** they treat Batches as contract modules and invest in shared scaffolding before more layers.

---

## D. Required actions

### Fix now before Batch 10

| Item | Why |
|---|---|
| Document this audit in-repo (this file) | Makes debt and ownership explicit for humans |
| Expand `docs/05-AI/README.md` Programme IV index to Batches 1–9 | Navigation integrity |
| Freeze Batch 10 design constraint: **no new isomorphic clone without shared helper extraction plan** | Prevents inflation |

### Accept as documented debt

| Item | Why |
|---|---|
| `case5_timeline_deterministic` | Unrelated Activity Graph audit-window instability |
| `case11_evaluation_does_not_contaminate_its_own_inputs` | Unrelated cognition feedback residue |
| One migration per evidence batch (064–072) | Already shipped; splitting was intentional governance |
| Separate evidence tables per batch | Preserves supersede/history isolation |

### Schedule later (before or during early Batch 10–12)

| Item | Approach (preserve contracts) |
|---|---|
| Shared evidence domain kit | Traits/macros for Status, HistoryEntry, Projection assemble, non-actionable validate |
| Generic evidence repository adapter | Table/column config + mappers; keep typed public repos |
| Kernel evidence service scaffold | Shared generate/load/explain/rollback; keep per-batch `compose` |
| Data-driven governance guards | One `evidenceGuard({…})` replacing eight copies |
| React projection helper consolidation or deletion | Single typed helper **or** drop until UI consumes them |
| Grouped domain prelude exports | `workspace_domain::evidence::reliability` style modules |
| Fix `main` compile (`resilience_validation.rs`) | Unblocks CI; independent of Programme IV |
| Activity Graph + Intelligence contamination fixes | Dedicated sprints; keep failing tests as contracts |

### Explicit non-actions

- Do **not** rewrite Programme IV ownership to silence case5/case11.
- Do **not** merge evidence tables/services into one “mega evidence” SoT.
- Do **not** add Batch 10 as another pure clone without addressing shared scaffolding (at least a written extraction plan in the Batch 10 PR).

---

## E. Consolidation opportunities (stable contracts)

Safe to extract without changing behaviour:

1. **Domain:** `EvidenceArtefactStatus`, history non-actionable checks, projection `assemble`, summary truncation, forbidden-phrase lists parameterized by engine.
2. **Database:** parameterized SQL for snapshot/history with engine-specific JSON columns.
3. **Kernel:** `generate_observational_artefact(repo, compose_fn, audit_event)`.
4. **Governance:** table-driven forbidden imports / foreign `generate` denylist.
5. **Docs:** single Programme IV “implementation map” linking doc → `packages/...` paths for each batch.

---

## F. Gate decision for Batch 10

| Gate | Status |
|---|---|
| Ownership integrity Batches 1–9 | Pass |
| Programme IV regressions in case5/case11 | None found |
| Blocking maintainability fix required | None (documentation + Batch 10 design constraint only) |
| Batch 10 implementation | **Cleared to start** under §D constraints |

---

## Explicit confirmation

This audit concludes that Workspace Evidence engines through Batch 9 observe recorded evidence characteristics only within their stated ownership. The two kernel failures are pre-existing Activity Graph / Intelligence issues, not Programme IV architectural drift. Commercial maintainability is adequate for continuation if Batch 10 proceeds with inflation controls rather than another unchecked clone wave.
