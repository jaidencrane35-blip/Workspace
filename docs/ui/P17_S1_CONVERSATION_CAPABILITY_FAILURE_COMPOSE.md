# P17.S1 — Conversation Capability Failure Compose

| Field | Value |
| --- | --- |
| **Program** | Product Optimization — Conversation Capability Failure Compose |
| **Prior audit** | `docs/ui/P17_A1_UNIFIED_ERROR_EXPERIENCE_AUDIT.md` |
| **Date** | 2026-08-08 |
| **Commit** | _(stamped on ship)_ |
| **Branch** | `v2-dev` |

---

## 1. Files modified

| File | Change |
| --- | --- |
| `packages/kernel/src/operator/compose.rs` | `sanitize_owner_message`, `compose_failure_reply`; soft-fail path sanitized; unit tests |
| `packages/kernel/src/operator/mod.rs` | Export helpers; `execute_turn` composes hard fails |
| `packages/kernel/src/commands/capability_intent.rs` | Hard `Err` → log + `Ok(compose_failure_reply)`; clarify sanitized |
| `app/src/lib/operator/runtimeBridge.ts` | Transport catch never forwards `Error.message` |
| `app/src/lib/operator/index.ts` | Export `composeTransportFailureMessage` |
| `tests/operator-intelligence.test.ts` | P17.S1 transport compose checks |

---

## 2. Architectural rationale

No new messaging system. Conversation already had Operator `compose_user_reply` for soft provider outcomes. Hard failures exited as `KernelError` → IPC failure → TS `error.message`, creating a second authority.

**Fix:** Keep Kernel Operator compose as the single Owner-facing authority. Hard failures become `OperatorTurnResult { ok: false, message }` after `log::warn!` of the original `KernelError`. Transport/IPC catch uses the same Conversation voice as defense-in-depth.

Diagnostics / support bundles / logging unchanged — engineering detail remains in logs (`workspace_capability` target + console.warn on transport).

---

## 3. Before / after examples

| Scenario | Before (Conversation) | After |
| --- | --- | --- |
| Notification WinRT Err | `Notification failed: show notification: …` | `I couldn’t show that notification.` |
| Plan: requires query | `application operation requires a query` | `Which app or window did you mean?` |
| Plan: clipboard text | `clipboard write requires text` | `What text should I copy?` |
| Clarify website | `Which website should I open?` | unchanged |
| Soft provider `ok:false` with OS text | Often raw / partially stripped | Sanitized via same helper |
| IPC transport failure | `An unknown error occurred.` / PublicError text | Domain calm line / truthful refusal |
| Internal log | — | `capability intent failed … err={error:?}` retained |

---

## 4. Validation performed

```text
cargo test -p workspace-kernel compose::tests --lib -- --test-threads=1
# 6 passed

pnpm exec vitest run tests/operator-intelligence.test.ts
# (run in deliverable session)
```

---

## 5. Remaining follow-ups (not in this slice)

- Specialized tool `ws-toast` / `formatError` mapper (A1 Track A)
- Moments list silent empty on load failure
- Resume banner snake_case outcomes

---

## 6. Stop

One vertical slice complete. Await Product Owner review. Do not begin the next optimization program.
