# Programme III — Coherent Workspace Runtime

| Field | Value |
|-------|-------|
| **Purpose** | Establish one provenance-rich, read-only runtime view of operational, cognitive, governance, and evidence state while preserving all existing lifecycle and persistence authorities |
| **Status** | Active — Batch 3 Historical Reconstruction implemented |
| **Depends on** | Programme I governance / recovery; Programme II cognitive stack |
| **Non-goals** | Event sourcing; authoritative replay logs; simulation; model hosting; autonomous execution; replacement permission system; replacement for domain-owned projections |

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
10. **Policy explains authority; PermissionGateway remains the final authoriser.**
11. **Unknown context fails closed** — never assume Compliant / safe.
12. **Reconstruction explains change over time** — it does not become the source of truth; no event sourcing; no replay authority.

## Batch map

| Batch | Theme | Role | Status |
|-------|-------|------|--------|
| **1** | Unified Workspace State Model | Canonical composition envelope | Done (accepted) |
| **2** | Policy & Governance Engine | Context-aware policy reasoning / evidence | Done (accepted) |
| **3** | Historical Workspace Reconstruction | Temporal comparison / change explanation | Done (implemented) |
| **4+** | Collaboration / Simulation / Local Intelligence | Build on coherent state + policy + history | Planned |

## Batch 2 summary

`PolicyGovernanceService` evaluates the Workspace State Envelope against versioned
`PolicyDefinition` rules and produces `PolicyEvaluation` evidence plus
`GovernanceRecommendation` / `GovernanceExplanation` surfaces.
Dual-channel `PolicyGovernanceSnapshot`. History is append-only terminal evidence.
Policy never grants permissions, executes commands, or bypasses Gateway.
See [Policy & Governance Architecture](./POLICY-GOVERNANCE-ARCHITECTURE.md).

## Batch 3 summary

`WorkspaceHistoricalReconstructionService` composes temporal views, revision
comparisons, evidence timelines, and change explanations from durable workspace
state envelopes (via `list_durable_envelopes` — never `generate`).
Reconstruction completeness remains explicit (`Complete` / `Partial` / `Unknown` /
`Contradictory` / `Unavailable`). Missing evidence never becomes invented history.
Dual-channel `HistoricalReconstructionSnapshot`. Commands:
`GenerateHistoricalWorkspaceView`, `GetHistoricalWorkspaceView`,
`GetHistoricalWorkspaceSummary`, `CompareWorkspaceRevisions`, `ExplainHistoricalChange`.
See [Historical Workspace Reconstruction Architecture](./HISTORICAL-WORKSPACE-RECONSTRUCTION-ARCHITECTURE.md).

## Sequencing rule

Ship Batch *N* only when Batch *N−1* has durable contracts, tests, and governance ownership entries.
Batch 4+ remains gated on Batch 3 audit acceptance.

## Related

- [Unified Workspace State Architecture](./UNIFIED-WORKSPACE-STATE-ARCHITECTURE.md) (Batch 1)
- [Policy & Governance Architecture](./POLICY-GOVERNANCE-ARCHITECTURE.md) (Batch 2)
- [Historical Workspace Reconstruction Architecture](./HISTORICAL-WORKSPACE-RECONSTRUCTION-ARCHITECTURE.md) (Batch 3)
- [Programme II — Cognitive Workspace](./PROGRAMME-II-COGNITIVE-WORKSPACE.md)
- [Projection Integrity](../03-Engineering/PROJECTION-INTEGRITY.md)
- [Architecture Governance](../03-Engineering/ARCHITECTURE-GOVERNANCE.md)
- [Operational Recovery](../03-Engineering/OPERATIONAL-RECOVERY.md)
