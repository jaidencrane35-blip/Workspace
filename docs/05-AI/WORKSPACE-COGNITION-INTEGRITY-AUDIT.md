# Workspace Cognition Integrity Audit (Phase 5.5)

| Field | Value |
|-------|-------|
| **Date** | 2026-07-26 |
| **Scope** | Cognition stack through Sprint 92 (Readiness) |
| **Mode** | Architectural hardening — no new features |
| **Verdict** | Foundation is sound enough for Phase 6 with targeted hardening |

---

## 1. Architecture health score

**Overall: 78 / 100 — Healthy with known debt**

| Dimension | Score | Notes |
|-----------|------:|-------|
| Single responsibility | 82 | Layers mostly unique; OS / Readiness / Attention edge overlap |
| Source of truth | 75 | Aggregators mostly clean; DQ overlays + TG sync-on-read are exceptions |
| Projection quality | 72 | OS is a valuable unify layer; some Intelligence legacy mirrors remain |
| Dependency acyclicity | 85 | Intelligence shared-input path is acyclic; standalone `generate` diamonds are costly |
| Authority discipline | 80 | No launch/execute creep; write-on-read overlays + DQ accept→proposal need framing |
| Naming clarity | 70 | Dual “Recommendation” surfaces; Health vs Readiness |
| Extensibility | 78 | Governance spine supports Phase 6; restore/automation must stay Intent→Gateway |
| Developer experience | 74 | Per-model docs exist; missing single cognition map (addressed here) |

---

## 2. Strengths

1. **Permanent governance spine is intact** — Human Intent → Planning → Command Pipeline → Permission Gateway → Execution → Audit.
2. **Aggregators declare `authority_effect: none`** and most Phase 5 layers hard-fail `attempt_execute`.
3. **Intelligence shared-input path** avoids circular `generate` recursion via `generate_with_inputs` + `enrich_*`.
4. **`PLATFORM_CONCEPT_OWNERS`** records aggregator vs durable ownership (Task Graph / Memory / Preferences).
5. **Decision Queue `aggregate_readonly`** correctly prevents nested overlay spam from Activity / Continuity consumers.
6. **Vocabulary distinctions are documented** for Queue vs Engine, Health vs Readiness, Adaptation vs Recommendation.
7. **Accept paths for Adaptation / Decision Engine return handoffs** — they do not bypass Gateway.

---

## 3. Weaknesses

1. **Dual recommendation surfaces** — Intelligence `recommended_actions` (Attention tops) vs `recommendation_engine` (typed next-steps).
2. **Operating State ≈ situational supersummary** of Continuity + Attention + Environment — valuable, but easy to misuse as a second SoT.
3. **Standalone `generate()` diamonds** — Readiness/Adaptation/Pattern each rebuild Operating State (and more) when called outside Intelligence.
4. **Work UI Promise.all fan-out** — initial load regenerated ~15 standalone aggregators in parallel with Intelligence.
5. **Incomplete `attempt_execute` coverage** — Intelligence, Decision Queue, Attention, Continuity, Activity lacked architecture-guard stubs.
6. **No dedicated `GateIntelligenceRead`** — Intelligence was gated only via workflow/workspace queries.
7. **Adaptation status overlays are process-local** while Decision Queue / Decision Engine overlays are durable — lifecycle inconsistency.

---

## 4. Redundant systems

| Item | Verdict |
|------|---------|
| Intelligence `pending_approvals` / `blocked_actions` / `pending_plans` | **Keep as derived mirrors** of Decision Queue (not a second scan) — deprecate from new product UI gradually |
| Intelligence `recommended_actions` | **Keep temporarily**; product copy must say “Attention priorities”, not Recommendation Engine |
| Operating State vs Readiness | **Keep both** — OS = “what is happening”; Readiness = “can I continue” |
| Pattern vs Evolution | **Keep both** — Evolution = change narrative; Pattern = recurrence observation |
| Adaptation vs Recommendation | **Keep both** — RE = next step; Adaptation = structural improvement proposal |

**No layer should be deleted before Phase 6.** Consolidation candidates are documentation and UI surface reduction, not domain deletion.

---

## 5. Projection candidates for consolidation

| Candidate | Recommendation |
|-----------|----------------|
| Merge OS into Intelligence | **Reject** — OS is a reusable situational projection for Pattern/Readiness/Adaptation |
| Merge Readiness into OS | **Reject** — distinct product question (“prepared?” vs “happening?”) |
| Merge Attention into Decision Queue | **Reject** — Queue is inbox; Attention is prioritization across Continuity/Activity/etc. |
| Drop Intelligence legacy mirrors | **Phase 6 UX cleanup** — keep fields for compatibility; stop featuring them in Work copy |
| Collapse standalone UI regenerates | **Do now (hardening)** — load Intelligence once; refresh full states on demand |

---

## 6. Naming concerns

| Term | Risk | Guidance |
|------|------|----------|
| Recommendation | High confusion | Prefer **Recommendation Engine** vs **Attention priority** in UI |
| Decision | Medium | **Decision Queue** = inbox; **Decision Engine** = ranked candidates |
| Adaptation | Low–Med | Always “proposal”; never “apply” |
| Readiness vs Health | Medium | Health = kernel lifecycle; Readiness = work preparedness |
| Operating State | Low | Keep; do not rename to “status” (collides with task status) |
| Pattern | Low | Not ML profiling; observational only |

---

## 7. Authority concerns

| Concern | Severity | Disposition |
|---------|----------|-------------|
| Intelligence calls `DecisionQueueService::generate` (overlay write-on-read) | High framing / Med effect | **Intentional sole-writer sync** on product Intelligence path; nested consumers must use `aggregate_readonly`. Documented; do not silently change without DQ UX plan. |
| `GateDecisionQueueRead` wrapping overlay-persisting `generate` | High framing | Document; Phase 6 may introduce `GateDecisionQueueSync` if gate taxonomy is refined |
| Decision Queue accept → Intent Proposal mutation | Medium | Intentional delegation for proposal sources; permissions remain handoff-only |
| Task Graph sync-on-generate | Medium | TG is `DurableStore`, not a pure aggregator — keep ownership explicit |
| Adaptation accept handoff | Low | Correct — Intent only |
| Hidden launch/execute from aggregators | None found | Maintain `attempt_execute` regression suite |

---

## 8. Dependency concerns

### Canonical Intelligence order (acyclic)

```
Decision Queue → Activity → Continuity → Task Graph
→ Environment → Composition → Purpose → Evolution
→ Attention(base) → Recommendation Engine(base)
→ Attention+RE → Operating State → Pattern
→ RE+patterns → Attention+patterns → Readiness
→ RE+readiness → Attention(final) → Adaptation
→ Memory/Prefs → Decision Engine → assemble
```

### Standalone diamonds (cost)

- `ReadinessService::generate` → OS + Pattern (Pattern → OS again)
- `AdaptationService::generate` → Pattern + RE + OS (+ DQ/AG rebuilt for Continuity/Composition)
- `PatternService::generate` → OS (which rebuilds Attention+RE)

**Rule:** Product paths must prefer Intelligence `generate_with_inputs`. Standalone generates are diagnostics / IPC refresh only.

---

## 9. Performance concerns

| Concern | Severity | Action |
|---------|----------|--------|
| Work tab parallel standalone regenerates | High | Hardened: initial load uses Intelligence + interactive surfaces only |
| Standalone Pattern/Readiness/Adaptation diamonds | Medium | Document; do not prematurely cache — share inputs via Intelligence |
| Audit fan-out per generate (`.generated` + `.updated`) | Low | Acceptable for foundation; revisit if audit volume becomes operational pain |
| Decision Engine `transition` → standalone `generate` | Medium | Phase 6 candidate: inject shared inputs into transition |

---

## 10. Future Phase 6 risks

| Capability | Risk if ignored | Required constraint |
|------------|-----------------|---------------------|
| Workspace restoration | Adaptation/Readiness mistaken for auto-prepare | Restore = human Intent → Gateway only |
| Governed automation | Contracts bypass Decision Queue | Automation stays proposal → Queue → Intent |
| Plugins | Private execution paths | Plugins must enter Command Pipeline + capabilities |
| Multiple desktops | Environment becomes SoT | Environment remains read model; Composition remains logical |
| Cloud sync | Overlay/process-local Adaptation lost | Prefer durable overlays or explicit sync model |
| Collaboration | Decision ownership ambiguity | Queue items need actor/workspace scoping (already partial) |
| Mobile companion | Duplicate Intelligence rebuilds | Companion must consume projections, not re-own work |

---

## 11. Concrete hardening (this audit)

Implemented in Phase 5.5:

1. This audit document + interactive review canvas.
2. Cognition ownership / assemble-order map in Platform Coherence.
3. Vocabulary clarification for Attention priorities vs Recommendation Engine.
4. `GateIntelligenceRead` for Intelligence queries.
5. `attempt_execute` architecture guards for Intelligence, Decision Queue, Attention, Continuity, Activity Graph.
6. Work Intelligence Panel: remove initial-load Promise.all fan-out of pure projections; refresh-on-demand remains.

**Explicitly deferred (not silent redesign):**

- Changing Intelligence away from DQ sole-writer `generate`
- Merging or deleting any cognition layer
- Durable Adaptation overlays
- Aggressive projection caching

---

## Final principle check

| Principle | Status |
|-----------|--------|
| Understand | ✓ Environment → Pattern |
| Explain | ✓ Continuity, Purpose, Evolution, OS |
| Recommend | ✓ Recommendation Engine (+ Attention priorities) |
| Propose | ✓ Adaptation |
| Assess | ✓ Readiness |
| Never bypass governance | ✓ (with documented overlay/delegation caveats) |

The stack can support the next several phases **without structural redesign**, provided Phase 6 features enter through Intent → Gateway and treat cognition layers as read-only consumers.
