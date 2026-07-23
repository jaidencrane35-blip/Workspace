# UX Principles

| Field | Value |
|-------|-------|
| **Purpose** | Define the interaction design values and constraints that govern all user-facing experiences in Workspace |
| **Owner** | UX Lead (TBD) |
| **Dependencies** | [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md), [Product Vision](../01-Product/PRODUCT-VISION.md) |
| **Update Process** | UX Lead proposes changes. Material changes require Product Owner review and Decision Log entry. |

---

## 1. UX Mission

Design an experience where users feel their desktop works for them — not the other way around. Every interaction should reinforce control, consistency, and clarity.

---

## 2. Core Principles

### 2.1 User Control

- The user initiates all meaningful actions
- AI suggestions are offers, not commands
- Automations require explicit approval before activation
- Undo must be available for reversible actions
- Settings and preferences are always accessible

### 2.2 Consistency

- Navigation locations remain in the same place between sessions
- Panel behaviour is predictable — same resize, move, and dock rules everywhere
- Interaction patterns repeat across domains (apps, windows, devices, audio)
- Terminology is consistent — one concept, one name

### 2.3 Customisation Without Complexity

- Everything is customisable, but sensible defaults exist
- Customisation is progressive — simple options first, advanced options available
- Users should never be forced to configure before they can use Workspace
- Modes are presets, not restrictions — switching modes never removes capability

### 2.4 Adaptation, Not Disruption

- Workspace learns and adapts to the user
- Layout and workflow changes are suggested, not imposed
- Visual changes are gradual and explained
- The user can always revert to a prior state

### 2.5 Clarity Over Cleverness

- UI communicates state clearly — what is active, what is available, what is happening
- No hidden functionality — if it exists, the user can find it
- Error states are helpful, not cryptic
- Loading and progress are visible for operations that take time

### 2.6 Respect for Context

- Workspace does not interrupt focused work for non-urgent suggestions
- Notification and suggestion timing respects user activity
- Audio, device, and app contexts are understood before making recommendations
- Do-not-disturb and focus modes are honoured

---

## 3. Layout and Panel Principles

| Principle | Rule |
|-----------|------|
| **Resizable** | All panels can be resized by the user |
| **Movable** | All panels can be moved and repositioned |
| **Persistent** | Layouts are saved and restored between sessions |
| **Evolvable** | Layouts can change over time with user approval |
| **Recoverable** | Previous layouts can be restored |

Specific layout systems (grid, free-form, zones) are design decisions for future phases.

---

## 4. Navigation Principles

- Primary navigation is always visible or one action away
- Navigation structure is stable — items do not move without user action
- Current location is always indicated
- Breadcrumbs or equivalent for deep navigation
- Keyboard navigation supported for all primary actions

---

## 5. AI Interaction Principles

AI in the UI must follow these rules:

| Rule | Detail |
|------|--------|
| **Suggest, don't act** | AI presents suggestions as dismissible prompts |
| **Explain why** | Every suggestion includes its reasoning |
| **Easy dismissal** | One action to dismiss; no repeated nagging for dismissed suggestions |
| **Approval is explicit** | Automations require a clear accept action — not pre-checked boxes |
| **Transparent learning** | User can see what Workspace has learned about their patterns |
| **Reset available** | User can clear learned patterns at any time |

See [AI Principles](../05-AI/AI-PRINCIPLES.md) for the full AI interaction model.

---

## 6. Accessibility (Baseline)

Accessibility requirements will be fully defined before implementation. Baseline expectations:

- Keyboard navigable
- Screen reader compatible
- Sufficient colour contrast
- Scalable text and UI elements
- No information conveyed by colour alone

Specific WCAG target level is an open decision. See [Open Questions](../09-Decisions/OPEN-QUESTIONS.md).

---

## 7. Visual Design Constraints

Visual design (colours, typography, iconography) is not defined in this phase. Constraints:

- Must feel native to Windows, not foreign
- Must support light and dark modes
- Must not clash with Windows system theme without user choice
- Visual identity decisions require UX Lead and Product Owner approval

---

## 8. UX Anti-Patterns

| Anti-Pattern | Why |
|--------------|-----|
| Modal overload | Interrupts flow; use inline and panel-based interactions |
| Surprise layout changes | Violates consistency and trust |
| Hidden destructive actions | User must always know what will happen |
| Pre-checked automation consent | Violates permission model |
| Un-dismissable suggestions | Violates user control |
| Forced onboarding wizard | User should reach value immediately |
| Identical UI for all modes | Modes are presets with purpose — they should feel different |

---

## 9. UX Decision Process

1. Propose interaction pattern with rationale
2. Validate against these principles
3. Product Owner review for user-facing changes
4. Prototype if pattern is novel (future phase)
5. Record decision if precedent-setting

---

## Related Documents

- [Product Vision](../01-Product/PRODUCT-VISION.md)
- [AI Principles](../05-AI/AI-PRINCIPLES.md)
- [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md)
