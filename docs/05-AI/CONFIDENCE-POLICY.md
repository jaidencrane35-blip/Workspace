# Confidence Policy

| Field | Value |
|-------|-------|
| **Purpose** | Define AI confidence levels, suggestion thresholds, uncertainty handling, and automation requirements |
| **Owner** | Project Owner |
| **Dependencies** | [AI Operating Model](AI-OPERATING-MODEL.md), [Memory Policy](MEMORY-POLICY.md), [AI Principles](AI-PRINCIPLES.md) |
| **Update Process** | Project Owner approves changes. Specific threshold values require OQ-015 resolution and Decision Log entry. |

---

## 1. Purpose

Confidence governs when AI suggests, asks, or stays silent. Higher confidence requirements protect user trust. Specific numeric thresholds are pending OQ-015 — this document defines the framework.

---

## 2. Confidence Levels

| Level | Name | Description | AI Behaviour |
|-------|------|-------------|--------------|
| **L0** | Insufficient | Too few observations or high variance | Do not suggest; continue observing |
| **L1** | Emerging | Pattern detected but weak | Do not suggest; continue observing |
| **L2** | Moderate | Consistent pattern, moderate frequency | Ask clarifying question (optional) — do not automate |
| **L3** | High | Strong, repeated pattern | May suggest automation |
| **L4** | Very High | Highly consistent, frequent pattern | May suggest with full confidence statement |

### 2.1 Confidence Inputs

Confidence is derived from:

| Factor | Weight (TBD) |
|--------|--------------|
| Occurrence count | High |
| Consistency (variance) | High |
| Recency | Medium |
| Context stability | Medium |
| User feedback history (accept/dismiss ratio) | Medium |
| Time span of observation | Low |

Exact weighting requires implementation and tuning — record final values in Decision Log when OQ-015 is resolved.

---

## 3. Suggestion Thresholds

AI may present a suggestion only when confidence reaches **L3 (High)** or above.

| Confidence | Suggestion | Automation Proposal |
|------------|------------|---------------------|
| L0–L1 | No | No |
| L2 | Clarifying question only (optional) | No |
| L3 | Yes — with pattern explanation | Yes — requires user approval |
| L4 | Yes — with strong confidence statement | Yes — requires user approval |

**No confidence level permits autonomous action.** Approval is always required regardless of L3 or L4.

### 3.1 Proposed Numeric Thresholds (Pending OQ-015)

| Parameter | Proposed Starting Point | Status |
|-----------|------------------------|--------|
| Minimum observations before L2 | 3 occurrences | Proposed |
| Minimum observations before L3 | 5 occurrences over 3+ days | Proposed |
| Minimum observations before L4 | 10 occurrences over 7+ days | Proposed |
| Dismissal impact on confidence | −1 level per dismissal | Proposed |
| Cooldown after dismissal | 7 days before re-suggestion | Proposed |

These are starting proposals only — Project Owner must approve before implementation.

---

## 4. Uncertainty Handling

When AI is uncertain, it must **fail silent or ask** — never guess.

| Situation | Required Behaviour |
|-----------|-------------------|
| Confidence between L1 and L2 | Continue observing; no user interaction |
| Confidence at L2 with ambiguous pattern | Ask clarifying question: *"I noticed X sometimes followed by Y or Z. Which do you prefer?"* |
| Conflicting patterns | Explain conflict; present options; do not suggest |
| New context never seen before | Observe only |
| Pattern involves sensitive context (calls, screen share) | Require L4 before suggesting |
| User recently dismissed similar suggestion | Suppress until cooldown expires regardless of confidence |

---

## 5. When AI Should Ask Instead of Suggest

AI asks clarifying questions (instead of suggesting automation) when:

1. Confidence is L2 — pattern emerging but not confirmed
2. Multiple valid interpretations exist
3. User history shows mixed accept/dismiss for similar patterns
4. Pattern would affect a domain the user recently disabled and re-enabled
5. Suggested automation would cross domain boundaries for the first time

Clarifying questions are dismissible and do not imply consent for future automation.

---

## 6. Confidence Requirements Before Automation

Even after user approval, these rules apply:

| Rule | Detail |
|------|--------|
| Approval is independent of confidence | User may approve a L3 suggestion; AI may not auto-approve |
| Re-approval on context change | If pattern confidence drops below L2, pause automation and notify user |
| No confidence escalation | AI cannot re-classify L2 as L3 without new observations |
| Execution confidence check | Before each persistent automation run, verify pattern still meets L2 minimum |
| Failed execution | Reduce confidence by one level; notify user |

---

## 7. Sensitive Context Modifiers

Certain contexts require elevated confidence before suggestion:

| Context | Minimum Level Required |
|---------|----------------------|
| Audio routing changes | L3 |
| Application launching | L3 |
| Layout modification | L3 |
| During active call or meeting | L4 |
| First suggestion ever to user | L3 with extended explanation |
| Automation affecting multiple apps | L4 |

---

## 8. User-Configurable Confidence

Users may adjust (future setting):

- Suggestion frequency (see OQ-012)
- Minimum confidence threshold for suggestions (never below L3)
- Disable suggestions entirely while keeping observation

Users **cannot** reduce approval requirements — approval is always mandatory.

---

## Related Documents

- [AI Operating Model](AI-OPERATING-MODEL.md)
- [Memory Policy](MEMORY-POLICY.md)
- [AI Principles](AI-PRINCIPLES.md)
- [Open Questions](../09-Decisions/OPEN-QUESTIONS.md) — OQ-012, OQ-015
