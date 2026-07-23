# AI Operating Model

| Field | Value |
|-------|-------|
| **Purpose** | Define how Workspace AI operates — responsibilities, limits, approval boundaries, and escalation rules |
| **Owner** | Project Owner |
| **Dependencies** | [AI Principles](AI-PRINCIPLES.md), [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md), [Memory Policy](MEMORY-POLICY.md), [Confidence Policy](CONFIDENCE-POLICY.md) |
| **Update Process** | Project Owner approves changes. Permission model changes require Decision Log entry. |

---

## 1. Scope

This document governs **Workspace product AI** — the in-application subsystem that observes, learns, suggests, and automates. It does not govern AI development tools (e.g., Cursor agents) used to build Workspace. See [Governance Model](../00-Constitution/GOVERNANCE.md) for AI contributor rules.

---

## 2. AI Responsibilities

Workspace AI is responsible for:

| Responsibility | Description |
|----------------|-------------|
| **Observation** | Passively monitor permitted system events via the Platform Kernel event bus |
| **Pattern detection** | Identify recurring sequences, contexts, and preferences in observed data |
| **Suggestion generation** | Propose automations or workflow improvements based on detected patterns |
| **Explanation** | Clearly state what pattern was detected and why a suggestion is offered |
| **Permission requests** | Present approval prompts before any state-changing action |
| **Approved automation execution** | Execute only automations the user has explicitly approved |
| **Transparency** | Maintain visible records of observations, learned patterns, and active automations |

---

## 3. AI Limitations

Workspace AI must **never**:

| Limitation | Detail |
|------------|--------|
| Act without permission | No state-changing operations without explicit user approval |
| Observe prohibited data | Keystrokes, credentials, screen content, communication content — see [Memory Policy](MEMORY-POLICY.md) |
| Escalate its own permissions | Cannot grant itself additional access at runtime |
| Override user actions | User input always takes precedence |
| Modify automations silently | Approved automations execute as defined — no scope expansion |
| Learn from other users | No cross-user pattern sharing without explicit opt-in (see OQ-018) |
| Transmit data externally | No learned data leaves the machine without explicit opt-in |
| Use manipulative UX | No dark patterns, pre-checked consent, or pressure tactics |

---

## 4. The Operating Sequence

All AI behaviour follows this sequence without exception:

```
Observe → Learn → Suggest → Receive Permission → Automate
```

| Stage | AI Role | User Role |
|-------|---------|-----------|
| **Observe** | Monitor permitted events | Can view, pause, or disable observation |
| **Learn** | Store patterns meeting retention rules | Can inspect, export, or delete learned data |
| **Suggest** | Present dismissible recommendation | Accept, modify, dismiss, or ignore |
| **Receive Permission** | Wait for explicit approval | Choose approval type (one-time, session, persistent) |
| **Automate** | Execute approved action only | Can pause, stop, or revoke at any time |

Skipping any stage is prohibited. See [AI Principles](AI-PRINCIPLES.md) §2.

---

## 5. When AI Can Recommend

AI **may** generate a suggestion when **all** conditions are met:

1. A pattern meets minimum confidence threshold — see [Confidence Policy](CONFIDENCE-POLICY.md)
2. Observation domain is enabled by the user
3. The suggestion falls within permitted AI capabilities
4. Cooldown period has elapsed since last suggestion for the same pattern
5. User is not in a do-not-disturb or focus state (if configured)
6. The recommendation is actionable and specific

**Examples of valid recommendations:**

- *"You often open Discord after Steam. Automate that?"*
- *"Spotify volume drops to 20% during Discord calls. Save as automation?"*
- *"You rearrange panels every morning. Save this as your default layout?"*

---

## 6. When AI Must Request Approval

AI **must** request explicit approval before:

| Action | Approval Required |
|--------|-------------------|
| Creating any automation | Always — per [Confidence Policy](CONFIDENCE-POLICY.md) |
| Executing any state-changing operation | Always |
| Expanding observation scope | User must enable the domain |
| Changing a persistent automation | Re-approval required |
| Any action affecting audio routing or levels | Always |
| Launching applications | Always |
| Modifying layouts without user initiation | Always |
| Any action crossing domain boundaries | Always |

Approval flows through the **Permission Gateway** (Platform Kernel). AI submits requests; the gateway enforces policy and presents UI. AI cannot bypass the gateway.

---

## 7. When AI Must Refuse or Escalate

AI **must refuse** to act when:

| Condition | Response |
|-----------|----------|
| Confidence below minimum threshold | Do not suggest; continue observing |
| Observation domain disabled | Do not use data from that domain |
| User dismissed this pattern (within cooldown) | Do not re-suggest |
| Action would observe prohibited data | Refuse and log |
| Action would bypass Permission Gateway | Refuse — architectural violation |
| Request conflicts with [Memory Policy](MEMORY-POLICY.md) | Refuse storage or observation |
| Automation scope exceeds approved definition | Refuse execution; notify user |

AI **must escalate to the user** (not act autonomously) when:

| Condition | Response |
|-----------|----------|
| Confidence is borderline (see Confidence Policy) | Ask clarifying question instead of suggesting |
| Multiple valid automations detected | Present options; let user choose |
| Pattern involves sensitive context (calls, meetings) | Ask before suggesting, even if confidence is high |
| Conflicting patterns detected | Explain conflict; ask user to clarify preference |
| Automation failed previously | Notify user; ask whether to retry or modify |
| Requested action has security implications | Explain risk; require explicit confirmation |

Escalation means presenting information and options to the user — never delegating to another automated system.

---

## 8. Human Approval Boundaries

### 8.1 What Requires Human Approval

Everything that changes system state outside the user's direct manual action:

- Automation creation and execution
- Persistent preference changes instigated by AI
- Observation scope expansion
- Data retention beyond default policy

### 8.2 What Does Not Require Approval

Passive operations that do not change state:

- Observing enabled event domains
- Learning patterns internally (within Memory Policy)
- Logging for audit (non-sensitive metadata)
- Failing safe when AI subsystem is unavailable

### 8.3 Approval Types

| Type | Scope | User Chooses |
|------|-------|--------------|
| One-time | Single execution | Always offered |
| Session | Until restart | Always offered |
| Persistent | Until revoked | Always offered |

AI never defaults to persistent approval.

---

## 9. Permission Gateway Integration

The Permission Gateway is owned by the **Platform Kernel**, not the AI Subsystem.

```
AI Subsystem → Permission Gateway (Platform Kernel) → User Prompt
                              ↓
                     User Approval / Denial
                              ↓
              Domain Service (if approved)
```

AI Subsystem submits permission requests. The gateway validates policy, presents UI, records audit entries, and routes approved actions to domain services. See [System Overview](../02-Architecture/SYSTEM-OVERVIEW.md).

---

## 10. Failure Modes

| Failure | Behaviour |
|---------|-----------|
| AI subsystem crash | Workspace continues; AI features unavailable |
| Permission Gateway unavailable | No automations execute; suggestions paused |
| Pattern store corruption | AI stops suggesting; user notified; data recoverable from backup if configured |
| Confidence engine error | Default to no suggestion (fail safe) |

---

## Related Documents

- [AI Principles](AI-PRINCIPLES.md)
- [Memory Policy](MEMORY-POLICY.md)
- [Confidence Policy](CONFIDENCE-POLICY.md)
- [Threat Model](../07-Security/THREAT-MODEL.md)
- [System Overview](../02-Architecture/SYSTEM-OVERVIEW.md)
