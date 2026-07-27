# Threat Model (Initial)

| Field | Value |
|-------|-------|
| **Purpose** | Identify threats to Workspace subsystems and define mitigations as implementation evolves |
| **Owner** | Project Owner |
| **Dependencies** | [Security Principles](SECURITY-PRINCIPLES.md), [AI Operating Model](../05-AI/AI-OPERATING-MODEL.md), [Plugin Architecture Vision](../06-Plugins/PLUGIN-ARCHITECTURE-VISION.md), [System Overview](../02-Architecture/SYSTEM-OVERVIEW.md) |
| **Update Process** | Review before Phase 1 implementation and after each major subsystem is added. Update when new attack surface is introduced. |

---

## 1. Scope

This threat model covers the implemented architecture baseline and is iteratively refined as subsystems evolve. Technology stack and Windows integration decisions are resolved (DEC-007, DEC-008).

**In scope:** Platform Kernel, Domain Services, AI Subsystem, Plugin Runtime, Windows Integration Layer, stored user data.

**Out of scope (this revision):** Network services, cloud sync, plugin marketplace, mobile companion.

---

## 2. Methodology

Analysis uses **STRIDE** categorisation:

| Category | Description |
|----------|-------------|
| **S**poofing | Impersonating something or someone |
| **T**ampering | Modifying data or code |
| **R**epudiation | Denying actions occurred |
| **I**nformation disclosure | Exposing data to unauthorised parties |
| **D**enial of service | Disrupting availability |
| **E**levation of privilege | Gaining capabilities without authorisation |

---

## 3. System Boundaries

```
┌─────────────────────────────────────────────────┐
│  Trust Boundary: Workspace Process              │
│  ┌─────────┐ ┌─────────┐ ┌──────────────────┐  │
│  │  Shell  │ │ Domains │ │  AI Subsystem    │  │
│  └────┬────┘ └────┬────┘ └────────┬─────────┘  │
│       └───────────┼───────────────┘             │
│                   ▼                             │
│  ┌────────────────────────────────────────────┐ │
│  │  Platform Kernel + Permission Gateway      │ │
│  └────────────────────┬───────────────────────┘ │
│  ┌────────────────────▼───────────────────────┐ │
│  │  Plugin Runtime (untrusted code)           │ │
│  └────────────────────┬───────────────────────┘ │
│  ┌────────────────────▼───────────────────────┐ │
│  │  Windows Integration Layer                 │ │
│  └────────────────────┬───────────────────────┘ │
└───────────────────────┼─────────────────────────┘
                        ▼
┌─────────────────────────────────────────────────┐
│  Trust Boundary: Windows OS + Other Processes   │
└─────────────────────────────────────────────────┘
                        ▼
┌─────────────────────────────────────────────────┐
│  Trust Boundary: External (future cloud/sync)    │
└─────────────────────────────────────────────────┘
```

---

## 4. Threat Analysis by Subsystem

### 4.1 Platform Kernel

| ID | Threat | STRIDE | Description | Initial Mitigation |
|----|--------|--------|-------------|-------------------|
| TK-01 | Permission bypass | E | Component calls domain service without Permission Gateway | Gateway as sole path for state changes; code review; architectural tests |
| TK-02 | Event bus injection | T | Malicious events injected to trigger automations | Event source authentication; schema validation |
| TK-03 | Config tampering | T | User preferences modified without authorisation | File permissions; integrity checks |
| TK-04 | Audit log deletion | R, T | Security events erased to hide actions | Append-only audit log; protected storage |
| TK-05 | Kernel crash | D | Kernel failure disables entire application | Graceful degradation; subsystem isolation (OQ-002) |

### 4.2 AI Subsystem

| ID | Threat | STRIDE | Description | Initial Mitigation |
|----|--------|--------|-------------|-------------------|
| TA-01 | Unauthorised automation | E | AI executes action without user approval | Permission Gateway; AI cannot call domain services directly |
| TA-02 | Observation scope creep | I | AI observes keystrokes, credentials, or screen content | [Memory Policy](../05-AI/MEMORY-POLICY.md) prohibitions; domain toggles |
| TA-03 | Prompt injection | T, E | Manipulated input causes AI to suggest harmful automations | Input sanitisation; suggestion content validation; no direct OS access |
| TA-04 | Pattern poisoning | T | Adversarial behaviour trains bad patterns | Confidence thresholds; user approval required; pattern inspection |
| TA-05 | Data exfiltration via AI | I | Learned patterns transmitted externally | Local-first default; no external transmission without opt-in |
| TA-06 | Suggestion manipulation | T | AI uses dark patterns to increase acceptance | UX rules prohibit pre-checked consent; [Confidence Policy](../05-AI/CONFIDENCE-POLICY.md) |
| TA-07 | AI privilege escalation | E | AI grants itself broader permissions | AI cannot modify permission grants; kernel enforces |

### 4.3 Plugin Runtime

| ID | Threat | STRIDE | Description | Initial Mitigation |
|----|--------|--------|-------------|-------------------|
| TP-01 | Malicious plugin | E, I | Plugin exfiltrates user data or executes harmful actions | Sandboxing; declared permissions; user approval at install |
| TP-02 | Permission escalation | E | Plugin gains permissions beyond declaration | Runtime permission enforcement; no runtime escalation |
| TP-03 | Plugin-to-plugin attack | I, T | One plugin accesses another's data | Plugin isolation; separate storage per plugin |
| TP-04 | Supply chain attack | T | Compromised plugin package in registry | Review process; signature verification (future) |
| TP-05 | Filesystem escape | E | Plugin accesses files outside designated area | Sandbox filesystem restrictions |
| TP-06 | Direct OS API access | E | Plugin bypasses Workspace APIs | All OS access through Windows Integration Layer |

### 4.4 Windows Integration Layer

| ID | Threat | STRIDE | Description | Initial Mitigation |
|----|--------|--------|-------------|-------------------|
| TW-01 | Excessive elevation | E | Workspace requests unnecessary UAC elevation | Least privilege; transparent elevation prompts |
| TW-02 | Windows API abuse | T, E | Integration layer used to manipulate other apps maliciously | Permission Gateway; audit logging |
| TW-03 | API breaking changes | D | Windows update breaks integration | Abstraction layer; version compatibility tests |
| TW-04 | DLL hijacking | T | Malicious DLL loaded via integration layer | Secure loading paths; code signing |

### 4.5 Stored User Data

| ID | Threat | STRIDE | Description | Initial Mitigation |
|----|--------|--------|-------------|-------------------|
| TD-01 | Local data exposure | I | Layout, patterns, or preferences readable by other apps | File permissions; tiered encryption at rest ([DEC-015](../09-Decisions/DECISION-LOG.md)) |
| TD-02 | Data not deleted on uninstall | I | User data remains after removal | Secure deletion procedure |
| TD-03 | Backup exposure | I | Sensitive data in unencrypted backups | User notification; encryption guidance |
| TD-04 | Memory dump exposure | I | Pattern data readable from process memory | Minimise sensitive data in memory; secure allocation |

### 4.6 Domain Services

| ID | Threat | STRIDE | Description | Initial Mitigation |
|----|--------|--------|-------------|-------------------|
| TS-01 | Unauthorised app launch | E | Automation or plugin launches app without approval | Permission Gateway for all launches |
| TS-02 | Audio hijacking | T | Automation changes audio routing maliciously | User approval; audio domain permissions |
| TS-03 | Layout manipulation | T | External component modifies saved layouts | Layout ownership by user; permission checks |

---

## 5. Cross-Cutting Threats

| ID | Threat | Affected Subsystems | Initial Mitigation |
|----|--------|---------------------|-------------------|
| TX-01 | Supply chain (dependencies) | All | [Dependency Policy](../03-Engineering/DEPENDENCY-POLICY.md) |
| TX-02 | Secrets in source code | All | `.gitignore`; pre-commit scanning; CI checks |
| TX-03 | Insufficient input validation | All | Validate at system boundaries |
| TX-04 | Logging sensitive data | AI, Kernel, Plugins | Log sanitisation policy |
| TX-05 | AI development tool confusion | Governance | Separate product AI from Cursor agent governance |

---

## 6. Risk Prioritisation

| Priority | Threat IDs | Action Required |
|----------|-----------|-----------------|
| **Critical** | TA-01, TK-01, TP-01, TA-03 | Mitigate in architecture before first automation code |
| **High** | TA-02, TA-05, TP-02, TD-01, TW-01 | Mitigate in Phase 1–2 |
| **Medium** | TA-04, TA-06, TP-03, TW-03, TD-02 | Mitigate before public release |
| **Low** | TW-04, TD-03, TX-05 | Address during hardening |

---

## 7. Security Testing Requirements

Before MVP release, verify:

- [ ] Permission Gateway blocks unapproved state changes (automated test)
- [ ] AI cannot call domain services directly (architectural test)
- [ ] Observation respects disabled domains (automated test)
- [ ] Plugin sandbox prevents filesystem escape (when plugins exist)
- [ ] No secrets in repository (CI check)
- [ ] Dependency vulnerability scan passes (CI check)

See [Testing Strategy](../03-Engineering/TESTING-STRATEGY.md).

---

## 8. Review Schedule

| Trigger | Action |
|---------|--------|
| Major architecture change | Review against current stack and Windows model |
| AI subsystem implementation | Deep review of TA-* threats |
| Plugin runtime implementation | Deep review of TP-* threats |
| Pre-release | Full threat model review |
| Security incident | Immediate review and update |

---

## Related Documents

- [Security Principles](SECURITY-PRINCIPLES.md)
- [Risk Register](RISK-REGISTER.md)
- [AI Operating Model](../05-AI/AI-OPERATING-MODEL.md)
- [Memory Policy](../05-AI/MEMORY-POLICY.md)
- [Plugin Architecture Vision](../06-Plugins/PLUGIN-ARCHITECTURE-VISION.md)
