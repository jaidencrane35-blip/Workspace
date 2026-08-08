# P20.S1 — Empty Conversation First-Session Cue

| Field | Value |
| --- | --- |
| **Program** | Release Candidate — Empty Conversation First-Session Cue |
| **Prior audit** | `docs/capability-runtime/product-proof/P20_A1_RELEASE_CANDIDATE_PRODUCT_PROOF.md` |
| **Date** | 2026-08-08 |
| **Commit** | _(stamped on ship)_ |
| **Branch** | `v2-dev` |

---

## 1. Files modified

| File | Change |
| --- | --- |
| `app/src/components/operator/OperatorRoot.tsx` | First-session invite + composer placeholder; gated visibility |
| `app/src/components/operator/VoiceMicButton.tsx` | `onCaptureActiveChange` for cue suppression |
| `app/src/App.css` | Quiet invite + placeholder styling |
| Verifier + tests + handoff/report | Regression + docs |

**Unchanged:** Voice F10, Soft Send, P17 working-state, P18 Moments continuity, P19 tray ShellMode.

---

## 2. Before / after first-launch behaviour

| Moment | Before | After |
| --- | --- | --- |
| Fresh Conversation | Empty waiting void; `placeholder=""` | Quiet invite: “Say or type what you need.” + placeholder “Say or type…” |
| After first message | — | Invite gone |
| Busy / Voice capture / Moments active / draft | Silent void | No invite (waiting shell only) |

---

## 3. Validation

```text
node scripts/verify-conversation-first-session.mjs
pnpm typecheck
pnpm --filter @workspace/tests exec vitest run conversation-first-session
```

---

## 4. Remaining follow-ups (not in this slice)

- C2 close-vs-hide shutdown policy
- toolDock restore after Collapse
- Dock ↔ Conversation gravity
- Owner Voice Product Proof
- Signing / updater

---

## 5. Stop

One vertical slice complete. Await Product Owner review.
