# P16.PF1 — Premium Finish Audit

| Field | Value |
| --- | --- |
| **Program** | P16.PF1 — Premium Finish Program |
| **Kind** | Release-candidate polish decision + one production trust improvement |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Commit** | `4654b1a` |
| **Inputs (not modified)** | Spec v2 · EES v1 · Capability Integration · Interaction Language · Product Quality Standard · Production Gate Specification |
| **Decision rule** | Prefer production confidence over further UX polish when evidence favors trust |

---

## Decision

**UX polish (PX1–PQ1) has removed the highest Conversation/Voice frictions.**  
Remaining unfinished feel is dominated by **release/install trust gaps** (unsigned installer, no published integrity hashes, no tray/updater yet).

| Option | User-confidence impact | Risk | Verdict |
| --- | --- | --- | --- |
| More Voice/composer chrome | Incremental | Low | Defer Track A |
| **A1 artifact checksums** | Integrity for every download | Low | **Implement Now** |
| D1 / E1 / C1 | High (surface / tray / config) | Med | Next after Owner |
| A2 signing | Highest release trust | External cert | BlockedExternal |

**Implemented:** Gate **A1-artifact-checksums**.

---

## 1. Full experience audit (launch → exit)

| Stage | Repo evidence | Finished? |
| --- | --- | --- |
| Installer | A0 NSIS foundation | Eng-complete; unsigned |
| First launch | Conversation gravity default | Strong |
| Splash/loading | Preparing voice states | Adequate |
| Window appearance | Undecorated transparent | Strong |
| Conversation / composer | PX1–PQ1 polish | Strong |
| Dictation / listen / stop / send | Keyboard + calm motion + soft Send + short cue | Strong |
| Thinking / executing / waiting | Streaming truthful replies | Strong |
| Errors / recovery | Soft fail vs Settings; support bundle B1 | Strong / OA pending |
| Empty states | Sparse | Track A |
| Notifications | Provider + Conversation | Track A |
| Window management | Collapse / single-instance C0 | Strong |
| Tray | Spec’d E1 — not implemented | Production Gate |
| Shutdown / restart | C2 blocked by tray | Production Gate |
| Diagnostics / support | B1 Complete | OA pending |
| Updates | B2 blocked by A2 | Production Gate |

---

## 2. Premium Experience Matrix

| Domain | Naturalness | Flow | Clarity | Confidence | Notes |
| --- | --- | --- | --- | --- | --- |
| Conversation | High | High | High | High | Gravity + calm cues |
| Voice start→send | High | High | High | High | F10 preserved |
| Voice failure | Med–High | High | High | High | Settings path honest |
| Visual / motion | High | — | High | High | PX2/PX3 |
| Production install | Med | Med | Med | **Low–Med** | A0 yes; integrity/signing gaps |
| Tray / update | Low | — | — | Low | Not shipped |
| Diagnostics | Med–High | High | High | Med | B1; OA pending |

**Emotional quality:** Conversation communicates calm competence. Production distribution still fails the “finished software” test for download trust.

---

## 3. Top 25 friction ranking

| # | Friction | Impact | Freq | Effort↓ | Trust | Premium | Risk | Class |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | No published installer integrity hashes | High | Every download | High | High | High | Low | **Implement Now (A1)** |
| 2 | Unsigned installer / SmartScreen | High | Every install | High | Highest | Highest | Ext | Production Gate A2 |
| 3 | No tray lifecycle | High | Daily | Med | Med | High | Med | Production Gate E1 |
| 4 | No auto-updater | High | Ongoing | High | High | High | Blocked A2 | Production Gate B2 |
| 5 | Broad IPC surface | Med–High | Rare abuse | Med | High | Med | Med | Production Gate D1 |
| 6 | Config/DB validation UX | Med | Failures | Med | High | Med | Med | Production Gate C1 |
| 7 | No visual Stop/Cancel chrome | Med | Voice sessions | Low | Low | Med | Low | Track A |
| 8 | Empty composer cue sparse | Low–Med | Always | Low | Low | Med | Low | Track A |
| 9 | Support bundle OA unproven | Med | Support | Med | Med | Med | Low | Track A / OA |
| 10 | Installer OA unproven | Med | Install | Med | Med | Med | Low | Track A / OA |
| 11 | Notification micro-polish | Low | Occasional | Low | Low | Low | Low | Track A |
| 12 | Long voice permission copy | Low–Med | First grant | Low | Med | Med | Low | Track A |
| 13 | CI release automation absent | Med | Release | High | Med | Med | Med | Production Gate F1 |
| 14 | Crash capture optional | Low–Med | Rare | Med | Med | Low | Med | Optional B3 |
| 15 | Shutdown policy incomplete | Med | Exit | Med | Med | Med | Med | Gate C2 |
| 16 | Install rollback | Med | Fail install | Med | Med | Med | Med | Gate A3 |
| 17 | Permission audit | Med | Rare | Low | Med | Low | Med | Gate D2 |
| 18 | Hold-to-talk absent | Low | Power users | Med | Low | Med | Med | Future / Reject ambient |
| 19 | Auto-send after voice | — | — | — | Breaks F10 | — | — | **Reject** |
| 20 | Feature catalogue chrome | — | — | — | Gravity | — | — | **Reject** |
| 21 | Neon mic status (legacy) | — | — | — | — | Fixed PX2 | — | Done |
| 22 | Mic hunt to stop | — | — | — | — | Fixed PX1 | — | Done |
| 23 | Instructional review tutorial | — | — | — | — | Fixed PQ1 | — | Done |
| 24 | Soft Send after voice missing | — | — | — | — | Fixed PX3 | — | Done |
| 25 | Dual-instance corruption | — | — | — | — | Fixed C0 | — | Done |

---

## 4. Benchmark summary (interaction quality only)

| Reference | Lesson | Workspace |
| --- | --- | --- |
| ChatGPT Desktop | Quiet listen + clear send | Matched for Conversation/Voice feel |
| Raycast | Instant keyboard trust | Strong in-app; install trust weaker |
| Apple HIG | Calm hierarchy, purpose motion | Aligned post-PX/PQ |
| Microsoft Fluent / Win11 | System confidence (signed apps) | **Gap: distribution trust** |
| VS Code | Predictable updates | Gap: updater blocked |

---

## 5. Implementation (single improvement)

**Gate A1 — Artifact checksums**

- `scripts/generate-artifact-checksums.mjs` — `pnpm checksums:generate`
- `scripts/verify-artifact-checksums.mjs` — `pnpm verify:artifact-checksums` (self-test + live match)
- Documented in `docs/production/INSTALLER.md`
- `production-gates-dependency.json`: A1 → Complete; `nextReadyNow` → `D1-ipc-quarantine`
- Catalog regenerated; Dependency Authority updated

Not Release Ready. Not signed. Not updater.

---

## 6. Explicit answers

| Question | Answer |
| --- | --- |
| Biggest reason not finished? | **Distribution trust** — unsigned install + (now closing) missing integrity hashes; tray/updater still open |
| Most increase perceived quality? | Signed installs + calm daily tray presence |
| Most increase trust? | **A2 code signing** (then updater) |
| Most increase delight? | Effortless voice→desktop outcomes (already strong) + silent trustworthy updates later |
| Highest-value remaining? | **A2 signing** (external) or **D1/E1** among ReadyNow |
| Next: UX or Production Gate? | **Production Gate** |
| Which gate largest confidence? | Among ReadyNow after A1: **E1 tray** for daily feel; **D1** for security surface; overall largest is **A2 signing** (blocked external) |

---

## 7. Stop

Await Product Owner review. No P17. No Spec/governance/standards reopen. No further gate until directed.
