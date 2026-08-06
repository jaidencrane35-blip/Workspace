# Workspace UI Architecture Specification
## Authoritative presentation law (P8)

| Field | Value |
| --- | --- |
| **Status** | Authoritative — extend, do not redesign |
| **Program** | P8 Final Product Presentation Refoundation |
| **Date** | 2026-08-07 |
| **Preserves** | Accepted shell Form A/B (P5–P6); compact sizing (P7) |
| **Subordinate to** | Architectural Constitution V2 · Product Constitution |
| **Companion docs** | Product Presentation · Component Ownership · Window Lifecycle · Layer Ownership |

---

## 0. Phase 1 answer (blocking product feel)

**What is the single biggest thing preventing Workspace from feeling finished today?**

> The presentation layer still behaves like a framed utility around conversation — chrome, teaching copy, and tool docks compete with the conversation as the product. There was no stable presentation authority, so each program re-polished ad hoc instead of converging.

Everything in P8 either **specifies** that authority or **moves the runtime toward conversation-defining presentation**.

---

## 1. Product identity (presentation)

Workspace **is** a conversational desktop operating layer.

| | |
| --- | --- |
| Conversation | Interface |
| Desktop operation | Capability |
| Trust | Product |

Workspace is **not** a chatbot, dashboard, launcher, or menu system.

**Hard rule:** Capabilities never create independent primary interfaces. They enter through Conversation and may open **satellites** around it.

---

## 2. Permanent product layers

Every future feature belongs to **exactly one** layer.

### Layer 1 — Desktop Operator
Persistent desktop presence. Tiny, movable, native, recoverable. Independent window (`operator`). Not a resized conversation.

### Layer 2 — Conversation
Primary interaction surface. Product begins and ends here. Window (`main`). Thread never replaced by a satellite.

### Layer 3 — Capability Runtime
Desktop operations (clipboard, apps, files, screenshots, voice, automation, integrations). **No competing UI.** Effects + optional satellites only.

### Layer 4 — Workspace Intelligence
Memory, intent, personalization, companion behaviour, workflow adaptation, governed learning. Never owns shell chrome.

### Engineering tracks (map to layers)

| Track | Layers | Owns |
| --- | --- | --- |
| **A — Shell** | 1–2 | Operator, conversation shell, window lifecycle, native feel, visual polish |
| **B — Capability Runtime** | 3 | Desktop operations; plugs into Conversation |
| **C — Intelligence** | 4 | Memory/intent/adaptation; plugs into capabilities |

Neither B nor C may redesign the shell.

---

## 3. Window model

```
Form A: Desktop Operator (only Workspace surface)
Form B: Conversation Window
         ├─ Conversation band (permanent while Form B)
         └─ Optional satellite dock (Expanded Workspace *presentation*)
```

| Concept | Meaning |
| --- | --- |
| **Shell forms** | Exactly two: Operator (0) ⇄ Conversation (1). Accepted. Do not reopen. |
| **Expanded Workspace** | Presentation state **inside Form B**: conversation + satellite. **Not** a third shell form. |
| **Conversation never moves** | Thread stays the intent channel; satellites appear beside/around it. |
| **One presentation at a time** | Operator XOR Conversation window visible. |

Growth rule: the application grows **around** conversation; conversation does not navigate away.

---

## 4. Surface catalogue

### 4.1 Desktop Operator (Layer 1)
| Rule | Law |
| --- | --- |
| Appearance | Companion mark — circular, lightweight, no mini-app frame |
| Click / double-click | Restore Conversation immediately |
| Drag | Move (after threshold; must not steal click) |
| Context menu | Not required for restore |
| Exit | Via Conversation Exit (or future tray) — not required for open |

### 4.2 Conversation Window (Layer 2)
| Rule | Law |
| --- | --- |
| Default size | Compact productivity (~340×480); clamp to work area |
| Chrome | Minimal — brand whisper, Collapse, Exit; no Expand, no Settings catalogue |
| Transcript | Visual hero — defines the window |
| Composer | Anchored bottom; quiet |
| Close / Collapse | → Form A (never Exit) |
| Exit | Ends process |

### 4.3 Expanded Workspace (presentation, Form B + dock)
| Rule | Law |
| --- | --- |
| Opens when | Conversation requests a capability surface (Save, Continue, Guide, …) |
| Layout | Conversation remains primary column; satellite secondary |
| Closes when | User dismisses satellite / Collapse / return to Operator |
| Never | Replaces conversation; never a capability launcher grid |

### 4.4 Developer surfaces
| Surface | Visibility |
| --- | --- |
| Repository Health | Developer mode only (gesture / intent) |
| Evidence dashboard | Opt-in developer only — never auto-mount for review |
| Capability evolution backlog | Developer Health panel |

### 4.5 Settings
No Settings catalogue surface in the shell. Trust limits → Guide via conversation. Shell actions → Collapse / Exit.

### 4.6 Evidence
Developer-only. Not part of product identity.

---

## 5. Visibility law — what appears / never appears

### Appears (product)
- Desktop Operator companion
- Conversation transcript + composer
- Collapse / Exit (quiet)
- Capability satellites **when earned by conversation**
- Permission / plan detail when an action requires it

### Never appears (product)
- Capability menus / suggested-prompt carousels
- Onboarding tours
- Expand-as-shell-mode controls
- Settings preference catalogues
- Fake AI / invented capabilities
- Teaching docks (“try Save, Continue…”) as default chrome
- Terminal / Cargo / Vite as part of UX
- Evidence overlays for Product Owner review by default

---

## 6. Interaction rules

| Intent | Result |
| --- | --- |
| Click Operator | Form B Conversation |
| Collapse / Close conversation | Form A Operator |
| Exit | Process end |
| Ask Save / Continue / Guide… | Satellite dock in Form B + honest reply |
| Unknown capability | Truthful refusal — no invented UI |

---

## 7. Sizing & positioning

| Surface | Rule |
| --- | --- |
| Operator | Fixed companion size (~44×44); durable position |
| Conversation default | Compact productivity; migrate legacy oversized defaults |
| Conversation resize | Remember within soft max; never exceed work area |
| First open | Prefer lower-right work area — not centered dominance |
| Multi-monitor | Clamp to current work area; restore intelligently |

---

## 8. Transition & animation rules

| Transition | Feel |
| --- | --- |
| Operator → Conversation | Instant show of `main`; hide `operator` |
| Conversation → Operator | Instant hide of `main` (off taskbar); show `operator` |
| Satellite open/close | Dock within Form B; conversation column stays |
| Animations | Prefer reliability over flourish; no theatrical resize-as-collapse |

Collapse is a **mode switch**, never a resize of conversation into the operator.

---

## 9. Visual design rules

Owner law: **the application should visually disappear around the conversation.**

For every element: *Does this improve conversation?* If not → remove, move, hide, or quiet.

| Prefer | Avoid |
| --- | --- |
| Transparent / quiet chrome | Heavy title bars and tool frames |
| Transcript as hero | Dashboard density |
| Soft message treatment | Card grids and bubble stacks as identity |
| Native Windows decorations on `main` | Custom OS-chrome cosplay that fights Windows |

---

## 10. Component ownership (summary)

See `COMPONENT_OWNERSHIP_MAP.md`.

| Component | Layer | Track |
| --- | --- | --- |
| `DesktopOperator` | 1 | A |
| `OperatorRoot` (conversation shell) | 2 | A |
| `shellRuntime` / `shellWindows` / `shellStateMachine` | 1–2 | A |
| Intent bridge | 2→3 bridge | A/B boundary |
| Product Proof satellites (Save/Continue/…) | 3 surfaces | B (hosted by A shell) |
| `RepositoryHealthPanel` | Dev | A (developer) |
| Capability evolution | 4 proposals | C |

---

## 11. Stability contract

Future execution programs **extend** this specification.

They must **not**:
- Reintroduce four-mode shell forms
- Create capability-primary UIs
- Redesign Operator/Conversation lifecycle without Owner rejection of this law

They **should**:
- Add Track B capabilities into Conversation → Capability Runtime
- Add Track C intelligence behind permission and honesty
- Polish Track A within these rules

---

## 12. Constitutional validation

| Check | Result |
| --- | --- |
| Product Constitution P1–P4 | Conversation-first; no catalogue; companion layer |
| Arch Const. honesty / permission | Satellites for consent; no fabricated capabilities |
| Accepted P6 shell | Form A/B preserved |
| Accepted P7 compact | Sizing rules preserved and referenced |
