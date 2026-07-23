# Risk Register

| Field | Value |
|-------|-------|
| **Purpose** | Track identified project risks, their impact, likelihood, and mitigation strategies |
| **Owner** | Project Lead |
| **Dependencies** | [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md), [Security Principles](SECURITY-PRINCIPLES.md) |
| **Update Process** | Review monthly. Add risks as identified. Update status during sprint retrospectives. |

---

## 1. Risk Assessment Matrix

| | Low Impact | Medium Impact | High Impact |
|---|-----------|---------------|-------------|
| **High Likelihood** | Monitor | Mitigate | Mitigate immediately |
| **Medium Likelihood** | Accept | Monitor | Mitigate |
| **Low Likelihood** | Accept | Monitor | Monitor |

---

## 2. Active Risks

### R-001: Technology Stack Undecided

| Field | Value |
|-------|-------|
| **Category** | Architecture |
| **Description** | No technology stack selected. Delaying this decision blocks Phase 1. |
| **Impact** | High — blocks all implementation |
| **Likelihood** | High — decision not yet made |
| **Status** | Open |
| **Mitigation** | Evaluate options against architecture principles. Record decision in Decision Log. See OQ-001. |
| **Owner** | Architect (TBD) |

### R-002: Scope Ambiguity

| Field | Value |
|-------|-------|
| **Category** | Product |
| **Description** | Product domains (apps, windows, devices, audio, automation, AI) are defined at vision level but not at feature level. Contributors may invent behaviour. |
| **Impact** | High — uncontrolled scope leads to incoherent product |
| **Likelihood** | Medium — mitigated by Open Questions process |
| **Status** | Mitigated (process in place) |
| **Mitigation** | [Scope Management](../01-Product/SCOPE-MANAGEMENT.md) and [Open Questions](../09-Decisions/OPEN-QUESTIONS.md) processes enforced. "Identify, don't guess" principle. |
| **Owner** | Product Owner (TBD) |

### R-003: AI Trust Failure

| Field | Value |
|-------|-------|
| **Category** | AI / Product |
| **Description** | If AI acts without permission or makes poor suggestions, users lose trust in the entire product. |
| **Impact** | High — core value proposition depends on trust |
| **Likelihood** | Medium — complex subsystem with many edge cases |
| **Status** | Mitigated (principles in place) |
| **Mitigation** | [AI Principles](../05-AI/AI-PRINCIPLES.md) with mandatory permission model. Permission gateway in architecture. |
| **Owner** | AI Lead (TBD) |

### R-004: Windows API Dependency

| Field | Value |
|-------|-------|
| **Category** | Architecture |
| **Description** | Workspace depends heavily on Windows APIs. Windows updates may break integrations. |
| **Impact** | Medium — degraded functionality |
| **Likelihood** | Medium — Windows updates are frequent |
| **Status** | Open |
| **Mitigation** | Windows Integration Layer abstracts API calls. Integration tests against Windows versions. Monitor Windows insider builds. |
| **Owner** | Architect (TBD) |

### R-005: Solo / Small Team Capacity

| Field | Value |
|-------|-------|
| **Category** | Engineering |
| **Description** | Project may begin with a small team. Six product domains plus AI and plugins is ambitious. |
| **Impact** | High — delayed delivery or quality compromise |
| **Likelihood** | High — team size unknown |
| **Status** | Open |
| **Mitigation** | Phased roadmap with clear priorities. Phase gates prevent premature expansion. AI contributors augment capacity. |
| **Owner** | Project Lead |

### R-006: Plugin Security

| Field | Value |
|-------|-------|
| **Category** | Security |
| **Description** | Third-party plugins could introduce security vulnerabilities or malicious behaviour. |
| **Impact** | High — user data and system integrity at risk |
| **Likelihood** | Medium — depends on plugin ecosystem size |
| **Status** | Mitigated (design phase) |
| **Mitigation** | Sandboxing, permission model, review process defined in [Plugin Architecture Vision](../06-Plugins/PLUGIN-ARCHITECTURE-VISION.md). |
| **Owner** | Security Lead (TBD) |

### R-007: Documentation Drift

| Field | Value |
|-------|-------|
| **Category** | Engineering |
| **Description** | As implementation begins, documentation may fall out of sync with code. |
| **Impact** | Medium — misleads contributors and AI agents |
| **Likelihood** | High — common in fast-moving projects |
| **Status** | Mitigated (process in place) |
| **Mitigation** | Documentation updates required in Definition of Done. Quarterly stale document review. |
| **Owner** | Lead Software Engineer |

### R-008: Over-Engineering in Foundation Phase

| Field | Value |
|-------|-------|
| **Category** | Engineering |
| **Description** | Excessive documentation or architecture without implementation validation. |
| **Impact** | Medium — wasted effort, wrong assumptions baked in |
| **Likelihood** | Medium |
| **Status** | Monitor |
| **Mitigation** | Phase gates require implementation validation. Open Questions track unresolved assumptions. Roadmap limits foundation phase duration. |
| **Owner** | Lead Software Engineer |

### R-009: Performance at Scale

| Field | Value |
|-------|-------|
| **Category** | Architecture |
| **Description** | Monitoring many apps, windows, devices, and audio endpoints simultaneously may cause performance issues. |
| **Impact** | Medium — degraded user experience |
| **Likelihood** | Medium — depends on implementation choices |
| **Status** | Open |
| **Mitigation** | Define performance budgets before implementation. Event-driven architecture reduces polling. Lazy loading for non-critical subsystems. |
| **Owner** | Architect (TBD) |

### R-010: License and Legal

| Field | Value |
|-------|-------|
| **Category** | Legal |
| **Description** | Project license not yet determined. Third-party dependencies may impose restrictions. |
| **Impact** | Medium — affects distribution and contribution |
| **Likelihood** | Medium |
| **Status** | Open |
| **Mitigation** | Determine license before Phase 1. Audit dependencies for license compatibility. See OQ-010. |
| **Owner** | Project Lead |

---

## 3. Closed Risks

_None yet._

---

## 4. Review Log

| Date | Reviewer | Changes |
|------|----------|---------|
| 2026-07-23 | Lead Software Engineer | Initial risk register created |
| 2026-07-23 | Lead Software Engineer | Phase 0 improvement pass — threat model added; R-001 mitigation updated |

---

## Related Documents

- [Security Principles](SECURITY-PRINCIPLES.md)
- [Open Questions](../09-Decisions/OPEN-QUESTIONS.md)
- [Roadmap](../08-Roadmap/ROADMAP.md)
