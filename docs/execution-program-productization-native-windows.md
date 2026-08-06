# Execution Program — P7 Productization & Native Windows Experience
## Milestone: Workspace feels like a real Windows application

| Field | Value |
| --- | --- |
| **Program** | P7 Productization & Native Windows Experience |
| **Date** | 2026-08-07 |
| **Prior** | P6 accepted (`11bab9d`) — shell Form A/B stable |
| **Handoff** | `AWAITING_PROJECT_OWNER_PRODUCTIZATION_REVIEW` |

---

## Scope

Track A productization only. No Track B capabilities. No Track C intelligence.

---

## Delivered

### Compact conversation
- Default **340×480** (productivity-sized)
- Migrates legacy 360×520 / 420×560 persists
- Clamps to work area (never larger than usable desktop)
- Remembers intentional user resize within soft max
- First-open placement: lower-right of work area

### Visual productization
- Lighter chrome (transparent title/composer bars)
- Conversation-first messages (less framed bubbles)
- Reduced padding / visual weight around the transcript

### Native window behaviour
- Min size enforced (300×400)
- Collapse/restore lifecycle unchanged (P6 accepted)
- Conversation skip-taskbar while Operator is active

### Repository health → product readiness
Developer dashboard now separates:
- Engineering Health
- Product Readiness
- Current Milestone
- Accepted Reviews (includes P6)
- Outstanding Product Debt

---

## Product Owner checklist

- [ ] Shell lifecycle still passes (P6)
- [ ] Conversation opens compact — does not dominate desktop
- [ ] Placement / restore feel native
- [ ] Visual polish noticeably lighter
- [ ] Feels like a Windows application, not a prototype
- [ ] Repository Health shows engineering + product readiness
- [ ] Repo committed and pushed; fresh runtime

---

## Stop

Await Product Owner review. Reassess next program from updated repository truth.
