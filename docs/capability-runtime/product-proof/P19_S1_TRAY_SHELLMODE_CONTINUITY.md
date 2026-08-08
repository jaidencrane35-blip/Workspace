# P19.S1 — Tray Show ↔ ShellMode Continuity

| Field | Value |
| --- | --- |
| **Program** | Product Optimization — Tray Show ↔ ShellMode Continuity |
| **Prior audit** | `docs/capability-runtime/product-proof/P19_A1_PRODUCTION_PRODUCT_PROOF_AUDIT.md` |
| **Date** | 2026-08-08 |
| **Commit** | _(stamped on ship)_ |
| **Branch** | `v2-dev` |

---

## 1. Files modified

| File | Change |
| --- | --- |
| `app/src-tauri/src/tray.rs` | Emit `workspace-show-conversation` before native show |
| `app/src/lib/shellRuntime.ts` | `SHOW_CONVERSATION_EVENT` constant |
| `app/src/lib/shellWindows.ts` | `restoreConversationShell` + tray listener install |
| `app/src/main.tsx` | Install restore listener on both webviews |
| `app/src/components/operator/DesktopOperator.tsx` | Reuse restore path for Form A click |
| Verifier + tests + handoff/report | Regression + docs |

**Unchanged:** ShellMode state machine, tray menu language, P17/P18 Moments authorities, Conversation semantics.

---

## 2. Before / after lifecycle timeline

```text
BEFORE
Collapse → ShellMode 0 → main hidden, operator shown
Tray Show → main.show()/focus only
React still mode 0 → empty stub (“window came back, Workspace did not”)

AFTER
Collapse → ShellMode 0 → main hidden, operator shown
Tray Show → emit workspace-show-conversation
         → restoreConversationShell: save mode 1 + sync React + applyShellMode(1)
         → Conversation Form + prior App navigation / Moments session (still mounted)
```

| Step | Before | After |
| --- | --- | --- |
| Collapse | Form A | Form A (unchanged) |
| Tray Show native | Window visible | Window visible |
| Tray Show ShellMode | Stays 0 | Restores 1 |
| Conversation Form | Missing / empty | Restored |
| App view / Moments session | Mounted but hidden | Still mounted; Form returns |

---

## 3. Validation

```text
node scripts/verify-tray-shellmode.mjs
node scripts/verify-tray-lifecycle.mjs
pnpm typecheck
pnpm --filter @workspace/tests exec vitest run tray-shellmode
```

---

## 4. Remaining follow-ups (not in this slice)

- C2 close-vs-hide shutdown policy
- toolDock restore after Collapse (explicitly cleared on Collapse today)
- Empty Conversation first-session cue
- A2 signing → B2 updater
- Owner Voice Product Proof

---

## 5. Stop

One vertical slice complete. Await Product Owner review.
