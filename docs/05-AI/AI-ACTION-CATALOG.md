# AI Action Catalog & Tool Awareness

| Field | Value |
|-------|-------|
| **Purpose** | Let AI know what actions exist and what capabilities they require — without granting authority |
| **Status** | Sprints 52–53 (Batch 4) |
| **Dependencies** | [AI Context Foundation](AI-CONTEXT-FOUNDATION.md), ActionIntentRegistry (Sprint 15) |

---

## 1. Rule

**Knowing an action exists ≠ being allowed to perform it.**

```
ActionIntentRegistry
        ↓
ActionCatalog  ("what exists?")
        ↓
AI Understanding / AiActionAwareness
        ↓
Action Proposal
        ↓
Permission Gateway  ("is this allowed?")
```

---

## 2. Two discovery surfaces

| Surface | Question | Grants authority? |
|---------|----------|-------------------|
| `ActionCatalog` | What actions are registered? | **No** |
| `CapabilityDiscovery` | What may *this actor* execute now? | **No** (probe only; still not a grant) |

AI planning uses the catalog for explanations. Execution still requires gateway Allow.

---

## 3. Models

- `ActionCatalogEntry` — intent id, name, description, command, required capability, target rules
- `ActionCatalog` — sorted registry snapshot
- `AiActionAwareness` — catalog + known intent ids for planning

---

## 4. Example

```
AI: "Opening applications is catalogued (LaunchApplication)."
System: "Required capability: application.launch"
AI: proposes AiActionRequest
Gateway: ApprovalRequired (AI has empty capabilities)
```

---

## 5. Diagnostic

IPC `get_action_catalog` — Operator Console “Explore actions”.

Audit: `action.catalog.viewed` (informational_only=true).

---

## 6. Out of scope

Automatic tool selection, tool-calling frameworks, agent loops, lasting AI permissions.
