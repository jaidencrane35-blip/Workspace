# Engineering Milestone Report
## P24 - Personal AI + Intelligence Experience Architecture Reset

| Field | Value |
| --- | --- |
| **Capability ID** | None created — architecture reset; proposed Atlas IDs listed in contract §29.3 (Owner approval required before Atlas amendment) |
| **Artifacts** | `docs/architecture/WORKSPACE_INTELLIGENCE_EXPERIENCE_CONTRACT.md`; `docs/architecture/WORKSPACE_VISUAL_REFERENCE_SPEC.md`; `docs/architecture/assets/nova-character-sheet.png` (character #8 canonical) |
| **Date** | 2026-08-08 |
| **Status** | **Architecture complete** — **not authorised for implementation**; capability-by-capability work paused pending Owner roadmap selection |
| **Max layer** | Documentation / governance only (no runtime, no IPC, no providers, no Nova code) |
| **Readiness** | No capability readiness advanced |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) — P24 is the new behavioral/experience authority |

### Summary

Owner testing showed Workspace still behaves like a truthful command/help system
despite substantial desktop capability. Audit measured the cause: ~469 intent
regexes, zero in-product language models, zero generated Owner sentences,
session context that dies on exit, and **zero** occurrences of “Nova” in the
repository. Form A’s 44×44 transparent host already exists — Nova is missing as
identity, not as window infrastructure. Large personalization substrate
(`ai_memory_entries`, `user_preferences`, suggestion/decision lifecycle,
`ProviderRegistry::list`) is **orphaned**: built and IPC-exposed but not on the
Conversation path.

P24 establishes: product identity (personal AI on-device); Language Faculty
Boundary (comprehension + expression, never authority); Owner/Device/Memory
models with register laws; context hierarchy; Nova as Form A embodiment
(character #8 only; cast rejected); visual frames 1–18; Conflict C assessment
(blocks only generative multi-step; P0–P4 unblocked); and roadmap P0–P6
prioritising connection of orphaned substrate and understanding over new
capabilities.

**Does not implement** the roadmap, Language Faculty, Nova runtime, ambient
listening, or self-modification. Conflict A ADR and B-RES-001 remain Owner
decisions before any model work.

### Validation

- Repository audit (Atlas, conversation path, voice lifecycle, memory/device,
  UI/Nova) — documentation-only deliverables
- No `pnpm` / Cargo product-code changes in this program
- Canonical Nova art committed under `docs/architecture/assets/`

---

## P23.S6 - Observed Application Identity (C-OBS-001 / C-OBS-002 observation data, consumed by C-REA-002)

| Field | Value |
| --- | --- |
| **Capability ID** | C-OBS-001 / C-OBS-002 observation enriched; C-REA-002 consumes it — no new ID |
| **Artifacts** | `process_name` on `DesktopWindowSnapshot` and `ApplicationWindowItem`, `to_legacy_snapshot` reuse in the Window Provider, `processName` on the TS observation view, application-vs-window answers with truthful fallback, `scripts/verify-application-identity.mjs` (new), kernel `application_identity_tests`, `tests/application-identity.test.ts`, Atlas C-OBS-001/002 + C-REA-002 |
| **Date** | 2026-08-08 |
| **Status** | **Engineering complete** — Owner Product Proof required (Owner-visible answers) |
| **Max layer** | Windows observation data + Kernel item + Intent/Conversation composition (no new capability, provider, or IPC channel) |
| **Readiness** | C-REA-002 **57%**, C-OBS-001 / C-OBS-002 unchanged; PP/Trusted/Production not advanced |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

P23.S4 and P23.S5 both ended at the same admission: Workspace could name the
window but not the application. The audit found the information was never
missing. `win32.rs` already resolves the executable image basename for every
enumerated window (`QueryFullProcessImageNameW` under
`PROCESS_QUERY_LIMITED_INFORMATION`) and stores it on `CapturedDesktopWindow`;
`to_legacy_snapshot` then dropped it, because `DesktopWindowSnapshot` did not
declare the field. The same value already reaches the product elsewhere — the
Workspace State pipeline surfaces `process_name` on active applications — so no
new authority, API, or permission was involved. The Window Provider also held
two hand-written copies of that conversion, one of which dropped the field
independently; both were replaced by the shared conversion.

Identity is defined narrowly: the executable image basename exactly as Windows
spells it (`Code.exe`), never prettified into a product name. Any case fixing,
extension stripping, or executable-to-product mapping would be a claim the
observation cannot support, and verbatim reporting keeps the chain checkable —
the string in the answer is the string Windows produced. `FileDescription` from
the version resource remains the documented dependency for a friendlier name.

The field is optional end to end because Windows genuinely withholds it for
protected processes. When it is absent Conversation says so rather than reading
an application out of the title, and both the tests and the new verifier fail if
that stops being true. Window questions stay window questions; the open-windows
inventory still lists titles and names applications only where titles collide.

One substitution was closed on the way: "What application is active?" was being
handed to ChatGPT because the intelligence cascade resolved an external handoff
before the observation bridge saw the request. The bridge now replaces a spoken
non-answer or an external handoff — never an action that acts on the desktop —
because the Answer Source Ladder already ranks an authorized observation above
an external source.

### Validation

- `cargo test -p workspace-kernel --lib` and `-p workspace-windows-integration --lib` (incl. 4 new provider identity tests and the snapshot-conversion test)
- `pnpm typecheck`, `pnpm build`, `pnpm test` — 86 files / 625 tests, all verifiers
- Falsification: title-as-identity fails 3 tests across P23.S5 and P23.S6; a fabricated `process_name` and a source-owned application name are both rejected by `verify-application-identity`

---

## P23.S5 - Active Window Answer (C-REA-002 rung 3 second need, reusing C-OBS-002)

| Field | Value |
| --- | --- |
| **Capability ID** | C-REA-002 (extended) reusing C-OBS-002 — no new ID |
| **Artifacts** | `activeWindowAnswerSource`, second `ObservationNeed`, total `Record<ObservationNeed, IntentAction>` translation, `groundGoalInContext` in Workspace Context, extended `verify-observation-answer`, `tests/active-window-answer.test.ts`, Atlas C-OBS-002 + C-REA-002 |
| **Date** | 2026-08-08 |
| **Status** | **Engineering complete** — Owner Product Proof required (Owner-visible answers) |
| **Max layer** | Intent Layer + Conversation façade (no Kernel change, no new IPC, no Rust) |
| **Readiness** | C-REA-002 **57%**, C-OBS-002 **71%** — unchanged; PP/Trusted/Production not advanced |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Measured before any change: "What window am I using?" and "Which application am
I using?" were refused, while "Which window is active?" already reached the
active-window observation and recited the Kernel's operational phrasing. All
three comprehend identically as PERCEIVE_MACHINE in the desktop domain — the
same discarded-meaning failure as P23.S4, on a different observation. The audit
found `window/active` already returning the focused `ApplicationWindowItem`
through the Permission Gateway to Conversation, so the slice needed no new
authority, no new IPC, and no Rust.

The architectural question was whether the observation rung was an abstraction
or a coincidence of having one member. It holds at two, but only because three
properties were made explicit and enforced: needs are a fixed vocabulary of
information rather than things Workspace can run; each need is covered by
exactly one source, so a request has one meaning instead of a shortlist to rank;
and translation is a **total** `Record<ObservationNeed, IntentAction>`, so a
need with no pre-existing request does not compile. A conditional chain could
invent a route — a total map can only translate one that already existed.
Falsified: a third need, a `Partial` map, and grounding that resolved to
`REACH_STATE` were each rejected by the verifier.

"Which one am I using?" is grounded through existing Workspace Context, which
already held the previous turn's meaning. Grounding refines meaning only, and
only into perception: ungrounded, it still receives the honest limitation and
performs no IPC at all; "Open that one." remains an effect request.

Application identity is now visible in the product rather than silently wrong —
asked which application, Workspace answers with the window title and says it
cannot name the program. This is the second slice blocked by the missing
`process_name` on `ApplicationWindowItem`, which promotes it to a real
dependency.

---

# Engineering Milestone Report
## P23.S4 - Observation Answer Bridge (C-REA-002 rung 3, reusing C-OBS-001)

| Field | Value |
| --- | --- |
| **Capability ID** | C-REA-002 (extended) reusing C-OBS-001 — no new ID |
| **Artifacts** | `app/src/lib/observationAnswerSource.ts`, `ObservationAnswerSource`/`ObservationNeed` contract in `answerSource.ts`, `bridgeObservationRequest`, façade answer composition, `verify-observation-answer`, `tests/observation-answer.test.ts`, Atlas C-REA-002 + C-OBS-001 |
| **Date** | 2026-08-08 |
| **Status** | **Engineering complete** — Owner Product Proof required (Owner-visible answers) |
| **Max layer** | Intent Layer + Conversation façade (no Kernel change, no new IPC, no Rust) |
| **Readiness** | C-REA-002 **57%**, C-OBS-001 **71%** — unchanged; PP/Trusted/Production not advanced |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

"What windows are currently open?" was refused — measured, before any change:
comprehension already produced PERCEIVE_MACHINE in the desktop domain, but
literal matching missed that phrasing and the request fell out as "I can work
with open windows when I know which one you mean." "What applications are
open?" got a generic refusal. Meanwhile "Which windows are open?" worked. The
meaning was right and was being discarded.

The audit found no missing Kernel interface: `execute_capability_intent` already
carries `window/enumerate` through the Permission Gateway and returns structured
`items`, and Conversation was already receiving them and showing the Kernel's
bulleted operational text instead. So the bridge needed no new authority, no new
IPC, and no Rust — only two connections. The comprehended need now reaches the
*existing* authorized observation, and the observation that comes back is
composed into an answer.

The ladder gained its `capability-observation` rung, whose sources are
structurally incapable of observing: a source declares a semantic
`ObservationNeed` and composes whatever authorized observation returns. Exactly
one need exists and widening it is verifier-rejected (falsified: adding
`"active-window"` was refused), which is what prevents a second planner. The
answer is derived only from observed items — proved with window titles that
appear nowhere in the repository — and a refused or empty observation is
reported as it stands rather than filled in.

---

# Engineering Milestone Report
## P23.S3 - Answer Source Ladder: Local Time and Date (C-REA-002 extension)

| Field | Value |
| --- | --- |
| **Capability ID** | C-REA-002 (extended — no new ID) |
| **Artifacts** | `app/src/lib/answerSource.ts`, `app/src/lib/temporalAnswerSource.ts`, clock/date comprehension shapes, `intelligenceRouting` zone-table extraction, `verify-answer-source`, `tests/answer-source.test.ts`, Atlas C-REA-002 |
| **Date** | 2026-08-08 |
| **Status** | **Engineering complete** — Owner Product Proof required (Owner-visible answers) |
| **Max layer** | Intent Layer only (no Kernel, no provider, no IPC change, no Rust) |
| **Readiness** | Overall **57%** (Arch/Deps/Impl/Ver 100%; PP/Trusted/Production 0%) |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Workspace can now answer a question instead of handing it to another AI. The
measured Owner failure — "What is the time in Queensland, Australia?" opening
ChatGPT — resolves locally end to end: comprehension gives KNOW in the time
domain, Queensland resolves to Australia/Brisbane, and the runtime's clock
supplies the answer. The desktop cascade never runs for it, so no browser and
no handoff are reachable.

The mechanism is the point, not the phrase. The **Answer Source Ladder** asks
which trusted source knows something, never which capability executes; all five
rungs are named and only the deterministic-local rung is implemented. Sources
emit `{ text, sourceId, rung }` and nothing executable — the verifier rejects
either module for touching IPC, a capability registry, an action, or provider
identity. The temporal source is now the repository's single time-zone
authority: routing's private alias table was removed rather than duplicated, and
daylight saving comes from `Intl.DateTimeFormat`, never from offset arithmetic
(falsification: an injected Brisbane offset constant was rejected by the
verifier). Ambiguous places resolve to nothing and reach the existing
clarification; unlisted places fall through to C-REA-003, which remains the only
external route. Gated by P23.S2's predicates, so the ladder cannot pre-empt
genuine desktop work.

---

# Engineering Milestone Report
## P23.S2 - Substitution Prohibition Enforcement (C-ITL-007)

| Field | Value |
| --- | --- |
| **Capability ID** | C-ITL-007 |
| **Artifacts** | `app/src/lib/substitutionProhibition.ts`, `informationHandoff` mark, `softMiss` mark, collapse question guard, `verify-substitution-prohibition`, `tests/substitution-prohibition.test.ts`, Atlas C-ITL-007 |
| **Date** | 2026-08-08 |
| **Status** | **Engineering complete** — Owner Product Proof required (Owner-visible reply behaviour) |
| **Max layer** | Intent Layer only (no Kernel, no provider, no IPC change) |
| **Readiness** | Overall **57%** (Arch/Deps/Impl/Ver 100%; PP/Trusted/Production 0%) |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Workspace no longer substitutes a desktop effect for an answer it does not
have. When the Goal Contract says a request is fulfilled by knowledge,
computation, or conversation, an effect resolved from action-shaped vocabulary
is refused and replaced with a truthful reply framed in the Owner's goal.

Three substitutions were measured and corrected: a question containing
"minimize" collapsed the conversation surface, a social pleasantry opened a
browser, and a greeting was answered with a desktop suggestion. Enforcement may
only remove an effect — the verifier rejects the module if it constructs any
non-speaking action — and refusal requires positive comprehension evidence, so
the bare-question default cannot disable a capability. Hybrid, observation, and
action goals are untouched. The external handoff remains C-REA-003's decision,
marked `informationHandoff`; a KNOW outcome never implies it.

---

## P23.S1 - Outcome-First Comprehension (C-ITL-006 Goal Contract)

| Field | Value |
| --- | --- |
| **Capability ID** | C-ITL-006 |
| **Artifacts** | `app/src/lib/goalContract.ts`, `resolveIntentWithGoal`, Workspace Context `currentGoal`, `verify-goal-contract`, `tests/goal-contract.test.ts`, Atlas C-ITL-006 |
| **Date** | 2026-08-08 |
| **Status** | **Engineering complete** — internal comprehension; no Owner Product Proof surface of its own |
| **Max layer** | Intent Layer only (no Kernel, no provider, no IPC change) |
| **Readiness** | Overall **57%** (Arch/Deps/Impl/Ver 100%; PP/Trusted/Production 0%) |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

First implementation slice from the Intelligence Layer Behavioral Constitution.
Comprehension now derives what the Owner wants to be true from accumulated
signals over the whole utterance, before Workspace Context and before any
desktop matching, and the result is preserved instead of discarded. Compound
requests keep their final requested result rather than collapsing to the first
verb; machine questions containing effect words stay observational; unresolved
references and unrecognised targets are represented rather than invented.

The Goal Contract carries no capability id, provider, operation, or step, and
does not yet influence which action is chosen — capability selection remains
Kernel Operator authority. It stops at the Conversation façade because
`CapabilityIntent` has no meaning field; Conflict C (constitution §30) must be
resolved before comprehension can drive execution. P22.S1 routing unchanged.

---

## C-PROC-002 - Prepare Coding Workspace Runtime Implementation

| Field | Value |
| --- | --- |
| **Capability ID** | C-PROC-002 |
| **Artifacts** | Atlas C-PROC-002.1–.9, `prepareCodingWorkspace.ts`, Kernel `desktop.prepare_coding_workspace`, `verify-prepare-coding-workspace`, `tests/prepare-coding-workspace.test.ts` |
| **Date** | 2026-08-08 |
| **Status** | **Engineering complete** — Owner Product Proof required; Trusted/Production incomplete |
| **Max layer** | Intent Layer + Kernel Operator composition (reuse C-ACT-001/006, C-CMP-002, C-VER-003, C-CMP-001) |
| **Readiness** | Overall **~57%** (Arch/Deps/Impl/Ver 100%; PP/Trusted/Production 0%) |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Authorized runtime slice implements the corrected C-PROC-002 contract:
Intent resolves prepare-with-targets phrasing into an ordered set; Kernel
preflights whole-set `launch_alias`/URL executability before any Effect;
opens via existing Find→Focus|Launch / browser Open units; waits with
C-VER-003 `window_available` (hwnd preferred; unique identity required);
aggregates completed/partial/failed from observed identity evidence only.
No Launch retry, no C-VER-002, no final-focus inference, no default apps.
Targetless Continue and bare prepare clarification preserved. Product Proof
defined, not run, not accepted.

---

## B-DEF-001 - Prepare Coding Workspace Definition (C-PROC-002)

| Field | Value |
| --- | --- |
| **Capability ID** | C-PROC-002 |
| **Artifacts** | Atlas C-PROC-002.1–.9, `verify-prepare-coding-workspace-definition`, `P22_S7_PREPARE_CODING_WORKSPACE_BLOCKED.md` resolution record |
| **Date** | 2026-08-08 |
| **Status** | **Contract corrected** — superseded by runtime implementation milestone above |
| **Max layer** | Capability definition + Repository Standards + Documentation |
| **Readiness** | Was ~31%; runtime now ~57% |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Definition/correction slice established PCW-001…PCW-004 composition over
existing authorities. Runtime implementation followed under Owner authorization.

---

## P22.S6 - Bounded Retry (C-VER-002)

| Field | Value |
| --- | --- |
| **Capability ID** | C-VER-002 |
| **Artifacts** | `P22_S6_BOUNDED_RETRY.md`, `verify-bounded-retry.mjs`, `tests/bounded-retry.test.ts`, `operator/retry.rs` |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete — Product Proof required |
| **Max layer** | Kernel Operator retry policy + Wait reuse + Completion Contract |
| **Readiness** | Overall **57%** (Arch/Deps/Impl/Ver 100%; PP/Trusted/Production 0%) |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Bounded Retry: at most one re-attempt of the same authorized click/type after a retryable verification miss, using C-VER-003 Wait Conditions between attempts. Non-retryable failures stop immediately. Happy-path click/type unchanged. Not an agent loop. Owner Product Proof per P22.S6.

---

## P22.S5 - Wait Conditions (C-VER-003)

| Field | Value |
| --- | --- |
| **Capability ID** | C-VER-003 |
| **Artifacts** | `P22_S5_WAIT_CONDITIONS.md`, `verify-wait-conditions.mjs`, `tests/wait-conditions.test.ts` |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete — Product Proof required |
| **Max layer** | Window Provider wait + Kernel Operator compose + Intent |
| **Readiness** | Overall **57%** (Arch/Deps/Impl/Ver 100%; PP/Trusted/Production 0%) |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Bounded Wait Conditions: poll existing Window/UIA observation until control/window state matches or timeout (default 2s, max 8s). Operator composition `window.wait_condition` with Completion Contract. Does not alter click/type compositions. Not a retry engine, agent loop, or workflow. Owner Product Proof per P22.S5.

---

## P22.S4 - Desktop Control Interaction (C-ACT-004 + C-ACT-005)

| Field | Value |
| --- | --- |
| **Capability IDs** | C-ACT-004 · C-ACT-005 |
| **Artifacts** | `P22_S4_DESKTOP_CONTROL_INTERACTION.md`, `verify-desktop-control-interaction.mjs`, `tests/desktop-control-interaction.test.ts` |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete — joint Product Proof required |
| **Max layer** | Windows Integration + Window Provider + Kernel Operator compose + Intent |
| **Readiness** | C-ACT-004 **57%** · C-ACT-005 **57%** (separate scores) |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Paired milestone: Mouse Click + Keyboard Input via UIA InvokePattern / ValuePattern. Operator compositions `window.click_control` and `window.type_control` enforce Locate → Interact → Verify with Completion Contract completed/partial/failed. Never invent success. Atlas Pair Rule §0.1 honored (separate IDs/records/readiness; shared implementation).

---

## P22.S3 - Window Control Discovery (C-OBS-004)

| Field | Value |
| --- | --- |
| **Capability ID** | C-OBS-004 |
| **Artifacts** | `docs/capability-runtime/product-proof/P22_S3_WINDOW_CONTROL_DISCOVERY.md`, `scripts/verify-window-control-discovery.mjs`, `tests/window-control-discovery.test.ts`, Atlas Readiness Scores |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete — Product Proof required |
| **Max layer** | Windows Integration + Window Provider + Intent |
| **Readiness** | Overall **57%** (Arch/Deps/Impl/Ver 100%; PP/Trusted/Production 0%) |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Window Control Discovery: locate a named UIA control via `find_control` / `match_control` under Kernel. Conversation → Intent (before semantic find-locate soft-miss) → Kernel → Window Provider. Observation only. Atlas v2.0 gained Capability Readiness Score schema + matrix. Owner Product Proof per P22.S3.

---

## P22.S2 - Desktop UI Tree (C-OBS-003)

| Field | Value |
| --- | --- |
| **Capability ID** | C-OBS-003 |
| **Artifacts** | `docs/capability-runtime/product-proof/P22_S2_DESKTOP_UI_TREE.md`, `scripts/verify-desktop-ui-tree.mjs`, `tests/desktop-ui-tree.test.ts`, Atlas v2.0 |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete — Product Proof required |
| **Max layer** | Windows Integration + Window Provider + Intent |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Desktop UI Tree: WRAP Windows UI Automation for observation-only control enumeration (`enumerate_controls`). Conversation → Intent → Kernel → Window Provider → `UiAutomationPort`. No click/type. Atlas synchronized to v2.0 permanent IDs. Owner Product Proof per P22.S2 workbook.

---

## P22.S1 - Intelligence Kind Routing & Reasoning Provider

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/P22_S1_INTELLIGENCE_KIND_ROUTING.md`, `scripts/verify-intelligence-routing.mjs`, `tests/intelligence-routing.test.ts` |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete - Intent / Conversation intelligence routing |
| **Max layer** | Intent Layer / Conversation |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Intelligence Kind Routing: classify REASONING_LOCAL / REASONING_PROVIDER / HYBRID / CLARIFICATION before desktop soft-miss. Local arithmetic, units, time/date, and Registry capability explain. World knowledge hands off to ChatGPT via Browser Provider. File walls, invent-exe, and desktop commands unchanged. R2 Release Hold remains.

---

## P22.A1 - Intelligence Routing Audit

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/P22_A1_INTELLIGENCE_ROUTING_AUDIT.md` |
| **Date** | 2026-08-08 |
| **Status** | Audit complete - accepted; S1 implemented |
| **Max layer** | Documentation / routing audit |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Desktop-or-refuse pipeline caused unnecessary refusals for knowledge/calc/time. Recommended P22.S1 Intelligence Kind Routing + ChatGPT handoff.

---

## P21.S2 - Compound Goal Decomposition

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/P21_S2_COMPOUND_GOAL_DECOMPOSITION.md`, `scripts/verify-compound-open.mjs`, `tests/compound-open.test.ts` |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete - Intent + Kernel sequential open |
| **Max layer** | Intent Layer + Kernel Operator compose |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Compound Goal Decomposition: resolvable "open A and B" runs sequential open_or_focus / browser opens with Completion Contract aggregate reporting. Unresolved compounds still refuse invent. Beside path unchanged.

---

## P21.A2 - Task Decomposition and Initiative Audit

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/P21_A2_TASK_DECOMPOSITION_INITIATIVE_AUDIT.md` |
| **Date** | 2026-08-08 |
| **Status** | Audit complete - **no implementation** |
| **Max layer** | Documentation / Intent-Kernel initiative audit |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Initiative audit: Workspace often yields after one Intent when the Owner already named resolvable targets. Moments restore and file walls are correct stops. Recommended slice: Compound Open Decomposition (sequential opens for known A and B). Do not implement until Owner authorizes.

---

## P21.S1 - Capability Completion Contract

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/P21_S1_CAPABILITY_COMPLETION_CONTRACT.md`, `scripts/verify-capability-completion.mjs` |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete - Kernel compose discipline |
| **Max layer** | Kernel Operator plan + compose |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Capability Completion Contract: browser.open_beside runs Open, locate, snap left/right, enumerate verify. Conversation reports completed/partial/failed from actual results — never "Opened beside" from Open-only. Other composed paths inherit truthful multi-step reporting.

---

## P21.A1 - Goal Completion Audit

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/P21_A1_GOAL_COMPLETION_AUDIT.md` |
| **Date** | 2026-08-08 |
| **Status** | Audit complete - **no implementation** |
| **Max layer** | Documentation / Capability Runtime audit |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Goal completion audit: Workspace starts helping then abandons after the first step. Flagship gap: `browser.open_beside` plans Open only while Conversation claims beside layout. Recommended slice: Beside Layout Completion (Kernel composition of existing Browser + Window). Do not implement until Owner authorizes.

---

## T1 - Companion Conversation Behavior

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/T1_COMPANION_CONVERSATION_BEHAVIOR.md`, `scripts/verify-companion-conversation.mjs`, `tests/companion-conversation.test.ts` |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete - Conversation behaviour only |
| **Max layer** | Intent Layer / Conversation reply composition |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

T1 slice from T2 audit: companion greetings, retire invent/pretend chorus, no default command catalogues, lightweight pronoun continuity via Workspace Context. Desktop execution unchanged. Await Owner Product Proof re-review.

---

## Prior: T2 - Product Proof Rejection Audit

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/T2_PRODUCT_PROOF_REJECTION_AUDIT.md` |
| **Date** | 2026-08-08 |
| **Status** | Audit complete - informed T1 |
| **Max layer** | Documentation / Product Proof audit |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` (Release Hold) |

### Summary

Owner rejected live Product Proof (Release Hold T2). Systemic causes: Help-shaped Conversation voice, command-turn Intent model, operator presence gap. Recommended single slice: Companion Conversation Voice (implemented as T1).

---

## Prior: R2 - Engineering Standby and Release Lock

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/production/R2_RELEASE_HOLD.md`, `scripts/verify-release-hold.mjs` |
| **Date** | 2026-08-08 |
| **Status** | Release Hold authoritative - **no active engineering program** |
| **Max layer** | Documentation / repository lock |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |

### Summary

Repository locked at Stage 2 Owner Acceptance. Engineering status: Release Hold. Next event: A2 after Owner Accept and Authenticode certificate. Resume only via triggers T1-T5. No UX, signing, or updater work. P17-P20 closed.

---

## Prior: A2/F2 - Release Execution Playbook

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/production/A2_F2_RELEASE_EXECUTION_PLAYBOOK.md` |
| **Date** | 2026-08-08 |
| **Status** | Playbook complete - dormant under Release Hold |
| **Max layer** | Documentation / release operations |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |

### Summary

A2 to F2 to B2 execution order documented. Awaits cert under R2 Hold.

---

## Prior: F1 - Release Pipeline Hardening

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/production/P16_F1_CI_AUTOMATION.md`, `docs/production/RELEASE_PIPELINE.md`, `scripts/verify-f1-ci-automation.mjs`, `.github/workflows/ci-release.yml` |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete - unsigned pipeline |
| **Max layer** | Release engineering / CI |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |

### Summary

F1 Complete. Unsigned CI release path deterministic. Next: A2 after certificate (see playbook).

---

## Prior: R1 - Release Engineering Readiness

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/production/R1_RELEASE_ENGINEERING_READINESS.md` |
| **Date** | 2026-08-08 |
| **Status** | Assessment complete - F1 implemented |
| **Max layer** | Documentation / release readiness |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |

### Summary

Stage 3 discovery. Recommended F1 first (implemented).

---

## Prior: P20.S1 - Empty Conversation First-Session Cue

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/P20_S1_EMPTY_CONVERSATION_FIRST_SESSION_CUE.md`, `scripts/verify-conversation-first-session.mjs` |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete - await Owner |
| **Max layer** | Presentation (Conversation empty invite only) |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |

### Summary

First-session cue shipped. Product-quality arc P17-P20 closed for reopen.

---

## Prior: P20.A1 - Release Candidate Product Proof

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/P20_A1_RELEASE_CANDIDATE_PRODUCT_PROOF.md` |
| **Date** | 2026-08-08 |
| **Status** | Audit complete - S1 implemented |
| **Max layer** | Documentation / Release Candidate Product Proof |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |

### Summary

RC Product Proof. Recommended S1: Empty Conversation First-Session Cue (implemented).

---

## Prior: P19.S1 - Tray Show ShellMode Continuity

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/P19_S1_TRAY_SHELLMODE_CONTINUITY.md`, `scripts/verify-tray-shellmode.mjs` |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete - await Owner |
| **Max layer** | Presentation / shell lifecycle (no new framework) |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |

### Summary

Tray Show restores ShellMode Conversation Form via workspace-show-conversation.

---

## Prior: P19.A1 - Production Product Proof Audit

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/P19_A1_PRODUCTION_PRODUCT_PROOF_AUDIT.md` |
| **Date** | 2026-08-08 |
| **Status** | Audit complete - S1 implemented |
| **Max layer** | Documentation / Product Proof audit |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |

### Summary

Production Product Proof audit. Recommended S1: Tray Show ? ShellMode Continuity (implemented).

---

## Prior: P18.S2 - Moments Tool Session Continuity

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/P18_S2_MOMENTS_TOOL_SESSION_CONTINUITY.md`, `scripts/verify-moments-tool-session.mjs` |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete - await Owner |
| **Max layer** | Presentation (session continuity; no new runtime authority) |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |

### Summary

Moments tool sessions survive normal remounts. Save never blanks when expandHost is null (inline write). Resume/Save step, preview, and draft live in MomentsToolSession above view swaps. Home ? Continue ? Save keeps focus and pin. Explicit Back / Save another resets truthfully.

---

## Prior: P18.A2 - Runtime Continuity Audit

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/P18_A2_RUNTIME_CONTINUITY_AUDIT.md` |
| **Date** | 2026-08-08 |
| **Status** | Audit complete - S2 implemented |
| **Max layer** | Documentation / product quality audit |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |

### Summary

Runtime continuity after P18.S1. Recommended S2: Moments Tool Session Continuity (implemented).

---

## Prior: P18.S1 - Moments Browse Truth

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/P18_S1_MOMENTS_BROWSE_TRUTH.md`, `scripts/verify-moments-browse-truth.mjs` |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete - await Owner |
| **Max layer** | Presentation (browse truth only) |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |

### Summary

App path mounts ActiveMomentProvider. Home and Continue share one moments list. Empty Continue is truthful.

---

## Prior: P18.A1 - Product Proof Regression Audit

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/capability-runtime/product-proof/P18_A1_PRODUCT_PROOF_REGRESSION_AUDIT.md` |
| **Date** | 2026-08-08 |
| **Status** | Audit complete - S1 implemented |
| **Max layer** | Documentation / Product Proof audit |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |

### Summary

Post-P17 production-candidate review. Greatest Owner friction was false-empty Moments Continue.

---

## Prior: P17.S5 - Moments Status Ownership

| Field | Value |
| --- | --- |
| **Artifacts** | `docs/ui/P17_S5_MOMENTS_STATUS_OWNERSHIP.md`, `scripts/verify-moments-status-ownership.mjs` |
| **Date** | 2026-08-08 |
| **Status** | Engineering complete - await Owner |
| **Max layer** | Presentation (Moments status ownership only) |
| **Handoff** | `P16_ENGINEERING_COMPLETE_PRODUCT_PROOF_PENDING` |

### Summary

Moments owns Delete acknowledgement inline; calm restore outcomes; dismissible/ephemeral banners.

---

## Prior: Engineering Execution Standard v1

See repository history and `docs/00-Constitution/WORKSPACE_ENGINEERING_EXECUTION_STANDARD_V1.md`.
