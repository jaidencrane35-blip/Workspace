# Constitutional Compliance Checklist

| Field | Value |
| --- | --- |
| **Kind** | Engineering governance — not constitutional law |
| **Authority** | Evaluates proposals against Workspace Constitutional Specification v2 |
| **Spec** | `WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md` |

Use this checklist before accepting any architectural proposal, capability, implementation change that affects ownership/authority, or production program with architectural impact.

---

## 1. Identity

- [ ] Preserves Conversational Desktop Operator identity  
- [ ] Does not redefine product purpose  

## 2. Information ownership

- [ ] Uses existing Information Owners only (Input, Understanding, Orchestration, Authority, Execution, Evidence, Presentation)  
- [ ] Does not invent a new Information Owner  

## 3. Transformation Chain

- [ ] Advances Signal → Meaning → Plan → [Authority] → Effect → Fact → Experience  
- [ ] Does not redefine or bypass the chain  

## 4. Laws & invariants

- [ ] Forward Form  
- [ ] No Hidden Authority  
- [ ] Observability  
- [ ] Singular Truth  
- [ ] Constitutional Closure / Minimalism / Conservation / Anti-Entropy as applicable  
- [ ] No illegal transitions (Meaning↛Effect, Experience↛Plan, Plan→Effect without Authority, etc.)  

## 5. Principles (guidance)

- [ ] Product Gravity / Presentation Purity / Composition respected where relevant  
- [ ] User Agency and Explicit Authority preserved  

## 6. Fitness Test (Spec §14)

1. New Information Owner?  
2. Violate invariant/law?  
3. Hidden Authority?  
4. Reduce traceability/observability?  
5. Existing Information Owner absorb?  

Answers **No / No / No / No / Yes** ⇒ MUST NOT become new architecture.

## 7. Compliance Clause

- [ ] Evaluated before acceptance  
- [ ] If fail ⇒ reject OR approved Spec amendment first  

## Verdict

| Result | Action |
| --- | --- |
| Pass | MAY proceed to engineering / Product Proof / Owner acceptance as applicable |
| Fail | MUST NOT accept without Spec amendment |
