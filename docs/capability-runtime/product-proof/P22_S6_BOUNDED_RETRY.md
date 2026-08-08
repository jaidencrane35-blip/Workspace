# P22.S6 — Bounded Retry (C-VER-002)

| Field | Value |
| --- | --- |
| **Capability ID** | **C-VER-002** Retry |
| **Program** | P22.S6 — Bounded Retry |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Engineering milestone + Owner Product Proof harness |
| **Authority** | Workspace Capability Atlas v2.0 |
| **Depends on** | C-ACT-004/005; C-VER-003; C-VER-001 Completion Contract |
| **Not** | Agent loop · planner · workflows · OCR · File Provider · indefinite polling |

---

## 1. Objective

When an authorized click/type verification indicates a **retryable** transient miss, Workspace may repeat the **same** action once after a bounded Wait Conditions pause — then report completed / partial / failed truthfully.

Never invent success. Never invent a new goal. Never retry forever.

---

## 2. Pipeline

```text
Conversation → Intent (click_control | type_control)
  → Kernel Operator
       Locate → Act → Verify
         └─ if retryable → Wait (C-VER-003) → Locate → Act → Verify  (max 2 attempts)
  → Completion Contract
```

Existing happy-path click/type (first attempt success) performs no wait and no retry.

---

## 3. Retry policy

| Class | Examples | Behaviour |
| --- | --- | --- |
| Retryable | `control_not_found` (first miss), `control_click_unverified`, `control_type_unverified`, window `not_found` | Wait → same action again |
| Non-retryable | `need_control_name`, `need_text`, `control_click_failed`, `control_type_failed`, auth/unsupported | Stop immediately |

Bound: **MAX_INTERACTION_ATTEMPTS = 2**. Wait duration: **400ms** via C-VER-003.

---

## 4. Readiness after engineering

| Capability | Overall |
| --- | --- |
| C-VER-002 | **57%** (Arch/Deps/Impl/Ver 100%; PP/Trusted/Production 0%) |

---

## 5. Product Proof (Owner)

Launch Workspace once (Owner only — engineering must not unlock or auto-launch):

1. Open Notepad.  
2. “Click File in Notepad” → expect completed on first try (no visible thrash).  
3. “Click NoSuchControlZZZ in Notepad” → failed after at most one wait/retry; never invent success.  
4. “Type hello into Edit in Notepad” → still works.  
5. “Wait for Edit in Notepad” → Wait Conditions still works unchanged.

---

## 6. Validation

| Check | Expected |
| --- | --- |
| Kernel `operator::retry` tests | pass |
| `tests/bounded-retry.test.ts` | pass |
| `node scripts/verify-bounded-retry.mjs` | ok |
| `verify-desktop-control-interaction` / `verify-wait-conditions` | ok |
| `pnpm typecheck` / `pnpm test` | pass |
