# Confidence Policy

| Field | Value |
|-------|-------|
| **Purpose** | Define AI confidence levels, suggestion thresholds, uncertainty handling, and automation requirements |
| **Owner** | Project Owner |
| **Dependencies** | [AI Operating Model](AI-OPERATING-MODEL.md), [Memory Policy](MEMORY-POLICY.md), [AI Principles](AI-PRINCIPLES.md) |
| **Update Process** | Project Owner approves changes. Framework approved in DEC-013. Numeric tuning requires Decision Log entry. |

---

## 1. Approved Framework — DEC-013

**Status:** Accepted (2026-07-23)

Workspace adopts the **L0–L4 confidence model**. AI cannot silently escalate confidence. Automation requires explicit user approval at all levels.

---

## 2. Confidence Levels

| Level | Name | Description | AI Behaviour |
|-------|------|-------------|--------------|
| **L0** | No Confidence | Insufficient observations or high variance | No suggestion — continue observing |
| **L1** | Observation Only | Pattern emerging but not actionable | Observe and store — no user interaction |
| **L2** | Suggestion Allowed | Consistent pattern detected | May present suggestion to user |
| **L3** | Permission Required | Strong pattern — automation proposed | Must request explicit user permission before any action |
| **L4** | Previously Approved | User has approved this automation before | May execute within approved scope — user can revoke |

### 2.1 Escalation Rules

- AI **cannot** silently escalate from a lower level to a higher level
- Each level transition requires additional observations and/or user action
- Dismissal reduces confidence — never increases it automatically
- L4 applies only to automations with active, non-revoked user approval

### 2.2 Confidence Inputs

Confidence is derived from:

| Factor | Weight |
|--------|--------|
| Occurrence count | High |
| Consistency (variance) | High |
| Recency | Medium |
| Context stability | Medium |
| User feedback history (accept/dismiss ratio) | Medium |
| Time span of observation | Low |

Specific numeric thresholds will be tuned during Phase 2 implementation.

---

## 3. Suggestion and Permission Thresholds

| Confidence | Suggestion | Automation |
|------------|------------|------------|
| L0 | No | No |
| L1 | No | No |
| L2 | Yes — present suggestion | No — approval not yet requested |
| L3 | Yes — with permission request | Only after explicit user approval |
| L4 | N/A — automation active | Yes — within approved scope only |

**No confidence level permits autonomous action without prior user approval.**

---

## 4. Uncertainty Handling

When AI is uncertain, it must **fail silent or ask** — never guess.

| Situation | Required Behaviour |
|-----------|-------------------|
| Confidence at L0 | Continue observing; no user interaction |
| Confidence at L1 | Continue observing; no user interaction |
| Confidence at L2 with ambiguous pattern | Present suggestion with clarifying context |
| Conflicting patterns | Explain conflict; present options; do not automate |
| User dismissed suggestion | Reduce confidence; apply cooldown before re-suggestion |
| Pattern involves sensitive context | Require L3 before permission request |

---

## 5. When AI Should Ask Instead of Act

AI asks or suggests (never acts) when:

1. Confidence is L2 — suggestion allowed but permission not yet appropriate
2. Confidence reaches L3 — permission request required before any action
3. Multiple valid interpretations exist
4. User history shows mixed accept/dismiss for similar patterns
5. Automation would cross domain boundaries for the first time

---

## 6. L4 — Previously Approved Automation

L4 is not higher permission — it is **remembered approval**:

- User previously approved a specific automation at L3
- Automation executes within its approved definition only
- User can revoke at any time — drops back to L0 for that pattern
- If pattern changes significantly, revert to L2/L3 and re-request permission
- Scope creep is prohibited — approved automation cannot expand silently

---

## 7. User-Configurable Settings

Users may adjust (future setting):

- Suggestion frequency (OQ-012)
- Disable suggestions entirely while keeping observation

Users **cannot** reduce approval requirements — L3 permission is always required for new automations.

---

## Related Documents

- [Decision Log](../09-Decisions/DECISION-LOG.md) — DEC-013
- [AI Operating Model](AI-OPERATING-MODEL.md)
- [Memory Policy](MEMORY-POLICY.md)
- [AI Principles](AI-PRINCIPLES.md)
