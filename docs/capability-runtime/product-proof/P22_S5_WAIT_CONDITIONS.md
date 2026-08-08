# P22.S5 — Wait Conditions (C-VER-003)

| Field | Value |
| --- | --- |
| **Capability ID** | **C-VER-003** Wait Conditions |
| **Program** | P22.S5 — Wait Conditions |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Engineering milestone + Owner Product Proof harness |
| **Authority** | Workspace Capability Atlas v2.0 |
| **Depends on** | C-OBS-003/004, C-ACT-004/005 (eng); Completion Contract |
| **Not** | Retry engine · agent loop · OCR · VLM · workflows · File Provider |

---

## 1. Objective

Deterministic bounded wait for an observable desktop condition before continuing composition:

```text
Action → Wait for expected state → Observe → Continue OR truthful failure
```

Never invent success because a prior click/type succeeded. Every wait uses a bounded timeout — never wait indefinitely.

---

## 2. Pipeline

```text
Conversation → Intent (winWaitCondition)
  → Kernel Operator composition `window.wait_condition`
       WaitCondition [→ FindControl when control_available]
  → Completion Contract (completed | partial | failed)
```

Existing `window.click_control` / `window.type_control` compositions remain unchanged.

---

## 3. Condition set (bounded)

| Condition | Meaning |
| --- | --- |
| `control_available` | Named control becomes discoverable |
| `control_gone` | Named control is no longer discoverable |
| `window_available` | Window resolves |
| `window_active` | Resolved window is the active/foreground window |

Timeout default **2s**, max **8s**, poll **50ms**.

---

## 4. Readiness after engineering

| Capability | Overall |
| --- | --- |
| C-VER-003 | **57%** (Arch/Deps/Impl/Ver 100%; PP/Trusted/Production 0%) |

---

## 5. Product Proof (Owner)

Launch Workspace once (Owner only — engineering must not auto-launch):

1. Open Notepad.  
2. “Wait for Edit in Notepad” → expect completed (already true) or honest timeout.  
3. “Wait until NoSuchControlZZZ appears in Notepad” → must timeout / fail truthfully (never invent).  
4. Optionally after “Click File in Notepad”: “Wait for Save in Notepad” — wait success is independent of click success.  
5. Confirm click/type still work unchanged.

---

## 6. Validation

| Check | Expected |
| --- | --- |
| Kernel wait_condition tests | pass |
| `tests/wait-conditions.test.ts` | pass |
| `node scripts/verify-wait-conditions.mjs` | ok |
| Existing `verify-desktop-control-interaction` | ok |
| `pnpm typecheck` / `pnpm test` | pass |
