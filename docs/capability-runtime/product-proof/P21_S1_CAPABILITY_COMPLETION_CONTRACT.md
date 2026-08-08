# P21.S1 — Capability Completion Contract

| Field | Value |
| --- | --- |
| **Program** | P21.S1 — Capability Completion Contract |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Bounded engineering slice — Kernel compose discipline |
| **Authority** | P21.A1 Goal Completion Audit |
| **Reference path** | `browser.open_beside` |
| **Max layer** | Kernel Operator plan + compose |
| **Architecture** | Unchanged — existing Browser + Window providers only |

---

## 1. Objective

Bind Intent → Kernel execution → Conversation reporting to the **same observable desktop outcome**.  
Never report a goal complete because the first provider succeeded.

---

## 2. Files modified

| Path | Change |
| --- | --- |
| `packages/kernel/src/operator/plan.rs` | `open_beside` plans Open → Focus beside → Snap left → Snap right → Enumerate |
| `packages/kernel/src/operator/compose.rs` | `compose_completion` — completed / partial / failed; layout verify; no overclaim |
| `packages/kernel/src/operator/mod.rs` | Plan regression test for multi-step beside |
| `app/src/lib/operator/intentMap.ts` | Document snap as window-title hint (edges owned by Kernel) |
| `scripts/verify-capability-completion.mjs` | Machine check |
| `docs/capability-runtime/product-proof/P21_S1_CAPABILITY_COMPLETION_CONTRACT.md` | This report |

---

## 3. Before / after execution timeline

### Before

```text
Goal: open site beside Cursor
  → Browser Open
  → Conversation: "Opened beside Cursor."   ← false completion
  → Layout never attempted
```

### After (Capability Completion Contract)

```text
Goal: open site beside Cursor
  → Browser Open
  → Window Focus (locate Cursor)
  → Window Snap Cursor left
  → Window Snap opened surface right
  → Window Enumerate (verify side-by-side)
  → Conversation:
       completed → "Opened beside Cursor."
       partial   → what actually finished (open / find / arrange)
       failed    → could not open
```

---

## 4. Validation

- `browser.open_beside` plans ≥5 steps including Snap + Enumerate.
- Compose never emits “Opened beside…” unless layout verification passes.
- Partial paths report intermediate truth (opened but couldn’t find / couldn’t finish layout / couldn’t confirm).
- Single-step capabilities unchanged (compose_completion only when `composition_id` matches).
- Diagnostics / provider surfaces unchanged.

---

## 5. Other composed capabilities inheriting the Contract

| Composition | Completion reporting |
| --- | --- |
| `browser.open_beside` | Full contract + layout verify (reference) |
| `browser.open_foreground` | Completed only if Open + Focus both ok; else partial |
| `window.focus_minimize` | Completed only if Focus + Minimize both ok; else partial |
| `screenshots.capture_and_copy` | Completed only if Capture + Copy both ok; else partial |
| `app.open_maximize` | Completed only if open + maximize both ok; else partial |

---

## 6. Compliance

| Constraint | Posture |
| --- | --- |
| No new planning engine / framework | Observed |
| Existing providers only | Observed |
| No Spec / constitutional redesign | Observed |
| Single vertical slice | Observed |
| Release Hold | Remains |

---

## Stop

P21.S1 complete. Do not begin another engineering program.
