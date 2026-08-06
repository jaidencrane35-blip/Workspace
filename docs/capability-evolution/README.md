# Capability Evolution Foundation

| Field | Value |
| --- | --- |
| **Status** | Foundation shipped (proposals only) |
| **Code** | `app/src/lib/capabilityEvolution.ts` |
| **Registry template** | `docs/capability-evolution/registry.json` |
| **Verifier** | `pnpm verify:capability-evolution` |

## Law

Workspace **must not rewrite itself**.

Conversation may:

1. Interpret a change request
2. Classify it (preference / workflow / bug / capability / enhancement)
3. Generate a proposal with impact + plan
4. Accept explicit approval / rejection
5. Track audit, version, undo, approved backlog

Implementation of approved proposals happens **only** through constitutional engineering execution programs.

## Conversation examples

| Utterance | Category (typical) |
| --- | --- |
| Add a screenshot button | capability |
| Move the chat | user_preference |
| Make the background transparent | user_preference |
| Resize the operator | user_preference |

Follow-ups: `approve proposal`, `reject proposal`, `undo proposal`, `list proposals`.

## Persistence

- Runtime: `localStorage` key `workspace.capabilityEvolution.v1`
- Repository template / engineering mirror: `registry.json` (schema + empty approved backlog seed)
