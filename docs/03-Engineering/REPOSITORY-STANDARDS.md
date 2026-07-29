# Repository Standards

| Field | Value |
|-------|-------|
| **Purpose** | Define Git workflow, branch strategy, commit conventions, and repository hygiene |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [Engineering Principles](ENGINEERING-PRINCIPLES.md), [Repository Structure](../02-Architecture/REPOSITORY-STRUCTURE.md) |
| **Update Process** | Update when branching strategy or tooling changes. Record material changes in Decision Log. |

---

## 1. Branch Strategy

### 1.1 Main Branches

| Branch | Purpose | Protection |
|--------|---------|------------|
| `main` | Production-ready code and approved documentation | Protected; requires PR review |
| `develop` | Integration branch for active development (when needed) | Protected; requires PR review |

Historical note: during Phase 0 (documentation-only), all work merged directly to `main` via PR.

### 1.2 Working Branches

| Pattern | Use |
|---------|-----|
| `docs/<topic>` | Documentation changes |
| `feature/<ticket>-<description>` | New features |
| `fix/<ticket>-<description>` | Bug fixes |
| `refactor/<description>` | Code refactoring |
| `chore/<description>` | Tooling, CI, dependencies |

Branch names use `kebab-case`. Include ticket/issue number when available.

### 1.3 Branch Rules

- Branch from `main` (or `develop` when active)
- Keep branches short-lived (target: merge within 1 week)
- Delete branch after merge
- Never force-push to `main` or `develop`
- Never rewrite published history

---

## 2. Commit Conventions

### 2.1 Message Format

```
<type>(<scope>): <subject>

[optional body]

[optional footer]
```

### 2.2 Types

| Type | Use |
|------|-----|
| `docs` | Documentation only |
| `feat` | New feature |
| `fix` | Bug fix |
| `refactor` | Code change that neither fixes nor adds |
| `test` | Adding or updating tests |
| `chore` | Tooling, CI, dependencies |
| `style` | Formatting only (no logic change) |
| `perf` | Performance improvement |

### 2.3 Rules

- Subject line: imperative mood, max 72 characters, no period
- Body: explain **why**, not **what**
- Reference issues: `Closes #123` or `Refs #123`
- One logical change per commit
- Group related changes; do not mix unrelated changes
- Prefer intent-revealing scopes (`window-management`, `desktop-arrangement`) over vague ones

Major batches also require completion reports and architecture docs — see [Engineering Governance](ENGINEERING-GOVERNANCE.md).

### 2.4 Examples

```
docs: add project constitution and governance model

Establish non-negotiable principles and decision authority
for all contributors including AI agents.

feat(shell): add panel resize handler

Implements user-resizable panels per UX Principles.
Refs #45.

fix(audio): prevent volume reset on device change

Volume preferences were not persisted when the default
audio endpoint changed. Closes #78.
```

---

## 3. Pull Request Standards

### 3.1 Requirements

Every PR must:

- Use the PR template
- Have a clear title following commit convention
- Reference related issues or sprint items
- Pass CI checks (when CI is configured)
- Have at least one approval before merge
- Meet [Definition of Done](DEFINITION-OF-DONE.md)

### 3.2 Size Guidelines

| Size | Lines Changed | Review Time |
|------|---------------|-------------|
| Small | < 100 | < 15 min |
| Medium | 100–400 | < 30 min |
| Large | 400+ | Split if possible |

### 3.3 Merge Strategy

- **Squash merge** for feature branches (clean history)
- **Merge commit** for release branches (preserve history)
- Never rebase and force-push shared branches

---

## 4. Repository Hygiene

### 4.1 Files That Must Never Be Committed

- Secrets, API keys, credentials
- `.env` files with real values
- Build artifacts (`dist/`, `build/`, `out/`)
- IDE-specific files (except shared settings)
- OS files (`.DS_Store`, `Thumbs.db`)
- Large binaries without LFS

See [`.gitignore`](../../.gitignore).

### 4.2 Files That Must Be Committed

- All documentation in `docs/`
- Configuration templates (`.env.example`)
- CI/CD configuration
- Lock files (`pnpm-lock.yaml`, `Cargo.lock`) — package manager is selected and committed

### 4.3 File Size

- No file exceeds 1 MB without justification
- Large assets use Git LFS or external asset management

---

## 5. Issue Management

- All work tracked via GitHub Issues
- Use issue templates (bug, feature, decision, documentation)
- Label issues by domain: `architecture`, `product`, `ux`, `ai`, `security`, `plugin`
- Link issues to PRs and sprint items
- Close issues only when Definition of Done is met

---

## 6. Release Tags

When releases begin:

- Tags follow `v<major>.<minor>.<patch>` (semver)
- Tags annotated with release notes
- Pre-release tags: `v0.1.0-alpha.1`

---

## 7. AI Contributor Rules

AI agents working in this repository must:

- Create branches following naming conventions
- Write commits following message conventions
- Fill out PR templates completely
- Never force-push
- Never commit secrets
- Never merge their own PRs without human approval

---

## Related Documents

- [Engineering Principles](ENGINEERING-PRINCIPLES.md)
- [Coding Standards](CODING-STANDARDS.md)
- [Definition of Done](DEFINITION-OF-DONE.md)
- [CONTRIBUTING.md](../../CONTRIBUTING.md)
