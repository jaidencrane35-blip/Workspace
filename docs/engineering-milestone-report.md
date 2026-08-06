# Engineering Milestone Report
## P12.5 Conversation Integration & Product Proof

| Field | Value |
| --- | --- |
| **Execution program** | P12.5 Conversation Integration & Product Proof |
| **Date** | 2026-08-07 |
| **Prior** | P12 Window Provider engineering (`66465b8`) — not reimplemented |
| **Commit** | `ca63fc8` |
| **Handoff** | `AWAITING_PROJECT_OWNER_WINDOW_PRODUCT_PROOF_REVIEW` |
| **Index** | `docs/capability-runtime/product-proof/WINDOW_PROVIDER_PRODUCT_PROOF.md` |

---

## Mission

Close the Product Proof gap identified by the Product Owner (“Nothing has changed”).  
Connect the completed Window Provider to Conversation so the capability is naturally usable, with truthful feedback and no provider jargon.

---

## Product Proof Rule (permanent)

Documented in `docs/capability-runtime/PRODUCT_PROOF_RULE.md` and protocol v1.2.  
Providers require Engineering Completion **and** Product Proof before they are Product Complete.

---

## Deliverables

| Artifact | Path |
| --- | --- |
| Conversation window intents | `app/src/lib/intentBridge.ts` (`resolveWindowIntent`) |
| Truthful Conversation execution | `app/src/components/operator/OperatorRoot.tsx` |
| Active-window deixis (“this”) | `packages/kernel/src/capability_runtime/window_provider.rs` |
| Product Proof Rule | `docs/capability-runtime/PRODUCT_PROOF_RULE.md` |
| Window Product Proof harness | `docs/capability-runtime/product-proof/*` |
| Verifier | `pnpm verify:product-proof-harness` |
| Protocol | `.cursor/rules/constitutional-execution-protocol.mdc` v1.2 |

---

## User-visible

Conversation understands natural window requests (list / active / snap / center / maximize / restore / focus / monitor move / resize) and reports real outcomes.

---

## Explicit non-goals

P13 Notifications · UI redesign · Conversation chrome redesign · Reimplementing P12 Window Provider

---

## Stop

Wait for Product Owner Product Proof review before P13.  
Acceptance requires the Owner can use Window Provider through Conversation.
