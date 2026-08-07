# Screenshot Provider — Product Proof
## P15 — Owner checklist

| Field | Value |
| --- | --- |
| **Program** | P15 Screenshot Provider |
| **Status** | **Product Complete** — harness green |
| **IPC** | `execute_capability_intent` |
| **Harness** | `screenshot-provider.proof.json` + `tests/screenshot-provider-product-proof.test.ts` |

---

## Owner checklist (Conversation only)

1. “Can you take screenshots?” → availability reply  
2. “Take a screenshot.” / “Capture my desktop.” → desktop PNG saved  
3. “Screenshot this window.” → active window capture  
4. “Capture monitor two.” → monitor 2 (or truthful missing-monitor clarify)  
5. “Copy this screenshot.” / “Take a screenshot and copy it.” → clipboard image  
6. “Save a screenshot.” → PNG save  
7. “Screenshot Cursor.” / “Screenshot Chrome.” → named window (or truthful missing)  
8. Invalid monitor / missing window / unavailable → truthful failure  
9. Replies sound like desktop operation  
10. No Provider / Runtime / DXGI / WGC / `xcap` terminology  

---

## Pipeline proof

```
Conversation → Intent → execute_capability_intent → Kernel Operator
  → Capability Runtime → Router → Registry → Screenshot Provider → OS
  → desktop-language reply
```

---

## Closure stamp

**P15 Screenshot Provider**  
**PERMANENTLY CLOSED**  
**ACCEPTED**  
**REPOSITORY TRUTH**  
**DO NOT REOPEN**  
(Bug fixes only.)
