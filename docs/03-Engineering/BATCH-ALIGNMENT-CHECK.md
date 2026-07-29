# Batch Alignment Check (template)

| Field | Value |
|-------|-------|
| **Batch name** | |
| **Date** | |
| **Author** | |
| **Related architecture doc** | |

Copy this file’s checklist into the batch completion report (or fill and link a dated copy). **Do not start major implementation until all items pass.**

---

## Product drift record (required for major batches)

| Field | Content |
|-------|---------|
| **Current mission** | |
| **Product goal** | |
| **Allowed changes** | |
| **Forbidden changes** | |
| **Validation method** | |

---

## Alignment checklist

| # | Confirm | Pass? (Y/N) | Notes |
|---|---------|-------------|-------|
| 1 | We are working on **Workspace** only | | |
| 2 | Original product vision (desktop workspace management) remains the goal | | |
| 3 | Existing architecture is reused where appropriate | | |
| 4 | No unrelated projects / prior AI chat contexts dictate design | | |
| 5 | No new subsystem without a clear product requirement | | |
| 6 | Change does not invent an AI window manager / layout recommendation engine / autonomous agent | | |
| 7 | Window control (if any) stays in desktop control layer + Permission Gateway | | |
| 8 | Assistant (if touched) only requests/explains — does not execute desktop control | | |
| 9 | Feature quality questions answered (why / problem / owner / non-goals) | | |
| 10 | Completion report + architecture doc planned for this batch | | |
| 11 | Maintainability target understood (≥ 8 for new systems per [AI Engineering Governance](../00-Governance/AI_ENGINEERING_GOVERNANCE.md)) | | |
| 12 | Assistant remains sidecar; Workspace remains primary product surface | | |
| 13 | Product priority respected (Workspace → desktop control → workflows → usability → performance → Assistant) | | |
| 14 | Batch groups related work (not fragmented micro-approvals) | | |

**Gate decision:** ☐ Proceed  ☐ Rework scope  ☐ Stop

---

## Product reminder

Primary: manage apps, layouts, real windows, save/restore, work modes, desktop control.  
Secondary: AI Assistant as supporting capability.

Priority order: Workspace functionality → Desktop control → User workflows → Usability → Performance → Assistant expansion.

See also: [`AGENTS.md`](../../AGENTS.md), [AI Engineering Governance](../00-Governance/AI_ENGINEERING_GOVERNANCE.md).
