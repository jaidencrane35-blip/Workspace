# P18.S2 — Moments Tool Session Continuity

| Field | Value |
| --- | --- |
| **Program** | Product Optimization — Moments Tool Session Continuity |
| **Prior audit** | `docs/capability-runtime/product-proof/P18_A2_RUNTIME_CONTINUITY_AUDIT.md` |
| **Date** | 2026-08-08 |
| **Commit** | _(stamped on ship)_ |
| **Branch** | `v2-dev` |

---

## 1. Files modified

| File | Change |
| --- | --- |
| `app/src/components/MomentsToolSession.tsx` | Continuity holder for Resume/Save flow above remount |
| `app/src/App.tsx` | Mount session provider; preserve focus among Moments views |
| `app/src/components/ResumeContextPanel.tsx` | Resume step/preview/draft via session |
| `app/src/components/SaveContextPanel.tsx` | Save draft/step via session; inline write when no expandHost |
| `app/src/components/ActiveMoment.tsx` | Keep pin across Home/Continue/Save |
| Verifier + tests + handoff/report | Regression + docs |

**Unchanged:** P17.S1–S5, P18.S1 browse list authority, agency keyboard, Conversation semantics.

---

## 2. Before / after runtime behaviour

| Situation | Before | After |
| --- | --- | --- |
| Save with Moments + no expandHost | Blank attach shell | Inline Leave a note form |
| Preview → Home → Continue | Preview lost | Preview session retained |
| Save draft → Continue → Save | Draft lost | Draft retained |
| Home ↔ Continue ↔ Save focus | Cleared / pin dropped | Focus + pin preserved |
| Explicit Back to browse / Save another | — | Intentional reset (`resetResume` / `resetSave`) |

---

## 3. Validation

```text
node scripts/verify-moments-tool-session.mjs
node scripts/verify-moments-browse-truth.mjs
pnpm typecheck
pnpm --filter @workspace/tests exec vitest run moments-tool-session
```

---

## 4. Remaining follow-ups (not in this slice)

- Tray Show ↔ React ShellMode restore
- toolDock restore after Collapse
- Cognitive/composition on App path
- Full PersistentMomentStage ambient

---

## 5. Stop

One vertical slice complete. Await Product Owner review.
