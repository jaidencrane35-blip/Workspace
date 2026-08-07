# Notifications Provider — Product Proof (P13)

| Field | Value |
| --- | --- |
| **Program** | P13 Notifications Provider |
| **Harness** | `notifications-provider.proof.json` |
| **Conversation IPC** | `execute_capability_intent` |
| **Handoff** | `AWAITING_PROJECT_OWNER_NOTIFICATIONS_PRODUCT_PROOF_REVIEW` |

---

## Product Owner review checklist

Verify entirely through Conversation:

### Availability
- “Can you send notifications?”
- “Are notifications available?”

### Show
- “Show me a notification.”
- “Send me a desktop notification.”
- “Show me a notification: Restore finished.”

### Clarification (not invented watching)
- “Notify me when that’s finished.” → explains watching isn’t available yet  
- “Tell me when the download completes.” → same  
- “Notify me when Cursor finishes.” → same  

### Honesty
- If Windows blocks delivery → real failure text  
- No provider / router / registry jargon  

---

## Observable Product Proof

A desktop toast should appear when Conversation shows a notification (Windows Focus Assist / OS settings may suppress — report truthfully).

---

## Acceptance

P13 is **Product Complete** only when Engineering Completion **and** this Product Proof succeed under Owner review.
