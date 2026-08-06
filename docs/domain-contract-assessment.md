# Workspace Domain Contract Assessment

| Field | Value |
| --- | --- |
| **Purpose** | Assess Rust domain models vs TypeScript equivalents and describe a contract-generation path |
| **Authority** | `packages/domain`, `app/src/types/domain.ts`, scripts under `scripts/` |
| **Date** | 2026-08-07 |
| **Constraint** | Assessment — Product Proof codegen **implemented** 2026-08-07 (`pnpm verify:contracts`); non-PP types still manual |

---

## Summary

| Side | Location | Nature |
| --- | --- | --- |
| Canonical domain | `packages/domain` (Rust) | Source of truth for kernel/DB types |
| UI mirror | `app/src/types/domain.ts` (~4888 lines) | **Manual**, hand-maintained |
| Generated TS | `app/src/generated/explanationCatalog.ts` | **Generated** from kernel JSON (not domain types) |
| Domain codegen scripts | — | **Missing** |

Prior documentation (`docs/03-Engineering/FOUNDATION-HARDENING-REPORT.md`) already notes hand-sync and deferred codegen. This assessment confirms that deferral remains true as of 2026-08-07.

---

## Rust domain surface

**Crate:** `workspace-domain`  
**Entry:** `packages/domain/src/lib.rs`

- **~78** `pub mod` modules covering entities, IDs, capabilities, observation/save/resume, execution, AI, automation, decision/recommendation, and workspace-* cognition types.
- **No** persistence, UI, or Win32 dependencies.
- Consumed by `workspace-database`, `workspace-kernel`, `workspace-windows-integration`, `workspace-app`.

Representative authoritative types:

| Area | Examples |
| --- | --- |
| Identity / entities | `Workspace`, `Zone`, `Application`, `ResourceRef`, typed IDs |
| Actors / capabilities | `Actor`, `ActorContext`, `Capability`, `CapabilitySet` |
| Observation | Observation pass/window/identity types, deltas, events |
| Desktop state | `WorkspaceState`, `WorkspaceRuntimeState`, `RuntimeHealth` |
| Save / resume | `SavedContext`, restore plan/summary types, `DesktopAction` |
| Session | `PersistentWorkspaceSession` |
| Execution | outcomes, guards, cancellation, reconciliation |
| AI | memory, planning, orchestration, assistant, evaluation, model |
| Automation | contracts, triggers, proposals |
| Decisions | DecisionEngine / DecisionQueue / Recommendation lifecycle types |
| Cognition RMs | activity, continuity, attention, environment, … |

---

## TypeScript equivalents

### `app/src/types/domain.ts`

| Property | Finding |
| --- | --- |
| Header / GENERATED banner | **None** |
| Classification | **Manual / duplicated** |
| Size | **~4888 lines** |
| Shape | Large set of `export interface` / `export type` mirroring Rust serde JSON shapes |
| Sync mechanism | Human edits when IPC payloads change |
| CI drift check | **None** for domain.ts vs Rust |

### Other TS type files

| File | Role | Classification |
| --- | --- | --- |
| `app/src/types/workspace.ts` | IPC envelope, status, health, settings | Manual (narrow) |
| `app/src/types/layout.ts` | Layout UI types | Manual |
| `app/src/generated/explanationCatalog.ts` | Explanation fixtures | **Generated** (+ `scripts/verify-explanation-catalog.mjs`) |

### Divergence / missing / duplicate patterns

| Pattern | Status |
| --- | --- |
| Duplicated contracts | Core IPC payload types exist in Rust domain **and** TS domain.ts |
| Missing codegen | No `ts-rs`, `specta`, `typeshare`, or custom exporter for domain |
| Possible divergence | Any field renamed/added in Rust without TS update → runtime JSON mismatch (TypeScript cannot catch) |
| Intentionally UI-only types | CognitiveEngine / experience presentation types live only in TS (`experience/*`, components) — **not** domain duplicates |
| Intentionally Rust-only | Internal validation helpers, some governance structs never sent over IPC |

Without a schema diff tool, **exact field-level divergence is UNKNOWN**; risk is established by process (manual sync) and prior audit (2026-07-27).

---

## IPC as the de facto contract

Wire format today:

```
Rust domain types
  → serde_json via Tauri invoke
    → IpcResponse<T> envelope (workspace.ts)
      → TypeScript casts/generics on invokeIpc<T>(...)
```

Contract enforcement mechanisms that **do** exist:

| Mechanism | What it covers |
| --- | --- |
| Kernel + Vitest behavioural tests | Selected payloads (experience E2E, parity tests) |
| `production-integration-parity.test.ts` | Catalog ↔ Tauri registration |
| Explanation catalog verifier | Catalog JSON ↔ generated TS |
| UI experience boundary verifier | Components must not import resolver/domain reason internals incorrectly |

Contract enforcement that **does not** exist:

| Gap |
| --- |
| Generated TS from Rust domain |
| CI schema equality for `domain.ts` |
| OpenAPI/JSON Schema export of IPC |

---

## How a contract-generation pipeline could eliminate duplication

**Goal:** Make Rust domain (or an explicit IPC DTO layer) the single editable source; regenerate TypeScript; fail CI on drift.

### Recommended shape (design only)

1. **Mark IPC DTOs**  
   Introduce (or annotate) a stable set of types that cross the Tauri boundary — either:
   - export selected `workspace-domain` types with a codegen attribute, or
   - add a thin `workspace-ipc-types` crate that re-exports/wraps wire types only.

2. **Choose a generator** (examples; selection not made here):
   - `ts-rs` / `typeshare` / `specta` (Tauri-friendly) / custom `serde_json` schema → `json-schema-to-typescript`.

3. **Emit** into `app/src/generated/domain.ts` (or split per domain) with a banner:
   ```ts
   // GENERATED FILE — do not edit. Source: packages/domain (or ipc-types).
   ```

4. **Replace imports** gradually from hand `types/domain.ts` → generated module; delete manual duplicates when coverage complete.

5. **Verifier script** (mirror explanation-catalog pattern):
   - `scripts/verify-domain-contracts.mjs` fails if working tree generated file ≠ freshly generated output.
   - Wire into `pnpm test` / CI.

6. **Keep UI-only types separate**  
   Experience/cognitive presentation types remain hand-written under `app/src/experience` / components — out of domain codegen scope.

7. **Envelope types**  
   Keep `IpcResponse` / error codes in a small hand or generated settings module; ensure error code enums stay shared.

### Non-goals for that pipeline

- Generating React components
- Replacing runtime validation in Rust
- Auto-reducing IPC command count
- Syncing localStorage experience schemas (separate concern)

### Risks to document before implementation

| Risk | Note |
| --- | --- |
| Serde rename attributes | Must round-trip identically |
| Optional vs null | TS `| null` vs omitted fields |
| Enum representations | Externally tagged vs string unions |
| Large surface | 197 commands × payloads → generator must be selective or fast |
| Quarantined commands | Still need types if registered |

---

## Classification matrix (current)

| Artifact | Generated | Manual | Duplicated | Missing | Diverged |
| --- | --- | --- | --- | --- | --- |
| Rust domain modules | — | Canonical | — | — | — |
| `app/src/types/domain.ts` | No | Yes | Yes (vs Rust) | Codegen | **Unknown exact** (process risk) |
| `explanationCatalog.ts` | Yes | No | No | — | Guarded by verifier |
| IPC command list | No | Yes (`lib.rs`) | Doc inventory | Auto inventory | Doc vs `get_action_catalog` note |
| Experience catalog | No | Yes | Subset of IPC | — | Parity tested |

---

## Relationship to other maps

- IPC payloads: `docs/ipc-surface-map.md`
- Who owns durable instances of domain types: `docs/state-authority-map.md`
- Permission subjects/capabilities as domain concepts: `docs/governance-map.md`
