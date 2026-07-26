# Workspace Adaptation Proposal

| Field | Value |
|-------|-------|
| **Purpose** | Read-only proposals for possible Workspace improvements |
| **Owner** | Architecture |
| **Status** | Phase 5 Batch 12 foundation (Sprint 91) |
| **Authority** | Never creates authority (`authority_effect: none`) |

---

## Principle

Adaptation is a proposal.

It is not execution, automation, permission, or autonomous improvement.

```
Pattern + Recommendation Engine + Operating State
  + Composition + Environment + Continuity + Purpose
        ↓
Adaptation Proposal   ← this document
        ↓
Human Decision (review / accept / reject)
        ↓
Intent → Planning → Command Pipeline → Permission Gateway → Execution → Audit
```

Accept returns an Intent handoff only. Adaptation never skips governance.

**Sprint 139:** `RecommendationOutcome` may inform future Adaptation as evidence only —
never auto-applies or rescoring. See
[WORKSPACE-RECOMMENDATION-OUTCOME.md](./WORKSPACE-RECOMMENDATION-OUTCOME.md).

**Sprint 140:** Adaptation governance —
[WORKSPACE-ADAPTATION-GOVERNANCE.md](./WORKSPACE-ADAPTATION-GOVERNANCE.md).
`OutcomeAdaptationProposal` requires explicit review; never silent scoring mutation.

**Sprint 141:** Adaptation review & change authority —
[WORKSPACE-ADAPTATION-REVIEW.md](./WORKSPACE-ADAPTATION-REVIEW.md).
LocalUser reviewer required; self-approval forbidden; approve ≠ apply ≠ Gateway grant.

**Sprint 142:** Controlled change & evaluation —
[WORKSPACE-CONTROLLED-CHANGE.md](./WORKSPACE-CONTROLLED-CHANGE.md).
BehaviourVersion drafts + ChangeEvaluation; no runtime cognition mutation.

---

## Ownership

| Concept | Kind | Owner |
|---------|------|-------|
| Adaptation proposal | Aggregator | `WorkspaceAdaptationService` |

Distinct from Recommendation Engine (next-step suggestions) and Decision Engine (planner candidates).

---

## Lifecycle

`proposed` → `reviewed` → `accepted` | `rejected`

- **Review** required before accept
- **Accept** → `submit_assistant_goal` handoff (never executes)
- **Reject** → audit only; workspace unchanged

Audits: `workspace.adaptation.proposal_generated`, `.reviewed`, `.accepted`, `.rejected` — all `authority_effect: none`.

IPC: `generate_workspace_adaptation`, `review_adaptation_proposal`, `accept_adaptation_proposal`, `reject_adaptation_proposal`
