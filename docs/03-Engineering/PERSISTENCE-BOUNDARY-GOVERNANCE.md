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
| Recommendation lifecycle | `WorkspaceRecommendationEngineService` | Stored-state transition matrix; controlled generation reopen requires changed fingerprint and retained prior outcome; same-state terminal writes cannot erase or replace outcome/seal/handoff/confirmation evidence |
| Decision Engine lifecycle | `DecisionEngineService` | Terminal outcomes cannot reopen; invalidated intake cannot reactivate; invalidated provenance is frozen; evaluate-once artifacts (evaluations, dispositions, creation, origins, resolutions, scores, selections, progression) reject replacement |
| Decision Queue overlay | `DecisionQueueService` | Overlay transition matrix; dismissed remains terminal |
| Task Graph | `TaskGraphService` | Terminal tasks cannot return to open status; completed progress/explanation cannot be weakened by sync |
| Observation snapshots | `WorkspaceObservationService` | Snapshot validation and transactional persistence |
| Audit evidence | `AuditService` | Insert-only repository and database-enforced update/delete rejection |
| Suggestions | Suggestion services | No separate repository; projections consume immutable audit evidence |

## Immutable artifacts and terminal evidence

Not every row is immutable. Repositories protect authoritative artifacts only:

- Recommendation terminal evidence (outcome, sealed intake, handoff, acceptance,
  confirmed/declined confirmation, prior outcomes, fingerprint) cannot be erased
  or replaced on same-state writes. Metadata and pre-terminal confirmation
  accumulation remain allowed. Generation reopen stays a controlled
  terminal→available transition.
- Decision Engine evaluate-once records accept identical idempotent retries and
  reject divergent replacements via `DatabaseError::ImmutableArtifact`.
- Invalidated intake provenance fields remain historically frozen while lifecycle
  reason metadata may still update under service control.
- Completed task progress and explanation cannot be weakened by synchronization.

## Error classification

Repository transition rejection uses `DatabaseError::InvalidTransition`.
Immutable artifact / terminal evidence rejection uses
`DatabaseError::ImmutableArtifact`. Kernel service persistence helpers map both
to the owning subsystem validation public error
(`workspace_recommendation_engine_validation_error`,
`decision_engine_validation_error`, `decision_queue_validation_error`,
`task_graph_validation_error`) rather than opaque `database_error`. Valid
service-controlled transitions and idempotent same-state metadata updates retain
their existing behavior.

## Boundary rule

Production writers must call the owning service. Repository guards are
defense-in-depth against future caller mistakes and do not transfer authority to
the database layer. Cross-domain repository access remains read-only.

## Related

- [Lifecycle Governance](./LIFECYCLE-GOVERNANCE.md)
- [Governed Execution Audit Durability](./GOVERNED-AUDIT-DURABILITY.md)
- [Kernel Error Taxonomy](./KERNEL-ERROR-TAXONOMY.md)
