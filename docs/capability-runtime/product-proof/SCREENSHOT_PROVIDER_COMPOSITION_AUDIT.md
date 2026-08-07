# Screenshot Provider — Composition Audit (P15)

## 1. Is Screenshot independently useful?

**Yes.** Users can ask directly: “Take a screenshot.” / “Capture my desktop.” / “Screenshot Cursor.”  
No other provider is required for still capture, PNG save, or image clipboard copy.

## 2. Which providers compose today?

| Pair | Composition | Authority |
| --- | --- | --- |
| Screenshots alone | Capture / save / copy | Screenshot Provider |
| Screenshots + Window (language) | Named window target (“Screenshot Chrome”) resolved inside ScreenshotPort by title | Same provider; Window Provider not called |
| Screenshots + Operator | `capture_and_copy` | Kernel Operator only |

Image clipboard is implemented **inside** `ScreenshotPort` (arboard), not via ClipboardProvider — preserving “providers never call providers.”

## 3. Which future capabilities become possible?

| Future | Depends on Screenshot |
| --- | --- |
| OCR of a capture | Screenshot → OCR domain |
| Annotate / edit | Screenshot → editor surface |
| Memory “restore what I saw” | Screenshot + Memory |
| Notify after capture | Screenshot + Notifications (Operator) |
| Voice “screenshot this” | Voice → same Intent |
| Automation recipes | Automation → Operator plans including capture |
| Workspace Intelligence “compare layouts” | Screenshot stills as evidence |

## 4. Did Screenshot introduce architectural coupling?

**No.**

- Conversation never invokes Screenshot Provider  
- Screenshot Provider never calls other providers  
- Clipboard image path stays in the port (commodity detail)  
- Kernel Operator remains sole orchestration authority  

If a future program needs text Clipboard + image Screenshot together, composition belongs in the Operator — not provider-to-provider calls.

---

## Capability Independence Rule (satisfied)

1. **Independent** — direct user ask works  
2. **Composable** — enables OCR/Memory/Automation/Voice futures; `capture_and_copy` today  
3. **Invisible** — Conversation uses desktop language only  
