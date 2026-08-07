# Notifications Provider — Product Proof (P13)

| Field | Value |
| --- | --- |
| **Program** | P13 Notifications Provider |
| **Harness** | `notifications-provider.proof.json` |
| **Composition audit** | `NOTIFICATIONS_PROVIDER_COMPOSITION_AUDIT.md` |
| **Conversation IPC** | `execute_capability_intent` |
| **Series status** | **P13 permanently closed** |

---

## Product Owner review checklist (satisfied)

### Availability
- “Can you send notifications?”
- “Can you send me a notification?”
- “Are notifications available?”

### Show
- “Show me a notification.”
- “Show me a desktop notification.”
- “Send me a desktop notification.”
- “Notify me that the build finished.”
- “Show me a notification: Restore finished.”

### Dismiss
- “Dismiss that notification.”

### Clarification (not invented watching)
- “Notify me when that’s finished.” → watching not available  
- “Tell me when the download completes.” → same  
- “Notify me when Cursor finishes.” → same  

### Honesty
- Windows blocks delivery → real failure text  
- No provider / router / registry jargon  

---

## Observable Product Proof

Desktop toast verified via WinRT WRAP (`winrt_show_desktop_toast_product_proof`) and Conversation Intent → Kernel Operator path.

---

## Acceptance

P13 is **Product Complete** and **permanently closed**.  
No further P13.x except genuine bug fixes.  
**P14 Browser Provider** is next.
