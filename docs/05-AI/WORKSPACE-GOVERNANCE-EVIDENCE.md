# Workspace Governance Decision Evidence & Publication Readiness

Sprint 146 — evidence required for governance decisions and readiness before future publication.

**Evidence does not grant execution. Readiness does not activate runtime. Dissent never erases history.**

## Principle

```
Risk Classification
    ↓
GovernanceDecisionEvidence
    ↓
GovernanceReviewDecision
    ↓
GovernanceRecord
    ↓
PublishRequest / PublicationReadiness
```

---

## GovernanceDecisionEvidence

| Field | Meaning |
|-------|---------|
| `supporting_evidence_references` | Outcome ids, experience keys, provenance refs, audit refs |
| `reviewer_concerns` | Open concerns collected during review |
| `required_conditions` | Conditions that must accompany approval |
| `dissent_records` | Append-only dissent (never erases approvals) |
| `final_rationale` | Closing rationale for the decision package |

Rules:

- Approval paths require non-empty supporting refs + final rationale
- `may_grant_execution_authority() == false`
- `record_dissent` only appends
- `authority_effect: none`

---

## PublicationReadiness lifecycle

Architecture only:

```
Draft
    ↓
RiskReviewed
    ↓
Approved
    ↓
ReadyForPublication
    ↓
Published (future — activation hard-fails)
```

`PublicationReadiness::attempt_activate_published` → `PublicationReadinessCannotActivate`.

`history` is append-only; dissent after approval does not remove `Approved` from history.

---

## Audit of existing evidence systems

| System | Pattern | Reuse |
|--------|---------|-------|
| **AuditService events** | Append-only actor/timestamp/event_type/metadata | Supporting evidence references |
| **Experience traces** | `match_key` / DisplayReason translation diagnostics | Attach as evidence refs — not authority |
| **Recommendation provenance** | Frozen reasoning origins + evidence | Core snapshot on evidence package |
| **RecommendationOutcome** | Result kind + quality + experience keys | Primary supporting evidence |
| **Decision records / AI planning events** | Id-only lifecycle audits | Reference ids without chain-of-thought |

**Finding:** Append-only reference ids + frozen provenance are the reusable shape. Permission /
command audits remain the execution trail and must not be minted by evidence assembly.

---

## Ledger connection (Sprint 146 fields)

| Field | Meaning |
|-------|---------|
| `evidence_reference` | `GovernanceDecisionEvidence` id |
| `dissent_references` | Dissent ids (append-only) |
| `publication_readiness` | Current readiness state |

Approving `GovernanceReviewDecision`s should carry `evidence_reference` via `with_evidence`.

---

## Related docs

- [WORKSPACE-GOVERNANCE-RISK.md](./WORKSPACE-GOVERNANCE-RISK.md)
- [WORKSPACE-GOVERNANCE-POLICY.md](./WORKSPACE-GOVERNANCE-POLICY.md)
- [WORKSPACE-GOVERNANCE-LEDGER.md](./WORKSPACE-GOVERNANCE-LEDGER.md)
- [WORKSPACE-RECOMMENDATION-PROVENANCE.md](./WORKSPACE-RECOMMENDATION-PROVENANCE.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
