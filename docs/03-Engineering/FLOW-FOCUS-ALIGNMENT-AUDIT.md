# Flow / Focus — Alignment Audit

| Field | Value |
|-------|-------|
| **Status** | Analysis only — **no implementation** |
| **Date** | 2026-07-29 |
| **Branch** | `cursor/flow-focus-design-charter-34a5` |
| **Charter** | [FLOW-FOCUS-MODE-DESIGN-CHARTER.md](../01-Product/FLOW-FOCUS-MODE-DESIGN-CHARTER.md) |
| **References** | `docs/01-Product/references/workspace-concept-01.png`, `workspace-concept-02.png` |
| **Audience** | Product / engineering approval gate before Milestone B |

**Do not implement from this document without human approval.**

---

## Scores

| Dimension | Score | Why |
|-----------|------:|-----|
| **Reference alignment** | **8/10** | Charter maps concept-01 Flow ↔ Focus principles (density switch, apps stay open, Assistant side companion) without pixel copying. Concept-02 treated as vocabulary, not nine products. Gap: live OS stage still incomplete in the app today. |
| **Maintainability** | **8/10** | Modes designed to **extend** DesktopArrangement + WindowController + existing shell; forbids new engines and forked layout systems. Clear non-goals reduce tribal “mode framework” risk. |
| **Human usability** | **7/10** | Scenarios and hierarchies are concrete (multitask vs deep work). First ship still needs a simple mode control and honest empty states; scenic Focus art is deferred so users aren’t promised wallpaper magic. |
| **Implementation risk** | **6/10** | Medium: OS geometry apply + mode metadata decisions; confusion between canvas zones and OS arrangements; Assistant-as-persistent-rail still Milestone C-ish chrome. Lower if first ship is chrome-density only; higher if first ship applies Win32 transforms on switch. |

**Overall readiness to design:** High.  
**Overall readiness to code:** Blocked on human approval + scope choice (chrome-only vs geometry apply).

---

## Reference alignment detail

| Concept principle | Charter coverage | Current product gap |
|-------------------|------------------|---------------------|
| High-density Flow | Defined | Stage is registry/canvas-heavy, not live multitask desktop |
| Low-density Focus | Defined | No mode control |
| Apps stay open | Explicit rule | N/A until mode ships |
| User-driven switch | Explicit rule | Missing UI |
| Assistant side rail both modes | Explicit | Companion only when Assistant tool selected |
| Audio / utilities | Deferred honestly | Future controls |

---

## Maintainability detail

**Good path:** Mode as presentation + optional arrangement variant IDs; reuse restore diagnostics; no AI.

**Bad path (forbidden by charter):** New “ModeEngine”, recommendation of layouts, autonomous switch, duplicate geometry store.

---

## Human usability detail

Strengths: clear purpose/scenarios; visible vs hidden controls listed.  
Watch-outs: users may expect Focus = kill apps; charter forbids that. Copy must stay explicit at ship time.

---

## Implementation risk detail

| Risk | Level | Mitigation |
|------|-------|------------|
| Scope creep into nine presets | Med | Charter: two modes only |
| Geometry apply failures on Windows | Med | Reuse restore gaps; smoke on Windows |
| Canvas vs OS confusion | Med | Naming + Layouts stage honesty |
| Persistent Assistant rail vs tools tab | Med | Coordinate with Milestone C or ship mode without forcing full sidecar |
| AI sneaking into mode suggest | Low | Governance + charter non-goals |

---

## Recommendation

| Decision | Ask human |
|----------|-----------|
| Approve charter? | ☐ Yes ☐ Changes requested |
| First implementation scope | ☐ Chrome density only ☐ Also OS arrangement apply on switch |
| Proceed to Milestone B code? | ☐ Approved ☐ Deferred |

**Agent recommendation:** Approve charter; prefer **chrome-density-first** Milestone B slice if risk must stay low, then geometry apply once restore is Windows-verified.

---

## Explicit confirmation

> Audit only. No application code modified for Flow/Focus.  
> Awaiting human approval before any implementation batch.
