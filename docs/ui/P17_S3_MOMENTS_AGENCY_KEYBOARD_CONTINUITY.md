# P17.S3 — Moments Agency Keyboard Continuity

| Field | Value |
| --- | --- |
| **Program** | Product Optimization — Moments Agency Keyboard Continuity |
| **Prior audit** | `docs/ui/P17_A3_INTERACTION_CONTINUITY_AUDIT.md` |
| **Date** | 2026-08-08 |
| **Commit** | _(stamped on ship)_ |
| **Branch** | `v2-dev` |

---

## 1. Files modified

| File | Change |
| --- | --- |
| `app/src/hooks/useAgencyKeyboard.ts` | Shared Esc / Enter / focus-in / Conversation focus-return |
| `app/src/components/objects/ContinuePreviewObject.tsx` | Preview agency keyboard + `role="dialog"` |
| `app/src/components/ResumeContextPanel.tsx` | Delete confirm keyboard; inline preview when no `expandHost` |
| `app/src/components/operator/OperatorRoot.tsx` | `id="workspace-conversation-input"` for focus return |
| `docs/ui/WORKSPACE_INTERACTION_LANGUAGE.md` | §6.1 Moments agency keys |
| `scripts/verify-moments-agency-keyboard.mjs` | Regression verifier |
| `tests/moments-agency-keyboard.test.ts` | Source continuity tests |
| `package.json` / `scripts/sync-project-health.mjs` | Wire verifier |

**Unchanged:** Voice F10, Soft Send, S1 failure compose, S2 success cohesion, approve/delete IPC logic, pointer handlers.

---

## 2. Before / after interaction flow

### Restore preview
| Before | After |
| --- | --- |
| Pointer-only Approve / Not now; often no card without expand host | Focus lands on Approve; Enter approves; Esc = Not now; focus → Conversation |
| Preview missing when `expandHost` null | Inline preview card so agency is reachable on App path |

### Delete confirm
| Before | After |
| --- | --- |
| Cancel / Delete click only | Focus on Delete permanently; Enter deletes; Esc = Cancel; focus → Conversation |

---

## 3. Accessibility considerations

- Agency cards expose `role="dialog"` + `aria-label`.
- Focus moves to the primary control on open (not a permanent trap).
- Tab / Shift+Tab remain native between controls.
- Space still activates the focused button (Enter reserved for primary per Interaction Language).
- Textareas / inputs / `contenteditable` / `<summary>` are not intercepted.
- Voice capture keeps capture-phase Esc/Enter ownership (agency uses bubble phase).
- Conversation composer remains the focus return target (Product Gravity).

---

## 4. Validation

```text
node scripts/verify-moments-agency-keyboard.mjs
pnpm --filter @workspace/app exec tsc --noEmit -p tsconfig.json
pnpm --filter @workspace/tests exec vitest run moments-agency-keyboard
```

---

## 5. Remaining follow-ups (not in this slice)

- Unify App vs OperatorRoot busy channels (A3 Track A)
- ActiveMomentProvider on default App tree (structural)
- Owner-visible cancel for long Moments IPC
- Sticky error-banner Esc dismiss

---

## 6. Stop

One vertical slice complete. Await Product Owner review.
