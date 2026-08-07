# Capability Runtime Pack

| Field | Value |
| --- | --- |
| **P10–P12** | Complete (P12 series permanently closed) |
| **P13** | Notifications Provider — **permanently closed** |
| **P12.5** | Window Product Proof — **ACCEPTED — DO NOT REOPEN** |
| **P14** | Browser Provider — **PERMANENTLY CLOSED** |
| **P15** | Screenshot Provider — **PERMANENTLY CLOSED** |
| **P16** | Voice Input — **next eligible** |
| **Acceptance** | [PROVIDER_ACCEPTANCE_STANDARD.md](./PROVIDER_ACCEPTANCE_STANDARD.md) |
| **Operator law** | Conversation → Intent → Kernel Operator → Runtime |
| **Composition law** | Kernel owns composition |
| **Independence law** | Capability Independence Rule |

## Documents

| Document | Purpose |
| --- | --- |
| [PROVIDER_ACCEPTANCE_STANDARD.md](./PROVIDER_ACCEPTANCE_STANDARD.md) | Permanent provider completion sequence + Independence Rule |
| [SCREENSHOT_PROVIDER.md](./SCREENSHOT_PROVIDER.md) | **P15 Screenshot** (closed) |
| [research/SCREENSHOT_RESEARCH.md](./research/SCREENSHOT_RESEARCH.md) | WRAP `xcap` decision |
| [product-proof/SCREENSHOT_PROVIDER_PRODUCT_PROOF.md](./product-proof/SCREENSHOT_PROVIDER_PRODUCT_PROOF.md) | Owner checklist |
| [product-proof/SCREENSHOT_PROVIDER_COMPOSITION_AUDIT.md](./product-proof/SCREENSHOT_PROVIDER_COMPOSITION_AUDIT.md) | Composition audit |
| [BROWSER_PROVIDER.md](./BROWSER_PROVIDER.md) | **P14 Browser** |
| [research/BROWSER_RESEARCH.md](./research/BROWSER_RESEARCH.md) | WRAP decision |
| [product-proof/BROWSER_PROVIDER_PRODUCT_PROOF.md](./product-proof/BROWSER_PROVIDER_PRODUCT_PROOF.md) | Owner checklist |
| [NOTIFICATIONS_PROVIDER.md](./NOTIFICATIONS_PROVIDER.md) | **P13 Notifications** (closed) |
| [../operator/00_INDEX.md](../operator/00_INDEX.md) | Kernel Operator |
| [CAPABILITY_RUNTIME_FOUNDATION.md](./CAPABILITY_RUNTIME_FOUNDATION.md) | Pipeline freeze |
| [PRODUCT_PROOF_RULE.md](./PRODUCT_PROOF_RULE.md) | Product Proof law |
| [FIVE_PROGRAM_ROADMAP.md](./FIVE_PROGRAM_ROADMAP.md) | Rolling roadmap |

Verifiers: `pnpm verify:screenshot-provider` · `pnpm verify:browser-provider` · `pnpm verify:notifications-provider` · `pnpm verify:operator-intelligence` · `pnpm verify:capability-runtime-foundation` · `pnpm verify:product-proof-harness`
