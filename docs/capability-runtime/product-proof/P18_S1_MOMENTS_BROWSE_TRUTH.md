# P18.S1 — Moments Browse Truth

| Field | Value |
| --- | --- |
| **Program** | Product Optimization — Moments Browse Truth |
| **Prior audit** | `docs/capability-runtime/product-proof/P18_A1_PRODUCT_PROOF_REGRESSION_AUDIT.md` |
| **Date** | 2026-08-08 |
| **Commit** | _(stamped on ship)_ |
| **Branch** | `v2-dev` |

---

## 1. Files modified

| File | Change |
| --- | --- |
| `app/src/App.tsx` | Mount existing `ActiveMomentProvider` on App product path |
| `app/src/components/ActiveMoment.tsx` | Expose `moments` (full `list_saved_contexts` list) |
| `app/src/components/HomeWorkspacePanel.tsx` | Browse via `useActiveMoment().moments` — no separate fetch |
| `app/src/components/ResumeContextPanel.tsx` | Empty on `moments.length === 0`; Continue list matches Home |
| `scripts/verify-moments-browse-truth.mjs` | Regression verifier |
| `tests/moments-browse-truth.test.ts` | Source continuity tests |

**Unchanged:** P17.S1–S5 behaviour, agency keyboard, Save/Restore IPC, Conversation, toast infrastructure.

---

## 2. Before / after browse behaviour

| Surface | Before | After |
| --- | --- | --- |
| Home | Own `list_saved_contexts` fetch | Same ActiveMoment `moments` list |
| Continue | `!primary` stub → false “Nothing to continue yet” | Empty only when `moments.length === 0` |
| Continue with Moments | Often empty while Home listed them | Same Moments list; open preview from item |

---

## 3. Validation

```text
node scripts/verify-moments-browse-truth.mjs
pnpm typecheck
pnpm --filter @workspace/tests exec vitest run moments-browse-truth
```

---

## 4. Remaining follow-ups (not in this slice)

- Dock Conversation-still-primary visual contract
- Ambient / PersistentMomentStage on App path
- Empty Conversation first-session cue
- Owner Voice Product Proof

---

## 5. Stop

One vertical slice complete. Await Product Owner review.
