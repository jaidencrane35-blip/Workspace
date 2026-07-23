# Memory Policy

| Field | Value |
|-------|-------|
| **Purpose** | Define what Workspace may remember, what it must not store, and how users control AI memory |
| **Owner** | Project Owner |
| **Dependencies** | [AI Principles](AI-PRINCIPLES.md), [AI Operating Model](AI-OPERATING-MODEL.md), [Security Principles](../07-Security/SECURITY-PRINCIPLES.md) |
| **Update Process** | Project Owner approves changes. Material changes require Decision Log entry. Specific retention periods require OQ-016 resolution. |

---

## 1. Purpose

Workspace memory is the data AI retains to detect patterns and make suggestions. Memory must be minimal, transparent, user-controlled, and local by default (DEC-005).

---

## 2. What Workspace May Remember

### 2.1 Permitted Memory Categories

| Category | Examples | Storage Location |
|----------|----------|------------------|
| **Application events** | App launched, focused, closed; launch order | AI pattern store (local) |
| **Window events** | Window created, moved, resized, closed | AI pattern store (local) |
| **Audio events** | Device changed, volume level changes, call state | AI pattern store (local) |
| **Device events** | Device connected, disconnected | AI pattern store (local) |
| **User layout actions** | Panel moved, resized, layout saved | Layout store + pattern store |
| **Automation history** | Approved automations, execution log | Automation service (local) |
| **User preferences** | Settings, modes, observation toggles | Platform Kernel config (local) |
| **Suggestion history** | Suggestions shown, accepted, dismissed | AI pattern store (local) |
| **Temporal metadata** | Time of day, day of week, frequency counts | AI pattern store (local) |

### 2.2 Permitted Pattern Types

- Sequential patterns (A then B)
- Temporal patterns (A at time T)
- Contextual patterns (A when B is active)
- Preference patterns (user chose X over Y)
- Frequency patterns (A occurs N times per period)

---

## 3. What Must Not Be Stored

The following must **never** be written to any Workspace store:

| Prohibited Data | Reason |
|-----------------|--------|
| Keystroke content | Privacy — not Workspace's domain |
| Clipboard content | Privacy |
| Screen pixels / screenshots | Privacy — unless future screen-aware feature approved via Decision Log |
| File contents | Privacy |
| Credentials, passwords, tokens | Security |
| Communication content (messages, email body) | Privacy — presence/state only permitted |
| Data from disabled observation domains | User choice |
| Data from other users' machines | Privacy — unless OQ-018 resolved with opt-in |
| Raw network traffic | Privacy and security |
| Biometric data | Privacy |

### 3.1 Ambiguous Cases (Require Decision Before Storage)

| Data | Current Status |
|------|----------------|
| File names and paths | Not stored until Product Owner approves scope — log as Open Question if needed |
| Window titles | Required for window patterns — store title strings only, not window content |
| Application arguments | Not stored by default — may contain sensitive data |

---

## 4. User Control Over Memory

Users must always be able to:

| Control | Description |
|---------|-------------|
| **View** | See all stored patterns and observation data |
| **Pause observation** | Stop new data collection without deleting existing data |
| **Disable domains** | Turn off observation for specific domains (apps, audio, etc.) |
| **Delete patterns** | Remove specific learned patterns |
| **Clear all AI memory** | Delete all learned patterns and suggestion history |
| **Export** | Export their memory data in a readable format |
| **Revoke automations** | Stop automations without deleting underlying patterns |

Memory controls are accessible from Workspace settings at all times.

---

## 5. Deletion Rules

| Trigger | What Is Deleted |
|---------|-----------------|
| User clears specific pattern | That pattern and associated suggestion history |
| User clears all AI memory | All patterns, suggestion history; automations revoked |
| User revokes automation | Automation definition stopped; underlying pattern preserved unless user also clears it |
| User uninstalls Workspace | All local data deleted — see OQ-017 for secure deletion requirements |
| Observation domain disabled | No new data collected; existing data for that domain retained until user deletes |

Deletion is immediate and irreversible. Confirm destructive actions with the user.

---

## 6. Retention Principles

Specific retention periods are pending OQ-016 resolution. Until decided, these principles apply:

| Principle | Rule |
|-----------|------|
| **Minimal retention** | Store only what is needed for pattern detection |
| **User-initiated persistence** | Patterns persist until user deletes or retention policy expires |
| **No silent expiry** | If auto-expiry is adopted, user is notified before data ages out |
| **Dismissed pattern cooldown** | Dismissed suggestions suppress re-suggestion; underlying observation data may still accumulate |
| **Stale pattern handling** | Patterns not reinforced over time should decay in confidence, not be deleted silently |
| **Local by default** | No external transmission without explicit opt-in |

### 6.1 Proposed Retention Tiers (Pending OQ-016)

| Tier | Proposed Duration | Status |
|------|-------------------|--------|
| Active patterns | Indefinite while reinforced | Proposed |
| Unreinforced patterns | Review for decay after 90 days | Proposed — requires decision |
| Suggestion history | 30 days | Proposed — requires decision |
| Automation execution logs | 90 days | Proposed — requires decision |
| Audit log | See Security Principles | TBD |

---

## 7. Learned Pattern Handling

### 7.1 Pattern Lifecycle

```
Observe → Store Raw Events → Aggregate → Detect Pattern → Score Confidence
                                                              ↓
                                              Above threshold → Suggest
                                              Below threshold → Continue observing
```

### 7.2 Pattern Storage Rules

- Patterns stored separately from raw events where possible
- Raw events aggregated and discarded after pattern extraction (exact timing TBD in OQ-016)
- Each pattern includes: source domain, confidence score, last observed, occurrence count
- Patterns are never modified by AI to increase confidence artificially

### 7.3 Cross-Device Learning

Cross-device pattern sharing is **prohibited** until OQ-018 is resolved. Each machine maintains independent memory.

---

## Related Documents

- [AI Principles](AI-PRINCIPLES.md)
- [AI Operating Model](AI-OPERATING-MODEL.md)
- [Confidence Policy](CONFIDENCE-POLICY.md)
- [Threat Model](../07-Security/THREAT-MODEL.md)
- [Open Questions](../09-Decisions/OPEN-QUESTIONS.md) — OQ-016, OQ-018
