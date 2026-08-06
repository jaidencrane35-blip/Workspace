# Operator / Kernel Architecture — Product Proof
## P12 Finalization Owner checklist

| Field | Value |
| --- | --- |
| **Program** | P12 Finalization |
| **Architecture** | Conversation → Intent → Kernel Operator → Runtime → Provider → OS |
| **IPC** | `execute_capability_intent` only from Conversation |

---

## Owner verifies Conversation can (behaviour unchanged or better)

- [ ] Open applications  
- [ ] Focus applications  
- [ ] Move / snap / restore windows  
- [ ] Clarify ambiguous requests  
- [ ] Report truthful failures  

Without React containing operational decision logic.

---

## Architectural gates

- [ ] No React provider composition  
- [ ] No presentation execution-order policy  
- [ ] No presentation permission decisions  
- [ ] All Conversation capabilities through Kernel Operator  
- [ ] No remaining reason for another P12.x (except bugfixes)  

---

## After acceptance

P12 series is **complete**. **P13 Notifications Provider** is the next eligible execution program.
