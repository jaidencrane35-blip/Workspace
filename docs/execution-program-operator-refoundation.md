# Execution Program: P2 Conversational Desktop Operator Refoundation

| Field | Value |
| --- | --- |
| **Program** | Product Refoundation P2 — Operator modes + identity |
| **Date** | 2026-08-07 |
| **Phase 0 answer** | **No** — prior shell did not feel like a desktop operator (in-window float; Expand = proof dashboard; onboarding pedagogy) |
| **Handoff** | Awaiting Project Owner review |

---

## Phase 0 — Reassessment

| Observation | Architectural reading |
| --- | --- |
| Collapse left a float inside the same window | Mode 0 was a CSS overlay, not a desktop presence |
| Expand showed Product Proof tabs | Former proof harness still owned product identity |
| Onboarding (“Save your first moment”) | Violates Product Constitution P10 |
| Save / Check-in / Guide as equal destinations | Capabilities presented as the product |

**Redesign before polish:** runtime shell modes + real window transitions + reclassified surfaces.

---

## Runtime shell modes

| Mode | Name | Window | Behaviour |
| --- | --- | --- | --- |
| **0** | Desktop Operator | `operator` | Always-on-top icon; does not block Windows work |
| **1** | Compact Conversation | `main` (calculator) | Conversation only; default launch |
| **2** | Expanded Workspace | `main` (large) | Conversation primary; secondary slot for context |
| **3** | Specialized Work Surface | `main` (large) | Restore / Save / pilot / health / guide as *tools*, no five-tab identity |

Collapse: hide `main` → show `operator`.  
Operator click: hide `operator` → show `main` Mode 1.

---

## Surface reclassification

| Former surface | New class |
| --- | --- |
| Home onboarding / “first moment” | Conversational guidance + Moments list as optional Mode 3 tool |
| Save | Conversational capability → Mode 3 specialized panel |
| Continue | Conversational capability → Mode 3 specialized panel |
| Check-in | Optional / eval Mode 3 tool (developer or explicit ask) |
| Guide | Conversational answers preferred; Mode 3 only if needed |
| Five-tab dock / menubar brand | Removed from operator identity (not mounted in Modes 1–3 shell) |
| Repository health / evidence | Engineering Mode 3 surface |

---

## Capability evolution

Runtime proposals remain; full governed pipeline architecture documented in `docs/capability-evolution/PIPELINE_ARCHITECTURE.md` (design — not autonomous self-modify).

---

## Owner review checklist

See Engineering Milestone Report § Owner Review Checklist.
