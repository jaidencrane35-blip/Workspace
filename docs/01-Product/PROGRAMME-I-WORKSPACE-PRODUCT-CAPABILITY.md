# Programme I — Workspace Product Capability

| Field | Value |
|-------|-------|
| **Status** | Active — product experience maturing on approved architecture (IC1–IC4 complete; IC4 approved) |
| **Audience** | Principal Architect, Engineering, Cursor agents |
| **Nature** | Formal product architecture programme contract |
| **Not** | An implementation contract; not a feature backlog; not a roadmap of tasks |
| **Depends on** | Repository architectural evidence; existing WorkspaceState / observation / DAF foundations |
| **Related** | [Architectural Evidence Report](../02-Architecture/ARCHITECTURAL-EVIDENCE-REPORT.md), [Product Vision](PRODUCT-VISION.md), [Desktop Interaction Model](WORKSPACE-DESKTOP-INTERACTION-MODEL.md), Programmes II–IV (`../05-AI/`) |
| **Implementation contracts** | [IC1](PROGRAMME-I-IC1-PRODUCT-SHELL-CAPABILITY-INVENTORY.md) (complete); [IC2](PROGRAMME-I-IC2-PRODUCT-WORKSPACE-COMPOSITION.md) (complete); [IC3](PROGRAMME-I-IC3-DESKTOP-LAYOUT-EDITING-FOUNDATION.md) (complete); [IC4](PROGRAMME-I-IC4-DESKTOP-LAYOUT-EDITING-REFINEMENT.md) (complete — approved); [IC5](PROGRAMME-I-IC5-USER-CONFIDENCE-AND-DISCOVERABILITY.md) (planned — awaiting approval to commence) |

---

## Purpose

Workspace has reached a natural architectural plateau.

Repository archaeology has confirmed that the deterministic runtime, WorkspaceState pipeline, governance, permission model, and supporting intelligence infrastructure are mature enough to support the original product vision.

The next progression is no longer architectural expansion.

The next progression is **product capability**.

This document establishes **Programme I**.

---

## Programme mission

Transform the existing Workspace architecture into the Workspace product.

The objective is not to build another platform.

The objective is to expose existing architectural capability through coherent desktop workspace functionality.

Workspace is a **desktop operating environment**.

AI remains a **supporting capability**.

---

## Product north star

Workspace should enable a user to:

- Discover applications.
- Observe running applications.
- View desktop windows.
- Create workspace layouts.
- Organise windows.
- Group applications.
- Save layouts.
- Restore layouts.
- Switch between workspaces.
- Lock workspace arrangements.
- Manage their desktop from a single environment.

Every implementation undertaken during Programme I must directly improve one or more of these capabilities.

---

## Architectural foundations

Repository archaeology has confirmed the following foundations already exist and **must be reused rather than replaced**:

- WorkspaceState runtime.
- Observation pipeline.
- Behaviour.
- Runtime Memory.
- Semantics.
- Decision Support.
- Attention.
- Permission Gateway.
- Desktop observation.
- Arrangement persistence.
- Multi-monitor representation.
- Assistant integration.
- SQLite foundation.
- IPC architecture.

These remain authoritative.

Programme I **extends** them.

Programme I does **not** replace them.

Evidence baseline: [Architectural Evidence Report](../02-Architecture/ARCHITECTURAL-EVIDENCE-REPORT.md).

---

## Programme responsibility

Programme I is responsible for converting existing platform capability into **user-visible product capability**.

The programme should focus on:

- Workspace management.
- Desktop layouts.
- Workspace creation.
- Workspace editing.
- Layout persistence.
- Layout restoration.
- Workspace switching.
- Application organisation.
- Desktop interaction.
- Product shell coherence.

The programme is **not** responsible for expanding intelligence.

---

## Architectural boundaries

| Boundary | Rule |
|----------|------|
| **WorkspaceState** | Remains the sole runtime authority for interpreted desktop state. |
| **Observation** | Remains the source of desktop fact. |
| **Permission Gateway** | Remains responsible for privileged operations. |
| **Desktop integration** | Owns OS interaction (`windows-integration` only). |
| **Assistant** | Remains a sidecar. |
| **Intelligence** | Remains observational, explanatory, and assistive. |
| **Cognitive engines** | No additional cognitive engines shall be introduced. |

---

## Engineering principles

- Prefer extending existing systems.
- Do not duplicate ownership.
- Do not introduce wrapper architectures.
- Do not bypass existing boundaries.
- Maintain commercial-grade maintainability.
- Maintain deterministic ownership.
- Keep architecture understandable by human engineers.

---

## Success criteria

At the completion of Programme I:

- Workspace more closely resembles its original product vision.
- Existing architectural capability is surfaced through coherent product functionality.
- Product interaction is richer without increasing architectural complexity.
- Existing runtime systems remain authoritative.
- No duplicate desktop model exists.
- No unnecessary intelligence expansion has occurred.

---

## Relationship to other programmes

| Programme | Role relative to Programme I |
|-----------|------------------------------|
| **Programme I** | Product capability — expose architecture as desktop workspace product |
| **Programme II** | Cognitive Workspace (existing) — not expanded by Programme I |
| **Programme III** | Coherent Workspace Runtime / envelope (existing) — not desktop SoT |
| **Programme IV** | Interaction Runtime / evidence / assistant packages (existing) — Assistant remains sidecar |

Programme I does not subsume, replace, or reopen Programmes II–IV as expansion targets.

---

## Programme status (milestones)

| Milestone | Outcome |
|-----------|---------|
| **IC1** | Capability inventory — architecture richer than shell |
| **IC2** | Unified workflow: Profile → Desktop → Arrangement → Restore |
| **IC3** | Desktop layout editing foundation (composition) |
| **IC4** | Editing experience refinement — **Principal Architect approved** |
| **IC5** | Planned: user confidence & discoverability (awaiting approval to commence) |

IC4 review confirmed a healthy trajectory: richer product experience while continuing to consume WorkspaceState, Arrangement, Stage, and Restore — without duplicate engines.

---

## Stop condition (this document)

This document intentionally contains **no implementation tasks**.

It establishes the architectural mission and boundaries for Programme I.

Individual implementation contracts will be authored by the **Principal Architect** and executed by Cursor.

| Role | Responsibility |
|------|----------------|
| **Principal Architect** | Programme direction; author approved implementation contracts |
| **Cursor** | Implement approved contracts only, within Programme I boundaries |

Cursor is **not** responsible for determining programme direction.

Cursor is responsible **only** for implementing approved contracts within the architectural boundaries defined by Programme I.

---

*End of Programme I charter.*
