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
| Recommendation lifecycle | `WorkspaceRecommendationEngineService` | Stored-state transition matrix; controlled generation reopen requires changed fingerprint and retained prior outcome |
| Decision Engine lifecycle | `DecisionEngineService` | Terminal outcomes cannot reopen; invalidated intake cannot reactivate |
| Decision Queue overlay | `DecisionQueueService` | Overlay transition matrix; dismissed remains terminal |
| Task Graph | `TaskGraphService` | Terminal tasks cannot return to open status; terminal source synchronization remains allowed |
| Observation snapshots | `WorkspaceObservationService` | Snapshot validation and transactional persistence |
| Audit evidence | `AuditService` | Insert-only repository and database-enforced update/delete rejection |
| Suggestions | Suggestion services | No separate repository; projections consume immutable audit evidence |

Repository transition rejection uses `DatabaseError::InvalidTransition`. Valid
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
