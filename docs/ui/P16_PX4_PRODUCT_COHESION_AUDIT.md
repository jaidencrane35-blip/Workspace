# P16.PX4 — Product Cohesion Audit

| Field | Value |
| --- | --- |
| **Program** | P16.PX4 — Product Cohesion |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Commit** | `34ddddb` |
| **Kind** | Lived-experience cohesion — not polish theatre, not production, not architecture |
| **Inputs (not rewritten)** | Spec · EES · Interaction Language · Production Experience · Gates |
| **PQS note** | Heuristic #11 added by Owner direction (minimize steps without reducing agency) |

---

## 1. Product Cohesion Audit

| Workflow | Feels like one product? | Discontinuity |
| --- | --- | --- |
| Conversation | Yes | — |
| Voice capture | Mostly | Post-stop Conversation cue felt like a wizard (pre-PX4) |
| Voice → review → send | Improving | Soft Send helps; tutorial line hurt cohesion |
| Moments / Restore | Yes | Confirmations conversational |
| Tray → Conversation | Yes (PR2) | Shared Show Conversation language |
| Notifications | Mostly | Keep quiet |
| Diagnostics / Support | Yes | Conversation path |
| Startup → Conversation | Yes | Gravity |
| Shutdown / Exit | Yes | Tray Exit Workspace |
| Recovery | Partial | C1 honesty still open |
| Settings (voice) | Partial | Windows Settings handoff unavoidable |
| Install / update prep | Partial | Production trust journey |

**Personality:** Converging on **one calm desktop operator**; residual utility feel where engineering announces itself (post-dictation status lines, future updater).

---

## 2. Language Consistency Audit

| Surface | Sample | Cohesive? |
| --- | --- | --- |
| Tray | Show Conversation / Exit Workspace | Yes |
| Voice mic tooltips | Stop / Esc cancel | Yes |
| Soft Send | Send — Enter | Yes |
| Post-dictation Conversation | “Send when you're ready.” | **No** — engineering coach in the thread (removed PX4) |
| Support export | Conversation NL | Yes |
| Installer metadata | Conversational Desktop Operator | Yes |
| Errors | Desktop language | Mostly |

---

## 3. Motion Audit

| Motion | Role | Load |
| --- | --- | --- |
| Mic pulse / wave (PX2) | Listening state | Low |
| Soft Send pulse (PX3) | Next action | Low — purposeful |
| Preparing mic | Acknowledgment | Low |
| Post-dictation message appear | Interrupted attention | **Removed PX4** |

Motion should communicate state in-place — not inject Conversation turns.

---

## 4. Calm Computing / Cognitive Load

| Workflow | Decisions | Removable? |
| --- | --- | --- |
| Voice turn | Start → stop → read cue → send | Cue decision removed (PX4) |
| Restore | Choose → approve | Keep (agency) |
| Tray restore | One click | Good |
| Support export | Ask Conversation | Good |

---

## 5. Premium Benchmark (qualities only)

| Quality | ChatGPT / Raycast / Notion / Linear / HIG / Fluent | Workspace |
| --- | --- | --- |
| Confidence | Quiet certainty | Strong in Conversation |
| Calmness | Minimal status chatter | Improved PX4 |
| Predictability | Same path every time | Strong F10 |
| Effortlessness | Few mechanical steps | Voice stop/send still multi-step; cue removed |
| Professionalism | No tutorial spam | PX4 aligns |

---

## 6. Product Personality Assessment

| Was | Becoming |
| --- | --- |
| Collection of good systems (voice + tray + composer + proofs) | **One calm desktop operator** |
| Engineering phases visible in copy | Fewer announcements |

---

## 7. Ranked cohesion opportunities

| Rank | Issue | Impact | Class |
| --- | --- | --- | --- |
| 1 | Post-dictation Conversation interrupt | High calm / cohesion | **Implement Now (PX4)** |
| 2 | Visual Stop during listen (ChatGPT-like) | Med–High | Track A |
| 3 | Voice permission copy length | Med | Track A |
| 4 | Empty composer stillness | Low–Med | Track A |
| 5 | C2 close/hide vs tray | Med | Production (later) |
| 6 | Auto-send | — | **Reject** (F10) |

---

## 8. Implementation (single)

**Remove Conversation status line after successful dictation.**

- Composer receives transcript + selection  
- `data-voice-ready` soft Send remains the one obvious action  
- F10 preserved — no auto-send  
- Aligns Owner heuristic: fewer mechanical/attention steps, agency intact  

---

## 9. Heuristic (PQS #11)

> Minimize interaction steps without reducing user agency.

If a workflow can feel faster by removing mechanical interactions while preserving explicit user intent, prefer the simpler interaction.

---

## 10. Stop

Await Product Owner review. No P17. No production/architecture/governance programs.
