# Capability Runtime Pack

| Field | Value |
| --- | --- |
| **P10–P12** | Complete (P12 series permanently closed) |
| **P13** | Notifications Provider — **permanently closed** |
| **P12.5** | Window Product Proof — **ACCEPTED — DO NOT REOPEN** |
| **P14** | Browser Provider — **PERMANENTLY CLOSED** |
| **P15** | Screenshot Provider — **PERMANENTLY CLOSED** |
| **P16** | Voice Input — **PERMANENTLY CLOSED** |
| **P17** | File Provider — **next eligible** |
| **Acceptance** | [PROVIDER_ACCEPTANCE_STANDARD.md](./PROVIDER_ACCEPTANCE_STANDARD.md) |
| **Operator law** | Conversation → Intent → Kernel Operator → Runtime |
| **Composition law** | Kernel owns composition |
| **Independence law** | Capability Independence Rule |

## Documents

| Document | Purpose |
| --- | --- |
| [PROVIDER_ACCEPTANCE_STANDARD.md](./PROVIDER_ACCEPTANCE_STANDARD.md) | Permanent provider completion sequence + Independence Rule |
| [VOICE_INPUT.md](./VOICE_INPUT.md) | **P16 Voice Input** (closed) |
| [research/VOICE_RESEARCH.md](./research/VOICE_RESEARCH.md) | WRAP WinRT decision |
| [product-proof/VOICE_INPUT_PRODUCT_PROOF.md](./product-proof/VOICE_INPUT_PRODUCT_PROOF.md) | Owner checklist |
| [SCREENSHOT_PROVIDER.md](./SCREENSHOT_PROVIDER.md) | **P15 Screenshot** (closed) |
| [BROWSER_PROVIDER.md](./BROWSER_PROVIDER.md) | **P14 Browser** |
| [NOTIFICATIONS_PROVIDER.md](./NOTIFICATIONS_PROVIDER.md) | **P13 Notifications** (closed) |
| [../operator/00_INDEX.md](../operator/00_INDEX.md) | Kernel Operator |
| [CAPABILITY_RUNTIME_FOUNDATION.md](./CAPABILITY_RUNTIME_FOUNDATION.md) | Pipeline freeze |
| [PRODUCT_PROOF_RULE.md](./PRODUCT_PROOF_RULE.md) | Product Proof law |
| [FIVE_PROGRAM_ROADMAP.md](./FIVE_PROGRAM_ROADMAP.md) | Rolling roadmap |

Verifiers: `pnpm verify:voice-input` · `pnpm verify:screenshot-provider` · `pnpm verify:browser-provider` · `pnpm verify:notifications-provider` · `pnpm verify:operator-intelligence` · `pnpm verify:capability-runtime-foundation` · `pnpm verify:product-proof-harness`
