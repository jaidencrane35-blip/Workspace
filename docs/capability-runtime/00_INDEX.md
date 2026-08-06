# Capability Runtime Pack

| Field | Value |
| --- | --- |
| **P9** | Research complete (accepted) |
| **P10** | Foundation implemented — awaiting Owner review |
| **Layer** | 3 — Capability Runtime (Track B) |
| **UI law** | Frozen (`docs/ui/UI_ARCHITECTURE_SPECIFICATION.md`) |

## P10 Foundation

| Document | Purpose |
| --- | --- |
| [CAPABILITY_RUNTIME_FOUNDATION.md](./CAPABILITY_RUNTIME_FOUNDATION.md) | Foundation overview |
| [INTENT_LAYER_SPECIFICATION.md](./INTENT_LAYER_SPECIFICATION.md) | Intent Layer laws |
| [CAPABILITY_ROUTER_SPECIFICATION.md](./CAPABILITY_ROUTER_SPECIFICATION.md) | Router + pipeline freeze |
| [PROVIDER_REGISTRY.md](./PROVIDER_REGISTRY.md) | Provider architecture |
| [CLIPBOARD_PROVIDER.md](./CLIPBOARD_PROVIDER.md) | Reference provider contract |
| [FIVE_PROGRAM_ROADMAP.md](./FIVE_PROGRAM_ROADMAP.md) | P10–P14 design (P11+ not executed) |

Verifier: `pnpm verify:capability-runtime-foundation`

## P9 Research (retained)

| Document | Purpose |
| --- | --- |
| [CAPABILITY_RUNTIME_RESEARCH_REPORT.md](./CAPABILITY_RUNTIME_RESEARCH_REPORT.md) | Executive research |
| [CAPABILITY_DOMAIN_CATALOGUE.md](./CAPABILITY_DOMAIN_CATALOGUE.md) | Permanent domains |
| [OPEN_SOURCE_ADOPTION_MATRIX.md](./OPEN_SOURCE_ADOPTION_MATRIX.md) | ADOPT / ADAPT / WRAP / STUDY / REJECT |
| [CAPABILITY_CONTRACTS.md](./CAPABILITY_CONTRACTS.md) | Conversation-entry contracts |
| [ADOPTION_RISK_ASSESSMENT.md](./ADOPTION_RISK_ASSESSMENT.md) | Legal, security, identity risks |
| [RECOMMENDED_IMPLEMENTATION_ORDER.md](./RECOMMENDED_IMPLEMENTATION_ORDER.md) | Phased Track B order |

Verifier: `pnpm verify:capability-runtime-research`

## Verdict

Workspace owns contracts, permissions, audit, and Win32 authority; wraps commodity libraries; never adopts an external agent suite as product identity. P10 proves the pipeline with Clipboard only.  
