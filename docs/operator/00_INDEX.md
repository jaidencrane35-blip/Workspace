# Operator Intelligence Pack

| Field | Value |
| --- | --- |
| **Authority** | Kernel Operator (P12 Finalization) |
| **IPC** | `execute_capability_intent` |
| **P12 series** | Permanently complete — P13 next |

## Permanent rules

| Rule | Doc |
| --- | --- |
| Operator Authority | [OPERATOR_AUTHORITY_RULE.md](./OPERATOR_AUTHORITY_RULE.md) |
| Kernel Authority | [KERNEL_AUTHORITY_RULE.md](./KERNEL_AUTHORITY_RULE.md) |
| Presentation Purity | [PRESENTATION_PURITY_RULE.md](./PRESENTATION_PURITY_RULE.md) |
| Capability Composition | [CAPABILITY_COMPOSITION_RULE.md](./CAPABILITY_COMPOSITION_RULE.md) |

## Architecture

| Document | Purpose |
| --- | --- |
| [OPERATOR_INTELLIGENCE_FOUNDATION.md](./OPERATOR_INTELLIGENCE_FOUNDATION.md) | Foundation |
| [OPERATOR_CONTRACTS.md](./OPERATOR_CONTRACTS.md) | Contracts |
| [CONVERSATION_OPERATOR_PROTOCOL.md](./CONVERSATION_OPERATOR_PROTOCOL.md) | Conversation → façade |
| [OPERATOR_RUNTIME_PROTOCOL.md](./OPERATOR_RUNTIME_PROTOCOL.md) | Single IPC → Kernel |
| [OPERATOR_STATE_MACHINE.md](./OPERATOR_STATE_MACHINE.md) | Phases |
| [COMPOSITION_CATALOGUE.md](./COMPOSITION_CATALOGUE.md) | Recipes |
| [policies/](./policies/) | Clarification, truthfulness, … |
| [product-proof/OPERATOR_PRODUCT_PROOF.md](./product-proof/OPERATOR_PRODUCT_PROOF.md) | Owner checklist |

Verifier: `pnpm verify:operator-intelligence`  
Code: `packages/kernel/src/operator/` · `app/src/lib/operator/` (thin façade)  
