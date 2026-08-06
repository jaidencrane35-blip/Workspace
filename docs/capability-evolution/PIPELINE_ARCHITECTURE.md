# Capability Evolution Pipeline — Architecture (Design)

| Field | Value |
| --- | --- |
| **Status** | Architecture only — **not** full implementation |
| **Law** | Workspace must not rewrite itself |
| **Runtime seed** | `app/src/lib/capabilityEvolution.ts` (propose / approve / audit) |
| **Constitution** | Architectural Constitution V2 + Product Constitution P2/P6 |

---

## Intent

When a user asks for a capability that does not exist (“Take a screenshot”, “Arrange these windows”), Workspace:

1. Tells the truth (does not fabricate)
2. Records a governed proposal
3. Eventually supports a reviewable engineering change path

This is a **constitutional engineering workflow initiated through conversation** — never silent self-modification.

---

## Pipeline stages

```
Utterance
  → Classify (preference | workflow | bug | capability | enhancement)
  → Draft proposal (summary, impact, plan)
  → Estimate impact (product / permission / IPC / UX)
  → Generate reviewable change package (diff / ADR / verifier stubs)  [future]
  → Validate (typecheck, tests, constitutional checks)               [future]
  → Explicit human approval (conversation + optional owner gate)
  → Schedule as ONE execution program
  → Implement via Constitutional Execution Protocol
  → Ship + audit
  → Rollback / undo path retained
```

| Stage | Owner today | Owner later |
| --- | --- | --- |
| Classify + draft + approve/reject | Runtime operator (shipped foundation) | — |
| Impact estimate depth | Stub text in proposal | Structured scoring |
| Reviewable change package | Human agent | Assisted generator under protocol |
| Validate | `pnpm test` / cargo / verifiers | Same, required gate |
| Execute | Human constitutional program | Same — never autonomous apply |

---

## Artifacts

| Artifact | Role |
| --- | --- |
| Proposal record | id, category, status, version, audit trail |
| Approved backlog | IDs awaiting execution programs |
| Registry | `docs/capability-evolution/registry.json` |
| Execution program | Exactly one approved change at a time |
| Rollback note | Prior version / reject / roll_back status |

---

## Explicit non-goals

- No self-editing of source from the running app
- No privileged desktop action from a proposal alone
- No skipping owner review when handoff requires it
- No inventing capabilities in conversation to “look smart”

---

## Alignment

| Principle | How enforced |
| --- | --- |
| Trust before intelligence | Proposals cannot mutate desktop |
| One pipeline | Future generators feed CommandPipeline / IPC tiers — no shadow path |
| Verification | Every executed program leaves verifier/generator when possible |
| Conversation first | Entry is utterance; UI catalogues are forbidden |
