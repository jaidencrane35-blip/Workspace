# Future Input Architecture (Design Only)

| Field | Value |
| --- | --- |
| **Status** | Architecture — **not implemented** |
| **Program** | P5 two-form shell (Operator ⇄ Conversation) |
| **Law** | Keyboard / Voice / Screenshot / Clipboard / Drag-drop feed **one** intent pipeline |

---

## Principle

Shell modes own presentation.  
The **intent bridge** (and later ModelProvider under permission) owns interpretation.  
Inputs are adapters — never separate products.

```
[Keyboard]──┐
[Voice]─────┼──► IntentEnvelope ──► resolveIntent / future NL ──► Shell + CommandPipeline
[Screenshot]┤
[Clipboard]─┤
[DragDrop]──┘
```

---

## IntentEnvelope (future)

| Field | Purpose |
| --- | --- |
| `source` | `keyboard` \| `voice` \| `screenshot` \| `clipboard` \| `dragdrop` |
| `utterance` | Normalized text when available |
| `attachments` | Optional media / paths (permission-gated) |
| `receivedAt` | Timestamp |
| `sessionMode` | Shell mode at receipt |

All sources produce the same envelope. No parallel “voice product” or “vision product”.

---

## Shell readiness (now)

| Requirement | Status |
| --- | --- |
| Stable Forms A–B (Operator ⇄ Conversation) | Implemented (P5) |
| Conversation as default intent surface | Implemented |
| Zero-Trap recovery | Implemented |
| Permission before desktop mutation | Existing CommandPipeline |
| Voice / Vision capture | **Not** in this program |

---

## Non-goals (this program)

- No speech recognition
- No screenshot OCR
- No clipboard watchers
- No drag-drop handlers beyond documentation
