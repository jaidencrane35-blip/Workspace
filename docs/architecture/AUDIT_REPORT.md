# Architecture Audit Report — Workspace

Date: 2026-07-27
Author: Automated architecture audit (pair programmer)

## Executive Summary

Overall architecture health: GOOD — PASS WITH RECOMMENDATIONS

Architecture score: 8.2 / 10

Major findings (high-level):
- Clear, intentional separation between Recommendation Engine (RE) and Decision Engine (DE). Evidence: typed domain artifacts and service implementations in `packages/kernel/src/services/workspace_recommendation.rs` and `packages/kernel/src/services/decision_engine.rs`, plus persistence repositories `packages/database/src/repositories/recommendation_lifecycle.rs` and `packages/database/src/repositories/decision_engine.rs`.
- Explicit IPC boundary implemented via Tauri: `app/src-tauri/src/lib.rs` registers a coherent IPC surface documented in `docs/03-Engineering/IPC-SURFACE.md`.
- Persistence model aligns with intended architecture: RE persists *lifecycle overlays* only (recommendation_lifecycle table), DE persists intake candidates, evaluation, scores, creations (decision_engine_intake_*, decision_candidate_*, decision_engine_candidate_creation, etc.). Migrations exist in `packages/database/migrations/`.
- Good test coverage for DE/RE behavioral invariants found under `packages/kernel/src/commands/decision_engine_tests.rs` and `workspace_recommendation_tests.rs`.

## Critical Issues

1) Severity: High
Description: Heavy reliance on debug-only assertions (debug_assert!) within core services to enforce RE↔DE invariants and read-only constraints.
Why it matters: `debug_assert!` is compiled out in release builds. In production (release) binaries these architectural invariants are not enforced at runtime, which risks silent violations (e.g., a code path accidentally mutating Recommendation overlays from the Decision Engine side).
Evidence:
- `packages/kernel/src/services/decision_engine.rs` contains many `debug_assert!(...)` checks validating that intake receipts, evaluations, candidate creations, scores, rankings, selections, progression requests and acknowledgements are observational-only and do not mutate RE overlays.
- Similar debug assertions appear across DE projection and audit phases.
Recommended fix:
- Replace critical debug-only assertions with explicit runtime validation that returns errors (Result::Err) or uses hardened checks that run in release builds; keep unit tests validating invariants. Add CI gating that runs a build with assertions enabled or a specialized integration test that exercises the invariants.

2) Severity: Medium-High
Description: Potential drift between domain model in the kernel/domain crates and the TypeScript `app/src/types/domain.ts` copy used by the UI.
Why it matters: UI and kernel must agree on contract semantics. Manual duplication risks drift and subtle UX/backend mismatches (type/field additions, enum variants). This weakens authority boundaries if the UI begins to assume fields that are not persisted or guaranteed by kernel.
Evidence:
- `app/src/types/domain.ts` contains a very large surface mirroring workspace domain types (Recommendation*, DecisionCandidate*, DecisionEngineState, etc.).
- There is evidence of auto-generated catalog artifacts (e.g. `app/src/generated/explanationCatalog.ts` — auto-generated from `packages/kernel/resources/explanation-catalog.json`), but `domain.ts` is not marked generated.
Recommended fix:
- Replace the hand-maintained `app/src/types/domain.ts` with a generated TypeScript definition produced from the canonical domain definitions (Rust types / schema) as part of the build or CI pipeline. If generation already exists, mark the file as generated and add a verification step in CI that it is up to date.

3) Severity: Medium
Description: IPC surface is large and many commands are registered but not actively used by the React UI (quarantined). This increases attack surface and maintenance burden.
Why it matters: Unused IPC endpoints may be accidentally relied upon in tests or external tooling, or be left undocumented; keeping parity is reasonable but it still increases audit surface.
Evidence:
- `app/src-tauri/src/lib.rs` registers a broad list of commands; `docs/03-Engineering/IPC-SURFACE.md` documents which are used by the React shell vs quarantined ones.
Recommended fix:
- Maintain IPC inventory as codegen from `lib.rs` and/or add a periodic inventory test that flags unused endpoints. Consider runtime feature flags for quarantined endpoints.

## Medium Issues

- Some lifecycle projection code paths perform non-trivial merges of persisted DE-owned artifacts and RE overlays; ensure provenance metadata is consistently populated and audited. Evidence: `DecisionEngineRepository::upsert_candidate_creation` and `RecommendationLifecycleRepository::upsert_overlay` persist provenance fields (created_at, content_fingerprint, etc.). Suggest adding more explicit provenance fields if absent (e.g., source-service-id, commit-hash, generation-run-id).

- Tests are concentrated in kernel unit/in-memory integration tests; there are fewer end-to-end tests covering the Tauri IPC surface together with the React consumer. Adding e2e tests that run a headless Tauri + app client to exercise the documented IPC surface would reduce regression risk.

## Minor Issues

- `app/src/types/domain.ts` is large; if it remains hand-edited add clear header comment (GENERATED or AUTHORITATIVE SOURCE) and CI check to reduce drift.
- Some debug logging and audit text rely on string literals; recommend centralizing important policy strings (authority_effect variants, lifecycle state names) to avoid mismatch.

## Documentation Drift

Findings:
- `docs/03-Engineering/IPC-SURFACE.md` matches the commands registered in `app/src-tauri/src/lib.rs` (verified). Good alignment.
- There is an explicit rule in the docs: React must call IPC via `app/src/lib/ipc.ts` and must not import database crates — codebase follows this rule (no occurrences of `workspace_database` under `app/`).
- Domain documentation (docs/*) is extensive; however the UI `domain.ts` duplication is a likely source of drift.

Recommended actions:
- Add a CI check to validate `app/src/types/domain.ts` is generated from the canonical domain or to run a lightweight schema check between TS/JSON and Rust types.

## Test Coverage Gaps

- Good coverage for Decision Engine behavioral invariants (in-memory tests): `packages/kernel/src/commands/decision_engine_tests.rs` covers ranking, attention carry-through, permission gating, handoff behavior, and execution prevention.
- Recommendation Engine unit tests exist (`workspace_recommendation_tests.rs`) but fewer end-to-end tests that include IPC and UI behavior.
- Missing explicit tests to cover invariants currently enforced only via `debug_assert!` in release builds. Add runtime/integration tests that exercise those invariants in release-mode execution.

## Dead Code

- Several IPC handlers are intentionally quarantined/unused by React and documented as such. This is intentional (kept for CommandHandler parity). No other obvious dead code found in sampled areas.

## Duplicate Concepts

- Domain concepts are duplicated in `app/src/types/domain.ts` and `packages/domain` / `packages/domain/src`. This duplication is likely intentional for type safety in UI, but should be generated or checked to avoid drift.

## Architectural Strengths

- Strong explicit authority boundaries: RE produces suggestions and lifecycle overlays; DE reads RE overlays observationally and owns its intake candidates and candidate creations. Kernel code documents and enforces these boundaries (see comments in `decision_engine.rs` and `workspace_recommendation.rs`).
- Repositories map cleanly to lifecycle overlays vs DE persistence (database migrations and repository impls present).
- IPC surface is documented and intentionally scoped (docs/03-Engineering/IPC-SURFACE.md) and the Tauri registration matches the doc.
- Good unit/in-memory test coverage for critical decision/recommendation invariants.

## Suggested Repair Order (highest architectural value first)

1. Replace debug-only assertions with release-safe runtime validations or error paths (DecisionEngineService and other places that enforce RE/DE invariants via debug_assert!). Add CI tests that run with asserts enabled and/or integration tests that exercise invariants in release-like builds.
2. Introduce canonical generation of frontend domain TypeScript types from the Rust domain (or add CI verification). Mark `app/src/types/domain.ts` as generated or remove manual edits.
3. Add e2e IPC tests exercising the documented IPC surface (happy-path + negative tests for quarantined endpoints).
4. Add provenance enrichment to persisted artifacts where gaps exist and add audits to verify provenance fields are populated.
5. Periodically verify migrations: add migration-integrity tests during CI to ensure migrations are idempotent and in-order.

## Final Verdict

PASS WITH RECOMMENDATIONS

Rationale: The implementation matches the intended architecture in most important dimensions: bounded contexts (RE vs DE), repository separation, IPC isolation, and domain model consistency. The single most significant risk is the use of debug-only assertions to express critical cross-context invariants — this must be hardened so release binaries preserve architecture guarantees.

---

## Evidence index (representative, not exhaustive)

- RE generation: `packages/kernel/src/services/workspace_recommendation.rs`
- DE generation and observational-only enforcement: `packages/kernel/src/services/decision_engine.rs`
- RE lifecycle persistence: `packages/database/src/repositories/recommendation_lifecycle.rs` and migration `packages/database/migrations/022_recommendation_lifecycle.sql`
- DE persistence: `packages/database/src/repositories/decision_engine.rs` and migrations `packages/database/migrations/017_decision_engine_lifecycle.sql`, `030_decision_engine_intake_candidate.sql`, `032_decision_engine_intake_evaluation.sql`, `033_decision_engine_intake_disposition.sql`, `034_decision_engine_candidate_creation.sql`, `035_decision_candidate_evaluation_origin.sql`, `036_decision_candidate_evaluation_resolution.sql`, `037_decision_candidate_score.sql`, `038_decision_candidate_selection.sql`, `039_decision_candidate_progression_request.sql`, `040_decision_candidate_progression_acknowledgement.sql`.
- IPC registration: `app/src-tauri/src/lib.rs` and documentation `docs/03-Engineering/IPC-SURFACE.md`.
- Frontend domain types: `app/src/types/domain.ts`.
- Tests: `packages/kernel/src/commands/decision_engine_tests.rs`, `packages/kernel/src/commands/workspace_recommendation_tests.rs`.


## Appendices / Next steps

- If you want, I can:
  - Produce a focused diff that replaces selected `debug_assert!` checks with runtime Result-based validations (non-invasive, non-production changes isolated to services) for review.
  - Add a CI job proposal that generates TS types from domain and enforces parity.



(End of report)
