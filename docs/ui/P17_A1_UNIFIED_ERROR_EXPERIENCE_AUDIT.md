# P17.A1 — Unified Error Experience Audit

| Field | Value |
| --- | --- |
| **Program** | Product Optimization — Unified Error Experience Audit |
| **ID** | P17.A1 *(audit naming only — not File Provider P17)* |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Read-only product quality audit — **no implementation** |
| **Authority inputs** | Spec v2.1 · EES v1 · Product Quality Standard · Interaction Language · Capability Integration Standard |
| **Method** | Repository evidence only — no speculation |

---

## 1. Executive Summary

Workspace already has a **strong Conversation error voice** for many paths: first-person honesty (“I couldn’t…”, “I won’t invent…”), Voice permission recovery that is Product-Proof grade, and Kernel Operator `compose` + `strip_jargon` for soft capability outcomes.

It does **not** yet behave as **one premium failure system**.

The dominant fracture is structural:

| Path | User experience |
| --- | --- |
| **A — Soft capability results** | Through Operator `compose_user_reply` → calm, stripped Conversation language |
| **B — Hard IPC / `KernelError` / plan validation Err** | Through `PublicError` / `IpcCommandError` → often **raw engineering strings** into Conversation (`runtimeBridge.ts`) |
| **C — Specialized tool banners** | `App.tsx` `ws-toast` via duplicated `formatError` → raw `Error.message`, third-person system tone |

Voice is the **gold-standard sanitizer** (`desktopVoiceMessage` + `classify_speech_failure` + permission gate). Other domains lack an equivalent.

**Single highest-leverage vertical slice (recommended next implementation — not done here):**  
Unify **Conversation-bound capability / KernelError failures** into the same first-person compose + jargon strip as soft fails — so Owners never see `"Notification failed:…"`, `"clipboard write requires text"`, or `"An unknown error occurred."` on the Product Gravity path.

---

## 2. Error Surface Inventory

| ID | Surface | Location (evidence) | Trigger | Current UI | Sample wording | Recovery | Blocking | Severity |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| S1 | Conversation replies | `OperatorRoot` → `intelligence` / `runtimeBridge` / compose | Utterance / capability / voice status | Chat bubble | “I can’t… yet — and I won’t invent it.” / raw Err on hard fail | In-reply suggestion | Soft (`busy`) | **Critical** (default path) |
| S2 | Voice mic / permission | `VoiceMicButton`, `permissionGuidance.ts`, `bridge.ts`, `voice.rs` | Mic, OS deny, soft fail, Settings | Conversation + mic chrome | Speech privacy / mic Settings copy; “I didn’t catch that…” | Explain → Settings → recheck | Soft gate | **High** |
| S3 | Operator compose (soft) | `packages/kernel/src/operator/compose.rs` | Provider `ok:false` / empty | Conversation | “That didn’t work — and I won’t pretend it did.” | Clarify / retry | No | **High** |
| S4 | Hard IPC / KernelError | `ipc.ts`, `runtimeBridge.ts`, kernel `PublicError` | Plan Err, port Err, invoke fail | Conversation (as message) | `"An unknown error occurred."`; WinRT/`Notification failed:…` | Often none structured | Soft | **Critical** |
| S5 | Specialized `ws-toast` | `App.tsx`, Save/Resume/Pilot `formatError` | Tool IPC catch | Banner toast | Raw `err.message`; “Resume finished: …” | None | No | **High** |
| S6 | Restore / Moments | `ResumeContextPanel`, `ContinuePreviewObject`, `restoreLimits` | Preview/Approve/Delete/list | Inline + banner | “Approve and restore”; “Failed”; “Could not restore” | Re-preview / approve | Approve blocks restore | **High** |
| S7 | Notifications | `notification.rs`, compose arms | show/status/dismiss | Conversation (+ OS toast) | “Desktop notification shown.” / “I can’t dismiss…” / Err leak on show | Rephrase; dismiss none | No | **Medium** |
| S8 | Support bundle | `intelligence.ts`, `support_bundle.rs` | Conversation export | Conversation | Fail: “I couldn’t create a support package…”; success exposes path | Retry | No | **Low–Med** |
| S9 | Permission (non-voice) | Kernel PublicError; OperatorConsole | Launch / gated | Banner (if console) | “This action was not permitted…”; `request_id=` | Approval (console) | Yes | **Medium** |
| S10 | Tray install fail | `tray.rs` / `lib.rs` | Tray setup Err | **None** (log only) | — | Window-only | No | **Low** |
| S11 | Installer / NSIS | `hooks.nsh` | Uninstall data purge | Native MessageBox | “Remove Workspace user data…?” | Choose No | Modal | **Medium** |
| S12 | Silent Moments list | `ActiveMoment` reload catch | List load fail | Looks empty | — | None visible | No | **Medium** |
| S13 | Health / registry wording | Health panel; some registry/explain | Dev / explain | Panel / Conversation | “Could not load project-health…”; “capability” / “Registry” | N/A | No | **Low–Med** |
| S14 | Browser alerts | — | — | **Absent** (good) | — | — | — | — |

**Not found in product UI:** browser `alert` / `confirm` / `prompt`; stack traces; Rust panic text in Conversation.

---

## 3. UX Consistency Audit

| Dimension | Consistent? | Evidence |
| --- | --- | --- |
| Terminology | **No** | Moments / Continue / restore vs “Resume finished” / snake_case outcomes; “toast” vs “desktop notification”; occasional “capability” / “Registry” |
| Button ordering | **Partial** | Restore: Approve / Not now coherent; delete confirm card OK; no shared dialog system |
| Iconography | **N/A / weak** | Mic `!` for error; no shared error icon language |
| Capitalization | **Mostly** | Sentence case in Conversation; mixed system phrases in PublicError |
| Animation | **Split** | Mic motion polished; banners static; no shared failure motion |
| Spacing / colour | **Split** | Conversation vs `ws-toast` banner chrome |
| Recovery actions | **No** | Voice: explicit Settings loop; capabilities: clarify or silence; banners: none |
| Retry behaviour | **No** | Voice soft-fail retries before Settings; support “try again”; tools often one-shot |
| Language tone | **No** | First person Conversation vs third-person / system PublicError vs tool banners |
| Duplicate implementations | **Yes** | Multiple `formatError` copies; mirrored Voice deny copy (TS+Rust — intentional); dual Conversation vs toast authorities |
| Engineering exposure | **Yes on hard Err** | WinRT Notification Display; plan requirement strings; `unknown_error`; approval `request_id` |
| User knows what happened? | **Often on soft path; often not on hard Err / silent list** | |
| User knows what to do next? | **Voice yes; compose often; hard Err / banner weak** | |

---

## 4. Interaction Language Audit

| Principle | Soft Conversation | Hard Err / Banner |
| --- | --- | --- |
| Conversational | Yes | Often no |
| Calm over clever | Yes | Raw messages feel anxious / technical |
| One obvious next action | Voice yes; soft compose partial | Rarely |
| Explain rather than expose | Voice + `strip_jargon` | Violated on KernelError path |
| Recover instead of punish | Voice gold standard | Banners punish with opacity |
| Trust over spectacle | Strong refusals (“won’t invent”) | Leaks undermine trust |
| Minimize steps without reducing agency | PX4 voice path | N/A |

**Verdict:** Interaction Language is **implemented for Voice and soft compose**, not for the full failure estate.

---

## 5. Product Quality Audit

| Checklist question | Soft Conversation | Hard / Banner |
| --- | --- | --- |
| Reduce effort? | Partial | No |
| Preserve trust? | Yes when truthful | Risk when jargon/leaks |
| Product Gravity? | Yes | Banner competes when specialized surface open |
| Feel conversational? | Yes | No |
| Reduce cognitive load? | Often | No — user must decode |
| Use again voluntarily? | Voice recovery supports | Opaque fails deter |
| Feel premium? | Soft path approaching | Hard path feels unfinished |
| Interaction Language? | Soft yes | Hard no |
| Spec / EES compliant? | Presentation purity OK | No Spec violation — quality debt |

---

## 6. Root Cause Analysis

Group findings — not hundreds of instances:

| Root cause | What it produces | Severity |
| --- | --- | --- |
| **RC1 — Dual presentation authorities** | Conversation vs `ws-toast`; no shared user-message mapper | Critical |
| **RC2 — Compose vs KernelError fork** | Soft fails sanitized; hard Err / plan Err skip `compose` / `strip_jargon` | **Critical** |
| **RC3 — Voice-only sanitizer maturity** | Other domains lack `desktopVoiceMessage`-class mapping | High |
| **RC4 — Duplicated `formatError`** | Raw IPC text in every specialized tool | High |
| **RC5 — Silent degradation** | Tray fail, Moments list catch → empty | Medium |
| **RC6 — Installer minimal productization** | Default NSIS on install fail; one uninstall MessageBox | Medium |
| **RC7 — Vocabulary drift** | Resume/Moments/toast/capability/Registry | Medium |
| **RC8 — Truthful refusal cluster (positive)** | Shared “won’t invent / pretend / fake” — preserve | Already Excellent |

---

## 7. Ranked Improvement Opportunities

| Rank | Opportunity | Class | Trust / cohesion lift | Notes |
| --- | --- | --- | --- | --- |
| 1 | **Conversation hard-fail compose** (map KernelError / IPC fail → first-person + strip) | Implement Now (next program) | Highest | Gravity path |
| 2 | Shared `formatUserError` for tool banners (Conversation tone) | Track A | High | After / with #1 patterns |
| 3 | Notification show Err through compose-equivalent | Part of #1 | High | WinRT leak |
| 4 | Moments list load failure honesty | Track A | Med | Stop silent empty |
| 5 | Resume outcome language (no snake_case banners) | Track A | Med | |
| 6 | Tray install failure optional calm note | Track A / Low | Low | Don’t spam |
| 7 | Kill user-visible “Registry” / over-expose “capability” | Track A | Med | |
| 8 | Installer install-failure product copy | Production later | Med | |
| 9 | Native browser alerts | — | — | **Already absent — keep** |
| 10 | Auto-dismiss / punish UX | — | — | **Reject** |

---

## 8. Estimated Engineering Effort

| Slice | Effort | Risk | Layers |
| --- | --- | --- | --- |
| #1 Conversation hard-fail compose | **M** (1 focused program) | Med (must preserve truthful detail for support without jargon) | Kernel Operator compose + IPC mapping; TS `runtimeBridge` thin |
| #2 Shared banner mapper | S–M | Low | Presentation |
| #4 Moments list honesty | S | Low | Presentation |
| #5 Resume outcome copy | S | Low | Presentation |
| Full “error design system” UI kit | L | High / out of scope | **Reject as framework** |

---

## 9. Recommended Implementation Order

1. **P17.A1 follow-on (name TBD): Conversation Failure Compose** — close RC2 on Product Gravity path.  
2. Apply same mapper vocabulary to specialized banners (RC1/RC4).  
3. Honesty fixes: Moments list, Resume outcome strings (RC5/RC7).  
4. Production install/update failure copy when those gates land.  
5. Do **not** invent a new error framework or Spec change.

---

## 10. Single Highest-Value Vertical Slice

### Conversation Capability Failure Compose

**Problem:** Soft provider outcomes use `compose_user_reply` + `strip_jargon`. Hard failures (`KernelError`, plan validation, WindowsIntegration `Err`, IPC `"An unknown error occurred."`) reach Conversation via `runtimeBridge` as **raw** messages — the largest trust/cohesion break on the default product path.

**Outcome when done:** Every Owner-visible capability failure sounds like Workspace (first person, calm, next step when known) without inventing success and without exposing Provider/WinRT/IPC vocabulary.

**In scope:**
- Map plan clarification + port/runtime Err into Operator-shaped user replies (reuse compose voice).  
- Ensure `execute_capability_intent` failure path never forwards unsanitized Display strings.  
- Keep F10 / truthful failure (never invent success).

**Out of scope:**
- New toast system, modal framework, Spec changes, P17 File Provider, production gate implementation, banner redesign as primary.

**Why this over banner polish:** Product Gravity — Conversation is the product. Fixing the Conversation failure fork raises perceived quality for every capability every day; banner polish only helps when specialized tools are open.

**Success test:** Owner Product Proof phrases that force notification/browser/screenshot/window/clipboard failure never show engineering strings; recovery or honest limitation is clear.

---

## Explicit answers (decision aids)

| Question | Answer |
| --- | --- |
| Does Workspace feel like one product in failure? | **Not yet** — soft Conversation yes; hard Err / banners no |
| Gold standard to copy internally? | **Voice** sanitizer + soft Operator compose |
| Worst leak class? | Hard capability / WinRT / plan Err → Conversation |
| Highest-leverage next program? | Conversation Failure Compose (slice §10) |
| New framework needed? | **No** |

---

## Stop

Audit complete. **No code implemented.**  
Await Product Owner review before any implementation program.
