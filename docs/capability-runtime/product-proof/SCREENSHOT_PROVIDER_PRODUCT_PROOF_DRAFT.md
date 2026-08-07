# Screenshot Provider — Product Proof Draft (P15 Prep)

| Field | Value |
| --- | --- |
| **Program** | P15 Screenshot Provider |
| **Status** | **Draft only** — harness not implemented |
| **Blocked on** | P14 Owner acceptance + P15 engineering |
| **IPC (planned)** | `execute_capability_intent` |

---

## Planned Owner checklist (Conversation only)

1. “Can you take screenshots?” → status / availability reply  
2. “Take a screenshot.” → clarify target **or** capture primary monitor (decide at implementation; prefer clarify if ambiguous)  
3. “Screenshot this window.” → capture active window  
4. “Screenshot Cursor.” → capture named window  
5. “Capture the right monitor.” → capture by monitor index/role  
6. Unavailable / blocked content → truthful failure  
7. Replies sound like desktop operation  
8. No Provider / Runtime / DXGI / WGC / `xcap` terminology  

---

## Planned pipeline proof

```
Conversation → Intent → Kernel Operator → Capability Runtime
  → Screenshot Provider → OS capture → desktop-language reply
```

---

## Deferred (not Product Proof for P15)

- “Watch my screen and screenshot when…”  
- Video recording  
- OCR of the capture  
- Automatic upload / share  

---

## Note

This draft exists so P15 can start immediately after P14 acceptance without redesigning Product Proof. No `*.proof.json` harness until implementation.
