# P16.32 — Evidence-Driven Product Completion & Trust Validation

| Field | Value |
| --- | --- |
| **Program** | P16.32 (not P17) |
| **Authority** | Product Owner evidence > engineering confidence |
| **Status** | Engineering complete for program — Owner live Product Proof pending |
| **Branch** | `v2-dev` |

---

## 1. Repository reassessment

- P10–P15 permanently closed; P16 Product Proof **OPEN**; P17 blocked.
- WinRT WRAP frozen unless objectively disproven (not disproven here).
- Prior “no remaining defects” claims are **untrusted** unless Owner evidence agrees.

---

## 2. Trust audit (assumptions challenged)

| Assumption | Verdict | Evidence |
| --- | --- | --- |
| Semantic Engine always runs before exe launch | **Held** after fix | `resolveIntent` calls `resolveSemanticIntent` first; hostile suite |
| Unknown open invents `{name}.exe` | **Falsified** (was true) | Kernel `launch_alias` now errors; Intent refuses unknown opens |
| Capability discovery is hard-coded | **Falsified** (was shallow) | `generateCapabilityDiscovery` from `CAPABILITY_GRAPH` only |
| Multi-word names invent `.exe` | **Already blocked** (P16.31) | Still refused |
| Single-token unknown invents `.exe` | **Falsified** (P16.32) | Kernel + Intent guards |
| Every stage leaves objective evidence | **Held** | `resolveIntentWithEvidence` + hostile tests |

---

## 3. Architectural assumption audit

Untrusted until proven: fuzzy app install search, deep tab APIs, multi-step automation, sleep/USB COM under all loads (Windows/WinRT).

Proven for this program: unknown entities do not become executables; discovery is registry-generated; hostile NL suite ≥200 variants with ≥92% resolve rate for required cores.

---

## 4–6. Validation areas

- **Semantic Engine** — hostile NL suite (`tests/semantic-hostile-nl.test.ts`)
- **Capability Registry** — limitations + documentation fields; scoped discovery
- **Desktop reasoning** — goal phrases (`take me to`, `show my Desktop`, Explorer `to Pictures`); folder locate

---

## 7–9. Commodity / missing behaviours

| Behaviour | Class |
| --- | --- |
| Protocol app launch (Store/Settings) | P16 — present |
| Verb+object desktop commands | P16 — present |
| Registry self-description | P16 — present |
| Deep in-browser tab control | **Outside P16** / future |
| Fuzzy Start-menu install search UI | Windows Search / **P17+** if owned |
| Multi-step automation scripts | **P17+** (Automation) |
| Plugin ecosystems (Raycast) | **Not required** for Workspace Operator |
| Local ASR alternative to WinRT | Track A / future if Owner rejects WRAP |

---

## 10. Pipeline evidence

`app/src/lib/intentPipeline.ts` records: normalize → discovery → grammar → semantic → resolve → executable guard.

---

## 11. Explicit answers

| Question | Answer |
| --- | --- |
| Reason instead of phrase match? | **Improved** — goal phrasing + entity reasoning; still deterministic, not ML |
| Explain every capability from live registry? | **Yes** for declared graph; scoped discovery supported |
| Trace every execution path? | **Yes** via `resolveIntentWithEvidence` for Intent Layer |
| Remaining exe-name failure path? | **No invented `.exe`** for unknowns (Intent + Kernel). Explicit `.exe` / path launches still allowed when Intent supplies a resolved path |
| Missing vs Kiro/Raycast/Search/ChatGPT Desktop? | Deep tabs, fuzzy install search, plugins, long automation |
| Belong to P16? | Exe invent + discovery depth + hostile evidence — addressed here |
| Belong to P17? | File/automation expansion |
| Outside Workspace? | OS Search UI, third-party plugin stores |
| Further eng justified by evidence? | **Only if Owner demonstrates a new reproducible defect with ownership** |

---

## STOP

Do not begin P17. Do not mark P16 permanently closed. Await Product Owner review.
