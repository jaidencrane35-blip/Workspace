# Resilience Readiness Audit — Phase Verification

Date: 2026-07-27
Author: Architectural readiness audit (inspection only)

## Purpose

This document is a Phase Resilience Contract Verification readiness audit. It is inspection-only and does not include code changes.

## Current Architecture State

- The repository implements a clearly bounded Recommendation Engine (RE) and Decision Engine (DE) architecture within `packages/kernel`.
- The Tauri IPC surface is explicitly enumerated in `app/src-tauri/src/lib.rs` and wrapped through safe IPC response envelopes in `app/src-tauri/src/commands/response.rs`.
- Permission gating is applied in `packages/kernel/src/commands/handler.rs` via `CommandPipeline` guards such as `GateDecisionEngineRead`, `GateDecisionEngineWrite`, `GateRecommendationEngineRead`, and `GateRecommendationEngineWrite`.
- Frontend application code is separated from kernel code; the UI uses `app/src/lib/ipc.ts` for invocation and a hand-maintained domain type bundle in `app/src/types/domain.ts`.

## Confirmed Strengths

- Strong architectural separation between RE and DE in kernel services:
  - `packages/kernel/src/services/workspace_recommendation.rs`
  - `packages/kernel/src/services/decision_engine.rs`
- Explicit command-layer permission enforcement for major AI-facing operations in `packages/kernel/src/commands/handler.rs`.
- Tauri IPC registration is centralized and explicit, avoiding implicit command discovery.
- Kernel command wrappers in `app/src-tauri/src/commands/*.rs` consistently translate kernel errors into IPC-safe `CommandError` forms.
- Existing in-kernel tests cover decision and recommendation service behavior in `packages/kernel/src/commands/decision_engine_tests.rs` and `packages/kernel/src/commands/workspace_recommendation_tests.rs`.

## Primary Risks

### P0: Debug-only invariants in critical runtime services

- `packages/kernel/src/services/decision_engine.rs` contains a large number of `debug_assert!` validations enforcing DE/RE boundary semantics and candidate lifecycle constraints.
- `packages/kernel/src/services/workspace_recommendation.rs` also uses `debug_assert!` to validate recommendation lifecycle states and rejection/hand-off boundaries.
- Risk: `debug_assert!` is compiled out in release builds. These invariants are therefore not enforced in production binaries, creating a gap between design intention and runtime behavior.
- Impact: A broken invariant may allow unauthorized DE mutation of RE overlays, invalid candidate lifecycles, or incorrect transition states without failing loud enough.

### P1: Frontend domain contract drift

- `app/src/types/domain.ts` is a manually maintained TypeScript copy of kernel domain concepts.
- Risk: Without generation or automated parity checks, frontend/back-end contract drift is likely for fields, enum variants, or semantics.
- Impact: UI assumptions may diverge from kernel invariants, particularly around Recommendation/Decision lifecycle and authority effects.

### P1: Large IPC surface with limited documented usage coverage

- `app/src-tauri/src/lib.rs` exposes a broad IPC surface, including many commands that may be unused by the React shell.
- Risk: Unused or lightly audited commands increase attack surface and maintenance burden.
- Impact: Unexpected frontend or external usage of quarantined IPC commands may bypass documented interface expectations.

### P2: Release-time test coverage gap for architecture invariants

- Existing tests appear to cover many DE/RE behaviors, but the most important production contract is the invariants currently expressed as `debug_assert!`.
- Risk: There is no explicit audit or integration test guaranteeing those invariants in release-mode execution.
- Impact: Regression in release builds may go undetected.

## Exact Files Impacted

- `packages/kernel/src/services/decision_engine.rs`
- `packages/kernel/src/services/workspace_recommendation.rs`
- `packages/kernel/src/commands/handler.rs`
- `app/src-tauri/src/lib.rs`
- `app/src-tauri/src/commands/decision_engine.rs`
- `app/src-tauri/src/commands/recommendation_engine.rs` (if present; command wrappers for recommendation and decision actions)
- `app/src/types/domain.ts`
- `app/src/lib/ipc.ts`
- `docs/03-Engineering/IPC-SURFACE.md`

## Recommended Repair Order

1. Harden runtime enforcement of RE/DE invariants.
   - Replace critical `debug_assert!` checks in `packages/kernel/src/services/decision_engine.rs` and `packages/kernel/src/services/workspace_recommendation.rs` with explicit runtime validation and error handling.
   - Keep the invariant assertions in tests, but ensure the production path returns a recoverable error or fails safely.

2. Add release-mode contract tests.
   - Add integration tests that exercise the same lifecycle invariants in a build configuration representative of production.
   - Add a CI job or test variant that runs with release assertions enabled or explicitly validates the invariants.

3. Improve frontend/back-end contract parity.
   - Generate `app/src/types/domain.ts` from canonical Rust domain definitions or add a CI check that validates domain parity.
   - If generation is not feasible immediately, add a clear header in `app/src/types/domain.ts` documenting that it is an authoritative copy and must be updated with kernel domain changes.

4. Audit IPC surface usage.
   - Create an inventory of commands actually used by the React UI.
   - Mark unused/quarantined endpoints explicitly in `docs/03-Engineering/IPC-SURFACE.md` and consider reducing or gating them.

5. Preserve provenance and audit trails.
   - Review any persisted artifacts and ensure provenance metadata is consistently stored for recommendation and decision lifecycle changes.

## Tests Required

- Unit tests for runtime validation code paths replacing `debug_assert!`.
- Integration tests that exercise RE/DE lifecycle transitions through `DecisionEngineService` and `WorkspaceRecommendationEngineService`.
- Release-like execution tests covering the invariants currently enforced only by debug assertions.
- IPC surface tests validating the wrapper commands in `app/src-tauri/src/commands/*.rs` and the response translation in `app/src/lib/ipc.ts`.
- Parity verification tests for `app/src/types/domain.ts` against the canonical kernel domain.

## What Should NOT Be Changed Yet

- Do not alter the high-level RE/DE architecture or command permission model without a design review.
- Do not remove the existing `CommandPipeline` permission guards in `packages/kernel/src/commands/handler.rs`; they are an important authorization layer.
- Do not rewrite the entire IPC surface or frontend command usage inventory in this audit phase. Focus first on hardening the current contract enforcement and verification path.
- Avoid changing UI terminology or domain field names solely to satisfy this audit; keep the existing contract stable while improving enforcement and parity.

## Conclusion

The repository shows a solid architecture with strong intended boundaries between Recommendation and Decision Engines and an explicit Tauri IPC layer. The highest-risk gap is the use of release-disabled `debug_assert!` checks to enforce critical RE/DE lifecycle invariants. Hardening those checks into runtime validation, adding release-mode coverage tests, and reducing frontend domain drift are the most valuable next steps for Phase Resilience Contract Verification readiness.
