# Product Gravity Rule
## Permanent presentation law (P12.6)

| Field | Value |
| --- | --- |
| **Status** | Authoritative |
| **Authority peers** | UI Architecture Spec · Product Presentation Spec · Constitutional Execution Protocol |
| **Program** | P12.6 Conversational Desktop Surface |

---

## Rule

The user's attention should always be naturally pulled toward **Conversation**.

Everything else exists only to support Conversation.

If a UI element competes with Conversation for attention, it probably does not belong in the default surface.

| Pillar | Meaning |
| --- | --- |
| Conversation | The product |
| Desktop operation | The capability |
| Trust | The outcome |

---

## Completion distinctions (permanent)

| Claim | Meaning |
| --- | --- |
| Provider Complete ≠ Product Complete | Infrastructure without Conversation use is incomplete (P12.5) |
| Conversation Complete ≠ Product Complete | Conversation is complete only when it feels like **desktop operation**, not like chat |

Conversation is not “done” when messages route correctly.  
It is done when Workspace feels like a lightweight desktop companion and the surrounding UI has receded.

---

## Launch gravity

On launch, the first thing noticed must be Conversation — not a framed application, not Operator chrome, not a utility shell.

| Surface | Role under gravity |
| --- | --- |
| Conversation (Form B) | Default visible product presence |
| Desktop Operator (Form A) | Collapse / idle companion — recovers Conversation on click |
| Satellites | Appear only when earned; never louder than Conversation |

Form A/B remains the frozen two-form shell. Gravity changes **default attention**, not the form model.

---

## Surface gravity

Conversation should feel as if it simply exists on top of Windows:

- Transparent / quiet host — desktop almost disappears behind it  
- Transcript is the visual hero  
- Chrome whispers (Collapse / Exit / brand)  
- No competing docks, menus, or teaching chrome by default  
- No OS title-bar utility frame competing with the thread  

---

## Tests for every future change

1. Does this pull attention toward Conversation?  
2. Does this make Workspace feel more like desktop operation (and less like chat)?  
3. If removed, does the product lose a real conversational ability — or only chrome noise?

If the answer to (1) or (2) is no, do not ship it in the default surface.

---

## Related product heuristic (not constitutional law)

When there is a choice between exposing engineering complexity and reducing user friction, prefer reducing user friction unless doing so violates the Constitutional Specification or Product Proof requirements.

This is a design preference for experience work — not a new framework and not Spec authority.
