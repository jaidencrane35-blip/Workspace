# Experience Component Catalogue

Research ID: EXP-001 · Stage 1
Status: Complete for discovery
Date: 2026-08-02

Sources: concept boards, shipped Experience (LEDGER-0034), Experience Roadmap
Phase 2–4 targets, Blueprint companion principles.

IDs use `EXP-C-###`. Components are presentation units — not capabilities.

---

## A. Navigation & chrome

| ID | Component | Visible/implied in | Purpose |
|---|---|---|---|
| EXP-C-001 | App shell | Concepts + ship | Window root; brand + content frame |
| EXP-C-002 | Top navigation / tab rail | Ship (Home/Save/Continue/Check-in/Guide); concepts (top bar) | Primary product navigation |
| EXP-C-003 | Brand mark + wordmark | Concepts + ship | Instant “this is Workspace” |
| EXP-C-004 | Sidebar (optional later) | Concepts 2,7,9 | Secondary tools; **not** Phase 2 required |
| EXP-C-005 | Bottom dock | Concepts 1,6, switchable board | Quick launch / mode switch; **Phase 4+** unless needed for Home |
| EXP-C-006 | Command palette | Concepts (search); VS Code/Raycast/Cursor patterns | Find moments/actions without leaving Home |
| EXP-C-007 | Breadcrumbs | Weakly implied | Drill into moment → inspect; optional |
| EXP-C-008 | Workspace selector | Concepts (Work/Study/Personal) | Multi-workspace switch; **defer** until multi-workspace UX authorised |
| EXP-C-009 | Status / trust strip | Implied by Guide limits | Compact honest restore-limits hint |

## B. Workspace / Home dashboard

| ID | Component | Visible/implied in | Purpose |
|---|---|---|---|
| EXP-C-010 | Workspace Home | Roadmap Phase 2 | Product centre |
| EXP-C-011 | Dashboard canvas | Concepts all; Roadmap Phase 2 | Spatial host for tiles |
| EXP-C-012 | Hero / centrepiece | Concepts; Roadmap Phase 2 | Eye-landing “current work” |
| EXP-C-013 | Recent moments strip | Roadmap Phase 2 | Visual memory of saved contexts |
| EXP-C-014 | Recent context gallery | Concepts masonry/grid | Varied-weight cards |
| EXP-C-015 | Continue CTA cluster | Ship + concepts | Save / Continue primary actions |
| EXP-C-016 | Empty-structure scaffold | Roadmap Phase 2 | Shape of memory without fake data |
| EXP-C-017 | Activity summary (honest) | Concepts “recent activity” | Only from owned data (counts, last save time) — never ambient |
| EXP-C-018 | Workspace overview header | Ship | Name + short companion lede |

## C. Save workflow

| ID | Component | Visible/implied in | Purpose |
|---|---|---|---|
| EXP-C-020 | Save workflow shell | Ship PP-M1-01 | Naming → review → confirm |
| EXP-C-021 | Handoff editor | Ship | User-authored next intention |
| EXP-C-022 | Name field | Ship | Moment title |
| EXP-C-023 | Review surface | Ship | Pre-capture consent scope |
| EXP-C-024 | Confirmation actions | Ship | Save / Cancel |
| EXP-C-025 | Saved snapshot preview | Ship (inspect details) | Post-save summary; details secondary |
| EXP-C-026 | Restore-limits notice | Ship PP-P01B | Shared truthful limits |

## D. Continue workflow

| ID | Component | Visible/implied in | Purpose |
|---|---|---|---|
| EXP-C-030 | Continue library | Roadmap Phase 3 | Recent-projects energy |
| EXP-C-031 | Moment / context card | Ship + concepts | Handoff-first card |
| EXP-C-032 | Day / timeline grouping | Phase 3 | Soft chronology |
| EXP-C-033 | Search (moments) | Concepts search | Filter saved moments |
| EXP-C-034 | Filters (day/workspace) | Phase 3 optional | Narrow library |
| EXP-C-035 | Restore preview panel | Ship | Plan before approve |
| EXP-C-036 | Restore confirmation | Ship | Approve and restore |
| EXP-C-037 | Inspect drawer / disclosure | Ship (`details`) | Window/monitor metadata secondary |
| EXP-C-038 | Delete confirmation dialog | Ship | Explicit destructive confirm |
| EXP-C-039 | Outcome summary | Ship | Per-item restore results |

## E. Cards & tiles

| ID | Component | Visible/implied in | Purpose |
|---|---|---|---|
| EXP-C-040 | Summary card | Ship | Generic content card |
| EXP-C-041 | Hero card | Roadmap Phase 2 | Featured / large weight |
| EXP-C-042 | Moment card (standard) | Phase 2–3 | Name + handoff + time |
| EXP-C-043 | Moment card (compact) | Phase 2 masonry | Smaller rhythm unit |
| EXP-C-044 | Metric / check-in stat card | Ship Check-in | Baseline / median / days |
| EXP-C-045 | Placeholder card | Phase 2 empty structure | Non-fabricating scaffold |
| EXP-C-046 | Guide content card | Ship Guide | Trust/help chunks |

## F. Layout systems

| ID | Component | Visible/implied in | Purpose |
|---|---|---|---|
| EXP-C-050 | Dashboard layout | Concepts | Primary Home composition |
| EXP-C-051 | Responsive grid | Concepts | Adaptive columns |
| EXP-C-052 | Masonry / varied tiles | Concepts 5; Roadmap Phase 2 | Visual rhythm |
| EXP-C-053 | Split view | Concepts 4,7 | Main + rail (later) |
| EXP-C-054 | Detail panel / drawer | Continue inspect | Secondary metadata |
| EXP-C-055 | Adaptive density | Concepts Flow vs Focus | Optional Phase 4 mode |

## G. Motion & feedback

| ID | Component | Visible/implied in | Purpose |
|---|---|---|---|
| EXP-C-060 | Hover elevation | Concepts | Desktop aliveness |
| EXP-C-061 | Focus rings | A11y + Fluent/HIG | Keyboard trust |
| EXP-C-062 | Expand / collapse | Inspect, Guide | Progressive disclosure |
| EXP-C-063 | Route / view transition | Roadmap Phase 4 | Home↔Save↔Continue |
| EXP-C-064 | Context reveal | Preview open | Calm appearance of plan |
| EXP-C-065 | Skeleton loading | Professional apps | Boot / list load |
| EXP-C-066 | Empty-state motion | Phase 4 | Subtle, non-gimmick |

## H. Primitives

| ID | Component | Visible/implied in | Purpose |
|---|---|---|---|
| EXP-C-070 | Button (primary/ghost/danger) | Ship | Actions |
| EXP-C-071 | Text input | Ship | Name, minutes |
| EXP-C-072 | Textarea | Ship | Handoff, interviews |
| EXP-C-073 | Checkbox | Ship Check-in | Correction flag |
| EXP-C-074 | Tabs | Ship nav | View switching |
| EXP-C-075 | Dialog / modal | Delete confirm | Blocking confirm |
| EXP-C-076 | Menu / dropdown | Later | Overflow actions |
| EXP-C-077 | List | Ship | Scope items |
| EXP-C-078 | Badge / pill | Soft signals | Counts, “latest” |
| EXP-C-079 | Tooltip | Later | Limits without clutter |
| EXP-C-080 | Toast / notification | Soft | Save/continue feedback (optional vs banner) |
| EXP-C-081 | Progress / spinner | Boot | Honest waiting |
| EXP-C-082 | Icon system | Concepts | Glanceable actions |
| EXP-C-083 | Scroll area | Libraries | Long Continue lists |
| EXP-C-084 | Separator | Cards | Quiet structure |
| EXP-C-085 | Form field + label | Ship | Accessible forms |

## I. Check-in & Guide (presentation)

| ID | Component | Visible/implied in | Purpose |
|---|---|---|---|
| EXP-C-090 | Consent panel | Ship PP-P01E | Explicit measurement consent |
| EXP-C-091 | Check-in dashboard | Ship + Phase 4 polish | Stats + calm forms |
| EXP-C-092 | Interview panel | Ship | Baseline / week-four |
| EXP-C-093 | Withdraw controls | Ship | Consent withdrawal |
| EXP-C-094 | Guide section cards | Ship | Trust documentation as show-able cards |

## J. Explicitly not needed (Product Proof wedge)

| ID | Tempting concept item | Why deferred / rejected for Experience |
|---|---|---|
| EXP-C-N01 | Live app window thumbnails from desktop | Ambient / sensing risk; out of Product Proof |
| EXP-C-N02 | AI assistant chat rail | Intelligence on critical path forbidden for wedge |
| EXP-C-N03 | System CPU/RAM gauges | Engineering dashboard; not interruption recovery |
| EXP-C-N04 | Audio mixer | Out of Product Proof scope |
| EXP-C-N05 | Phone mirroring | Capability expansion |
| EXP-C-N06 | Automation toggles (Focus/Meeting modes) | Capability expansion |
| EXP-C-N07 | Fabricated “recent activity” feed | Invented ambient narrative |

These may appear on concept boards as long-horizon vision; they must not enter
Experience Phase 2–4 without separate repository authority.
