# AI Personalization Foundation

| Field | Value |
|-------|-------|
| **Purpose** | Transparent, user-controlled preference-aware planning (Sprints 64–65 / P4-B3) |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [AI Memory Foundation](AI-MEMORY-FOUNDATION.md), [AI Planning Foundation](AI-PLANNING-FOUNDATION.md), [Permission Architecture](../07-Security/PERMISSION-ARCHITECTURE.md) |
| **Update Process** | Update when preference model / planning integration changes |

---

## Permanent separation

```
Memory          → What has happened?
Personalization → What does this user prefer?
Authority       → What is allowed?
```

Personalization may influence ordering, recommendations, explanations, and defaults.  
It must never influence permissions, capability grants, or gateway decisions.

---

## Model

| Type | Role |
|------|------|
| `UserPreference` | Explicit preference with source, confidence, scope, editable flag |
| `UserPreferenceProfile` | Inspectable preference set + enabled flag |
| `AiPersonalizationAwareness` | Bounded planning hints |
| `PreferenceCategory` | workflow / application / layout / communication / planning |
| `PreferenceSource` | user_defined / user_confirmed / imported only |

No automatic long-term inference or behavioral profiling in this batch.

Persistence: `user_preferences` (migration `011_user_preferences.sql`).

Toggle: `WorkspaceSettings.personalization_enabled` (default `true`; disable ⇒ neutral planning).

Capabilities: `personalization.read` / `personalization.write`.

---

## Planning integration

```
Preferences (if enabled)
    │
    v
AiPersonalizationAwareness
    │
    v
Deterministic ranking + explanations
    │
    v
AiActionProposal → Evaluation → Permission Gateway
```

Example explanation:

> Preferred because you marked VS Code as your default editor.

---

## Audits

| Event | Meaning |
|-------|---------|
| `ai.personalization.created` | Preference stored |
| `ai.personalization.updated` | Preference or toggle changed |
| `ai.personalization.deleted` | Preference removed |
| `ai.personalization.used` | Preferences listed or applied in planning |

All include `"authority_effect": "none"`.

---

## Diagnostics

Operator Console: list / create / edit / delete preferences, toggle personalization, generate personalized plan, compare personalized vs neutral.

---

## Related

- [Intelligence Roadmap](INTELLIGENCE-ROADMAP.md)
- [IPC Surface](../03-Engineering/IPC-SURFACE.md)
