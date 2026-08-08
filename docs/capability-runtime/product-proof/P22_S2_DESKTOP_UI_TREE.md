# P22.S2 — Desktop UI Tree (C-OBS-003)

| Field | Value |
| --- | --- |
| **Capability ID** | **C-OBS-003** |
| **Capability Name** | Desktop UI Tree |
| **Program** | P22.S2 — Desktop UI Tree (observation slice) |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Bounded engineering slice |
| **Authority** | Workspace Capability Atlas + P22.A2 acquisition audit |
| **Max layer** | Windows Integration + Window Provider + Intent |
| **Not** | Mouse click · keyboard input · invoke/set-value · File Provider · architecture redesign |

---

## 1. Objective

Give Conversation a truthful **observation only** path to list named UI Automation controls inside a resolved window (Desktop UI Tree), under Kernel Operator authority.

---

## 2. Scope (this slice)

| In | Out |
| --- | --- |
| `UiAutomationPort` WRAP (Win32 UIA) | Click / type / invoke (C-ACT-004 / C-ACT-005) |
| `enumerate_controls` Window Provider op | Window Control Discovery locate-by-name (C-OBS-004) |
| NL: “what controls are in Notepad” | OCR / OmniParser perception |
| Honest empty / fail when UIA exposes nothing | Invented control success |

---

## 3. Pipeline

```text
Conversation → Intent (winEnumerateControls)
  → Kernel Operator (window / enumerate_controls)
  → Window Provider → UiAutomationPort
  → truthful control list (or honest empty/fail)
```

---

## 4. Files

| Path | Change |
| --- | --- |
| `packages/windows-integration/src/uia.rs` | UIA observation port + memory fixture |
| `packages/kernel/.../types.rs` | `EnumerateControls` |
| `packages/kernel/.../window_provider.rs` | Operation + response |
| `app/src/lib/intentBridge.ts` | NL → `winEnumerateControls` |
| `app/src/lib/operator/intentMap.ts` | IPC map |
| `tests/desktop-ui-tree.test.ts` | Intent regression |
| `scripts/verify-desktop-ui-tree.mjs` | Machine check |
| `docs/capability-runtime/WORKSPACE_CAPABILITY_ATLAS.md` | Atlas v2.0 + C-OBS-003 |

---

## 5. Product Proof (Owner)

**Required after engineering.** Owner launches Workspace once and tries:

1. Open Notepad (or another accessibility-friendly app).  
2. Say or type: **“What controls are in Notepad?”**  
3. Expect a short list of named controls (menus/buttons/edit) — not provider jargon.  
4. Empty/custom-drawn UI → honest “didn’t find named controls” (never fake success).  
5. Confirm this path does **not** click or type.

Engineering must not auto-launch Workspace for this review.

---

## 6. Validation (engineering)

| Check | Expected |
| --- | --- |
| `cargo test -p workspace-kernel --lib capability_runtime::tests::window_enumerate_controls_through_router` | pass |
| `pnpm exec vitest run tests/desktop-ui-tree.test.ts` | pass |
| `node scripts/verify-desktop-ui-tree.mjs` | ok |
| `pnpm typecheck` | pass |

---

## 7. Lifecycle

Engineering Complete (this program) → **Product Proof required** → Trusted → Production.  
Click/type remain separate Atlas capabilities.
