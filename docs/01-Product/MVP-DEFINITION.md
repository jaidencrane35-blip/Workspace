# MVP Definition

| Field | Value |
|-------|-------|
| **Purpose** | Define the first useful product slice — the minimum experience that delivers real user value |
| **Owner** | Project Owner |
| **Dependencies** | [Product Vision](PRODUCT-VISION.md), [Scope Management](SCOPE-MANAGEMENT.md), [Roadmap](../08-Roadmap/ROADMAP.md) |
| **Update Process** | Project Owner approves changes. Scope changes require Decision Log entry. |

---

## 1. MVP Goal

Deliver a workspace the user can open, arrange, save, and restore — with AI that observes patterns and suggests (never imposes) user-approved automations.

**Success statement:** *"I opened Workspace, set up my layout, came back tomorrow, and everything was where I left it. It noticed I always open Discord after Steam and asked if I wanted that automated."*

---

## 2. MVP Scope — Included

### 2.1 Core Shell

| Capability | Description |
|------------|-------------|
| Open Workspace | User launches Workspace as a desktop application |
| Workspace layout | User arranges panels within the shell |
| Save layout | User saves current panel arrangement |
| Restore layout | Layout persists and restores between sessions |
| Resize and move panels | Panels are user-resizable and movable per UX Principles |

### 2.2 Application Integration

| Capability | Description |
|------------|-------------|
| Discover installed apps | Workspace lists installed applications |
| Launch applications | User launches apps from Workspace |
| App grouping (basic) | User can group apps within the shell |

### 2.3 AI — Observe and Suggest

| Capability | Description |
|------------|-------------|
| Observe usage patterns | AI passively monitors permitted events (app launch order, focus changes) |
| Detect patterns | AI identifies recurring sequences meeting confidence thresholds |
| Suggest improvements | AI presents dismissible suggestions with explanation |
| Request approval | User explicitly approves or dismisses each suggestion |

### 2.4 User-Approved Automation

| Capability | Description |
|------------|-------------|
| One automation type | App launch sequence (e.g., "open Discord after Steam") |
| Approval required | User chooses one-time, session, or persistent approval |
| Revoke automation | User can stop any active automation from settings |
| View active automations | User sees all approved automations |

---

## 3. MVP Scope — Explicitly Excluded

The following are **not** in MVP:

| Excluded | Reason |
|----------|--------|
| Autonomous control | Violates constitution — AI never acts without permission |
| Unrestricted automation | Only one automation type; all require approval |
| Plugin marketplace | Phase 3 scope |
| Plugin SDK | Phase 3 scope |
| Phone / device integration | Phase 2+ scope |
| Audio routing and management | Phase 2 scope |
| Window management beyond basic | Phase 2 scope |
| Cloud sync | Deferred — OQ-005 |
| Multi-monitor advanced layouts | Post-MVP |
| Production visual design | Phase 1 prototype; polish in later phases |
| Screen-aware AI features | Privacy-sensitive — requires separate approval |
| Cross-device learning | Prohibited until OQ-018 resolved |

---

## 4. MVP User Flows

### Flow 1: First Launch

1. User opens Workspace
2. Default layout presented with app discovery panel
3. User arranges panels
4. User saves layout (explicit or prompted)
5. AI observation begins (with visible indicator)

### Flow 2: Return Visit

1. User opens Workspace
2. Saved layout restored
3. User launches apps from Workspace
4. AI continues observing

### Flow 3: AI Suggestion

1. AI detects pattern (e.g., Discord after Steam, 5+ times)
2. User receives dismissible suggestion with explanation
3. User chooses: Accept (persistent) / Just once / No thanks
4. If accepted, automation registered and visible in settings

### Flow 4: Revoke Automation

1. User opens settings
2. User views active automations
3. User revokes automation
4. Automation stops immediately; pattern data retained unless user clears

---

## 5. MVP Acceptance Criteria

MVP is complete when all criteria pass Definition of Done:

- [ ] Workspace opens and renders shell on Windows
- [ ] User can arrange, resize, and move panels
- [ ] Layout saves and restores between sessions
- [ ] User can discover and launch installed applications
- [ ] AI observes app launch and focus events
- [ ] AI suggests automation when confidence reaches L3
- [ ] User can approve, dismiss, or revoke suggestions
- [ ] Approved app-launch automation executes correctly
- [ ] No action occurs without explicit user approval
- [ ] AI memory viewable and deletable by user
- [ ] CI pipeline passes on all PRs
- [ ] Core paths covered by automated tests

---

## 6. MVP Phase Mapping

| MVP Component | Roadmap Phase |
|---------------|---------------|
| Shell, layout persistence, app launch | Phase 1 (Core Platform) |
| AI observation, suggestion, basic automation | Phase 2 (Feature Expansion) |

MVP spans Phase 1 and Phase 2. Phase 1 delivers the shell; Phase 2 delivers AI and automation.

---

## 7. MVP Non-Goals

- Perfect visual design
- Complete domain coverage (audio, devices, windows)
- Performance optimisation beyond [Performance Budgets](../02-Architecture/PERFORMANCE-BUDGETS.md) targets
- Public release readiness (installer, updates, documentation)

---

## Related Documents

- [Product Vision](PRODUCT-VISION.md)
- [Roadmap](../08-Roadmap/ROADMAP.md)
- [AI Operating Model](../05-AI/AI-OPERATING-MODEL.md)
- [Scope Management](SCOPE-MANAGEMENT.md)
