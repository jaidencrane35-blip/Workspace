# Batch Alignment Check (template)

| Field | Value |
|-------|-------|
| **Batch name** | |
| **Date** | |
| **Author** | |
| **Related architecture doc** | |

Copy this file’s checklist into the batch completion report (or fill and link a dated copy). **Do not start major implementation until all items pass.**

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

**Gate decision:** ☐ Proceed  ☐ Rework scope  ☐ Stop

---

## Product reminder

Primary: manage apps, layouts, real windows, save/restore, work modes, desktop control.  
Secondary: AI Assistant as supporting capability.
