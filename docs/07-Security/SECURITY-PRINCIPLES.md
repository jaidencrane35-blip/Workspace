# Security Principles

| Field | Value |
|-------|-------|
| **Purpose** | Define the security values, constraints, and review requirements for Workspace |
| **Owner** | Security Lead (TBD) |
| **Dependencies** | [Project Constitution](../00-Constitution/PROJECT-CONSTITUTION.md), [Architecture Principles](../02-Architecture/ARCHITECTURE-PRINCIPLES.md), [AI Principles](../05-AI/AI-PRINCIPLES.md) |
| **Update Process** | Security Lead proposes changes. Material changes require Decision Log entry. |

---

## 1. Security Mission

Workspace operates at the intersection of applications, devices, audio, automation, and AI — all on a user's personal machine. Security failures erode the trust that is foundational to the product. Security is not a feature; it is a requirement.

---

## 2. Core Principles

### 2.1 Least Privilege

- Every component, plugin, and AI module operates with the minimum permissions required
- Permissions are explicit, declared, and user-approved
- No component grants itself additional access

### 2.2 Local-First Security

- User data stays on the user's machine by default
- External communication requires explicit opt-in
- No telemetry without informed consent
- Data export and deletion are always available to the user

### 2.3 Defense in Depth

- Security at every layer: OS integration, platform kernel, domain services, plugins, AI
- No single point of failure for security controls
- Assume any component can be compromised — limit blast radius

### 2.4 Secure by Default

- Safe defaults for all settings
- Insecure configurations require explicit user action
- New features are secure before they are functional

### 2.5 Transparency

- User can see what data Workspace collects, stores, and transmits
- Permission grants are visible and revocable
- Security-relevant actions are logged

### 2.6 No Secrets in Code

- API keys, credentials, and tokens never in source code
- Secrets managed via environment variables or secure storage
- `.gitignore` and pre-commit checks enforce this

---

## 3. Threat Categories

| Category | Examples | Mitigation Approach |
|----------|----------|---------------------|
| **Unauthorized automation** | AI or plugin acts without permission | Permission gateway; audit trail |
| **Data exfiltration** | Plugin or module sends user data externally | Sandboxing; network controls; opt-in |
| **Privilege escalation** | Plugin gains system-level access | Sandboxing; least privilege; permission model |
| **Malicious plugins** | Third-party plugin with harmful intent | Review process; sandboxing; permission approval |
| **Local data exposure** | Layout/workflow data accessible to other apps | File permissions; encryption at rest (TBD) |
| **Supply chain** | Compromised dependency | Dependency auditing; lock files; review |
| **Input validation** | Malformed input causes crashes or code execution | Validate at all boundaries |

Full threat model: [Threat Model](THREAT-MODEL.md). Initial analysis completed in Phase 0 improvement pass.

---

## 4. Security Requirements by Subsystem

### 4.1 Platform Kernel

- Permission enforcement for all state-changing operations via the Permission Gateway (see [Permission Architecture](PERMISSION-ARCHITECTURE.md))
- Audit logging for security-relevant events
- Secure configuration storage
- Future AI / plugin / automation actors enter the same authority path; they do not receive private execution APIs

### 4.2 AI Subsystem

- Cannot observe keystrokes, credentials, or private content
- Cannot act without permission
- Learned data stored locally; no external transmission by default
- See [AI Principles](../05-AI/AI-PRINCIPLES.md)

### 4.3 Plugin Runtime

- Sandboxed execution
- Declared permissions approved by user
- No direct OS access
- See [Plugin Architecture Vision](../06-Plugins/PLUGIN-ARCHITECTURE-VISION.md)

### 4.4 Windows Integration

- Use Windows security APIs correctly
- Handle elevation requests transparently
- Respect Windows security policies

### 4.5 Data Storage

- User data encrypted at rest (implementation TBD)
- Secure deletion when user clears data
- No data in world-readable locations

---

## 5. Security Review Process

| Trigger | Review Type |
|---------|-------------|
| New subsystem or module | Threat model review |
| Plugin API changes | Sandbox and permission review |
| AI behaviour changes | Observation and permission boundary review |
| New external integration | Data flow and transmission review |
| Dependency addition | License and vulnerability check |
| Pre-release | Full security review |

---

## 6. Vulnerability Handling

When a vulnerability is discovered:

1. Assess severity (Critical / High / Medium / Low)
2. Critical and High: fix before next release; do not disclose until patched
3. Document in security advisory
4. Notify users if their data or system was at risk
5. Post-mortem for Critical vulnerabilities

Specific disclosure policy to be defined before public release.

---

## 7. Compliance Considerations

Compliance requirements depend on distribution model and data handling. Open questions:

- GDPR (if EU users)
- Data residency requirements
- Windows Store security requirements (if distributed via Store)

See [Open Questions](../09-Decisions/OPEN-QUESTIONS.md).

---

## 8. Security Anti-Patterns

| Anti-Pattern | Why |
|--------------|-----|
| Security through obscurity | Does not work; assume attackers know the system |
| Hardcoded credentials | Trivially exploitable |
| Disabled security for debugging in production | Creates permanent vulnerabilities |
| Trusting plugin code | All plugins are untrusted until proven otherwise |
| Logging sensitive data | Logs become an attack vector |
| Skipping input validation | Primary source of vulnerabilities |

---

## Related Documents

- [AI Principles](../05-AI/AI-PRINCIPLES.md)
- [Plugin Architecture Vision](../06-Plugins/PLUGIN-ARCHITECTURE-VISION.md)
- [Risk Register](RISK-REGISTER.md)
- [Threat Model](THREAT-MODEL.md)
- [Architecture Principles](../02-Architecture/ARCHITECTURE-PRINCIPLES.md)
