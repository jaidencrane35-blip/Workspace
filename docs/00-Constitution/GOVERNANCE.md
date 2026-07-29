# Governance Model

| Field | Value |
|-------|-------|
| **Purpose** | Define how decisions are made, documented, and enforced across the Workspace project |
| **Owner** | Project Owner |
| **Dependencies** | [Project Constitution](PROJECT-CONSTITUTION.md) |
| **Update Process** | Update when decision processes change or new governance domains emerge. Record material changes in the Decision Log. |

---

## 1. Project Structure

Workspace is a **founder-led project**. It is not organised as a large company with separate departments. The human Project Owner retains final authority on all product, architecture, and scope decisions.

AI tools and agents are **advisory contributors** — they recommend, document, and review, but do not hold decision authority.

---

## 2. Governance Layers

Workspace governance operates at four layers. Lower layers cannot override higher layers.

```
┌─────────────────────────────────────┐
│  Constitution (non-negotiable)      │
├─────────────────────────────────────┤
│  Principles (Product, Arch, Eng, UX, │
│              AI, Security)            │
├─────────────────────────────────────┤
│  Decisions (recorded choices)       │
├─────────────────────────────────────┤
│  Implementation (code, configs)     │
└─────────────────────────────────────┘
```

---

## 3. Decision Authority

| Domain | Authority | Notes |
|--------|-----------|-------|
| All product, architecture, and scope decisions | **Project Owner** | Final authority |
| Engineering standards and delivery process | Lead Software Engineer | Implements Owner direction |
| Documentation quality | Lead Software Engineer | Advisory to Owner |
| AI product behaviour | Project Owner | With AI advisory input |
| Security model | Project Owner | With security advisory input |

When the Project Owner is unavailable, decisions are **blocked** and logged in [Open Questions](../09-Decisions/OPEN-QUESTIONS.md) — not assumed or delegated silently.

### 3.1 Advisory Roles (Not Decision-Makers)

These roles describe functions that AI agents or future contributors may perform. They advise; they do not approve.

| Advisory Role | Function |
|---------------|----------|
| **Architecture Reviewer** | Evaluate designs against architecture principles; flag risks |
| **Engineering Assistant** | Implement within approved scope; follow coding standards |
| **UX Analyst** | Review interactions against UX principles; suggest improvements |
| **Security Reviewer** | Review against threat model and security principles |
| **QA Reviewer** | Verify Definition of Done; identify test gaps |
| **Documentation Maintainer** | Keep docs accurate, cross-referenced, and current |

AI agents operating in Cursor or similar tools typically perform these advisory roles. Their output requires Project Owner review before it becomes a recorded decision or merged code.

---

## 4. Decision Types

### Type A — Constitutional

Affects non-negotiable principles. Requires Project Owner approval and Decision Log entry.

### Type B — Strategic

Affects architecture, product direction, AI behaviour, or security model. Requires Project Owner approval and Decision Log entry.

### Type C — Tactical

Affects implementation within approved architecture. Lead Software Engineer can approve within Owner-defined scope. Log if it sets precedent.

### Type D — Operational

Sprint-level choices within approved scope. Documented in sprint notes. No Decision Log entry unless precedent-setting.

---

## 5. Decision Workflow

1. **Identify** — Recognise that a decision is needed. If ambiguous, add to Open Questions.
2. **Document** — Write options, trade-offs, and recommendation.
3. **Review** — Project Owner reviews and approves, rejects, or defers.
4. **Record** — Accepted decisions go to Decision Log with date, owner, and status.
5. **Implement** — Update dependent documents before or alongside code.
6. **Validate** — Confirm implementation matches the recorded decision.

---

## 6. Escalation

Escalate to the Project Owner when:

- A decision spans multiple domains (e.g., AI + UX + Security)
- Contributors disagree on interpretation of principles
- A shortcut would violate constitution or principles
- Scope exceeds current roadmap phase
- An AI agent recommends a product or architecture change

There is no escalation above the Project Owner. The Owner may consult advisors but retains final authority.

---

## 7. AI Contributor Governance

AI tools (including Cursor agents) are advisory contributors. They must:

- Read constitution and relevant principles before acting
- Follow [`AGENTS.md`](../../AGENTS.md) and [AI Engineering Governance](../00-Governance/AI_ENGINEERING_GOVERNANCE.md) for engineering process
- Never silently change product direction
- Flag ambiguous requirements in Open Questions
- Follow Documentation Standards for all written output
- Not write production code unless explicitly authorised by Project Owner for the current sprint scope
- Present recommendations for Owner review — not implement decisions autonomously
- Prefer extending existing Workspace systems over creating AI-centric parallel products

AI agents may perform any advisory role (§3.1) but **never hold decision authority**.

---

## 8. Review Cadence

| Activity | Frequency |
|----------|-----------|
| Open Questions triage | Weekly (when active development begins) |
| Decision Log review | Monthly |
| Constitution and principles review | Quarterly |
| Risk Register review | Monthly |
| Roadmap review | Per phase gate |

---

## Related Documents

- [Project Constitution](PROJECT-CONSTITUTION.md)
- [AI Engineering Governance](../00-Governance/AI_ENGINEERING_GOVERNANCE.md)
- [Decision Log](../09-Decisions/DECISION-LOG.md)
- [Open Questions](../09-Decisions/OPEN-QUESTIONS.md)
- [Scope Management](../01-Product/SCOPE-MANAGEMENT.md)
