# Memory Policy

| Field | Value |
|-------|-------|
| **Purpose** | Define what Workspace may remember, what it must not store, and how users control AI memory |
| **Owner** | Project Owner |
| **Dependencies** | [AI Principles](AI-PRINCIPLES.md), [AI Operating Model](AI-OPERATING-MODEL.md), [Security Principles](../07-Security/SECURITY-PRINCIPLES.md) |
| **Update Process** | Project Owner approves changes. Retention model approved in DEC-014. |

---

## 1. Approved Model — DEC-014

**Status:** Accepted (2026-07-23)

Workspace uses **user-controlled adaptive memory**:

- Temporary observations expire
- Learned patterns require sufficient confidence before persistence
- Stored memories are reviewable
- User controls deletion
- No hidden learning

Stored in SQLite (DEC-010) with JSON export support.

---

## 2. What Workspace May Remember

### 2.1 Permitted Memory Categories

| Category | Examples | Storage |
|----------|----------|---------|
| **Application events** | App launched, focused, closed; launch order | SQLite (AI patterns) |
| **Window events** | Window created, moved, resized, closed | SQLite |
| **Audio events** | Device changed, volume level changes, call state | SQLite |
| **Device events** | Device connected, disconnected | SQLite |
| **User layout actions** | Zone moved, element placed, layout saved | SQLite (layouts) |
| **Automation history** | Approved automations, execution log | SQLite |
| **User preferences** | Settings, modes, observation toggles | SQLite |
| **Suggestion history** | Suggestions shown, accepted, dismissed | SQLite |
| **Temporal metadata** | Time of day, day of week, frequency counts | SQLite |

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
| Keystroke content | Privacy |
| Clipboard content | Privacy |
| Screen pixels / screenshots | Privacy — unless future feature approved via Decision Log |
| File contents | Privacy |
| Credentials, passwords, tokens | Security |
| Communication content | Privacy |
| Data from disabled observation domains | User choice |
| Data from other users' machines | Privacy — OQ-018 unresolved |
| Raw network traffic | Privacy and security |

---

## 4. User Control Over Memory

Users must always be able to:

| Control | Description |
|---------|-------------|
| **View** | See all stored patterns and observation data |
| **Pause observation** | Stop new data collection |
| **Disable domains** | Turn off observation per domain |
| **Delete patterns** | Remove specific learned patterns |
| **Clear all AI memory** | Delete all learned patterns and suggestion history |
| **Export** | Export memory data as JSON (DEC-010) |
| **Revoke automations** | Stop automations; memory preserved unless user clears |

---

## 5. Adaptive Retention Rules (DEC-014)

| Rule | Detail |
|------|--------|
| **Temporary observations expire** | Raw event data aggregated and discarded after pattern extraction |
| **Confidence-gated persistence** | Patterns persist only when confidence reaches L1+; suggestions require L2+ |
| **No hidden learning** | All stored memory visible in settings |
| **User deletion is immediate** | Deletion is irreversible; confirm destructive actions |
| **No silent expiry of user patterns** | Stored patterns remain until user deletes or confidence decays with notification |
| **Cross-device** | Prohibited until OQ-018 resolved |

Specific expiry durations will be defined during Phase 2 implementation and recorded in Decision Log if precedent-setting.

---

## 6. Deletion Rules

| Trigger | What Is Deleted |
|---------|-----------------|
| User clears specific pattern | Pattern and associated suggestion history |
| User clears all AI memory | All patterns, suggestion history; automations revoked |
| User revokes automation | Automation stopped; underlying pattern preserved unless cleared |
| User uninstalls Workspace | All local SQLite data deleted |
| Observation domain disabled | No new data; existing domain data retained until user deletes |

---

## 7. Learned Pattern Handling

```
Observe → Store Temporary Events → Aggregate → Detect Pattern → Score Confidence
                                                              ↓
                                    Below L1 → Discard temporary data
                                    L1+ → Persist to SQLite
                                    L2+ → Eligible for suggestion
```

Patterns stored in SQLite with: source domain, confidence level, last observed, occurrence count, created/updated timestamps.

---

## Related Documents

- [AI Memory Foundation](AI-MEMORY-FOUNDATION.md) — implemented model (Sprints 60–61)
- [Decision Log](../09-Decisions/DECISION-LOG.md) — DEC-014, DEC-010
- [Confidence Policy](CONFIDENCE-POLICY.md)
- [AI Operating Model](AI-OPERATING-MODEL.md)
- [Threat Model](../07-Security/THREAT-MODEL.md)
