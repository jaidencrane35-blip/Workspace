# P17.S4 — Conversation Working-State Continuity

| Field | Value |
| --- | --- |
| **Program** | Product Optimization — Conversation Working-State Continuity |
| **Prior audit** | `docs/ui/P17_A4_RESPONSIVE_INTERACTION_AUDIT.md` |
| **Date** | 2026-08-08 |
| **Commit** | _(stamped on ship)_ |
| **Branch** | `v2-dev` |

---

## 1. Files modified

| File | Change |
| --- | --- |
| `app/src/components/operator/OperatorRoot.tsx` | Busy from Send; working ack bubble; `holdBusy` reveal; `toolBusy` bridge |
| `app/src/App.tsx` | `toolBusy={busy}` Moments→Conversation bridge |
| `app/src/lib/operator/intelligence.ts` | `onWorking` before support-bundle IPC |
| `app/src/lib/operator/index.ts` | Export working-notify types |
| `docs/ui/WORKSPACE_INTERACTION_LANGUAGE.md` | Thinking/working row |
| `docs/ui/WORKSPACE_PRODUCT_QUALITY_STANDARD.md` | Thinking/executing row |
| `scripts/verify-conversation-working-state.mjs` | Regression verifier |
| `tests/conversation-working-state.test.ts` | Source continuity tests |

**Unchanged:** Voice F10, Soft Send, S1 failure compose, S2 success cohesion, S3 agency keys, IPC sequencing, cancellation semantics.

---

## 2. Before / after execution timeline

### Capability / support turn
| Phase | Before | After |
| --- | --- | --- |
| Send | User bubble; `busy=false` | User bubble + **Working on that…**; `busy=true` |
| Real IPC | Composer live; silence | Composer locked; working / **Creating…** (support) |
| Reply reveal | `busy=true` starts here | `holdBusy` stream; busy already true |
| Idle | After stream | After stream + turn `finally` |

### Moments App IPC
| Before | After |
| --- | --- |
| App busy; Conversation composer live | App busy → `toolBusy` → Conversation composer/mic locked |

---

## 3. Validation

```text
node scripts/verify-conversation-working-state.mjs
pnpm typecheck
pnpm --filter @workspace/tests exec vitest run conversation-working-state
```

---

## 4. Remaining follow-ups (not in this slice)

- Moments “Saving…” / “Restoring…” button labels while App busy
- Soften post-hoc `streamText` once working ack owns the wait
- Owner Cancel for Conversation / Moments IPC
- ActiveMoment ambient restoring on App path

---

## 5. Stop

One vertical slice complete. Await Product Owner review.
