# Architecture & Governance Authority Hierarchy

| Field | Value |
| --- | --- |
| **Status** | Subordinate governance companion — not constitutional law |
| **Date** | 2026-08-07 |
| **Purpose** | Locate the Constitutional Specification vs governance documents |
| **Canonical Spec** | `WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md` |
| **Review evidence** | `CONSTITUTIONAL_FINAL_REVIEW.md` |
| **Alignment audit** | `CONSTITUTIONAL_ALIGNMENT_AUDIT.md` |
| **Compliance checklist** | `CONSTITUTIONAL_COMPLIANCE_CHECKLIST.md` |
| **Engineering Execution Standard** | `WORKSPACE_ENGINEERING_EXECUTION_STANDARD_V1.md` |

---

## 1. Governing hierarchy

```
Workspace Constitutional Specification v2
        ↓
Architecture Standard
        ↓
Engineering Governance
        ↓
Repository Standards
        ↓
Capability Standards
        ↓
Engineering Programs
```

Only the top layer is constitutional. All layers below MAY evolve without amendment if they continue to satisfy the Spec.

---

## 2. Where documents sit

| Layer | Examples |
| --- | --- |
| **Constitutional Specification** | `WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md` |
| **Architecture Standard** | Operator rules, Product Gravity rule, knowledge maps, subordinate `architecture/ARCHITECTURAL_CONSTITUTION_V2.md` |
| **Engineering Governance** | **Engineering Execution Standard v1**, execution protocol, Product Proof Rule, engineering handoff, milestone report, compliance checklist, alignment audit |
| **Repository Standards** | project-health, verify/sync scripts, documentation standards |
| **Capability Standards** | Provider Acceptance, capability-runtime provider docs |
| **Engineering Programs** | Roadmaps, sprint history, Track work, capability delivery programs |

---

## 3. Cross-reference (subordinate)

| Document | Role |
| --- | --- |
| `WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md` | Sole architectural authority |
| `CONSTITUTIONAL_FINAL_REVIEW.md` | Review evidence (governance) |
| `architecture/ARCHITECTURAL_CONSTITUTION_V2.md` | Subordinate; Spec wins on conflict |
| `PRODUCT_CONSTITUTION.md` | Product experience governance |
| `PROJECT-CONSTITUTION.md` | Values governance |
| `PRODUCT_PROOF_RULE.md` | Engineering governance |
| `.cursor/rules/constitutional-execution-protocol.mdc` | Engineering governance |
| `ENGINEERING_HANDOFF.md` | Onboarding governance |
| Historical sprints / research essays | Historical evidence only |

---

## 4. Future prompt posture

Assume the Workspace Constitutional Specification v2 is the sole architectural authority. Operate entirely within the Engineering Execution Standard v1. Do not modify, reinterpret, or extend constitutional concepts unless a named constitutional review trigger has been satisfied. Classify work, declare max layer, and modify the lowest layer capable of solving the problem.
