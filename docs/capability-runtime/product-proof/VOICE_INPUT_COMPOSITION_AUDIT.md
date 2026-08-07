# Voice Input — Composition Audit (P16)

## 1. Is Voice independently useful?

**Yes.** Users can press the mic and speak desktop requests without typing.  
Status help (“Can you hear me?”) works through Conversation.

## 2. Which providers compose today?

Voice composes with **every** existing provider by feeding the same Conversation → Intent → Kernel Operator path:

| Provider | Example spoken utterance |
| --- | --- |
| Applications | “Open Notepad.” |
| Windows | “Bring Chrome to the front.” |
| Notifications | “Show a notification: Restore finished.” |
| Browser | “Open ChatGPT.” / “Open GitHub beside Cursor.” |
| Screenshots | “Take a screenshot.” / “Take a screenshot and copy it.” |

Voice never calls these providers. Composition is through Conversation text only.

## 3. Which future capabilities become possible?

| Future | Via Voice |
| --- | --- |
| Files | “Open my downloads folder.” |
| Memory | “What was I doing yesterday?” |
| Automation | “Run my morning setup.” |
| Workspace Intelligence | Spoken check-ins |
| Terminal | Spoken governed recipes (later) |

## 4. Did Voice introduce architectural coupling?

**No.**

- No Capability Runtime Voice domain that performs desktop ops  
- No provider-to-provider calls  
- Kernel Operator unchanged for recognition  
- Mic UI is Conversation chrome only  

---

## Capability Independence Rule (satisfied)

1. **Independent** — mic + speak works without other providers  
2. **Composable** — every provider gains spoken entry for free  
3. **Invisible** — users see Conversation, not “Voice Provider”  
