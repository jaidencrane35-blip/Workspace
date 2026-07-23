# Plugin Architecture Vision

| Field | Value |
|-------|-------|
| **Purpose** | Define the high-level vision for Workspace's plugin and extension platform |
| **Owner** | Architect (TBD) |
| **Dependencies** | [Architecture Principles](../02-Architecture/ARCHITECTURE-PRINCIPLES.md), [Security Principles](../07-Security/SECURITY-PRINCIPLES.md) |
| **Update Process** | Update as plugin platform decisions are made. API specification requires Decision Log entry. |

---

## 1. Vision

Workspace is an extensible platform. First-party features and third-party plugins share the same extension model. Plugins allow the community to extend Workspace without modifying core code.

---

## 2. Design Goals

| Goal | Detail |
|------|--------|
| **Unified model** | First-party and third-party plugins use the same SDK and APIs |
| **Safe by default** | Plugins run sandboxed with least-privilege permissions |
| **Discoverable** | Users can browse, install, and manage plugins from within Workspace |
| **Stable API** | Plugin API is versioned; breaking changes require migration period |
| **Testable** | Plugins can be developed and tested independently of the full application |

---

## 3. Plugin Capabilities (Conceptual)

Plugins may extend Workspace in these areas:

| Capability | Example |
|------------|---------|
| **Panel providers** | Custom panel content (weather, notes, system stats) |
| **App integrations** | Deep integration with specific applications |
| **Automation triggers** | Custom triggers and actions for automations |
| **Device handlers** | Support for new device types or protocols |
| **Audio processors** | Custom audio routing or effects |
| **AI enrichments** | Additional pattern sources or suggestion types |
| **Themes and appearance** | Visual customisation beyond built-in options |

Specific capability list will be finalised when the Plugin SDK is designed.

---

## 4. Permission Model

Plugins operate under a permission system analogous to the AI permission model:

### 4.1 Permission Categories

| Category | Access |
|----------|--------|
| **Read state** | Observe domain events and state (read-only) |
| **UI contribution** | Render content in designated plugin areas |
| **User action** | Respond to user-initiated interactions within plugin UI |
| **Domain action** | Request state changes through domain service APIs |
| **System access** | Access OS-level features (highly restricted) |

### 4.2 Rules

- Plugins declare required permissions at install time
- User approves permissions during installation
- Plugins cannot escalate permissions at runtime
- Revoking a permission disables dependent functionality
- First-party plugins follow the same permission model — no backdoors

---

## 5. Sandboxing

Plugins must not:

- Access the filesystem outside their designated storage area
- Make direct OS API calls (must go through Workspace APIs)
- Access other plugins' data
- Modify core Workspace state without going through domain APIs
- Execute arbitrary code outside the plugin runtime

Sandboxing technology is an open decision. See [Open Questions](../09-Decisions/OPEN-QUESTIONS.md).

---

## 6. Plugin Lifecycle

```
Develop → Package → Publish → Install → Enable → Update → Disable → Uninstall
```

| Stage | Detail |
|-------|--------|
| **Develop** | Using Plugin SDK, outside this repository |
| **Package** | Standard format with manifest declaring permissions and capabilities |
| **Publish** | To a plugin registry (future — may be GitHub-based initially) |
| **Install** | User-initiated; permissions reviewed and approved |
| **Enable/Disable** | User control; disabled plugins consume no resources |
| **Update** | User-approved; permission changes require re-approval |
| **Uninstall** | Clean removal of plugin data and permissions |

---

## 7. Plugin SDK (Future)

The Plugin SDK will provide:

- Type-safe API for all plugin capabilities
- Development tooling (scaffolding, testing, debugging)
- Documentation and examples
- Validation tools for package format and permissions

SDK design is Phase 2+ work. No SDK exists yet.

---

## 8. First-Party Plugins

First-party features that could be plugins (but may ship built-in initially):

- Individual app integrations
- Specific device handlers
- Theme packs
- Advanced automation templates

The goal is to architect these as plugins internally, even if bundled, so they can be extracted later.

---

## 9. What Plugins Are Not

- A way to bypass AI permission model
- A way to access user data without consent
- A replacement for core domain services
- Arbitrary code execution environments

---

## 10. Open Plugin Questions

See [Open Questions](../09-Decisions/OPEN-QUESTIONS.md):

- Plugin runtime technology (WebView, WASM, native modules, etc.)
- Plugin registry and distribution model
- Revenue/sharing model for third-party plugins (if any)
- Plugin review and security audit process

---

## Related Documents

- [Architecture Principles](../02-Architecture/ARCHITECTURE-PRINCIPLES.md)
- [Security Principles](../07-Security/SECURITY-PRINCIPLES.md)
- [System Overview](../02-Architecture/SYSTEM-OVERVIEW.md)
- [Repository Structure](../02-Architecture/REPOSITORY-STRUCTURE.md)
