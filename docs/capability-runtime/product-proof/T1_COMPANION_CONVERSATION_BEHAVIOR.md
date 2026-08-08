# T1 — Companion Conversation Behavior

| Field | Value |
| --- | --- |
| **Program** | Owner Product Proof — Accepted with changes (Release Hold Trigger **T1**) |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Product Proof slice — Conversation behaviour only |
| **Authority** | T2 audit recommendation: Companion Conversation Voice |
| **Max layer** | Intent Layer / Conversation reply composition |
| **Desktop execution** | Unchanged |

---

## 1. Objective

Transform Conversation from a truthful **command-help** experience into a truthful **conversational desktop companion**, without redesigning architecture or capability execution.

---

## 2. Files modified

| Path | Change |
| --- | --- |
| `app/src/lib/conversationGuidance.ts` | Companion greetings; retire invent/pretend; no default catalogues; recovery only when helpful; no consecutive suggestion repeat |
| `app/src/lib/intentBridge.ts` | Greeting uses companion reply; soft-miss / open-unknown voice |
| `app/src/lib/workspaceContext.ts` | Companion clarify voice; `open it` / window-state pronouns; `previous one` replay |
| `app/src/lib/goalResolution.ts` | Collaborative Continue / locate language |
| `app/src/lib/capabilityRegistry.ts` | Discovery/recovery without invent chorus or multi-Try menus |
| `app/src/lib/situationGoals.ts` | Continuity replies without invent disclaimers |
| `app/src/lib/semanticIntentEngine.ts` | Work-setup Continue voice |
| `app/src/lib/intentPipeline.ts` | Executable-guard matches honest open refusal |
| `app/src/lib/operator/runtimeBridge.ts` | Failure line without pretend chorus |
| `app/src/components/operator/OperatorRoot.tsx` | Moment miss restore copy |
| `packages/kernel/src/operator/compose.rs` | Owner failure sanitize → calm honesty |
| `packages/kernel/src/capability_runtime/application_provider.rs` | Unknown launch message (reply text only) |
| `tests/companion-conversation.test.ts` | New regression coverage |
| `tests/conversation-quality.test.ts` | Expectations for companion voice |
| `tests/cognitive-*.test.ts` | Recovery assertions updated |
| `scripts/verify-companion-conversation.mjs` | New verifier |
| `scripts/verify-conversation-quality.mjs` | Guards against invent chorus / catalogues |
| `scripts/verify-capability-registry.mjs` / `verify-semantic-intent.mjs` | Honest launch refusal tokens |

---

## 3. Before / after conversation examples

### Greeting

| Before | After |
| --- | --- |
| “Here when you need the desktop.” + Try “open ChatGPT”, “take a screenshot”… | “Hi — I’m here with you on the desktop.” *(no catalogue)* |

### Soft miss

| Before | After |
| --- | --- |
| “I can’t do that on the desktop yet — and I won’t invent it.” + Try windows/ChatGPT/screenshot… | “That’s outside what I can do on the desktop right now.” *(no catalogue)* |

### Files near-miss

| Before | After |
| --- | --- |
| “I can’t work with files… — and I won’t pretend I can.” + multi-command Try list | “I can’t work with files and folders yet.” + one collaborative recovery line |

### Continuity

| Before | After |
| --- | --- |
| “Open it” after Notepad often failed | Resolves to reopen Notepad from session context |
| “Minimize that” unbound | Minimizes the session referent |

---

## 4. Validation

- Greetings invite conversation; no default feature catalogue.
- Recovery remains truthful; invent/pretend/fake chorus retired from Conversation surfaces.
- Capability menus not appended by default; consecutive soft-miss recovery lines do not repeat.
- Pronouns (`that` / `it` / `again` / `previous one`) bind when Workspace Context already has a referent.
- Desktop action kinds unchanged for ordinary Product Proof paths.
- Verifiers: `pnpm verify:companion-conversation`, `pnpm verify:conversation-quality`.

---

## 5. Remaining Product Proof observations

Out of this slice (still true from T2; not implemented here):

- Full multi-step goal planning (first action only on complex asks).
- Operator window discipline (e.g. self-minimize surprises).
- Premium visual chrome (spacing / scrollbars / hierarchy).
- Stronger desktop awareness beyond session referents.

---

## 6. Compliance

| Constraint | Posture |
| --- | --- |
| Conversation only | Observed |
| No Spec / architecture redesign | Observed |
| No capability execution redesign | Observed |
| Honesty / non-invention preserved | Observed (calm phrasing) |
| Release Hold | Remains — slice authorized by T1 |
| Single program | Observed — stop |

---

## Stop

T1 Companion Conversation Behavior complete. Await Owner Product Proof re-review.
