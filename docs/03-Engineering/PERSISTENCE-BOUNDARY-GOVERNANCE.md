# Persistence Boundary Governance

| Field | Value |
|-------|-------|
| **Purpose** | Define repository enforcement beneath existing lifecycle owners |
| **Owner** | Platform Kernel |
| **Status** | Active |

## Ownership

Domain services remain lifecycle authorities. Repositories do not choose
transitions; they reject writes that contradict the stored authoritative state.

| Persistence boundary | Write owner | Repository enforcement |
|----------------------|-------------|------------------------|
| Execution lifecycle | `ExecutionLifecycleService` | Conditional claim/terminal SQL plus schema checks |
| Recommendation lifecycle | `WorkspaceRecommendationEngineService` | Stored-state transition matrix; controlled generation reopen; same-state terminal writes use field-level identity protection |
| Decision Engine lifecycle | `DecisionEngineService` | Terminal outcomes cannot reopen; intake provenance frozen on every conflict update including invalidation; evaluate-once artifacts reject replacement |
| Decision Queue overlay | `DecisionQueueService` | Overlay transition matrix; dismissed remains terminal |
| Task Graph | `TaskGraphService` | Terminal tasks cannot return to open status; same-terminal sync only; progress/explanation cannot weaken |
| Observation snapshots | `WorkspaceObservationService` | Snapshot validation and transactional persistence |
| Audit evidence | `AuditService` | Insert-only repository and database-enforced update/delete rejection |
| Suggestions | Suggestion services | No separate repository; projections consume immutable audit evidence |

## Immutable artifacts and terminal evidence

Not every row is immutable. Repositories protect authoritative identity fields:

### Recommendation same-state terminal writes

**Immutable (identity-bearing):**
- outcome payload
- prior outcome history
- resolution_type and resolved_at once committed
- content fingerprint once set
- sealed intake package (full seal equality)
- confirmation identity owners / authority flags; confirmed/declined confirmation freeze
- adapter preparation digests, prepared_at, contract/fingerprint/confirmation intent
- handoff digests, request identity timestamps, contract/family/confirmation intent
- DE acceptance digests, contract/family/confirmation intent

**Explicitly mutable (lifecycle evolution only):**
- overlay `updated_at`, `actor_id`
- confirmation state/intent/confirmed_at/note while still `required`/`not_required`
- adapter preparation revoke/rebind fields (`preparation_state`, `revoked_at`, `seal_aligned`, notes/flags)
- handoff revoke/rebind fields (`request_state`, `handoff_requested`, `revoked_at`, alignment flags, notes)
- DE acceptance accept/decline/revoke/rebind state timestamps and ownership declaration fields

Generation reopen remains a controlled terminal→available transition requiring changed
fingerprint and retained prior outcomes.

### Decision Engine

- Evaluate-once records accept identical idempotent retries and reject divergent
  replacements via `DatabaseError::ImmutableArtifact`.
- Intake provenance (receipt, recommendation reference, seal digest, acceptance
  reference, compatibility version, created_at) cannot change on conflict updates,
  including `active → invalidated`.

### Task Graph

- Open→terminal remains allowed.
- Terminal sync is same-status only (completed stays completed; cancelled stays
  cancelled), preserving progress and non-empty explanation.

## Error classification

Repository transition rejection uses `DatabaseError::InvalidTransition`.
Immutable artifact / terminal evidence rejection uses
`DatabaseError::ImmutableArtifact`. Kernel service persistence helpers map both
to the owning subsystem validation public error
(`workspace_recommendation_engine_validation_error`,
`decision_engine_validation_error`, `decision_queue_validation_error`,
`task_graph_validation_error`) rather than opaque `database_error`.

## Boundary rule

Production writers must call the owning service. Repository guards are
defense-in-depth against future caller mistakes and do not transfer authority to
the database layer. Cross-domain repository access remains read-only.

## Related

- [Lifecycle Governance](./LIFECYCLE-GOVERNANCE.md)
- [Governed Execution Audit Durability](./GOVERNED-AUDIT-DURABILITY.md)
- [Kernel Error Taxonomy](./KERNEL-ERROR-TAXONOMY.md)
