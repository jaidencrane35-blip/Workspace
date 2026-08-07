# P15 Screenshot Provider — Engineering Preparation Pack

| Field | Value |
| --- | --- |
| **Status** | Preparation complete — **implementation NOT begun** |
| **Blocked on** | P14 Product Owner Product Proof acceptance |
| **Research** | [`research/SCREENSHOT_RESEARCH.md`](./research/SCREENSHOT_RESEARCH.md) |
| **Architecture** | [`research/SCREENSHOT_ARCHITECTURE_PROPOSAL.md`](./research/SCREENSHOT_ARCHITECTURE_PROPOSAL.md) |
| **Product Proof draft** | [`product-proof/SCREENSHOT_PROVIDER_PRODUCT_PROOF_DRAFT.md`](./product-proof/SCREENSHOT_PROVIDER_PRODUCT_PROOF_DRAFT.md) |
| **Composition audit draft** | [`product-proof/SCREENSHOT_PROVIDER_COMPOSITION_AUDIT_DRAFT.md`](./product-proof/SCREENSHOT_PROVIDER_COMPOSITION_AUDIT_DRAFT.md) |

---

## Capability contract draft

| | |
| --- | --- |
| **Domain** | `screenshots` |
| **Purpose** | Capture display/window stills under explicit user intent |
| **Examples** | “Take a screenshot.” / “Screenshot Cursor.” / “Capture the right monitor.” |
| **Permissions** | `screenshot.read`, `screenshot.capture` |
| **Adoption** | WRAP `xcap` behind `ScreenshotPort` |
| **Arguments** | `target` (`monitor` \| `window`) · `query?` · `monitor_index?` · `hwnd?` |
| **Results** | `{ ok, path?, width?, height?, target?, message }` |
| **Failures** | Unavailable API; window not found; protected content; missing target |
| **Rollback** | Delete capture file when a later composed step fails (future) |
| **Audit** | target kind + dimensions + capture id (not pixels) |
| **Pipeline** | Intent → Kernel Operator → Runtime → ScreenshotProvider → reply |
| **Out of scope** | Recording · OCR · ambient capture · cloud upload |

---

## Provider contract draft

| Operation | Level | Effect |
| --- | --- | --- |
| `status` | 1 | Availability + brief support summary |
| `capture` / `capture_monitor` | 2 | Still of monitor (default primary) |
| `capture_window` | 2 | Still of window (query / active / hwnd) |

Descriptor (proposed):

```text
name: ScreenshotProvider
domain: screenshots
adoption: WRAP
operations: status, capture, capture_window
purpose: Capture monitor or window stills under Workspace permission.
```

---

## Composition opportunities

| Pairing | User-visible idea | When |
| --- | --- | --- |
| Screenshots + Window | “Screenshot this window.” | P15 |
| Screenshots + Application | “Screenshot Notepad.” (resolve via Window/App) | P15 |
| Screenshots + Clipboard | “Copy that screenshot.” | Future |
| Screenshots + Notifications | “Tell me when the capture finishes.” | N/A for stills; future for long jobs |
| Screenshots + OCR | “Read the text in that screenshot.” | Later program |
| Screenshots + Memory | Attach capture to a Moment | Future |
| Screenshots + Browser | “Screenshot ChatGPT.” after open | Operator composition later |

---

## Risk assessment

| Risk | Severity | Mitigation |
| --- | --- | --- |
| Ambient / always-on capture creep | Critical | Explicit intent only; REJECT scheduled capture in P15 |
| WRAP crate regression | High | Thin `ScreenshotPort`; pin version; Memory port for tests |
| Protected-content false success | High | Surface WGC denial truthfully |
| Confusing with DesktopCapturer | Medium | Separate trait + docs |
| Large PNG storage growth | Medium | App-data folder + future TTL |
| HDR / multi-GPU quirks | Medium | Truthful failure; STUDY `windows-capture` DXGI path |
| Conversation jargon leak | Medium | Compose layer strip; Product Proof harness |

---

## Recommended implementation order (when unblocked)

1. Research acceptance (this pack) — **done (prep)**  
2. `ScreenshotPort` + Memory port + WRAP `xcap` System port  
3. Domain permissions `screenshot.read` / `screenshot.capture`  
4. `ScreenshotProvider` register in Capability Runtime  
5. Commands + `execute_capability_intent` domain arm  
6. Kernel Operator plan/compose  
7. Intent Bridge kinds + Product Proof JSON/harness  
8. Provider Composition Audit  
9. Validation + launch verify  
10. Owner Product Proof review  
11. **STOP** — no P16 until accepted  

---

## Recommended commodity libraries

| Choice | Role |
| --- | --- |
| **`xcap` (Apache-2.0)** | Primary WRAP for monitor/window stills (WGC on Windows) |
| **`windows-capture` (MIT)** | STUDY alternative (WGC + DXGI) if `xcap` insufficient |
| **`image` (via xcap)** | Encode PNG — transitive, not product identity |

---

## Explicit statements

- **P14** awaits Product Owner review (Browser Provider).  
- **P15 implementation has NOT begun.**  
- This pack is documentation only.
