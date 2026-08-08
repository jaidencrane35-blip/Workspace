# P22.A2 — Open-Source Technology Acquisition Audit

| Field | Value |
| --- | --- |
| **Program** | P22.A2 — Open-Source Technology Acquisition Audit |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Engineering audit — **no implementation** |
| **Depends on** | P22.S1 Intelligence Kind Routing (complete); Release Hold |
| **Not this audit** | Architecture replacement · Conversation/Kernel/Moments redesign · AGI · Spec reopen |
| **Posture** | Architecture fixed; seek **techniques** that strengthen existing Capability Providers |

**Evaluation principle (Commodity Before Reinvention / Evidence Before Commitment):** Research mature implementations → classify ADOPT / WRAP / ADAPT / STUDY / REFERENCE / IGNORE → never expose commodity agent stacks on the product surface → Kernel Operator remains sole execution authority.

**Workspace baseline (measured):** Conversation → Intent (deterministic) → Kernel Operator → Capability Runtime. Live providers: Application, Window (enum/focus/snap/bounds), Browser, Screenshots (capture/copy), Clipboard, Notifications, Voice (WinRT WRAP). **Absent:** UI Automation element trees, click/type-by-control, OCR/tokenized screen, hybrid GUI–API app introspection, speculative multi-step GUI loops.

---

## 1. Executive Summary

The open-source “desktop AI agent / computer-use” ecosystem is dominated by **vision-language agent loops** (screenshot → VLM → mouse/keyboard). Those stacks are strong at unconstrained GUI traversal and weak at **governed, truthful, Owner-facing desktop operation** — the product Workspace already owns.

Workspace should **not** adopt UI-TARS Desktop, ShowUI agent runtimes, or Open Computer Use orchestrators as product architecture. They conflict with Kernel Authority, deterministic Intent, Presentation Purity, and non-invention Product Proof.

The highest-value reusable ideas are **OS-native perception and verification techniques**:

| Source | Technique | Fit |
| --- | --- | --- |
| **UFO2** | Windows UIA + hybrid control detection; post-action validation; prefer native API over click theatre | **ADAPT** into Window / Application providers |
| **DesktopCtl** | Tokenized local screen; selector-first actions; wait / verify loops; JSON contracts; permission UX | **ADAPT** observation + verification patterns |
| **OmniParser** | Screenshot → structured interactable regions | **REFERENCE / WRAP** later (license + GPU cost) |
| **UI-TARS / ShowUI** | End-to-end VLA computer-use | **IGNORE** as runtime; **REFERENCE** grounding research only |
| **Open Computer Use** (family) | MCP sandboxes / multi-agent planners | **IGNORE** for Workspace product path |

**Verdict:** Strengthen providers with **deterministic Windows UI Automation targeting + observe→act→verify**, not with a second agent OS.

**ONE recommended acquisition program:**  
**P22.S2 — UI Automation Control Surface (ADAPT UFO2 UIA + DesktopCtl verification)** — Kernel-gated element locate/invoke behind the Window Provider; no VLM in Intent; no architecture redesign.

---

## 2. Technology Radar

```text
                    ADOPT / ADAPT (near-term)
                 ┌─────────────────────────────┐
                 │ UFO2: UIA control tree      │
                 │ DesktopCtl: wait/verify,    │
                 │   tokenize, JSON contracts  │
                 └─────────────────────────────┘
                              │
         REFERENCE ───────────┼─────────── STUDY
    OmniParser parse     UI-TARS SDK operator
    UFO2 hybrid vision   ShowUI grounding
    UFO2 speculative     computer_use_ootb
         multi-action
                              │
                 ┌────────────┴────────────┐
                 │ IGNORE (product path)   │
                 │ UI-TARS Desktop agent   │
                 │ ShowUI/OOTB as product  │
                 │ Open Computer Use CUA   │
                 │ HostAgent/AppAgent OS   │
                 └─────────────────────────┘
```

| Project | Role in ecosystem | Radar band |
| --- | --- | --- |
| [UI-TARS Desktop](https://github.com/bytedance/UI-TARS-desktop) | Native GUI agent + `@ui-tars/sdk` + nut.js-style operators | IGNORE product; REFERENCE operator interface shape |
| [ShowUI](https://github.com/showlab/ShowUI) + [computer_use_ootb](https://github.com/showlab/computer_use_ootb) | Lightweight VLA + Gradio OOTB computer use | IGNORE product; REFERENCE grounding |
| [DesktopCtl](https://github.com/yaroshevych/desktopctl) | Local Rust CLI/daemon: OCR tokenize, click/type, waits | ADAPT techniques (Windows still maturing) |
| Open Computer Use (coasty / Wide-Moat / iFurySt MCP variants) | Full CUA / Docker MCP computers | IGNORE for Workspace OS path |
| [UFO2](https://arxiv.org/abs/2504.14603) / [microsoft/UFO](https://github.com/microsoft/UFO) | Windows AgentOS: UIA + hybrid GUI–API + PiP | ADAPT UIA/hybrid/verify; IGNORE HostAgent redesign |
| [OmniParser](https://github.com/microsoft/OmniParser) | Pure-vision screen → structured elements | REFERENCE / conditional WRAP |

---

## 3. Capability Gap Matrix

| Capability | Workspace today | Leading OSS strength | Gap severity | Fill without redesign? |
| --- | --- | --- | --- | --- |
| Window enum / focus / snap | Strong (Window Provider) | UFO2 / DesktopCtl comparable or weaker on Moments | Low | — |
| App launch / open site | Strong | Common everywhere | Low | — |
| Screenshot capture | Strong | Weaker product framing vs Workspace Conversation | Low | — |
| **In-window control targeting** | **Missing** | UFO2 UIA; DesktopCtl tokens; ShowUI coords | **Critical** | Yes — provider ops |
| Click / type / scroll primitives | Missing (by design) | UI-TARS nut.js; DesktopCtl pointer/keyboard | High | Yes — Kernel-gated only |
| Structured screen perception | Capture bytes only | DesktopCtl tokenize; OmniParser; ShowUI | High | WRAP/ADAPT observation |
| Hybrid native API (Office COM…) | Missing | UFO2 MCP AppAgents | Med | Later provider WRAP |
| Observe → act → verify loop | Compose truth (P21.S1) for layout; not UI controls | DesktopCtl waits; UFO2 validation | High | ADAPT into compose |
| Multi-step GUI agent planner | Intentionally absent | UI-TARS / ShowUI / Open CU | — | **Must not fill with VLM Intent** |
| Conversation + Moments trust | Strong differentiator | Weak / absent in CUAs | Workspace ahead | Preserve |
| Governed permission / audit | Strong constitutional ownership | Often ad hoc | Workspace ahead | Preserve |

---

## 4. Project Reviews

### 4.1 UI-TARS Desktop (ByteDance)

| Question | Answer |
| --- | --- |
| **Problem solved** | End-to-end natural-language GUI automation via VLM + screenshot + mouse/keyboard; ships desktop app + SDK operators |
| **Better than Workspace** | Pixel-level UI interaction; browser+desktop operator abstraction; large community iteration on computer-use loops |
| **Weaker than Workspace** | No Kernel/Moments/Conversation product gravity; probabilistic Intent; invents success; not Windows-constitution aligned |
| **Reuse without redesign?** | **Operator interface shape** (screenshot + execute) as REFERENCE for future Kernel ops — not the agent loop |
| **Effort if adapted** | L–XL to wrap agent (reject); S to study operator trait patterns |
| **Classification** | **IGNORE** as product/runtime · **REFERENCE** SDK Operator (`screenshot` / `execute`) shape |

### 4.2 ShowUI (+ Computer Use OOTB)

| Question | Answer |
| --- | --- |
| **Problem solved** | Lightweight VLA for GUI grounding/navigation; cheap local actor paired with planner LLMs |
| **Better** | Compact grounding model; iterative refine for click coords; OOTB Windows/macOS demos |
| **Weaker** | Research/demo UX; CUDA-centric local path; no governed Conversation; still agent-loop product |
| **Reuse?** | Grounding accuracy techniques as REFERENCE if Workspace later adds vision fallback |
| **Effort** | M–L to WRAP model; high Product Proof risk |
| **Classification** | **IGNORE** product · **REFERENCE** grounding / action-space taxonomy |

### 4.3 DesktopCtl

| Question | Answer |
| --- | --- |
| **Problem solved** | Local-first CLI for agents: tokenize screen, selector-first click/type, waits, JSON contracts, BYO model |
| **Better** | Perception/execution **separation**; OCR tokens without shipping screenshots to cloud; verification loops; Rust daemon; permission UX |
| **Weaker** | macOS-first maturity; not a Conversation product; Windows still BETA; not Moments/Operator authority |
| **Reuse?** | **Yes** — ADAPT tokenize/wait/verify/JSON error codes into `windows-integration` + Kernel compose; do not adopt CLI as user surface |
| **Effort** | M for Windows UIA+OCR observation ops; S for compose verify patterns |
| **Classification** | **ADAPT** (perception + verification contracts) · **IGNORE** as Owner-facing CLI |

### 4.4 Open Computer Use (family)

Includes Coasty-style full-stack CUA, Wide-Moat Docker MCP “computer for LLM”, iFurySt accessibility MCP, and similar.

| Question | Answer |
| --- | --- |
| **Problem solved** | Give LLMs a computer (browser/terminal/desktop) via agents or MCP sandboxes |
| **Better** | Sandbox isolation patterns; MCP tool packaging; accessibility-first experiments (iFurySt) |
| **Weaker** | Replaces product with agent platform; Docker/Linux-centric variants; conflicts with Presentation Purity |
| **Reuse?** | MCP *tool packaging* ideas are STUDY only — Workspace already has CapabilityIntent IPC |
| **Effort** | XL / architectural conflict |
| **Classification** | **IGNORE** for Workspace product path · **REFERENCE** accessibility MCP notes (iFurySt) narrowly |

### 4.5 UFO2 (Microsoft Research / microsoft/UFO)

| Question | Answer |
| --- | --- |
| **Problem solved** | Practical Windows desktop automation via deep OS integration — UIA, Win32, COM, hybrid vision, speculative multi-action, PiP desktop |
| **Better** | **UIA control detection**; hybrid GUI–API preference; post-step validation; Windows-native depth; empirical CUA improvements |
| **Weaker** | Multi-agent HostAgent/AppAgent OS; LLM-centric planning; RAG knowledge substrate; PiP RDP loopback UX — would rewrite Workspace product identity |
| **Reuse?** | **Yes for subsystems:** UIA tree query, invoke patterns, hybrid fallback order, speculative *validated* step batches inside Kernel compose — **not** HostAgent |
| **Effort** | M–L for UIA control surface provider ops; L+ for COM AppAgents (defer) |
| **Classification** | **ADAPT** UIA + verify + hybrid preference · **IGNORE** AgentOS / HostAgent / PiP as product |

### 4.6 OmniParser (additional high-value)

| Question | Answer |
| --- | --- |
| **Problem solved** | Parse screenshots into interactable regions + captions for grounding |
| **Better** | Structured vision overlay without HTML |
| **Weaker** | Not an OS authority layer; YOLO/AGPL weight licensing; GPU cost |
| **Reuse?** | Conditional WRAP behind Screenshots observation — after UIA primary path |
| **Effort** | M–L + license review |
| **Classification** | **REFERENCE** now · **WRAP** only after Evidence Before Commitment license pass |

---

## 5. Adoption Candidates (classified)

| Finding | Class | Justification |
| --- | --- | --- |
| Windows **UI Automation** element locate / invoke / set-value | **ADAPT** | Deterministic OS API; strengthens Window Provider; Kernel-gated; no Intent ML |
| DesktopCtl-style **wait + post-action verify** | **ADAPT** | Extends P21 Completion Contract to in-window actions |
| DesktopCtl / UFO2 **structured observation** (control list / tokens) | **ADAPT** | Truthful “what’s on this window” without inventing |
| Prefer **native API then GUI** (UFO2 hybrid) | **ADAPT** (policy) | Matches Commodity Before Reinvention for Office later |
| UI-TARS / ShowUI **VLM agent loop** as Conversation brain | **IGNORE** | Violates deterministic Intent + Kernel Authority |
| Open Computer Use **product replacement** | **IGNORE** | Replaces Conversation/Moments identity |
| UFO2 **HostAgent / PiP AgentOS** | **IGNORE** | Constitutional redesign |
| OmniParser **screen parse** | **REFERENCE** → future WRAP | Useful after UIA; license/GPU gates |
| nut.js / SendInput click primitives | **REFERENCE** | Only as Kernel-owned ops under UIA targeting — never free agent |
| MCP as Workspace’s IPC | **IGNORE** | CapabilityIntent already freezes the boundary |
| Speculative multi-action (UFO2) | **REFERENCE** | Study for Kernel compose batching *with* validation — not LLM speculation |

---

## 6. Engineering Value

| Candidate | Owner value | Eng effort | Constitutional risk | Provider strengthened |
| --- | --- | --- | --- | --- |
| **UIA Control Surface** | **Very High** — “click Save”, “type in the search box” becomes truthful desktop work | **M** | Low if Kernel-gated | Window (+ Application compose) |
| Wait/verify loops for UI actions | High — fewer false successes | S–M | Low | Kernel compose |
| Tokenized / OCR observation | Med–High — awareness inside apps | M | Med (OCR quality) | Screenshots / Window observe |
| OmniParser WRAP | Med | M–L | Med (license/GPU) | Screenshots |
| VLM computer-use product | High novelty, **negative trust** | XL | **Critical** | — reject |
| Office COM AppAgents | High for Excel/Outlook power users | L | Med | Future Application specialties |

**ROI winner:** UIA Control Surface + verify — reuses Windows commodity APIs, preserves architecture, closes the largest provider gap versus OSS CUAs without becoming one.

---

## 7. Risks

| Risk | Severity | Mitigation |
| --- | --- | --- |
| Adopting a VLM agent loop “because OSS does” | Critical | IGNORE product CUAs; keep Intent deterministic |
| UIA flaky / custom-drawn apps | Med | Hybrid fallback later (vision REFERENCE); truthful failure now |
| AGPL / weight licenses (OmniParser YOLO) | High if premature WRAP | License review before any WRAP |
| DesktopCtl Windows immaturity | Med | ADAPT ideas; do not vendor-lock CLI |
| Scope creep into File/Terminal providers under Hold | High | Bound program to Window control ops only |
| PiP / virtual desktop complexity | High | IGNORE for now |
| Exposing agent CLI/MCP to Owner | High | Presentation Purity — Conversation only |

---

## 8. ONE recommended acquisition program

### P22.S2 — UI Automation Control Surface  
**(ADAPT UFO2 UIA + DesktopCtl verification)**

**Not a new architecture.** Not a computer-use agent. Not Intent-layer ML.

**Problem:** Workspace can arrange windows and capture screens but cannot truthfully operate **controls inside** applications. OSS CUAs solve this with VLMs; Workspace should solve it with **Windows UI Automation** under Kernel authority.

**Outcome:** Owner can ask for in-window actions against resolvable controls (e.g. focus/click/type into a named control in a focused or titled window). Conversation reports success only after verification (control state / focus), per Completion Contract spirit.

**In scope:**

1. Research spike → WRAP/ADAPT Windows UIA in `windows-integration` (enumerate controls, invoke, set value).  
2. New Kernel operations on **Window** (or tightly related) domain — e.g. `locate_control`, `invoke_control`, `set_control_value` — planned/composed by Kernel Operator only.  
3. DesktopCtl-inspired **wait + verify** before success compose.  
4. Intent phrases for clear control actions (deterministic); refuse invent when control not found.  
5. Verifier + Product Proof path; no provider→provider calls.

**Out of scope:**

- UI-TARS / ShowUI / Open Computer Use runtimes  
- HostAgent / AppAgent OS  
- OmniParser production WRAP (follow-on)  
- File Provider / Terminal  
- Probabilistic desktop Intent  
- Replacing Conversation, Kernel, or Moments  

**Success test:** Owner asks to activate a visible, UIA-exposed control in an open app → desktop effect occurs → Conversation truth matches. Custom-drawn controls fail honestly (no invented clicks).

| Constraint | Posture |
| --- | --- |
| Preserve architecture | Yes |
| Strengthen providers | Window (+ compose) |
| No Conversation/Kernel/Moments replace | Yes |
| Release Hold | Remains until Owner authorizes S2 |
| Stop after audit | Observed |

---

## Explicit answers

| Question | Answer |
| --- | --- |
| Replace Workspace architecture? | **No** |
| Best OSS teacher for Windows depth? | **UFO2** (techniques, not AgentOS) |
| Best OSS teacher for observe/verify? | **DesktopCtl** |
| Adopt VLM computer-use? | **No** |
| Highest leverage acquisition? | **UIA Control Surface (P22.S2)** |

---

## Stop

P22.A2 Open-Source Technology Acquisition Audit complete. **Do not implement** until Owner authorizes **P22.S2 — UI Automation Control Surface** (or another ranked candidate).
