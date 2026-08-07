# Workspace Engineering Execution Standard v1

**The operating standard for every engineering program under the Workspace Constitutional Specification v2.**

| Field | Value |
| --- | --- |
| **Kind** | Engineering Governance — **not** constitutional law |
| **Version** | 1.0 |
| **Status** | Binding for engineering agents and programs |
| **Governs** | How work is classified, scoped, and executed |
| **Does not govern** | What Workspace is (see Constitutional Specification) |
| **Normative language** | MUST / MUST NOT / MAY / SHOULD per RFC 2119 |
| **Sole architectural authority** | `WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md` |

---

## 1. Purpose

This Standard defines how every future engineering program executes under the Constitution.

It exists so that prompts and agents do **not** reinvent lifecycle, authority, or scope.  
It prevents constitutional prompts from becoming runtime prompts, and runtime prompts from drifting upward into architecture.

---

## 2. Repository context (mandatory preamble)

Every future engineering prompt and agent session MUST assume:

> **Workspace Constitutional Specification v2 is the sole governing architectural specification.**

Before making changes, agents MUST:

1. **Classify** the work (Program Type).  
2. **Identify** governing documents for that type.  
3. **Verify** constitutional compliance where architecture or capability ownership is affected.  
4. **Determine** the lowest layer affected.  
5. **Refuse** unnecessary architectural or constitutional work.

If a prompt requests constitutional redesign without a named Review Trigger, the agent MUST refuse and cite Constitutional Closure.

---

## 3. Engineering invariant (lowest layer)

**Every engineering change SHALL modify the lowest architectural layer capable of solving the problem.**

| Preference order (lowest → highest) |
| --- |
| Documentation → Repository Standards → Runtime / Provider → Capability → Engineering Governance → Architecture Standard → Constitution |

Upward escalation (e.g. runtime fix → Architecture Standard or Constitution) MUST NOT occur unless the lower layer is objectively insufficient **and** a Review Trigger is satisfied when Constitution is implicated.

---

## 4. Maximum permitted layer

Every task MUST declare a maximum authority level. Agents MUST NOT modify or redefine layers above that maximum.

| Max layer | Agent MAY touch | Agent MUST NOT |
| --- | --- | --- |
| **Constitution** | Spec (only with Review Trigger + Product Owner) | — |
| **Architecture Standard** | Operator rules, Gravity detail, subordinate eng constitution | Spec concepts |
| **Engineering Governance** | Protocol, Product Proof process docs, handoff, this Standard | Spec; Architecture Standard redesign |
| **Repository Standards** | project-health, verifiers, doc standards | Spec; Architecture Standard |
| **Capability** | Provider contracts, capability docs, Integration under Spec | Spec laws/owners |
| **Runtime** | Code, ports, Kernel realization, Intent realization | Spec; invent new owners/laws |
| **Documentation** | Guides, maps, historical notes | Spec; runtime behaviour |
| **Production** | Installer, updater, diagnostics, tray, signing | Spec; capability architecture invent |
| **Maintenance** | Bugfix within existing ownership | Spec; new architecture |

Default for capability delivery: max layer = **Runtime** + **Capability** + **Documentation** (+ Product Proof governance).  
Default for production hardening: max layer = **Production** + **Repository Standards** + **Documentation**.

---

## 5. Program type classification (mandatory)

Before touching the repository, classify the program as **exactly one** primary type (secondary tags MAY apply):

| Type | Meaning |
| --- | --- |
| □ **Constitutional** | Amend Spec — **forbidden** unless Review Trigger |
| □ **Architecture Standard** | Change Architecture Standard layer only |
| □ **Governance** | Process, protocol, handoff, execution standards |
| □ **Capability** | New or extended user-facing capability domain |
| □ **Runtime** | Code/behaviour within existing ownership |
| □ **Provider** | Capability Runtime provider / port |
| □ **Documentation** | Docs only; no behaviour |
| □ **Production** | Ship/operate concerns (update, install, crash, support) |
| □ **Maintenance** | Defect fix; no expansion |

If classification is ambiguous, choose the **lower** layer and state the assumption.

---

## 6. Capability lifecycle (mandatory)

Every capability program (File, Terminal, Memory, Automation, etc.) MUST follow:

```
Proposal
    ↓
Constitutional Compliance
    ↓
Architecture Standard Compliance
    ↓
Capability Design
    ↓
Implementation
    ↓
Verification
    ↓
Product Proof
    ↓
Owner Acceptance
    ↓
Repository Health
    ↓
Release
```

| Stage | Governing artifact |
| --- | --- |
| Proposal | This Standard §5–§7 |
| Constitutional Compliance | Spec + `CONSTITUTIONAL_COMPLIANCE_CHECKLIST.md` |
| Architecture Standard Compliance | Operator / Gravity / subordinate eng constitution (Spec wins) |
| Capability Design | Capability Integration Standard (Spec) + Provider Acceptance |
| Implementation | Runtime / Provider code |
| Verification | `pnpm test` / provider verifiers / typecheck |
| Product Proof | `PRODUCT_PROOF_RULE.md` |
| Owner Acceptance | Product Owner |
| Repository Health | `project-health.json` + sync/verify |
| Release | Production / Repository Standards |

No prompt MAY invent an alternate lifecycle for capabilities.

---

## 7. Pre-flight checklist (every session)

Agents MUST complete before edits:

1. **Spec authority** — Spec v2 is sole architectural authority; no reinterpretation.  
2. **Program type** — classified (§5).  
3. **Max layer** — declared (§4).  
4. **Lowest layer** — invariant satisfied (§3).  
5. **Governing docs** — listed for this type.  
6. **Compliance** — checklist run if Capability / Architecture / Runtime ownership affected.  
7. **Refusal** — stop if work requires Constitution without Review Trigger.  
8. **P16/P17 gates** — respect `project-health.json` / handoff (governance sequencing).  

---

## 8. Governing documents by program type

| Type | Primary governors |
| --- | --- |
| Constitutional | Spec + Review Triggers + Product Owner |
| Architecture Standard | Spec → Architecture Standard docs |
| Governance | Spec → this Standard → protocol / Product Proof |
| Capability / Provider | Spec Integration → Provider Acceptance → Product Proof |
| Runtime / Maintenance | Spec (compliance) → handoff → code |
| Documentation | Spec (terminology) → hierarchy / maps |
| Production | Spec qualities → production Track programs → Repository Standards |

---

## 9. Audit epistemology (mandatory)

Engineering SHALL distinguish between **“no evidence of non-compliance”** and **“proof of universal compliance.”** Repository audits certify only the scope actually examined.

Use `REPOSITORY_CONFIDENCE_MODEL.md` (Verified / Supported / Hypothesis / Unknown). Do not elevate Supported or scoped Verified findings into universal constitutional certainty.

---

## 10. Forbidden drifts

Agents MUST NOT:

- Redesign the Constitutional Specification without a Review Trigger  
- Invent Information Owners, laws, or Transformation Chain variants  
- Escalate a runtime bugfix into Architecture Standard or Constitution  
- Skip Product Proof for capability delivery when required by Product Proof Rule  
- Begin deferred capability programs when handoff/health blocks them  
- Treat historical sprint / Blueprint docs as architectural authority  
- Treat a scoped compliance audit as proof that every dormant or future surface is compliant  

---

## 11. Relationship to other layers

```
Constitutional Specification v2     ← what Workspace is (immutable without trigger)
        ↓
Architecture Standard               ← how principles are expressed in this era
        ↓
Engineering Execution Standard (this document)  ← how work runs
        ↓
Engineering Protocol / Product Proof / Programs
        ↓
Repository Standards / Runtime / Release
```

This document MUST NOT amend the Spec. On conflict, the Spec wins.

---

## 12. Success criteria for meta-governance

Meta-architecture is complete when:

- Spec is frozen and sole architectural authority  
- Repository authority is aligned  
- This Execution Standard defines lifecycle, classification, max layer, and lowest-layer invariant  
- Future work is Proposal → Compliance → Design → Implement → Verify → Proof → Accept → Health → Release  

Thereafter, focus is implementation, Product Proof, production hardening, and capabilities — not constitutional design.
