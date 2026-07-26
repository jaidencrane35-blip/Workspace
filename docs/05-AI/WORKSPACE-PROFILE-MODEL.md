# Workspace Environment Profile Model

Phase 6 Batch 3 / Sprint 102.

## Principle

A Workspace Profile answers: **What workspace setup does the user consider meaningful?**

It is a durable, user-owned description of a preferred working environment. It is not automation, restoration, an agent, or an authority boundary.

## Domain

- `WorkspaceProfile` — durable profile (name, description, status, members)
- `WorkspaceProfileMember` — references Application / Layout / Project / Task / Work Context / Preference
- `WorkspaceProfileState` — read model with alignment, matching evidence, differences, missing members

Profiles **reference** existing systems. They do not own tasks, memory, permissions, or automation.

## Persistence

SQLite tables `workspace_profiles` / `workspace_profile_members` (migration `019`). Soft-delete on profiles. Members cascade with replace-on-update.

## Comparison

Comparing current workspace against a profile is informational only:

```
Profile expects: VS Code, Terminal, Project X
Current: VS Code, Browser
→ Missing: Terminal · Different: Project focus
```

No action is taken.

## Intelligence

`WorkspaceIntelligenceState.profiles` embeds after Interaction:

```
Interaction → Profiles → Intelligence envelope
```

Profile data remains evidence. It never influences permissions.

## Governance

- Gates: `GateWorkspaceProfileRead` / `GateWorkspaceProfileWrite` reuse `work_context.read` / `work_context.write`
- Audits: `workspace.profile.created`, `.updated`, `.compared`, `.generated` — all `authority_effect: none`
- `attempt_execute` always fails
- Must never: launch apps, move windows, restore layouts, grant permissions, auto-switch, or AI-select profiles

## Position

```
Human Intent
  → Profile (description / comparison)
  → Intent (explicit user request)
  → Command Pipeline
  → Permission Gateway
  → Execution
```

A profile describes a workspace. It does not control one.
