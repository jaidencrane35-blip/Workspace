# AI Actor Foundation

| Field | Value |
|-------|-------|
| **Purpose** | Describe how AI participates in Workspace as a governed actor (no autonomous authority) |
| **Status** | Sprints 46–47 |
| **Dependencies** | [Permission Architecture](../07-Security/PERMISSION-ARCHITECTURE.md), [AI Operating Model](AI-OPERATING-MODEL.md), [AI Principles](AI-PRINCIPLES.md) |

---

## 1. Rule

**AI may suggest. AI may request. AI may reason. Only the authority system decides.**

AI is not a privileged execution engine. It is an actor with empty default capabilities that must enter the same path as every other participant:

```
User Goal → AI Planner → Action Proposals
        ↓
AiActionRequest  ("what does AI want?")
        ↓
CommandHandler.submit_ai_*
        ↓
CommandPipeline
        ↓
PermissionGateway
        ↓
Allow | Deny | ApprovalRequired
```

Planning details: [AI Planning Foundation](AI-PLANNING-FOUNDATION.md).

There is no `AICommandPipeline`, no `AIExecutionService`, and no bypass of the Permission Gateway.

---

## 2. Actor identity

| Concept | Implementation |
|---------|----------------|
| Actor type | `ActorType::AIAssistant` (named “AI” in product language) |
| Constructor | `Actor::ai_assistant(id)` |
| Default capabilities | **Empty** (`CapabilitySet::for_actor_type`) |
| Default authority | **None** |

Sibling non-human types (`Automation`, `Plugin`, `RemoteSession`) follow the same empty-capability rule.

---

## 3. Intent generation boundary

| Layer | May | Must not |
|-------|-----|----------|
| AI reasoning | Analyze, suggest, build `AiActionRequest` | Call launch/approval services, spawn processes |
| `AiActionRequest` | Describe actor, action intent, target, command, short reason | Store model traces or private thoughts |
| Command pipeline | Evaluate + execute after Allow | Treat AI as a special executor |

Motivation intent for AI submissions: `IntentType::AISuggestion` via `IntentContext::ai_suggestion()`.

---

## 4. Request model

`AiActionRequest` answers: **What does AI want to do?**

- `requesting_actor_id`
- `action_intent_id` (e.g. `launch-application`)
- `target_resource`
- `command_name` (e.g. `LaunchApplication`)
- `reason` (optional, short)
- `created_at`

---

## 5. Default governed outcome

AI launch with no grant → **ApprovalRequired** → human Allow once / Deny → (on allow) consumable capability grant → retry through the same pipeline → grant consumed.

Audits use the normal permission events (`permission.allowed` / `denied` / `approval_required`) with `actor_type = AIAssistant`.

---

## 6. Diagnostic simulation

IPC `request_ai_application_launch` exercises the path for developers (Operator Console). It is **not** a chatbot or agent UI.

---

## 7. Explicitly out of scope (later)

Autonomous agents, memory systems, planning engines, tool-calling frameworks, lasting AI grants, multi-agent orchestration.
