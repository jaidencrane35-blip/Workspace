# Workspace Intelligence Integrity Audit (Batch 5.5)

| Field | Value |
|-------|-------|
| **Date** | 2026-07-24 |
| **Batch** | Phase 4 Batch 5.5 |
| **Status** | Complete (hardening applied) |
| **Scope** | Verify / harden / simplify / protect — no new product capabilities |

---

## 1. Current architecture assessment

Batch 5 correctly introduced a **work-context middle layer** separate from ephemeral planning:

| Layer | Owns | Durable? | Authority? |
|-------|------|----------|------------|
| Intent (`Project` / `Task` / `WorkGoal` / `WorkflowContext`) | What work exists and what is active | Yes (SQLite) | No |
| Intelligence (`WorkspaceIntelligenceState`) | Aggregated understanding for UX | Rebuilt on demand | No |
| Planning / Orchestration / Assistant | Proposals and governed workflows | Plans/workflows in-memory | No (gateway decides) |
| Permission Gateway | Allow / Deny / ApprovalRequired | Grants + approvals durable | **Yes — sole boundary** |

Permanent flow remains:

```
Human Intent → Workspace Intelligence (understanding)
  → Planning → Evaluation → Permission Gateway → Execution
```

**Verdict before hardening:** Authority model was sound. Integrity gaps were mainly **cross-workspace aggregation leakage**, **create-on-read mutation**, **incomplete active-work validation**, and **UI/test coverage gaps**.

---

## 2. Problems discovered

| # | Problem | Severity |
|---|---------|----------|
| 1 | Orchestrated plans / blocked actions not filtered by workspace | HIGH |
| 2 | Assistant workflows with `workspace_id = None` leaked into every snapshot | HIGH |
| 3 | Pending approvals listed globally without workspace scoping | HIGH |
| 4 | `set_active_work` did not verify project/task belong to target workspace | HIGH |
| 5 | `generate` called `get_or_create_workflow_context` (write on read) | MEDIUM |
| 6 | Intelligence refresh emitted up to 4 audit events (noise) | MEDIUM |
| 7 | `appears_active` hard-coded false | MEDIUM |
| 8 | Work tab self-compare + session-only project list; Assistant did not read intelligence | MEDIUM |
| 9 | Orchestrated plans / assistant workflows still session-scoped (documented, not fixed as feature) | MEDIUM (known) |
| 10 | Soft-delete columns without delete APIs; goals IPC incomplete | LOW–MEDIUM (deferred — lifecycle completeness, not authority) |

---

## 3. Recommended fixes (applied in 5.5)

1. Scope plans/blocked actions to applications belonging to the workspace.
2. Include assistant workflows only when `workspace_id` matches exactly.
3. Scope pending approvals by workspace application ids / subject hints; label remainder as out-of-scope (omit).
4. Enforce workspace membership in `set_active_work`.
5. Read-only workflow context on generate (empty in-memory if missing — no insert).
6. Collapse intelligence audits to one `workspace.intelligence.generated` event (`authority_effect: none`).
7. Derive `appears_active` from desktop window titles when available.
8. Work tab: load durable projects; remove useless self-compare; Assistant shows shared intelligence snapshot.
9. Harden CASE 1–10 integrity tests (isolation, shared path, audit authority).

**Not done (correctly deferred):** durable orchestrated plans, soft-delete APIs, full goal IPC, automation contracts.

---

## 4. Risk rating

| Area | Rating after hardening |
|------|------------------------|
| Authority leak | **LOW** |
| Work-context ownership | **LOW** |
| Cross-workspace isolation | **LOW** (hardened) |
| Persistence of intent | **LOW** |
| Session-only plans in intelligence | **MEDIUM** (documented; acceptable until Batch 6) |
| Ready for Batch 6 automation contracts? | **YES — with caveat** that automation contracts must not treat in-memory plan stores as durable work context |

**Overall integrity risk: LOW–MEDIUM → LOW after 5.5 fixes.**

---

## 5. Batch 6 readiness

Workspace Intelligence is ready as a **read-only understanding layer** for governed automation contracts **if** Batch 6:

- Stores automation contracts as durable, inspectable intent-adjacent objects
- Re-enters CommandPipeline → Permission Gateway on every trigger
- Does not grant authority from intelligence recommendations
- Scopes contracts by workspace / project / task explicitly

Do not proceed until isolation and read-only guarantees remain green in CASE tests.
