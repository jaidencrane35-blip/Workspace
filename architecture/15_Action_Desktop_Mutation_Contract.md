# Action Desktop Mutation Contract v1.0

Status: Active
Authority: Authoritative declared action types and desktop mutation semantics for the Action capability
Version: 1.0

This document declares action types and binds each to the semantics the Action
capability already contracts. It defines no transport, library, process
boundary, persistence engine, Win32 API, or vendor.

Authority order:

1. `00_Workspace_Blueprint.md`
2. Accepted ADRs in `architecture/decisions/`
3. `08_Workspace_Capability_Architecture.md` for capability ownership
4. `09_Capability_Interaction_Matrix.md` for allowed communication and trust boundaries
5. `10_Capability_Contracts.md` for the Action public interface and shared message semantics
6. `11_Contract_Schema_and_Acceptance_Specification.md` for conceptual fields, classes, and invariants
7. This document for declared action types and their bindings

If this document conflicts with a higher authority, the higher authority wins.

---

## Why this document exists

`10_Capability_Contracts.md` already defines the Action public interface
(`ACT-CMD-001`, `ACT-CMD-002`, `ACT-REQ-001`–`ACT-REQ-003`, `ACT-EVT-001`–
`ACT-EVT-007`), and requires that Action "execute only declared action types".
`08_Workspace_Capability_Architecture.md` records that Action owns the "action
allow-list binding to permission scopes".

No action type has ever been declared. That absence — not a missing contract
shape, and not a missing Win32 call — is what blocked `PP-M1-02` (LEDGER-0019).
This document closes it for Product Proof and nothing wider.

### What is deliberately not restated here

The following are already authoritative elsewhere and are inherited unchanged.
Restating them would create a second source of truth:

| Concern | Defined in |
|---|---|
| Message kinds, common envelope, identifiers | `10` Contract Model |
| Authorization proof and operation-control proof | `10` Authorization proof |
| Operation reference, status, terminal lookup, indeterminate | `10` Operation reference and recovery |
| Explainability envelope | `10` Explainability envelope |
| Common response outcomes and error contract | `10` Common response outcomes / Common error contract |
| Event immutability, deduplication, ordering | `10` Event rules |
| Cancellation classes | `11` Cancellation Semantics |
| Idempotency classes | `11` Idempotency Expectations |
| Retry philosophy | `11` Retry Philosophy |
| Timeout semantics | `11` Timeouts |
| Error categories | `11` Error Categories |

This document adds only: declared action types, their per-type bindings, the
item model for multi-effect operations, plan resolution, and Action-specific
error codes.

---

## 1. Declared action types

An action type is the unit of permission, safety classification, and
explanation. Action may execute a request only if every item in it names a type
declared here.

### Specified for Product Proof

| Action type | Effect | Permission scope |
|---|---|---|
| `window.place` | Set the position, size, monitor assignment, and minimized/restored state of one declared, already-existing window | `action.window.place` |
| `window.focus` | Bring one declared, already-existing window to the foreground | `action.window.focus` |

These are separately permissioned because they carry different risk. Placement
rearranges; focus changes where the user's next keystroke lands. A user may
permit one and refuse the other, and no grant covers both implicitly.

### Reserved but not specified

`08_Workspace_Capability_Architecture.md` and LEDGER-0013 place application
launch, application reuse, supported file opening, supported URL opening, and
supported workspace activation within Action's permitted domain. They are named
here so the catalogue is not mistaken for complete, and are **not specified**:

| Reserved type | Blocked by |
|---|---|
| `application.launch` | No executable identity is captured (LEDGER-0019); and an out-of-contract implementation already exists — see §14 |
| `application.reuse` | Requires launch-side identity for the same reason |
| `resource.openFile` | Product Proof captures no file targets |
| `resource.openUrl` | Product Proof captures no URL targets |
| `workspace.activate` | Not required by the Product Proof workflow |

Declaring a reserved type requires the same per-type bindings as §2 and a new
version of this document. Executing an undeclared or reserved type is
`ACTION_TYPE_NOT_DECLARED`.

### Explicitly excluded

Z-order restoration beyond `window.focus` is excluded from Product Proof. Full
stacking restoration multiplies disturbance risk for marginal return-to-work
benefit. A saved z-order is reported as an unsupported item rather than
silently dropped.

---

## 2. Per-type bindings

Each declared type binds one value from each existing class. No new class is
introduced.

| Binding | `window.place` | `window.focus` |
|---|---|---|
| Permission scope | `action.window.place` | `action.window.focus` |
| Responsiveness class (`11` Timeouts) | interactive | interactive |
| Cancellation class (`11`) | Irreversible — not cancellable after stated commit point | Irreversible — not cancellable after stated commit point |
| Commit point | The instant the window manager accepts the placement for that one window | The instant foreground assignment is accepted |
| Idempotency class (`11`) | State-setting | State-setting |
| Compensation | Compensable in principle by a future authorized action; no compensating type is declared (§8) | Compensable in principle; no compensating type is declared |
| Automatic retry | Forbidden. `11` Retry Philosophy already forbids automatic retry of unknown-outcome environment mutations and partial effects | Forbidden |
| Target | One declared window descriptor (§3) | One declared window descriptor (§3) |

Both types are irreversible per item and immediate in effect. The commit point
is per item, not per operation: an operation containing ten placements has ten
independent commit points, which is what makes honest partial success possible.

---

## 3. Target declaration and bounded resolution

A target is declared, never discovered. The requester supplies a **window
descriptor** containing the identity evidence it holds and the placement it
wants. Action resolves that descriptor to exactly one live window, or fails the
item.

Resolving a descriptor may require examining candidate windows, because identity
evidence that survives a restart is descriptive rather than a direct handle.
This is permitted strictly as a **matching operation**, bounded as follows.
These constraints are the boundary between Action and Context Sensing:

- Matching is permitted only against the specific descriptors in the request in
  front of Action, and only to decide match, no-match, or ambiguous.
- The only results that may leave the matching operation are those three
  dispositions and the minimized `target_summary` of a matched window. No
  candidate list, no unmatched window's attributes, and no count of what was
  examined may leave it, be reported, be logged, or be exposed.
- Action must not scan or sample the environment for any purpose other than
  resolving a declared descriptor, and must not do so outside an active plan or
  operation.
- Action must not retain resolved environment state beyond the bounded lifetime
  of the plan or operation that required it (§4).
- Action must not expose environment state as a queryable model, event, or
  snapshot to any capability.
- Action must not derive, infer, or persist meaning about the environment.

The distinction from Context Sensing is what the work produces, not whether it
looks. Context Sensing produces a picture of the environment for others to
consume. Matching produces a yes, a no, or a refusal for one declared target and
keeps nothing.

Resolution is point-of-use target validation, which `10` already assigns to
Action through its "invalid target" and "target unavailable" error conditions
and its duty to "enforce declared action type and exact target/scope". It is not
observation, and Action remains barred from owning observation (§15).

Resolution must be deterministic. Given the same environment and the same
descriptor it must produce the same result, because preview and execution both
depend on it (§4).

### Ambiguity is a failure, not a guess

If a descriptor resolves to more than one live window, Action must **not**
choose. The item fails with `ACTION_TARGET_AMBIGUOUS` and no effect occurs.
Guessing would risk moving a window the user did not mean, which
`LEDGER-0013` records as a trust invalidation that voids the proof regardless of
the success metric.

If a descriptor resolves to no live window, the item fails with
`ACTION_TARGET_NOT_FOUND`. This is the expected outcome for anything closed
since the context was saved, and it is reported, never repaired.

### Confidence is supplied, never invented

Where the requester's identity evidence carries a confidence classification,
Action must honour a declared minimum confidence and fail the item with
`ACTION_TARGET_CONFIDENCE_INSUFFICIENT` below it. Action does not compute
confidence and does not choose the threshold; both arrive with the request.

Deciding what confidence is good enough is a safety judgement about how much
risk of disturbing the wrong window the user is accepting. This contract
establishes only that the judgement is not Action's. Where it belongs is
deliberately not assigned here, so that it is not settled by default (§19).

---

## 4. Plan resolution — preview equals execution intent

One new public request is required. Nothing in the existing interface can
describe what would happen to a *specific set of targets*: `ACT-REQ-001
Action.describe` describes an action type, not a request.

```
ACT-REQ-004 Action.resolvePlan(action_request, plan_authorization_proof) -> ActionPlan
```

`ActionPlan` contains:

- `plan_id` — identity for this resolution
- `plan_digest` — a stable digest over the ordered, resolved item set
- one `ActionPlanItem` per requested item, in execution order, each carrying:
  - `item_id`
  - `action_type`
  - `target_summary` — minimized and non-secret, per the `10` explainability envelope
  - `projected_disposition` — `will_attempt`, `will_skip_unsupported`, or `will_skip_unresolvable`
  - `reason` — required whenever the disposition is not `will_attempt`

`ACT-REQ-004`:

- performs **no** environment mutation
- is read-only repeatable (`11` Idempotency Expectations)
- requires a plan-level authorization proof that grants **no** effect authority
- must be user-initiated, never scheduled or ambient, consistent with the zero
  ambient capture principle established by `PP-B02` (LEDGER-0016)

A plan carries a bounded validity and expires. This is a privacy constraint, not
a convenience one: an unexpiring plan is a retained picture of the environment,
which Action is not permitted to hold (§3, §15). A plan is discarded at
expiry, on consumption by an execution reaching a terminal outcome, and on
capability shutdown. An expired or consumed plan is `ACTION_PLAN_UNKNOWN`, and
resuming again requires a fresh preview. The duration is an implementation
decision; that it is bounded is architectural.

### Binding the approval

`ACT-CMD-001 Action.execute` carries the approved `plan_id` and `plan_digest`.
At execution Action re-resolves every item and compares.

- An item whose resolution is unchanged is attempted.
- An item whose resolution changed since approval is **not** attempted. It
  terminates as `refused_changed` with `ACTION_PLAN_ITEM_CHANGED`.
- An operation presenting an unknown or expired `plan_id` is rejected outright
  with `ACTION_PLAN_UNKNOWN`; no item is attempted.

Per-item refusal is deliberate rather than failing the whole operation. A single
window closing between preview and approval must not cancel an otherwise valid
restore, and must equally not become a silent substitution. Every item the user
approved either happens as previewed, or does not happen and says why.

This mirrors the consent binding already proven in `PP-M1-01`, where a save is
refused unless the confirmed capture scope is the scope the build would actually
capture (LEDGER-0018).

---

## 5. Item model, success, and partial success

An `action_request` carries an ordered, non-empty set of items. Each item names
one declared action type and one target.

Executing an ordered set within one accepted operation is not orchestration.
Companion Orchestration owns sequencing work *across* capabilities; this is one
capability applying one approved request in a stated order, and `11` already
contemplates "a multi-effect operation" in its permission cases. Action does not
choose the order, add items, chain a second operation, or react to its own
outcomes.

Each item reaches exactly one terminal disposition:

| Item disposition | Meaning |
|---|---|
| `completed` | The effect committed |
| `failed` | Attempted, did not commit; `error` required |
| `skipped_unsupported` | The saved intent has no declared action type — reported, never approximated |
| `skipped_unresolvable` | Target not found, ambiguous, or below required confidence |
| `refused_changed` | Resolution changed after approval (§4) |
| `not_attempted` | Operation ended before reaching this item |
| `outcome_unknown` | Attempted; Action cannot establish whether it committed |

The operation's terminal outcome is **derived**, never independently asserted:

- every item `completed` → `completed` (`ACT-EVT-003`)
- at least one `completed` and at least one not `completed` → `partially_completed` (`ACT-EVT-006`)
- no item `completed`, none `outcome_unknown` → `failed` (`ACT-EVT-004`)
- any item `outcome_unknown` after exhausted recovery → `indeterminate` (`ACT-EVT-007`)
- terminated by cancellation before completion → `cancelled` (`ACT-EVT-005`)

An operation reports `completed` only when every approved item committed.
Partial success is a first-class terminal outcome and is never rounded up. This
is the contract-level expression of the Product Proof rule that partial success
is acceptable and dishonest success is not.

`indeterminate` dominates: if any item's outcome cannot be established, the
operation cannot claim `completed` or `partially_completed` on the strength of
the items that did succeed. Those item outcomes are still reported individually.

---

## 6. Failure taxonomy

Action-specific codes extend, and never redefine, the `10` common error
contract. Each carries the standard `error_code`, `category`, `safe_message`,
`responsible_capability`, `retryable`, `correlation_id`, and `explanation`.

| Error code | Category | Level | Retryable |
|---|---|---|---|
| `ACTION_TYPE_NOT_DECLARED` | validation | operation | no |
| `ACTION_PLAN_UNKNOWN` | validation | operation | no — requires a new preview |
| `ACTION_REQUEST_EMPTY` | validation | operation | no |
| `ACTION_PLAN_ITEM_CHANGED` | conflict | item | no — requires a new preview |
| `ACTION_TARGET_NOT_FOUND` | conflict | item | no |
| `ACTION_TARGET_AMBIGUOUS` | conflict | item | no |
| `ACTION_TARGET_CONFIDENCE_INSUFFICIENT` | validation | item | no |
| `ACTION_TARGET_REFUSED_BY_ENVIRONMENT` | conflict | item | no |
| `ACTION_EFFECT_OUTCOME_UNKNOWN` | outcome-unknown | item | no |
| `ACTION_PLACEMENT_UNSATISFIABLE` | conflict | item | no |

`ACTION_TARGET_REFUSED_BY_ENVIRONMENT` covers a window manager declining the
effect — an elevated or otherwise protected window is the ordinary case. It is
reported as refusal, never escalated around and never retried.

`ACTION_PLACEMENT_UNSATISFIABLE` covers a placement the current monitor
arrangement cannot honour, such as a saved position on a monitor that is no
longer attached. Action does not relocate the window to somewhere else it
considers reasonable; choosing a substitute placement is a product decision that
the user must see and approve through a new preview.

No error code is retryable. Every recovery path is a fresh, user-visible attempt
with a new preview, consistent with `11` Retry Philosophy, which already forbids
automatic retry of unknown-outcome environment mutations and partial effects.

---

## 7. Explainability

Action supplies the `10` explainability envelope per operation **and** per item.
No new envelope is defined.

Per item, the following are required and non-optional:

- `what` — the effect attempted, stated as a checkable fact about one target
- `why` — the purpose carried by the request; Action never authors purpose
- `target_summary` — minimized, non-secret
- `status` — the item disposition from §5
- `reason` — required for every disposition other than `completed`
- `user_action_available` — `inspect` or `none`; never `retry`, because no
  Action failure is automatically retryable

Every item that did not complete must carry a reason a person can act on. An
operation may not report an unexplained skip. "Unsupported" without a stated
cause is a contract violation, not a terse explanation.

Action supplies explanation content. It does not present it, phrase it for a
particular surface, or decide what the user sees; Experience presents and
cannot alter domain meaning (`10`).

---

## 8. Rollback and compensation

Per `11` Cancellation Semantics, cancellation never implies rollback and
compensation is always a new explicitly authorized command.

Both declared types are irreversible at their commit point and **compensable in
principle** — a window can later be moved back. This contract deliberately
declares **no compensating action type**. Placement Undo is `PP-M1-03`.

One ownership question is left open rather than answered here. Compensation
needs the placement that existed before the effect. Action's contracted state is
"action catalogue, in-flight execution state, metadata-only execution audit, and
minimized effect summary" (`10`) — retaining prior environment state for later
reversal is not obviously within it, and may belong to Workspace Management as
part of the saved-context record instead.

Assigning that ownership is a `PP-M1-03` decision. It is recorded here as an
open question so that no implementation quietly resolves it by writing an undo
buffer into Action, which would give Action a memory it is not permitted to own
(§15).

Until then, an operation is honest about irreversibility: Action must not
present, imply, or accept a request premised on an undo that does not exist.

---

## 9. Cancellation

Both types are `Irreversible — not cancellable after stated commit point`
(`11`). Because commit points are per item, cancellation of a multi-item
operation is meaningful between items and meaningless within one.

On cancellation, Action:

- stops before the next unstarted item
- marks every unreached item `not_attempted`
- allows an in-flight item to reach its own commit point; that item reports its
  real disposition
- emits `ACT-EVT-005 ActionCancelled` with the full per-item outcome set
- never reverses a committed item

Cancellation follows `ACT-CMD-002` and requires the operation-control proof,
which per `10` remains valid for safety control even after effect authority is
revoked or expires. Cancelling never reverses and never erases; a cancelled
restore that already moved four windows says exactly that.

---

## 10. Idempotency

Both declared types are **State-setting** (`11`): applying the same placement or
the same foreground assignment again produces no additional effect.

Two situations must not be confused at the operation level:

- **Delivery retry.** Redelivery of the *same* accepted command identity returns
  the same operation identity and reference and creates no second operation,
  per `10`. This holds even after the plan is consumed, because the answer comes
  from the operation record, not from re-resolving the plan.
- **Reuse.** A *new* command citing an already consumed or expired `plan_id` is
  `ACTION_PLAN_UNKNOWN`. Resuming again requires a fresh preview.

Without this distinction a lost acceptance response would look like an attempt
to replay a spent approval, and a genuine replay would look like a retry.

State-setting types are safe to repeat but are still never retried
automatically, because an unknown outcome may mean the effect committed and the
report was lost.

---

## 11. Progress reporting

Action emits `ACT-EVT-002 ActionProgressed` as items reach terminal
dispositions. Progress carries the completed count, the total, and the
disposition of the item just finished.

Progress reports what has already happened. It never predicts, never estimates a
remaining duration, and never reports an item as complete before its commit
point. An operation with a single item may omit progress entirely; acceptance
and the terminal event are sufficient.

---

## 12. Event sequence

For one accepted operation, in order:

1. `ACT-EVT-001 ActionStarted` — once, after the operation is accepted
2. `ACT-EVT-002 ActionProgressed` — zero or more, one per item reaching a terminal disposition
3. exactly one terminal event: `ACT-EVT-003 ActionCompleted`, `ACT-EVT-004 ActionFailed`, `ACT-EVT-005 ActionCancelled`, `ACT-EVT-006 ActionPartiallyCompleted`, or `ACT-EVT-007 ActionIndeterminate`

Constraints, all inherited from `10` Event rules:

- exactly one terminal event per operation
- the terminal event carries the complete per-item outcome set, so a consumer
  that missed progress events still receives the whole truth
- terminal events reuse the accepted command's `correlation_id`
- events carry `authorization_id` only, never proof content
- a missing or delayed event never authorizes anything and never implies success

`ActionStarted` marks acceptance, not authorization. Each item's authorization is
validated at point of use immediately before that item's effect, per `IC-009`,
because each item is an independently meaningful effect. There is no
operation-wide validation that clears the whole batch in advance; that is what
makes revocation between items enforceable (ADM-AC-17).

---

## 13. Recovery

Action inherits recovery unchanged and defines nothing new:

- `ACT-REQ-002 Action.getStatus` / `OPR-REQ-001` — authoritative in-flight status
  against the operation reference and control proof
- `ACT-REQ-003 Action.lookupTerminalOutcome` / `OPR-REQ-002` — content-free
  terminal outcome under a fresh user-administration proof
- `OPR-EVT-001` / `ACT-EVT-007 ActionIndeterminate` — recovery exhausted

If Action restarts mid-operation, items already committed remain committed.
Action must report the operation as `indeterminate` unless it can establish each
item's real disposition. It must never re-attempt items to find out, because
re-attempting is a new environment mutation without authorization.

An operation whose terminal outcome was never observed is `outcome-unknown` to
the requester until Action's authoritative status resolves it. Silence is never
success.

---

## 14. Ownership boundaries

Action is the only owner of operating-system mutation. Within one restore:

| Concern | Owner |
|---|---|
| Which saved context exists, and what it contains | Workspace Management |
| Capturing the desktop, and any retained picture of it | Context Sensing |
| Deciding a restore should happen and sequencing its steps | Companion Orchestration |
| Deciding whether the restore is permitted, and issuing proof | Permission Authority |
| Presenting the preview and collecting the user's answer | Experience |
| Resolving declared targets and committing the effects | **Action** |
| Reporting what happened, per item | **Action** |

### Action knows nothing about saved contexts

Action must not accept, store, dereference, or resolve a saved-context
identifier, and must not be given a shape that requires knowing what a saved
context is. It receives declared targets in its own grammar and nothing else.

Translating a saved context into an action request happens outside Action.
Passing a context identifier across the boundary and letting Action read the
record would be the shortest path to a working restore and would couple Action
to Workspace Management's domain permanently. It is prohibited.

### Known drift this contract does not itself resolve

`LaunchApplication` in the current codebase mutates the environment — it spawns
a real process — as an ordinary kernel mutation command guarded by
`application.launch`. It does not pass through `ACT-CMD-001`, emits no
`ACT-EVT-*` events, carries no operation-control proof, and belongs to no action
catalogue (LEDGER-0019).

It is therefore environment mutation performed outside the capability that alone
owns it. This contract does not retroactively legalise it. `application.launch`
is listed as reserved and unspecified in §1 precisely so that declaring it later
forces the reconciliation rather than quietly blessing the existing path.

---

## 15. Explicit non-responsibilities

Action must never own, and must never be implemented to acquire:

- **Reasoning.** Action does not decide what to restore, infer intent, rank
  items, or judge whether a restore is worthwhile.
- **Orchestration.** Action executes one accepted request. It does not chain
  operations, schedule follow-ups, or react to its own outcomes.
- **Permission decisions.** Action validates a proof at point of use and refuses
  without one. It never grants, caches as authority, widens, or infers consent.
- **Memory.** Action retains in-flight state, metadata-only audit, and a
  minimized effect summary. It does not retain environment state, prior
  placements, user content, or history beyond its declared retention.
- **Observation.** Action resolves declared targets under §3 and nothing more.
  It does not enumerate, monitor, sample, or model the environment, and exposes
  no environment state to any capability.
- **User interaction.** Action supplies explanation content and never presents
  it, phrases it for a surface, prompts, or interprets a user's answer.
- **Target invention.** Action never substitutes, approximates, or best-guesses
  a target or a placement. Ambiguity fails; it does not resolve.

A contradiction with any of these is a contract violation, not an
implementation trade-off.

---

## 16. Acceptance criteria

Conceptual Given/When/Then cases for this contract, in its own namespace to
avoid collision with the numbered cases in `11`. The generic cases in `11`
Required Acceptance Cases apply in addition and are not restated.

### Declaration

- **ADM-AC-01** Given an item naming a type not declared in §1, when execute is called, then the operation is rejected with `ACTION_TYPE_NOT_DECLARED` and no item is attempted.
- **ADM-AC-02** Given an item naming a reserved but unspecified type, when execute is called, then it is rejected identically; reservation is not declaration.

### Preview equals execution

- **ADM-AC-03** Given a resolved plan, when the same environment is unchanged at execution, then the attempted item set is exactly the plan's `will_attempt` set.
- **ADM-AC-04** Given an approved plan, when one item's resolution changed before execution, then that item is `refused_changed`, no effect occurs for it, and the remaining approved items proceed.
- **ADM-AC-05** Given an unknown or consumed `plan_id`, when execute is called, then the operation is rejected with `ACTION_PLAN_UNKNOWN` and no item is attempted.
- **ADM-AC-06** Given `ACT-REQ-004` is called, when it returns, then no environment mutation has occurred and the plan grants no effect authority.

### Honest outcomes

- **ADM-AC-07** Given a mix of committed and failed items, when the operation ends, then the terminal outcome is `partially_completed`, never `completed`.
- **ADM-AC-08** Given any item with an unestablished outcome, when recovery is exhausted, then the operation is `indeterminate` regardless of how many items committed.
- **ADM-AC-09** Given any item that did not complete, when the outcome is reported, then that item carries a reason; an unexplained skip is a violation.
- **ADM-AC-10** Given a saved intent with no declared action type, when the plan is resolved, then it is `will_skip_unsupported` with a reason and is never approximated by a different type.

### Targeting safety

- **ADM-AC-11** Given a descriptor matching more than one live window, when resolved, then the item fails `ACTION_TARGET_AMBIGUOUS` and no window is moved.
- **ADM-AC-12** Given a descriptor matching no live window, when resolved, then the item is `skipped_unresolvable` with `ACTION_TARGET_NOT_FOUND` and nothing is launched to satisfy it.
- **ADM-AC-13** Given identity evidence below the requester's declared minimum confidence, when resolved, then the item fails `ACTION_TARGET_CONFIDENCE_INSUFFICIENT` and Action neither computes nor lowers the threshold.
- **ADM-AC-14** Given a saved placement the current monitor arrangement cannot honour, when resolved, then the item is `ACTION_PLACEMENT_UNSATISFIABLE` and Action does not substitute a placement of its own choosing.
- **ADM-AC-15** Given the window manager refuses an effect, when the item ends, then it is `failed` with `ACTION_TARGET_REFUSED_BY_ENVIRONMENT`, is not retried, and no elevation is attempted.

### Permission

- **ADM-AC-16** Given a proof scoped to `action.window.place`, when an item requires `action.window.focus`, then that item is denied at point of use; no grant covers both implicitly.
- **ADM-AC-17** Given effect authority is revoked between items, when the next item begins, then it does not begin, committed items are explained, and the operation terminates honestly.
- **ADM-AC-18** Given a plan-level proof, when execute is attempted with it, then execution is denied; plan authority is not effect authority.

### Cancellation and recovery

- **ADM-AC-19** Given cancellation between items, when it is honoured, then unreached items are `not_attempted`, committed items are not reversed, and `ACT-EVT-005` carries the full item set.
- **ADM-AC-20** Given cancellation arriving after an item's commit point, when reported, then too-late status is explained for that item and no reversal is claimed.
- **ADM-AC-21** Given Action restarts mid-operation, when status is requested, then unestablished items are `outcome_unknown` and Action does not re-attempt them to discover their state.

### Boundaries

- **ADM-AC-22** Given a resolved plan, when the operation ends, then no resolved environment state is retained and no environment model is exposed to any capability.
- **ADM-AC-23** Given a restore completes, when Action's retained state is inspected, then it contains no prior placement, no user content, and nothing supporting an undo Action is not permitted to own.
- **ADM-AC-24** Given an explanation or error, when presented, then it contains no proof content, no raw environment data, and no unrelated window's detail.
- **ADM-AC-25** Given descriptor matching examined several candidate windows, when the item is reported, then only match/no-match/ambiguous and the matched window's minimized summary leave the operation; no candidate list, unmatched attributes, or examined count appears anywhere.
- **ADM-AC-26** Given a plan that is neither executed nor refreshed, when its bounded validity expires, then it is discarded and a later execution against it is `ACTION_PLAN_UNKNOWN`.
- **ADM-AC-27** Given a request carrying a saved-context identifier, when it reaches Action, then it is rejected as contract-invalid; Action never dereferences a Workspace Management record.

---

## Contract Acceptance Record

Per `11` Contract Acceptance Record:

- **Contract identity and version** — Action Desktop Mutation Contract v1.0
- **Owner** — Action
- **Allowed initiators** — Companion Orchestration only (`IC-029`, `IC-030`)
- **Allowed subscribers** — the requesting Companion task (`IC-031`); Runtime Host for health only
- **Purpose** — declare the action types required to restore a bounded saved workspace context on explicit user approval
- **Message kinds** — inherits `ACT-CMD-001`, `ACT-CMD-002`, `ACT-REQ-001`–`ACT-REQ-003`, `ACT-EVT-001`–`ACT-EVT-007`; adds `ACT-REQ-004`
- **Conceptual field/presence catalogue** — inherited from `10` common envelope; adds `plan_id`, `plan_digest`, `item_id`, item disposition
- **Permission scopes and validation point** — `action.window.place`, `action.window.focus`; validated at point of use immediately before each item's effect
- **State owner/references** — Action owns catalogue, in-flight state, plan resolutions for their bounded lifetime, metadata-only audit, minimized effect summary
- **Failure/cancellation/timeout/retry/idempotency rules** — §2, §6, §9, §10; classes inherited from `11`
- **Privacy/retention classification** — no user content; minimized non-secret target summaries; no retained environment state
- **Local First behavior** — fully local; no network dependency in any path
- **Explainability mapping** — §7, per operation and per item
- **Compatibility/deprecation assessment** — additive. `ACT-REQ-004` is a new request; declaring a reserved type requires a new version of this document
- **Required acceptance cases and outcomes** — ADM-AC-01 to ADM-AC-27, plus the generic cases in `11`
- **Blueprint/ADR compliance** — §17
- **Unresolved risks** — §19
- **Approval status and date** — Accepted 2026-08-02
- **Engineering Ledger reference** — LEDGER-0020

---

## 17. Blueprint and ADR compliance

| Principle | How this contract satisfies it |
|---|---|
| Local First | Every path is local; no network dependency exists in preview, approval, execution, or reporting |
| Human First | Nothing executes without an explicit preview and an explicit approval bound to that preview |
| Privacy First | No user content is read or retained; target summaries are minimized; no environment model is retained or exposed |
| Permission Before Automation | Per-type scopes, no omnibus grant, validated at point of use before each item |
| Explain Every Action | Per-item explanation is mandatory, and every non-completion must state a reason |
| Build Only Where We Create Value | Two action types, one new request. Everything else is inherited |
| Complexity Must Justify Itself | The item model exists only because honest partial success requires it |
| Reduce Cognitive Load | One preview, one approval, one honest result |

Companion Principles are satisfied by construction: the contract cannot report
success it did not achieve, cannot act without prior preview, cannot guess a
target, and cannot retry behind the user's back.

`DEC-008` (only the Windows integration layer talks to OS APIs) is unaffected.
This contract governs the Action capability's semantics; whichever layer
ultimately performs the OS call remains bound by `DEC-008`.

---

## 18. Product Proof workflow verification

The contract was checked against the required workflow end to end.

| Step | Owner | Mechanism |
|---|---|---|
| Save | Workspace Management + Context Sensing | `PP-M1-01`, already delivered (LEDGER-0018) |
| Leave | — | No Workspace involvement; nothing runs |
| Resume | Experience | User opens a saved context; no trigger exists other than the user |
| Preview | Action | `ACT-REQ-004`, mutating nothing, presented by Experience |
| Approve | Permission Authority | Per-type scopes; the user's decision produces the proof |
| Restore | Action | `ACT-CMD-001` bound to the approved `plan_id` and `plan_digest` |
| Explain | Action supplies, Experience presents | §7, per operation and per item |
| Complete | Action | Exactly one terminal event carrying the full item set (§12) |

Confirmed absent from every step:

- **AI** — no reasoning, ranking, inference, or model participation anywhere in
  the path; §15 forbids it and no step requires it
- **Background observation** — matching occurs only inside a user-initiated plan
  or operation and retains nothing (§3, §4)
- **Ambient capture** — no trigger exists other than explicit user action,
  preserving `PP-B02`
- **Cloud** — every path is local; no network dependency exists
- **Plugins** — Extension Host is not an initiator; `IC-029` permits Companion only
- **Voice, adaptation, recommendations** — no step consults or requires them

The workflow is therefore satisfiable by this contract alone, given the
prerequisite in risk 4 below.

---

## 19. Unresolved risks

1. **Compensation ownership is undecided** (§8). Whether the pre-effect
   placement needed for Undo belongs to Workspace Management's saved-context
   record or to Action is a `PP-M1-03` decision. Recorded so no implementation
   settles it silently.
2. **`application.launch` drift persists** (§14). Environment mutation exists
   outside Action today. This contract isolates rather than resolves it.
3. **Window identity across a restart is unproven.** This contract requires the
   requester to supply identity evidence and a confidence threshold, and fails
   safely when they are insufficient. Whether identity can be re-established
   often enough for Resume to feel useful is a product question that `PP-M1-02`
   will answer empirically, not an architectural one.
4. **No saved context currently carries the identity evidence this contract
   requires.** `PP-M1-01` persisted title, process id, and geometry but not
   `hwnd` or `stable_window_id` (LEDGER-0019). Closing that schema gap is
   Workspace Management's work and is a prerequisite for `PP-M1-02`.
5. **Reserved types remain unspecified** (§1) and cannot be executed until
   declared, which requires a new version of this document.
6. **`09_Capability_Interaction_Matrix.md` requires one amendment.** `IC-030`
   enumerates the Companion → Action requests as cancel, reconcile, describe,
   and delayed terminal lookup. `ACT-REQ-004 Action.resolvePlan` is not among
   them, so the matrix does not yet authorize the interaction this contract
   depends on. The amendment is to extend `IC-030` to include plan resolution,
   with a plan-level authorization proof that confers no effect authority. It
   was not made in this session because `09` carries unrelated uncommitted
   `PP-P00` changes and was outside this session's authorized edit set. Until
   it is made, the matrix and this document disagree, and the matrix is the
   higher authority.
7. **The owner of the confidence threshold is unassigned** (§3). This contract
   establishes only that it is not Action's. Leaving it unowned is safe for now
   because Action fails closed below whatever threshold arrives, but it must be
   assigned before `PP-M1-02` implementation, or it will be settled by whoever
   writes the first caller.
