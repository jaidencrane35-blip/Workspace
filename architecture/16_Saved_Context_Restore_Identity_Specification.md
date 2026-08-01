# Saved Context Restore Identity Specification v1.0

Status: Active
Authority: Authoritative identity contract for restoring a saved Workspace Context
Version: 1.0

This document defines the minimum identity evidence Workspace Management retains
for a saved window and Action may use to resolve that declared target. It
selects no storage representation, serialization, matching library, or Windows
API.

Authority order:

1. `00_Workspace_Blueprint.md`
2. Accepted ADRs in `architecture/decisions/`
3. `08_Workspace_Capability_Architecture.md` for ownership
4. `09_Capability_Interaction_Matrix.md` for permitted communication
5. `10_Capability_Contracts.md` for public contract semantics
6. `11_Contract_Schema_and_Acceptance_Specification.md` for shared invariants
7. `15_Action_Desktop_Mutation_Contract.md` for Action matching and effects
8. This document for saved-context restore identity

Higher authority wins on conflict.

---

## 1. Scope

This specification supports only the Product Proof operations already declared
as `window.place` and `window.focus`:

- one local Windows device
- one continuing interactive Windows desktop session
- a window that still exists
- explicit user-initiated preview followed by explicit approval

It does not define application launch, application reuse after a process exits,
file or URL opening, cross-device restore, cross-session restore, or arbitrary
application-state restoration. Those effects remain reserved and unspecified in
`15_Action_Desktop_Mutation_Contract.md`.

This is the smallest truthful restore supported by the captured data. A wider
claim would require executable or resource identity that Product Proof does not
capture.

---

## 2. Ownership

There is exactly one owner for each decision:

- **Context Sensing owns observation-time evidence.** During the explicit Save
  capture it records the desktop-session identity, handle, process identifier,
  title, and capture time. It never authorizes or performs a restore.
- **Workspace Management owns the durable saved copy.** The restore descriptor
  is part of the user-approved saved context, immutable after Save, and deleted
  with it. Workspace Management does not refresh, match, or score it.
- **Action owns point-of-use matching and the minimum admissible matching
  confidence.** It resolves the declared descriptor, applies its fixed safety
  floor, and produces one exact target or refuses the item.
- **Permission Authority alone owns effect authorization.** Authorization
  cannot lower Action's floor or make an ambiguous target exact.
- **Companion Orchestration copies selected saved fields into ordered Action
  items and sequences preview, approval, and execution.** It never scores or
  chooses among candidate windows.
- **Experience presents limitations and outcomes.** It never matches or changes
  domain meaning.

No responsibility above is shared.

### Why Action owns the threshold

The threshold determines whether Action may commit an effect against the exact
target in front of it. Capability Architecture assigns exact target validation,
declared action safety, and environment mutation to Action.

Assigning it elsewhere would create duplicate responsibility:

- Permission Authority would mix permission truth with execution safety.
- Workspace Management would acquire action semantics despite not owning
  desktop control.
- Companion would become a second safety authority able to lower Action's
  floor.

Action may refuse a permitted operation as unsafe. It may never grant itself
permission because a match is safe.

---

## 3. Canonical saved identity

A Windows saved-window item has exactly one of:

1. `restore_identity`, containing all required fields below; or
2. `restore_identity_unavailable_reason`, explaining why this item cannot be
   restored.

A missing field never produces a partial descriptor.

`restore_identity` contains:

- `identity_schema_version` — required; interpretation version
- `desktop_session_id` — required; opaque identifier for one interactive
  Windows desktop session
- `captured_hwnd` — required; window handle observed at Save
- `captured_process_id` — required; process identifier observed at Save
- `title_fingerprint` — required; deterministic normalized fingerprint of the
  already-approved captured title
- `captured_at` — required; inherited from the saved context capture

Existing saved-window/context fields may satisfy these conceptual fields; they
must not be duplicated solely to create a nested runtime type. The only new
persisted facts are those the current saved context lacks.

The saved-window item identity remains the identity used for plan and outcome
correlation.

### Producer requirements

The Windows integration layer supplies `desktop_session_id` to Context Sensing
at capture and to Action during bounded matching. The identifier:

- is stable for one interactive Windows logon desktop even if Workspace exits
  and restarts
- changes after logoff, reboot, user switch, or movement to another device
- contains no user identity or secret
- grants no authority

The v1 title fingerprint is produced identically at Save and match time:

1. split the title on Unicode whitespace
2. join segments with one ASCII space
3. apply Unicode lowercase mapping

Changing these steps requires a new identity schema version. There is no fuzzy
comparison.

`captured_hwnd` and `captured_process_id` are volatile evidence. A handle may be
reused after a window closes, and a process identifier may be reused after a
process exits. Neither is stable, portable, or sufficient alone.

### Evidence deliberately excluded

The existing `stable_window_id` is not part of the restore descriptor. It is a
Context Sensing-local cross-pass correlation id, and Windows does not expose it
on a live window for Action to compare. Persisting it for restore would collect
data that Product Proof does not use.

The existing observation-time confidence is also excluded. Action's
point-of-use exact-session result is the only confidence relevant to committing
these effects. Keeping both would create two confidence authorities.

---

## 4. Capture and consent

Persisting restore identity widens the `PP-M1-01` capture scope. Implementation
must introduce a new saved-context scope identifier and update the pre-capture
checklist before any identity field is captured.

The checklist must explain, in user language:

- Workspace stores the Windows desktop-session, window, and process identifiers
  needed to recognize the same still-open window later
- those identifiers do not include a program name, executable path, document
  content, screenshot, keystroke, clipboard, credential, or telemetry
- restore is limited to the same continuing Windows desktop session

Consent to the previous scope does not authorize these fields. A stale scope is
refused exactly as `PP-M1-01` already requires.

If complete identity cannot be produced for one window, Save still records the
window and an explicit `restore_identity_unavailable_reason`. It does not invent
fields, drop the window, fail the whole context, or retry observation.

### Existing contexts

Contexts saved under the earlier scope have no restore identity:

- schema migration leaves identity absent and records the legacy reason
- Workspace never backfills through observation, inference, or background work
- browse and inspect remain available
- preview reports each affected window as unsupported with
  `ACTION_TARGET_IDENTITY_UNAVAILABLE`
- only a new explicit Save under the new scope can create restore identity

---

## 5. Matching inputs

Companion copies the complete `restore_identity`, the saved-window item
identity, the existing saved title as a minimized `target_summary`, and the
desired effect into one Action target descriptor. It passes no saved-context
identifier, and Action never reads Workspace Management state.

The title is already within the user-approved saved context. Action may hold the
summary only for the bounded request/operation and must not put it in durable
audit or retained state.

Action receives no confidence threshold from any caller. The threshold is part
of Action's declared `window.place` and `window.focus` safety policy.

At `Action.resolvePlan` and again immediately before execution, Action's bounded
matcher may compare only:

- current `desktop_session_id`
- current window handle
- current process identifier
- current title fingerprint under the descriptor's schema version

No candidate list, unmatched attributes, observed count, resolved live handle,
or environment snapshot may leave Action, be logged, or be retained.

---

## 6. Deterministic matching and confidence

Product Proof declares one admissible class: `exact_session`.

A target is `exact_session` only when:

1. the identity schema version is supported
2. the current desktop session equals `desktop_session_id`
3. exactly one live candidate has the same window handle
4. that candidate has the same process identifier
5. that candidate has the same title fingerprint

All conditions are required. The handle check prevents broad title/process
matching; the process and title checks defend against handle reuse.

The Action-owned minimum for `window.place` and `window.focus` is
`exact_session`:

- no caller, grant, preference, or automatic policy may lower it
- no heuristic, fuzzy comparison, ranking, or tie-break is permitted
- changing it requires an accepted Action contract revision

A title change, process restart, window recreation, desktop-session change, or
schema mismatch is not restorable in `PP-M1-02`.

---

## 7. Matching outputs

The bounded matcher produces exactly one internal result:

- `exact_session_match`
- `no_match`
- `ambiguous`
- `identity_unavailable`
- `identity_version_unsupported`
- `not_portable`

Only the disposition, safe reason, and minimized `target_summary` cross Action's
boundary. The existing Action outcomes remain authoritative:

- non-exact results are `will_skip_unresolvable` during preview
- changed resolution after approval is `refused_changed`
- no non-exact result may be upgraded during execution

---

## 8. Failure and explainability

Identity failures use:

- `ACTION_TARGET_IDENTITY_UNAVAILABLE`
- `ACTION_TARGET_IDENTITY_VERSION_UNSUPPORTED`
- existing `ACTION_TARGET_NOT_FOUND`
- existing `ACTION_TARGET_AMBIGUOUS`
- existing `ACTION_TARGET_CONFIDENCE_INSUFFICIENT`
- existing `ACTION_PLAN_ITEM_CHANGED`

Every non-exact item explains:

- the affected saved window using its already-approved title
- that nothing was moved for that item
- the safe reason: legacy/missing identity, different desktop session, window
  closed or recreated, title changed, ambiguous target, or unsupported version
- that Workspace did not retry, launch, substitute, or search in the background
- available user action: inspect or dismiss only

Action supplies the outcome and reason. Experience presents it unchanged.
Companion may sequence only repair already supported by a declared capability;
it may not invent automatic repair.

---

## 9. Lifetime and portability

The descriptor has the same lifetime as its saved context, is immutable, and is
deleted with the context. It is never refreshed automatically.

The effect is portable only within the same continuing interactive Windows
desktop session. The context remains browseable elsewhere, but its placement
items are unsupported.

No guarantee is made across:

- window or process recreation
- title change
- logoff, reboot, user switch, or another desktop session
- another device, profile, installation, or imported database
- a monitor arrangement that cannot satisfy the saved placement

Failure outside the boundary is normal Product Proof evidence, not corruption
and not a reason to broaden matching.

---

## 10. Acceptance criteria

- **SCRI-AC-01** Given a new Save, when identity is persisted, then the user
  first reviewed and confirmed the new capture scope.
- **SCRI-AC-02** Given a context saved under the old scope, when previewed, then
  its windows are unsupported and no observation or backfill occurs.
- **SCRI-AC-03** Given exact session, handle, process, and title-fingerprint
  continuity, when Action resolves the descriptor, then the item is
  `will_attempt`.
- **SCRI-AC-04** Given handle reuse by a different process or title, when
  resolved, then no effect is attempted.
- **SCRI-AC-05** Given process or window recreation, when resolved, then the item
  is unresolvable even if title text is unchanged.
- **SCRI-AC-06** Given a different desktop session, when resolved, then the item
  is non-portable and no candidate enumeration follows.
- **SCRI-AC-07** Given an unsupported identity version, when resolved, then the
  item fails closed without fallback normalization.
- **SCRI-AC-08** Given a caller supplies a threshold, when Action receives the
  request, then it is contract-invalid; callers do not control the safety floor.
- **SCRI-AC-09** Given valid permission but a non-exact match, when execution is
  requested, then no effect occurs.
- **SCRI-AC-10** Given an exact match but no valid item effect proof, when
  execution is requested, then no effect occurs.
- **SCRI-AC-11** Given matching examined candidates, when a plan or outcome
  crosses Action's boundary, then no handle, candidate list, unmatched detail,
  or observed count is present.
- **SCRI-AC-12** Given a previewed exact match changes before execution, when
  re-resolved, then the item is `refused_changed`.
- **SCRI-AC-13** Given identity cannot be captured for one window, when Save
  commits, then that window remains visible with an explicit unsupported reason
  and no other observation attempt occurs.
- **SCRI-AC-14** Given the context is deleted, when deletion commits, then its
  restore identities are deleted with it.
- **SCRI-AC-15** Given no internet, AI, plugin, voice, recommendation, or
  background sensing, when local prerequisites exist, then matching remains
  available.

---

## 11. Adversarial ownership review

- **Action owns too much:** no. It owns only bounded exact matching, its fixed
  safety floor, and the effect. It reads no saved-context record and retains no
  descriptor or candidate state after the bounded request/operation.
- **Workspace Management owns too much:** no. It stores the user-approved
  artifact and nothing live. It never generates confidence, matches, or
  refreshes.
- **Companion owns too much:** no. It copies a descriptor, preserves order,
  requests authorization, and sequences contracts. It cannot lower the floor,
  select a candidate, or reinterpret failure.
- **Matching leaks observation:** no. Candidate inspection occurs only inside an
  explicit Action plan/effect, and only disposition plus a minimized matched
  summary may leave. No snapshot, subscription, buffer, or background trigger
  exists.
- **Matching leaks memory:** no. Action retains identity only during the bounded
  request/operation. Workspace Management's durable copy is part of the
  explicitly saved artifact already accepted in `PP-M1-01`.
- **Matching leaks intelligence:** no. The algorithm is exact equality over
  versioned fields, with no ranking, inference, model, recommendation, or
  adaptive threshold.

No ownership violation remains.

---

## 12. Implementation prerequisites

`PP-M1-02` implementation must:

1. add nullable canonical identity storage and an unavailable reason for legacy
   or incomplete items
2. introduce and present a new Save capture-scope version before identity
   capture
3. copy explicit capture evidence into new saved contexts without later refresh
4. implement the shared v1 title fingerprint and desktop-session identifier
5. implement Action's exact-session matcher and fixed Action-owned floor
6. keep transient titles, handles, and candidate details out of Action
   persistence
7. report every legacy, stale, non-portable, changed, ambiguous, and unsupported
   item honestly
8. add automated evidence for `SCRI-AC-01` through `SCRI-AC-15` and applicable
   `ADM-AC-*` cases

These prerequisites do not authorize `PP-M1-03`, application launch/reuse,
resource opening, or cross-session restore.
