# Component Inventory and Gap Analysis

Research ID: EXP-001 · Stages 1 + 5
Status: Complete
Date: 2026-08-02

Classification key:

| Code | Meaning |
|---|---|
| I | Already implemented (usable) |
| R | Needs refinement |
| M | Missing |
| U | Should reuse OSS / registry pattern |
| B | Should build internally (product-specific) |
| N | Not needed for Product Proof Experience phases |

Reuse vs build is about **presentation**. Product Proof behaviour stays owned.

---

## Inventory

| ID | Component | Class | Reuse / build | Notes |
|---|---|---|---|---|
| EXP-C-001 | App shell | R | B + U | Exists; needs dashboard host |
| EXP-C-002 | Top navigation | R | B + U (Tabs) | Labels good; needs stronger selected/chrome |
| EXP-C-003 | Brand mark | R | B | Exists; elevate as centrepiece support |
| EXP-C-004 | Sidebar | N | — | Defer past Phase 2 |
| EXP-C-005 | Bottom dock | N | — | Concept vision; not Phase 2 |
| EXP-C-006 | Command palette | M | U (cmdk + Dialog) | Phase 3+ |
| EXP-C-007 | Breadcrumbs | N | — | Optional later |
| EXP-C-008 | Workspace selector | N | — | Multi-workspace UX deferred |
| EXP-C-009 | Trust strip | M | B | Compact limits; Phase 2 soft |
| EXP-C-010 | Workspace Home | R | B | Exists but page-first |
| EXP-C-011 | Dashboard canvas | M | B | **Phase 2 critical** |
| EXP-C-012 | Hero / centrepiece | M | B | **Phase 2 critical** |
| EXP-C-013 | Recent moments strip | R | B | Partial via featured + grid |
| EXP-C-014 | Varied gallery | M | B + U (grid/masonry CSS) | Rhythm missing |
| EXP-C-015 | Continue CTA cluster | I | B | Present |
| EXP-C-016 | Empty-structure scaffold | M | B | **Phase 2 critical** |
| EXP-C-017 | Honest activity summary | M | B | Counts/last-saved only |
| EXP-C-018 | Workspace overview header | R | B | Exists |
| EXP-C-020 | Save workflow shell | I | B | Keep behaviour |
| EXP-C-021 | Handoff editor | I | U (Textarea) | Refine visually |
| EXP-C-022 | Name field | I | U (Input) | Refine |
| EXP-C-023 | Review surface | R | B | Move further from form dump |
| EXP-C-024 | Confirmation actions | I | U (Button) | Keep |
| EXP-C-025 | Saved snapshot preview | R | B | Handoff first already |
| EXP-C-026 | Restore-limits notice | I | B | Keep shared module |
| EXP-C-030 | Continue library | R | B | Phase 3 focus |
| EXP-C-031 | Moment card | R | B | Uniform weight today |
| EXP-C-032 | Timeline grouping | M | B | Phase 3 |
| EXP-C-033 | Search moments | M | U | Phase 3 |
| EXP-C-034 | Filters | M | U | Phase 3 optional |
| EXP-C-035 | Restore preview | I | B | Keep honesty |
| EXP-C-036 | Restore confirmation | I | U (Button/Dialog) | Keep |
| EXP-C-037 | Inspect drawer | R | U (Vaul/Disclosure) | Upgrade from `<details>` |
| EXP-C-038 | Delete confirmation | R | U (AlertDialog) | Needs real dialog |
| EXP-C-039 | Outcome summary | I | B | Keep |
| EXP-C-040 | Summary card | R | U + B | Tokenize variants |
| EXP-C-041 | Hero card | M | B | Phase 2 |
| EXP-C-042 | Moment card standard | R | B | Phase 2 |
| EXP-C-043 | Moment card compact | M | B | Phase 2 |
| EXP-C-044 | Metric card | R | B | Check-in |
| EXP-C-045 | Placeholder card | M | B | Phase 2 |
| EXP-C-046 | Guide content card | R | B | Phase 4 polish |
| EXP-C-050–055 | Layout systems | M/R | B + CSS | Dashboard/masonry Phase 2 |
| EXP-C-060–066 | Motion | M | U (Motion) | Mostly Phase 4; light hover Phase 2 |
| EXP-C-070–085 | Primitives | R | **U (shadcn/Radix)** | Replace ad-hoc CSS buttons/inputs |
| EXP-C-090–094 | Check-in/Guide | R | B + U | Same data; better composition |
| EXP-C-N01–N07 | Vision-only | N | — | Forbidden without new authority |

---

## Gap summary for Phase 2

**Must create or substantially refine**

1. Dashboard canvas + hero centrepiece (EXP-C-011/012)
2. Varied recent-moment cards + empty-structure scaffold (EXP-C-014/016/041–045)
3. Honest activity summary from owned data (EXP-C-017)
4. Card size rhythm / masonry or asymmetric grid (EXP-C-052)

**Should adopt as infrastructure (not product features)**

1. Tailwind + design tokens
2. shadcn/ui (or Radix + owned styles) for Button, Input, Textarea, Tabs, Dialog
3. Lucide icons for actions
4. date-fns for relative time

**Keep as Workspace-owned behaviour**

- Save/Continue IPC flows, restore-limits module, consent measurement, handoff authorship

**Defer**

- Command palette, dock, sidebar, workspace selector, Flow/Focus slider, charts
