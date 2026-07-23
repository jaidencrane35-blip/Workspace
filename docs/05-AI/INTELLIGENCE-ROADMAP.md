# Intelligence Layer Roadmap (Phase 3 Batches 3–7)

| Field | Value |
|-------|-------|
| **Purpose** | Sequence safe intelligence work after AI actor + planning foundations |
| **Permanent rule** | AI → Intent → Pipeline → Permission Gateway → Execution |

---

## Batch status

| Batch | Theme | Status |
|-------|--------|--------|
| 1 | AI Actor | Complete (S46–47) |
| 2 | AI Planning | Complete (S48–49) |
| 3 | AI Context & Workspace Understanding | Complete (S50–51) |
| 4 | Capability Discovery & Tool Awareness | Next |
| 5 | Planning Quality & Evaluation | Deferred |
| 6 | Governed Action Orchestration | Deferred |
| 7 | Governed Assistant Foundation | Deferred |

---

## Batch 3 (done)

Read-only `AiWorkspaceAwareness` from `WorkspaceContext` + desktop window titles; context-aware launch proposals; gateway unchanged.

## Batch 4 (next)

Informational action catalog from `ActionIntentRegistry` / `CapabilityResolver` — “what actions exist?” without granting authority. Feed planning metadata; do not treat `available: bool` as permission.

## Batch 5

Proposal evaluation / failure classification (operational audits only — no chain-of-thought).

## Batch 6

Multi-step plan lifecycle (`proposed` → `awaiting_approval` → …) with per-step gateway checks; no silent continue after denial.

## Batch 7

User-facing governed assistant (goal input, proposal explanation, human decide). Not autonomous OS control.

---

## Never build

- AI → Tool → Execution bypass
- AI-only pipelines / permission systems
- Lasting AI self-grants
- Hidden background automation
