# Capability Composition Rule
## Permanent engineering law (P12.7 · Kernel composition P12 Finalization)

| Field | Value |
| --- | --- |
| **Status** | Authoritative |
| **Program** | P12.7 · composition authority → Kernel Operator (P12 Finalization) |

---

## Rule

Every provider must satisfy **both**:

1. **Independently useful** — a user can get value through Conversation via that provider alone.  
2. **Composable** — combinations with other providers create new user-visible capabilities.

Future execution programs maximize composability instead of isolated features.

---

## Examples (target compositions)

| Composition | User-visible capability |
| --- | --- |
| Clipboard + Applications | “Copy this into Cursor.” |
| Browser + Window | “Open ChatGPT beside Cursor.” |
| Files + Applications | “Open yesterday’s report in Word.” |
| Memory + Windows | “Restore yesterday’s workspace.” |

Composition authority belongs only to the **Kernel Operator**.  
Providers never call each other.

---

## Catalogue

Living catalogue: `COMPOSITION_CATALOGUE.md`  
Machine check: compositions declared there must name registered domains only.
