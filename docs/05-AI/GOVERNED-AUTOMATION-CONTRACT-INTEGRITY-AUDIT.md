# Governed Automation Contract Integrity Audit (Batch 6.5)

| Field | Value |
|-------|-------|
| **Date** | 2026-07-24 |
| **Batch** | Phase 4 Batch 6.5 |
| **Status** | Complete (hardening applied) |
| **Scope** | Verify / harden / protect — no trigger runtime |

---

## 1. Current architecture assessment

Batch 6 correctly introduced **durable future-intent definitions**:

| Layer | Role | Authority? |
|-------|------|------------|
| `AutomationContract` | Stored definition + lifecycle | No |
| `approval_state` | Consent for *this* definition | No (≠ grant) |
| `PrepareAutomationContractIntent` | Intent materialization boundary | No |
| Command Pipeline → Permission Gateway | Sole execution path | **Yes** |

Permanent path remains:

```
Automation Contract → Intent Creation → Command Pipeline → Permission Gateway → Execution → Audit
```

**Verdict before hardening:** Authority model was sound. Integrity gaps were mainly **stale approval on pending edits**, **missing approval attribution**, **incomplete prepare metadata**, and **UI trust language**.

---

## 2. Problems discovered

| # | Problem | Severity |
|---|---------|----------|
| 1 | Material edits while `approval_state=pending` did not invalidate pending consent | HIGH |
| 2 | No definition fingerprint binding approval to exact intent/trigger/caps/scope | HIGH |
| 3 | UI labeled `created_by_actor` as approver | HIGH (trust) |
| 4 | `AutomationContractIntentRequest` lacked actor / fingerprint / audit metadata | MEDIUM |
| 5 | `approve_definition` allowed `Completed` contracts to be re-approved | MEDIUM |
| 6 | Name-only updates force-cleared approval even when definition unchanged | LOW–MEDIUM |
| 7 | Prepare path emitted no audit event | LOW |
| 8 | No resume-from-paused path (paused stuck without re-approve) | LOW (lifecycle) |

---

## 3. Security / governance risks

- **Stale consent:** Pending or approved definitions could be edited without clear re-consent → fixed via material-change invalidation + fingerprint.
- **Authority confusion:** UI/metadata could imply creator = approver or approval = execution → fixed copy + `execution_authorized: false` / `authority_effect: none`.
- **Trigger readiness risk:** Without fingerprint checks, a future trigger engine could materialize intents for a definition the user never approved.

No evidence of Command Pipeline / Permission Gateway bypass in Batch 6.

---

## 4. Recommended fixes (applied)

1. Compute `definition_fingerprint` over intent, trigger, capabilities, scope, project, task.
2. On approve, persist `approved_by_actor`, `approved_at`, `approved_definition_fingerprint`.
3. Invalidate approval on **material** definition changes (intent/trigger/caps/scope/project/task) for both `pending` and `approved` states; cosmetic name/description edits do not.
4. `prepare` requires active status **and** fingerprint match; enrich request with actor/context/audit metadata; audit `automation.contract.intent.prepared` (`authority_effect: none`).
5. Block approve/request from `completed`/`revoked`; add `resume` for paused + still-valid fingerprint.
6. Fix Work tab trust copy (definition approval ≠ execution; show real approver).
7. Harden CASE 1–10 integrity tests.

**Deferred (correctly):** schedulers, workers, trigger evaluation, pattern detection.

---

## 5. Readiness for trigger engine

| Area | Rating |
|------|--------|
| Authority leak | **LOW** |
| Stale approval | **LOW** (hardened) |
| Revocation / pause | **LOW** |
| Persistence / isolation | **LOW** |
| Trigger runtime | **NOT BUILT** (intentional) |

**Trigger engine readiness: READY WITH CONDITIONS**

Conditions:
1. Every trigger evaluation must call `PrepareAutomationContractIntent` (or equivalent fingerprint-checked boundary) before any command.
2. Triggers must re-enter Command Pipeline → Permission Gateway per action.
3. Fingerprint mismatch / revoked / paused must hard-fail before intent creation.
4. Do not treat contract approval as a capability grant.

**Overall integrity risk after 6.5: LOW.**
