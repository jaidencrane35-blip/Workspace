# Capability Runtime Research Report
## P9 — Architecture before implementation

| Field | Value |
| --- | --- |
| **Program** | P9 Capability Runtime Research & Adoption Strategy |
| **Date** | 2026-08-07 |
| **Frozen** | UI Architecture Spec · Product Presentation · Desktop Operator Form A/B |
| **Implementation** | **None** in this program (Capability Runtime V1 blocked on Owner review) |

---

## 1. Mission outcome

Prevent unnecessary engineering by deciding **what Workspace owns** vs **what it wraps**.

| Workspace owns | Workspace does not need to own |
| --- | --- |
| Product identity | Chromium / PTY engines |
| Desktop Operator + Conversation | OCR/ASR model weights |
| Capability **contracts** | Commodity clipboard/capture crates |
| Permissions, audit, trust | External agent IDE platforms |
| Win32 effect authority | n8n / AutoHotkey product shells |

---

## 2. Constitutional alignment

| Authority | Implication for Track B |
| --- | --- |
| Arch Const. V2 | One OS authority path; permission before effect; honesty |
| Product Constitution | Capabilities enter via Conversation — no launcher UI |
| UI Architecture Spec | Layer 3 has **no competing primary UI** |
| Kiro study (prior) | ADOPT principles / WRAP implementations; REJECT identity |
| Existing `architecture/10_Capability_Contracts.md` | Message semantics remain; this pack adds *conversation-entry* contracts for Track B domains |

---

## 3. Research method

1. Enumerate permanent domains (`CAPABILITY_DOMAIN_CATALOGUE.md`).  
2. For each domain, survey mature OSS (Rust-first, Windows-capable, Tauri-friendly).  
3. Classify ADOPT / ADAPT / WRAP / STUDY / REJECT (`OPEN_SOURCE_ADOPTION_MATRIX.md`).  
4. Write conversation-entry Capability Contracts (`CAPABILITY_CONTRACTS.md`).  
5. Assess risks and propose implementation order **after** Owner approval.

**Kiro:** used as one reference (tool-gate outside model, inspectable local memory, gateway-not-replacement). Broadened to Win32 crates, capture/ASR/OCR, PTY, Tauri plugins. **No convergence on one project.**

---

## 4. Headline findings

1. **Zero platform ADOPT.** No external desktop/agent product should become Workspace.  
2. **Win32 stack is already the right core** — extend `workspace-windows-integration`, do not replace with AHK/PowerToys.  
3. **Commodity I/O should WRAP** (clipboard, screenshot, notifications, PTY).  
4. **Observation must stay consented** — reject ambient frameworks.  
5. **Voice / OCR / CDP are STUDY** — valuable later, not V1 blockers.  
6. **Memory/Search should deepen existing SQLite** before new search engines.  
7. **Automation is Critical trust** — last among early domains, never first.

---

## 5. Relationship to prior architecture docs

| Prior doc | Role after P9 |
| --- | --- |
| `architecture/10_Capability_Contracts.md` | Cross-capability message law (still authoritative) |
| This pack | Track B **domain catalogue + OSS strategy + conversation contracts** |
| `docs/ui/*` | Frozen presentation — capabilities plug in as satellites/effects only |

---

## 6. What this program deliberately did not do

- No new IPC commands for desktop effects  
- No UI redesign  
- No Operator / Conversation changes  
- No multi-domain implementation spikes  

---

## 7. Stop condition

Owner reviews this pack. Only then may **Capability Runtime V1** begin — scoped to the approved slice of `RECOMMENDED_IMPLEMENTATION_ORDER.md`.
