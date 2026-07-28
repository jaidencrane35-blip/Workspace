# Programme III — Coherent Workspace Runtime

| Field | Value |
|-------|-------|
| **Purpose** | Establish one provenance-rich, read-only runtime view of operational, cognitive, governance, and evidence state while preserving all existing lifecycle and persistence authorities |
| **Status** | Active — Batch 1 in progress |
| **Depends on** | Programme I governance / recovery; Programme II cognitive stack |
| **Non-goals** | Event sourcing; simulation; model hosting; autonomous execution; policy redesign; replacement for domain-owned projections |

## Programme principles (non-negotiable)

1. **Sources remain authoritative** — unified state owns composition only.
2. **No duplicated lifecycle payloads** — carry typed slices or canonical references.
3. **Every slice declares** source identity, source revision, observed/generated time, freshness, and completeness.
4. **Unknown, stale, missing, truncated, and contradictory are distinct states.**
5. **Composition is deterministic** for the same declared source revisions.
6. **Evidence and actionable state remain separate channels.**
7. **Unified state cannot** execute, approve, grant, repair, or silently refresh foreign sources.
8. **Recovery loads source-owned state** — it never fabricates a complete envelope.
9. **Consumer-specific views derive from the canonical envelope** — consumers do not rebuild it.
10. **Privacy and retention classification travel with each slice** (later batches).

## Batch map

| Batch | Theme | Role | Status |
|-------|-------|------|--------|
| **1** | Unified Workspace State Model | Canonical composition envelope | Active |
| **2** | Policy & Governance Engine | Context-aware constraints / capability semantics | Planned (gated on Batch 1 audit) |
| **3+** | Collaboration / History / Simulation / Local Intelligence | Build on coherent state + policy | Planned |

## Batch 1 summary

`WorkspaceStateCompositionService` produces a dual-channel `WorkspaceStateSnapshot`
whose current channel is a `WorkspaceStateEnvelope`. Freshness, completeness,
availability, and contradictions are explicit. Conflicts are evidence — never
auto-repair. See [Unified Workspace State Architecture](./UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md).

## Sequencing rule

Ship Batch *N* only when Batch *N−1* has durable contracts, tests, and governance ownership entries.
**Do not begin Batch 2 (Policy & Governance Engine) until Batch 1 is audited.**

## Related

- [Unified Workspace State Architecture](./UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md) (Batch 1)
- [Programme II — Cognitive Workspace](./PROGRAMME-II-COGNITIVE-WORKSPACE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
