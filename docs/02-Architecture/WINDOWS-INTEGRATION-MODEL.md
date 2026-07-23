# Windows Integration Model

| Field | Value |
|-------|-------|
| **Purpose** | Document approaches for how Workspace coexists with and integrates into Windows — without selecting an approach |
| **Owner** | Project Owner |
| **Dependencies** | [Architecture Principles](ARCHITECTURE-PRINCIPLES.md), [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md), [Stack Evaluation Criteria](STACK-EVALUATION-CRITERIA.md) |
| **Update Process** | Update when OQ-014 is resolved. Record chosen approach in Decision Log. |

---

## 1. Constraint

Workspace **must not replace Windows**. It must operate as a layer that enhances the existing desktop experience. Any approach that replaces Explorer as the default shell is disqualified.

This decision is tracked as **OQ-014**.

---

## 2. The Question

How does Workspace present itself on Windows while integrating with applications, windows, audio, and devices?

---

## 3. Possible Approaches

### 3.1 Overlay Approach

Workspace renders as a full-screen or partial overlay above the Windows desktop.

| Aspect | Detail |
|--------|--------|
| **Description** | Workspace UI floats over the existing desktop. Windows apps continue running underneath or alongside. |
| **User experience** | User toggles Workspace overlay on/off. Desktop remains visible when overlay is dismissed. |
| **Windows integration** | Observes windows via Win32/WinRT APIs; does not manage the shell |
| **Pros** | Clear separation; easy to dismiss; low risk of breaking Windows |
| **Cons** | May feel disconnected from native apps; z-order and focus management complexity |
| **Stack impact** | Most stacks support overlay windows; transparency and click-through require care |

### 3.2 Companion Application Approach

Workspace runs as a standard desktop application alongside other apps.

| Aspect | Detail |
|--------|--------|
| **Description** | Workspace is a normal Windows application with its own window. It manages an internal workspace view but does not overlay the full desktop. |
| **User experience** | User opens Workspace like any app. Other apps run in separate windows. Workspace may include panels for app launching and layout management. |
| **Windows integration** | Standard application APIs; window enumeration for observation |
| **Pros** | Simplest coexistence model; familiar to users; lowest Windows integration risk |
| **Cons** | Less "unified environment" feel; apps not truly embedded in Workspace |
| **Stack impact** | Works with all candidate stacks; least demanding |

### 3.3 Deeper System Integration Approach

Workspace integrates more deeply with the Windows shell without replacing it.

| Aspect | Detail |
|--------|--------|
| **Description** | Workspace registers shell extensions, custom taskbar elements, or system tray services. May manage window zones or provide enhanced snap layouts. |
| **User experience** | Workspace feels native to Windows — enhanced taskbar, shell context menus, or window management shortcuts. |
| **Windows integration** | Shell hooks, registry entries, COM registration, possibly Windows Shell extensions |
| **Pros** | Deepest integration; most "Windows feels better" outcome |
| **Cons** | Higher complexity; Windows update fragility (R-004); UAC/elevation requirements; harder to uninstall cleanly |
| **Stack impact** | Native .NET/C++ has advantage; Electron/Tauri may require helper processes |

### 3.4 Hybrid Approach

Combines elements of the above — e.g., companion app with optional overlay mode and selective shell enhancements.

| Aspect | Detail |
|--------|--------|
| **Description** | Core experience is companion app; overlay available for layout mode; shell enhancements added incrementally. |
| **User experience** | Progressive integration — starts simple, adds depth over phases. |
| **Pros** | Flexible; MVP can start minimal; deep integration added later |
| **Cons** | More architectural complexity; multiple integration paths to maintain |
| **Stack impact** | Depends on which hybrid elements are chosen |

---

## 4. Comparison Matrix

| Criterion | Overlay | Companion | Deep Integration | Hybrid |
|-----------|---------|-----------|------------------|--------|
| Constitution compliance | Yes | Yes | Yes (if not shell replacement) | Yes |
| MVP simplicity | Medium | High | Low | Medium |
| "Unified workspace" feel | High | Low | High | Medium–High |
| Windows update risk | Low | Low | High | Medium |
| UAC/elevation needs | Low | Low | Medium–High | Varies |
| Phase 1 feasibility | Medium | High | Low | Medium |
| Stack flexibility | Medium | High | Low–Medium | Medium |

---

## 5. Integration Layer Requirements (All Approaches)

Regardless of approach, the **Windows Integration Layer** must provide:

| Capability | Required For |
|------------|-------------|
| Window enumeration and events | Window service, AI observation |
| Process launch | Application service |
| Audio endpoint management | Audio service (Phase 2) |
| System tray / background operation | Persistent observation |
| File system access (controlled) | Layout and config persistence |
| Device connection events | Device service (Phase 2+) |

Only the Windows Integration Layer communicates directly with OS APIs. See [System Overview](SYSTEM-OVERVIEW.md).

---

## 6. Decision Dependencies

| Decision | Relationship |
|----------|-------------|
| OQ-001 (tech stack) | Influences feasibility of each approach |
| OQ-013 (layout system) | Overlay approach strongly coupled to layout design |
| MVP scope | Companion approach simplest for MVP |
| R-004 (Windows API dependency) | Deep integration has highest exposure |

---

## 7. Recommendation Process

1. Project Owner reviews approaches against MVP goals ([MVP Definition](../01-Product/MVP-DEFINITION.md))
2. Score approaches using [Stack Evaluation Criteria](STACK-EVALUATION-CRITERIA.md) Windows integration section
3. Prototype feasibility assessment (Phase 1 early — spike only, not production)
4. Record decision as DEC-00N in Decision Log
5. Update System Overview and architecture diagrams

**No approach is selected in this document.**

---

## Related Documents

- [Stack Evaluation Criteria](STACK-EVALUATION-CRITERIA.md)
- [System Overview](SYSTEM-OVERVIEW.md)
- [MVP Definition](../01-Product/MVP-DEFINITION.md)
- [Open Questions](../09-Decisions/OPEN-QUESTIONS.md) — OQ-014
