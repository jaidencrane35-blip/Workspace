# Event and API Standards

| Field | Value |
|-------|-------|
| **Purpose** | Define future rules for event naming, module communication, API contracts, and versioning |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [Architecture Principles](ARCHITECTURE-PRINCIPLES.md), [System Overview](SYSTEM-OVERVIEW.md), [Coding Standards](../03-Engineering/CODING-STANDARDS.md) |
| **Update Process** | Update when first module APIs are defined. Breaking changes require Decision Log entry and migration plan. |

---

## 1. Status

**Standards defined; no implementation yet.** These rules apply when Phase 1 modules are created. Specific schemas will be added as modules are designed.

---

## 2. Communication Model

Modules communicate through two mechanisms:

| Mechanism | Use Case |
|-----------|----------|
| **Event bus** | Broadcast notifications (observation, automation triggers, state changes) |
| **Service API** | Request/response operations (launch app, save layout, query state) |

Direct cross-module imports of internal implementation are prohibited. See [Architecture Principles](ARCHITECTURE-PRINCIPLES.md) §2.3.

---

## 3. Event Naming Conventions

### 3.1 Format

```
<domain>.<entity>.<action>
```

All lowercase. Dot-separated. Past tense for completed actions.

### 3.2 Examples

| Event | Description |
|-------|-------------|
| `app.application.launched` | An application was launched |
| `app.application.focused` | An application gained focus |
| `app.application.closed` | An application was closed |
| `window.window.created` | A window was created |
| `window.window.moved` | A window was moved |
| `audio.device.changed` | Default audio device changed |
| `audio.volume.adjusted` | Volume level changed |
| `layout.panel.moved` | User moved a panel |
| `layout.layout.saved` | User saved a layout |
| `automation.rule.approved` | User approved an automation |
| `automation.rule.revoked` | User revoked an automation |
| `automation.rule.executed` | An automation ran |
| `ai.pattern.detected` | AI detected a new pattern |
| `ai.suggestion.presented` | AI showed a suggestion to user |
| `ai.suggestion.dismissed` | User dismissed a suggestion |
| `permission.request.submitted` | Permission request sent to gateway |
| `permission.request.granted` | User granted permission |
| `permission.request.denied` | User denied permission |

### 3.3 Event Domains

| Domain Prefix | Owner Module |
|---------------|-------------|
| `app.*` | Application Service |
| `window.*` | Window Service |
| `audio.*` | Audio Service |
| `device.*` | Device Service |
| `layout.*` | Shell |
| `automation.*` | Automation Service |
| `ai.*` | AI Subsystem |
| `permission.*` | Platform Kernel (Permission Gateway) |
| `plugin.*` | Plugin Runtime |
| `system.*` | Platform Kernel |

---

## 4. Event Schema Rules

Every event must include:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string (UUID) | Yes | Unique event identifier |
| `type` | string | Yes | Event name (e.g., `app.application.launched`) |
| `timestamp` | ISO 8601 | Yes | When the event occurred |
| `source` | string | Yes | Module that emitted the event |
| `payload` | object | Yes | Event-specific data |
| `version` | string | Yes | Schema version (semver) |

### 4.1 Payload Rules

- Payloads contain metadata only — no user content (keystrokes, file contents)
- Payloads must not contain credentials or secrets
- Maximum payload size: 4 KB (configurable; large data passed by reference)
- Payload schemas defined per event type in module documentation

### 4.2 Example

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "type": "app.application.launched",
  "timestamp": "2026-07-23T10:00:00Z",
  "source": "domain-apps",
  "version": "1.0.0",
  "payload": {
    "applicationId": "steam",
    "applicationName": "Steam",
    "launchMethod": "user"
  }
}
```

Exact format (JSON, protobuf, etc.) determined with stack selection.

---

## 5. Module API Contract Rules

### 5.1 Public API Definition

Every package exposes a public API through an explicit entry point:

- Exported types, interfaces, and functions only
- Internal modules not exported
- API documented in package README

### 5.2 API Design Rules

| Rule | Detail |
|------|--------|
| **Typed contracts** | All API surfaces fully typed (when language supports) |
| **Explicit errors** | Return typed errors; no silent failures |
| **Idempotent queries** | Query methods must not mutate state |
| **Permission-aware** | State-changing methods require permission context |
| **No global state** | API methods operate on injected dependencies |
| **Async by default** | I/O operations return promises/async |

### 5.3 Service API vs. Event Bus

| Use Service API When | Use Event Bus When |
|---------------------|-------------------|
| Requesting an action (launch app, save layout) | Notifying that something happened |
| Querying current state | Multiple subscribers need the same notification |
| Synchronous response needed | Fire-and-forget broadcast |
| Permission check required before action | AI observation of system activity |

---

## 6. Versioning

### 6.1 API Versioning

- Public module APIs follow semantic versioning
- Breaking changes increment major version
- Deprecated APIs remain for at least one minor version with warnings
- Breaking changes documented in Decision Log

### 6.2 Event Schema Versioning

- Event schema version in every event payload
- Additive changes (new optional fields) are minor version bumps — backward compatible
- Breaking changes (removed/renamed fields) require new event type or major version
- Consumers must handle unknown event types gracefully (ignore or log)

### 6.3 Compatibility Rules

| Change Type | Compatibility | Action Required |
|-------------|---------------|-----------------|
| Add optional field | Backward compatible | Minor version bump |
| Add required field | Breaking | New event type or major version |
| Remove field | Breaking | Major version; migration period |
| Rename field | Breaking | Major version; migration period |
| New event type | Backward compatible | Register in event catalog |

---

## 7. Permission Gateway API

All state-changing operations pass through the Permission Gateway (Platform Kernel):

```
Client → permission.request(submit) → Gateway → User Prompt
Client ← permission.request(result)  ← Gateway ← User Decision
Client → domain.action (with permission token) → Domain Service
```

Permission tokens are:

- Scoped to a specific action
- Time-limited (one-time, session, or persistent per user choice)
- Non-transferable between actions
- Logged in audit trail

AI Subsystem, Plugin Runtime, and Shell all use the same Permission Gateway API.

---

## 8. Plugin API (Future)

Plugin API follows the same conventions:

- Events use `plugin.<pluginId>.<action>` namespace
- Plugin API calls go through Plugin Runtime → Permission Gateway → Domain Service
- Plugin API versioned independently — see [Plugin Architecture Vision](../06-Plugins/PLUGIN-ARCHITECTURE-VISION.md)

---

## 9. Event Catalog

A formal event catalog will be maintained as modules are implemented. Initial location: each package README until a central catalog document is warranted.

---

## Related Documents

- [Architecture Principles](ARCHITECTURE-PRINCIPLES.md)
- [System Overview](SYSTEM-OVERVIEW.md)
- [AI Operating Model](../05-AI/AI-OPERATING-MODEL.md)
- [Coding Standards](../03-Engineering/CODING-STANDARDS.md)
