# Capability Runtime Pack

| Field | Value |
| --- | --- |
| **P10–P12** | Complete (P12 series permanently closed) |
| **P13** | Notifications Provider — **permanently closed** |
| **P14** | Browser Provider — engineering complete; **Owner Product Proof review** |
| **P15** | Screenshot Provider — **preparation docs only** (no implementation) |
| **Next** | P15 implementation after P14 Owner acceptance |
| **Operator law** | Conversation → Intent → Kernel Operator → Runtime |
| **Composition law** | Kernel owns composition |

## Documents

| Document | Purpose |
| --- | --- |
| [SCREENSHOT_PROVIDER_PREPARATION.md](./SCREENSHOT_PROVIDER_PREPARATION.md) | **P15 prep pack** (no code) |
| [research/SCREENSHOT_RESEARCH.md](./research/SCREENSHOT_RESEARCH.md) | Capture API / WRAP decision |
| [research/SCREENSHOT_ARCHITECTURE_PROPOSAL.md](./research/SCREENSHOT_ARCHITECTURE_PROPOSAL.md) | Architecture proposal |
| [product-proof/SCREENSHOT_PROVIDER_PRODUCT_PROOF_DRAFT.md](./product-proof/SCREENSHOT_PROVIDER_PRODUCT_PROOF_DRAFT.md) | Product Proof draft |
| [BROWSER_PROVIDER.md](./BROWSER_PROVIDER.md) | **P14 Browser** |
| [research/BROWSER_RESEARCH.md](./research/BROWSER_RESEARCH.md) | WRAP decision |
| [product-proof/BROWSER_PROVIDER_PRODUCT_PROOF.md](./product-proof/BROWSER_PROVIDER_PRODUCT_PROOF.md) | Owner checklist |
| [product-proof/BROWSER_PROVIDER_COMPOSITION_AUDIT.md](./product-proof/BROWSER_PROVIDER_COMPOSITION_AUDIT.md) | Composition audit |
| [NOTIFICATIONS_PROVIDER.md](./NOTIFICATIONS_PROVIDER.md) | **P13 Notifications** (closed) |
| [research/NOTIFICATIONS_RESEARCH.md](./research/NOTIFICATIONS_RESEARCH.md) | WRAP decision |
| [product-proof/NOTIFICATIONS_PROVIDER_PRODUCT_PROOF.md](./product-proof/NOTIFICATIONS_PROVIDER_PRODUCT_PROOF.md) | Owner checklist |
| [../operator/00_INDEX.md](../operator/00_INDEX.md) | Kernel Operator |
| [CAPABILITY_RUNTIME_FOUNDATION.md](./CAPABILITY_RUNTIME_FOUNDATION.md) | Pipeline freeze |
| [PRODUCT_PROOF_RULE.md](./PRODUCT_PROOF_RULE.md) | Product Proof law |
| [FIVE_PROGRAM_ROADMAP.md](./FIVE_PROGRAM_ROADMAP.md) | Rolling roadmap |

Verifiers: `pnpm verify:browser-provider` · `pnpm verify:notifications-provider` · `pnpm verify:operator-intelligence` · `pnpm verify:capability-runtime-foundation` · `pnpm verify:product-proof-harness`
