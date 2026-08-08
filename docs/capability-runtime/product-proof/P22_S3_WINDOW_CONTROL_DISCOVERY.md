# P22.S3 — Window Control Discovery (C-OBS-004)

| Field | Value |
| --- | --- |
| **Capability ID** | **C-OBS-004** |
| **Capability Name** | Window Control Discovery |
| **Program** | P22.S3 — Window Control Discovery (observation slice) |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Bounded engineering slice |
| **Authority** | Workspace Capability Atlas v2.0 |
| **Max layer** | Windows Integration + Window Provider + Intent |
| **Depends on** | C-OBS-003 Desktop UI Tree (engineering complete) |
| **Not** | Mouse click · keyboard input · full tree dump · File Provider |

---

## 1. Objective

Locate a **named** UI Automation control inside a resolved window and report truthful found / not-found — **observation only**.

---

## 2. Scope

| In | Out |
| --- | --- |
| `find_control` Window Provider op | Click / type (C-ACT-004 / C-ACT-005) |
| Deterministic name match (exact → starts-with → contains) | Listing every control (C-OBS-003) |
| NL: “Find the Save button in Notepad” | Invented control presence |

---

## 3. Pipeline

```text
Conversation → Intent (winFindControl)
  → Kernel Operator (window / find_control)
  → Window Provider → UiAutomationPort::find_control
  → control_found | control_not_found
```

---

## 4. Capability Readiness (after eng)

```text
Architecture ............ 100%
Dependencies ............ 100%
Implementation .......... 100%
Verification ............ 100%
Product Proof ...........   0%
Trusted .................   0%
Production ..............   0%
Overall .................  ~57%
```

---

## 5. Product Proof (Owner)

**Required after engineering.** Owner launches Workspace once:

1. Open Notepad.  
2. “Find the File menu in Notepad” → expect found.  
3. “Find the NoSuchControlZZZ in Notepad” → expect honest not found.  
4. Confirm no click/type occurs.

Engineering must not auto-launch Workspace.

---

## 6. Validation

| Check | Expected |
| --- | --- |
| Kernel `window_find_control_through_router` | pass |
| `tests/window-control-discovery.test.ts` | pass |
| `node scripts/verify-window-control-discovery.mjs` | ok |
| `pnpm typecheck` | pass |
