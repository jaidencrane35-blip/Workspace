# P16.PR2 — Production Experience Audit

| Field | Value |
| --- | --- |
| **Program** | P16.PR2 — Production Experience Integration |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Commit** | _(stamped on ship)_ |
| **Inputs (not modified)** | Spec v2 · EES · Interaction Language · Product Quality Standard · Production Gate Specification |
| **Lens** | User Trust Journeys — not isolated gates |

---

## Decision

| Candidate | Daily usability | Trust | First impression | Cohesion | Risk | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| D1 IPC quarantine | Low (invisible) | Med–High | Low | Low | Med | ReadyNow — later |
| **E1 Tray lifecycle** | **Highest** | High | High (alive on desktop) | High | Med | **Implement Now** |
| A2 Signing | Low daily | Highest | Highest install | High | External | Blocked |
| B2 Updater | High daily | High | Med | High | Blocked by A2 | Wait |
| C1 Config/DB UX | Rare | High on failure | Low | Med | Low | Later |

Owner decision rules prefer **every user every day** → **E1 tray** over D1 (canonical next for security, lower daily feel).

**Implemented:** Gate **E1-tray-lifecycle** — Show Conversation / Exit Workspace; left-click restores Conversation; shared restore path with single-instance.

---

## 1. User Trust Journey

```
Download → Install → Launch → Recover → Update → Diagnose → Support export → Close → Restart → Continue
```

| Transition | State | Trust gap |
| --- | --- | --- |
| Download | A1 checksums exist | Unsigned (A2) |
| Install | A0 NSIS | SmartScreen fear |
| Launch | Conversation gravity; single-instance | Felt unfinished without tray |
| Recover | Session checksums; C1 open | Rare empty-state honesty |
| Update | Absent (B2←A2) | No calm update story |
| Diagnose / Support | B1 Conversation export | Strong if discovered |
| Close | Explicit Exit IPC; window close collapses | Exit hard to find without tray |
| Restart / Continue | C0 focus | Improved with tray restore |

---

## 2. Production Cohesion Matrix

| Surface | Tone match Conversation? | Duplicated? | Connected? |
| --- | --- | --- | --- |
| Installer | Product copy in bundle metadata | Docs only | Partial |
| Updater | N/A | — | Missing |
| Signing | N/A | — | Missing |
| **Tray** | **Show Conversation / Exit Workspace** | Shared restore with C0 | **Integrated (PR2)** |
| Diagnostics / Support | Conversation NL path | Single B1 path | Good |
| Recovery | Kernel facts; C1 UX open | — | Partial |
| Release | Checksums A1 | — | Partial |

**Overlaps removed/avoided:** Tray restore reuses `show_conversation` with single-instance focus — one restore language.

**Still disconnected:** Updater (absent), signing (absent), close-vs-hide policy (C2 after E1).

---

## 3. Daily Ownership Audit

| Scenario | Before PR2 | After PR2 |
| --- | --- | --- |
| Startup | Window only | Window + tray presence |
| Sleep / wake | OS | Unchanged |
| Restart / second launch | C0 focus | Same path as tray Show |
| Crash | No tray recovery story | Tray returns after relaunch |
| Auto-update | None | None |
| Tray notifications | None | None (no noise) |
| Conversation restore | Window / secondary launch | Tray left-click + menu |
| Single instance | Complete | Cohesive with tray |
| Support export | B1 | Unchanged |
| Exit | Menu/IPC only | Tray **Exit Workspace** |

---

## 4. Production Interaction Language (extension — practice, not new standard)

| Surface | Language |
| --- | --- |
| Tray | Quiet; two actions; Conversation nouns |
| Exit | “Exit Workspace” — not Quit/Kill |
| Show | “Show Conversation” — not Show Window |
| Support | Existing Conversation export copy |
| Installer | Existing INSTALLER.md user tone |

No new standards document created (constraint).

---

## 5. Benchmark (production experience)

| Product | Trust / tray / update lesson |
| --- | --- |
| VS Code | Tray optional; signed updates calm |
| Discord | Always-alive tray; Exit clear |
| Notion | Minimal tray; web-trust |
| ChatGPT Desktop | Simple presence |
| Raycast | Menu-bar native; instant restore |
| Cursor | Familiar VS Code trust path |

Workspace now closer on **presence**; still behind on **signed updates**.

---

## 6. Remaining gate ranking (post-PR2)

| Rank | Gate | Daily trust | Notes |
| --- | --- | --- | --- |
| 1 | A2 Signing | Highest first-impression | External cert |
| 2 | B2 Updater | Highest ongoing | Blocked by A2 |
| 3 | D1 IPC quarantine | Security trust | nextReadyNow |
| 4 | C1 Config/DB UX | Failure trust | Rare |
| 5 | C2 Shutdown policy | Close/hide clarity | Needs E1 ✓ |
| 6 | F1 CI release | Engineer trust | ReleaseOnly |

---

## 7. Implementation

| Artifact | Role |
| --- | --- |
| `app/src-tauri/src/tray.rs` | Tray menu + left-click restore |
| `lib.rs` | `install_tray` in setup; C0 uses `show_conversation` |
| `Cargo.toml` | `tray-icon` feature |
| `pnpm verify:tray-lifecycle` | Wiring verifier |
| Dependency JSON | E1 → Complete; nextReadyNow remains **D1** |

---

## 8. Explicit answers

| Question | Answer |
| --- | --- |
| Weakest production experience? | **Unsigned install + no updater** (A2/B2); daily presence was next — now tray |
| Largest daily trust gate remaining? | **B2 updater** (blocked); among ReadyNow **D1** (security) then **C1** |
| Largest first-impression gate? | **A2 signing** |
| Most disconnected feature? | **Updater** (absent); diagnostics already Conversation-native |
| One product feel (installer/updater/tray/diagnostics/recovery)? | **Not yet** — tray+diagnostics+installer starting to cohere; updater/signing missing |
| Single cohesion lift shipped? | **E1 tray** — daily alive presence + Exit/Show in Conversation language |

---

## 9. Stop

Await Product Owner review. No P17. No Spec/governance/standards reopen. nextReadyNow still **D1-ipc-quarantine**.
