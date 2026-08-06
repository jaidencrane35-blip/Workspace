# Operator Intelligence Foundation
## P12.7 — Architectural reasoning layer (not AGI)

| Field | Value |
| --- | --- |
| **Status** | Authoritative engineering foundation |
| **Not** | Planning AGI · autonomous agents · model runtime |
| **Is** | Governed decision layer: Conversation → desktop operation |
| **Code** | `app/src/lib/operator/` |
| **Rules** | Operator Authority · Capability Composition · Product Proof · Product Gravity |

---

## Responsibilities

| Concern | Owner |
| --- | --- |
| Conversational intent resolution | Intent Layer (input) + Operator (accept / clarify / refuse) |
| Ambiguity handling | Operator |
| Clarification policy | Operator |
| Provider selection | Operator |
| Permission policy (decide when to proceed) | Operator (Gateway still enforces) |
| Response composition | Operator |
| Multi-provider orchestration | Operator |
| Conversational context lifetime | Operator |
| Operator personality boundaries | Operator (honest, non-inventive, non-jargon) |
| Truthfulness policy | Operator |

---

## Contracts

See `OPERATOR_CONTRACTS.md`, `CONVERSATION_OPERATOR_PROTOCOL.md`, `OPERATOR_RUNTIME_PROTOCOL.md`.

## Policies

| Policy | Doc |
| --- | --- |
| Clarification | `policies/CLARIFICATION_POLICY.md` |
| Truthfulness | `policies/TRUTHFULNESS_POLICY.md` |
| Orchestration | `policies/ORCHESTRATION_POLICY.md` |
| Permission decision | `policies/PERMISSION_POLICY.md` |
| Response composition | `policies/RESPONSE_COMPOSITION_POLICY.md` |
| Context lifetime | `policies/CONTEXT_LIFETIME_POLICY.md` |

## State machine

`OPERATOR_STATE_MACHINE.md`

## Product Proof

`product-proof/OPERATOR_PRODUCT_PROOF.md`

---

## Decision (P12.7)

| Option | Verdict |
| --- | --- |
| Rust-only Operator inside kernel | REJECT for foundation — Conversation already owns intent language in TS; double hop adds no law value yet |
| TypeScript Operator as sole Conversation bridge | **ADOPT** — owns decide/orchestrate/compose; sole caller of capability IPC |
| Providers grow conversational APIs | REJECT — Operator Authority |

Kernel Capability Runtime remains the effect authority. Operator never bypasses Router/Registry.
