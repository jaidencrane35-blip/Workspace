# AI Model Provider Foundation

| Field | Value |
|-------|-------|
| **Purpose** | Abstract model intelligence from system authority (Sprints 62–63 / P4-B2) |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [AI Planning Foundation](AI-PLANNING-FOUNDATION.md), [Permission Architecture](../07-Security/PERMISSION-ARCHITECTURE.md) |
| **Update Process** | Update when provider interface / routing changes |

---

## Permanent rule

```
Models generate intelligence.
The system controls authority.

AI proposes → System evaluates → Permissions decide → Execution follows authorization
```

---

## Separation

```
AI Capability
      │
      v
Model Provider Interface
      │
      v
Specific Model (deterministic / future remote / local)
      │
      v
Structured proposal candidates
      │
      v
AiActionProposal → Evaluation → Permission Gateway → Execution
```

Provider output answers: **What could be done?**  
System answers: **Is it allowed?**

Never combine these.

---

## What providers may do

- Generate text
- Generate structured proposal candidates
- Interpret planning context
- Assist planning explanations

## What providers may not do

- Execute commands
- Access OS controls
- Grant permissions
- Bypass Permission Gateway
- Call privileged services
- Create authority decisions

---

## Core types

| Type | Role |
|------|------|
| `ModelProviderDescriptor` | provider/model identity, capabilities, availability, version |
| `ModelRequest` | task + context summary + expected response format |
| `ModelResponse` | text/candidates + provider metadata + status |
| `ModelProposalCandidate` | suggestion-only structured action candidate |
| `ModelInvocationSummary` | compact plan-attached invocation metadata |

Provider capabilities (`text_generation`, `structured_proposals`, …) are **intelligence features**, not system permissions.

---

## Registry and routing

`ModelProviderRegistry` lists providers and selects by:

- availability
- required intelligence capability
- configuration preference

Routing must **not** consider permission authority, approval bypass, or execution rights.

Explicit provider selection does not silently fall back (no autonomous switching). Default routing may skip an unavailable configured preference.

Shipped stubs:

| Provider | Role |
|----------|------|
| `deterministic` / `rules-v1` | Default planning behavior |
| `echo` / `echo-v1` | Secondary selectable stub |
| `unavailable` | Failure-path diagnostics |

---

## Failure handling

Failures become AI/system validation errors (`AiModelValidation`):

- provider unavailable
- timeout (reserved)
- invalid / malformed structured output
- unsupported capability
- no selectable provider

Unsafe fallbacks (auto-execute, auto-approve, privilege escalation) are forbidden.

---

## Audits (operational only)

| Event | Meaning |
|-------|---------|
| `ai.model.provider_selected` | Routing chose a provider |
| `ai.model.requested` | Request issued |
| `ai.model.response_received` | Response accepted |
| `ai.model.failed` | Provider/request failure |

Metadata includes provider/model identity and `"authority_effect": "none"`.

Never store chain-of-thought or hidden reasoning traces.

---

## Diagnostics (Operator Console)

- List available model providers
- Inspect model metadata
- Test provider request
- Run proposal generation test (`diagnose_model_proposal_generation`)

Proves: Provider → AI output → Proposal system → (later) Permission Gateway. No shortcut path.

---

## Related

- [Intelligence Roadmap](INTELLIGENCE-ROADMAP.md)
- [IPC Surface](../03-Engineering/IPC-SURFACE.md)
