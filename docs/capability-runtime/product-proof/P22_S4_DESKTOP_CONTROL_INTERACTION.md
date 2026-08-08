# P22.S4 — Desktop Control Interaction (C-ACT-004 + C-ACT-005)

| Field | Value |
| --- | --- |
| **Capability IDs** | **C-ACT-004** Mouse Click · **C-ACT-005** Keyboard Input |
| **Program** | P22.S4 — Desktop Control Interaction (Capability Pair) |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Coordinated paired engineering milestone (§0.1) |
| **Authority** | Workspace Capability Atlas v2.0 + Pair Rule |
| **Depends on** | C-OBS-003, C-OBS-004 (eng complete; PP parallel) |
| **Not** | OCR · OmniParser · VLMs · File Provider · workflows · agent loops |

---

## 1. Objective

Deterministic in-window interaction: Locate → Interact → Verify → truthful report. never invent success. Never intentionally click the wrong control.

---

## 2. Pair Rule compliance

| Requirement | Evidence |
| --- | --- |
| Meaningful value together | Click + type = usable control surface |
| Same dependencies | C-OBS-003 / C-OBS-004 |
| Shared verification | `verify-desktop-control-interaction` |
| Joint Product Proof | This document — one Owner session |
| Separate IDs / Readiness / records | Atlas C-ACT-004 and C-ACT-005 |

---

## 3. Pipeline

```text
Conversation → Intent (winClickControl | winTypeControl)
  → Kernel Operator composition
       FindControl → InvokeControl|SetControlValue → FindControl
  → Completion Contract compose (completed | partial | failed)
```

---

## 4. Readiness after engineering

| Capability | Overall |
| --- | --- |
| C-ACT-004 | **57%** (Arch/Deps/Impl/Ver 100%; PP/Trusted/Production 0%) |
| C-ACT-005 | **57%** (same) |

---

## 5. Product Proof (Owner — joint)

Launch Workspace once:

1. Open Notepad.  
2. “Find the File menu in Notepad” (discovery still works).  
3. “Click File in Notepad” → expect click completed or honest fail.  
4. “Type hello into Edit in Notepad” → expect typed + confirmed, or honest refuse.  
5. “Click NoSuchControlZZZ in Notepad” → must not invent success.

Engineering must not auto-launch Workspace.

---

## 6. Validation

| Check | Expected |
| --- | --- |
| Kernel `window_invoke_and_set_control_through_router` | pass |
| `tests/desktop-control-interaction.test.ts` | pass |
| `node scripts/verify-desktop-control-interaction.mjs` | ok |
| `pnpm typecheck` / `pnpm test` | pass |
