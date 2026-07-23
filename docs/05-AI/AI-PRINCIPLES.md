# AI Principles

| Field | Value |
|-------|-------|
| **Purpose** | Define the behaviour, boundaries, and governance of AI within Workspace |
| **Owner** | AI Lead (TBD) |
| **Dependencies** | [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md), [Product Vision](../01-Product/PRODUCT-VISION.md), [UX Principles](../04-UX/UX-PRINCIPLES.md) |
| **Update Process** | AI Lead proposes changes. Any change to the permission model requires Product Owner approval and Decision Log entry. |

---

## 1. AI Mission

Workspace AI observes user patterns, learns workflows, and suggests automations — always with the user's explicit permission. AI is a subsystem that supports the workspace; it is not the product itself.

---

## 2. The AI Sequence

All AI behaviour must follow this sequence without exception:

```
Observe → Learn → Suggest → Receive Permission → Automate
```

| Stage | Description | User Visibility |
|-------|-------------|-----------------|
| **Observe** | Passively monitor system events and user actions | Transparent — user can see what is observed |
| **Learn** | Identify patterns and correlations in observed data | Transparent — user can review learned patterns |
| **Suggest** | Propose an automation or workflow improvement | Active — user receives a clear, dismissible prompt |
| **Receive Permission** | Wait for explicit user approval | Required — no default opt-in |
| **Automate** | Execute the approved automation | Active — user can see and stop automations |

**Skipping any stage is prohibited.**

---

## 3. Permission Model

### 3.1 Core Rules

1. AI never performs a state-changing action without explicit user approval
2. Approval is per-automation, not blanket
3. Approval can be revoked at any time
4. Dismissed suggestions are not re-suggested for the same pattern unless conditions change significantly
5. Pre-checked consent boxes are prohibited

### 3.2 Approval Types

| Type | Scope | Example |
|------|-------|---------|
| **One-time** | Single execution | "Open Discord now?" |
| **Session** | Until app restart | "Reduce Spotify volume during this call?" |
| **Persistent** | Until user revokes | "Always open Discord after Steam?" |

The user always chooses the approval type. AI does not default to persistent.

### 3.3 Revocation

- User can revoke any automation from settings
- Revoking stops future executions immediately
- Learned patterns are preserved unless user explicitly clears them
- Clearing patterns is a separate, explicit action

---

## 4. Observation Boundaries

### 4.1 What AI May Observe

| Data Type | Examples |
|-----------|----------|
| Application events | App launched, focused, closed |
| Window events | Window created, moved, resized, closed |
| Audio events | Device changed, volume adjusted, call started |
| Device events | Device connected, disconnected |
| User actions | Panel interactions, layout changes, mode switches |
| Temporal patterns | Time of day, day of week, sequence of actions |

### 4.2 What AI Must Not Observe

| Data Type | Reason |
|-----------|--------|
| Keystrokes / input content | Privacy — content is not Workspace's domain |
| Screen content / pixels | Privacy — unless explicit screen-aware feature approved |
| File contents | Privacy — file metadata (name, path) may be observed if approved |
| Credentials / passwords | Security |
| Communication content | Privacy — presence and state only |
| Data from other applications' private stores | Privacy and security |

Specific observation scope may be refined per feature with Product Owner approval.

### 4.3 Observation Transparency

- User can view everything AI is observing
- User can disable observation for specific domains
- User can pause all observation
- Observation state is visible in settings at all times

---

## 5. Learning Boundaries

### 5.1 What AI May Learn

- Sequential patterns (A then B)
- Temporal patterns (A at time T)
- Contextual patterns (A when B is active)
- Preference patterns (user chose X over Y)
- Frequency patterns (A happens N times per day)

### 5.2 What AI Must Not Learn

- Content of user input
- Credentials or authentication data
- Data from disabled observation domains
- Patterns from other users (no cross-user learning without explicit consent and Product Owner approval)

### 5.3 Learning Storage

- All learned data stored locally by default
- Data format and retention policy are open decisions
- User can export, inspect, and delete all learned data
- No learned data transmitted externally without explicit opt-in

---

## 6. Suggestion Guidelines

### 6.1 Good Suggestions

- Based on observed patterns with sufficient confidence
- Clearly explain the pattern detected
- Offer a specific, actionable automation
- Easy to accept, modify, or dismiss
- Appropriately timed (not during focused work)

### 6.2 Examples

**Good:**
> "I noticed you always open Discord after launching Steam. Would you like me to automate that?"
> [Yes, always] [Just this once] [No thanks]

**Good:**
> "I noticed Spotify is usually reduced to 20% when Discord voice chat starts. Would you like me to save this as an automation?"
> [Save automation] [Not now]

**Bad (prohibited):**
> ~~"I've opened Discord for you since you launched Steam."~~ ( acted without permission )
> ~~"Enable smart automations?" [✓ Enabled by default]~~ ( pre-checked consent )
> ~~"I noticed you use Chrome a lot. Switch your default browser?"~~ ( unsolicited opinion )

### 6.3 Suggestion Frequency

- Maximum suggestion frequency is an open design decision
- Suggestions must not feel nagging
- Dismissed suggestions have a cooldown period
- User can adjust suggestion frequency in settings

---

## 7. Automation Execution

Once approved:

- Automations execute as defined — no scope creep
- User can pause or stop any running automation
- Automation execution is logged and visible to the user
- Failed automations notify the user; no silent retries
- Automations respect current context (e.g., do-not-disturb)

---

## 8. AI Safety

| Rule | Detail |
|------|--------|
| Fail safe | If AI subsystem fails, Workspace continues without AI features |
| No escalation | AI cannot grant itself additional permissions |
| Audit trail | All AI decisions and actions are logged locally |
| Human override | User actions always take precedence over AI |
| No manipulation | AI must not use dark patterns to increase acceptance rates |

---

## 9. AI Architecture Constraints

- AI is a subsystem, not embedded in every module
- AI accesses domain state through the event bus and defined APIs
- AI cannot call OS APIs directly
- AI cannot bypass the Permission Gateway (owned by Platform Kernel)
- AI submits permission requests; it does not enforce permissions
- AI model selection and deployment are open decisions (OQ-004)

See [System Overview](../02-Architecture/SYSTEM-OVERVIEW.md) §2.7 and [AI Operating Model](AI-OPERATING-MODEL.md).

---

## 10. Open AI Questions

See [Open Questions](../09-Decisions/OPEN-QUESTIONS.md) for unresolved items including:

- AI model selection (OQ-004)
- Confidence thresholds (OQ-015)
- Suggestion frequency limits (OQ-012)
- Learned-data retention (OQ-016)
- Cross-device learning scope (OQ-018)

---

## Related Documents

- [AI Operating Model](AI-OPERATING-MODEL.md)
- [Memory Policy](MEMORY-POLICY.md)
- [Confidence Policy](CONFIDENCE-POLICY.md)
- [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md)
- [UX Principles](../04-UX/UX-PRINCIPLES.md)
- [Security Principles](../07-Security/SECURITY-PRINCIPLES.md)
- [Threat Model](../07-Security/THREAT-MODEL.md)
- [System Overview](../02-Architecture/SYSTEM-OVERVIEW.md)
