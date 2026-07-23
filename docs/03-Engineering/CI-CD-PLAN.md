# CI/CD Plan

| Field | Value |
|-------|-------|
| **Purpose** | Define the future continuous integration and delivery pipeline for Workspace |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [Repository Standards](REPOSITORY-STANDARDS.md), [Testing Strategy](TESTING-STRATEGY.md), [Definition of Done](DEFINITION-OF-DONE.md), [Dependency Policy](DEPENDENCY-POLICY.md) |
| **Update Process** | Update when technology stack is selected (OQ-001) and when pipeline stages change. Record material changes in Decision Log. |

---

## 1. Status

**Pipeline not yet implemented.** This document defines intent. Workflows will be created in `.github/workflows/` when Phase 1 begins and the technology stack is decided.

---

## 2. Pipeline Goals

- Every PR passes automated quality checks before merge
- No secrets committed to the repository
- Tests run on every PR
- Build succeeds on every PR
- Dependency vulnerabilities detected automatically
- Main branch always represents a buildable state

---

## 3. Pipeline Stages

### Stage 1: Validate (Every PR)

| Check | Description | Blocks Merge |
|-------|-------------|--------------|
| **Lint** | Code style and static analysis | Yes |
| **Type check** | Type safety verification (when applicable) | Yes |
| **Format check** | Automated formatter compliance | Yes |
| **Secret scan** | Detect credentials in diff | Yes |
| **Documentation links** | Verify internal doc links (future) | No (warn) |

### Stage 2: Test (Every PR)

| Check | Description | Blocks Merge |
|-------|-------------|--------------|
| **Unit tests** | Package-level unit tests | Yes |
| **Integration tests** | Cross-module tests (when applicable) | Yes |
| **AI behaviour tests** | Permission and confidence tests (Phase 2+) | Yes |

### Stage 3: Build (Every PR)

| Check | Description | Blocks Merge |
|-------|-------------|--------------|
| **Build** | Full application build succeeds | Yes |
| **Package audit** | Dependency vulnerability scan | Yes (Critical/High) |

### Stage 4: E2E (Main branch and release candidates)

| Check | Description | Blocks Merge |
|-------|-------------|--------------|
| **E2E tests** | End-to-end user flow tests | Yes (main) |
| **Performance smoke** | Startup time within budget | No (warn initially) |

---

## 4. Branch Strategy Integration

| Branch | Pipeline |
|--------|----------|
| Feature branches | Stages 1–3 on PR |
| `main` | Stages 1–4 on merge |
| Release branches | Full pipeline + release validation |

See [Repository Standards](REPOSITORY-STANDARDS.md).

---

## 5. Branch Protection Intentions

When CI is operational, `main` branch protection will require:

- [ ] Pull request required before merge
- [ ] At least one approval
- [ ] Status checks must pass (Stages 1–3 minimum)
- [ ] Branches must be up to date before merge
- [ ] No force pushes
- [ ] No direct commits to `main`

Configuration applied via GitHub repository settings when Phase 1 begins.

---

## 6. Release Validation

Before any release tag:

| Validation | Requirement |
|------------|-------------|
| All CI stages pass on release commit | Required |
| Definition of Done met for all release items | Required |
| No Critical or High dependency vulnerabilities | Required |
| Threat model reviewed for release scope | Required (Phase 2+) |
| Changelog updated | Required |
| Version number follows semver | Required |

---

## 7. Environment Strategy

| Environment | Purpose | Deployment |
|-------------|---------|------------|
| **Local** | Developer machine | Manual |
| **CI** | Automated testing | Every PR |
| **Staging** | Pre-release validation | Manual (future) |
| **Production** | User releases | Tagged releases (future) |

No production deployment pipeline until distribution model is decided (OQ-009).

---

## 8. Planned Workflow Files

When stack is selected, create in `.github/workflows/`:

| File | Trigger | Stages |
|------|---------|--------|
| `ci-pr.yml` | Pull request | 1–3 |
| `ci-main.yml` | Push to main | 1–4 |
| `dependency-audit.yml` | Weekly schedule | Package audit |
| `secret-scan.yml` | Pull request | Secret scan |

Exact tooling (GitHub Actions runners, cache strategy, monorepo CI) depends on OQ-001 and OQ-019.

---

## 9. Implementation Checklist (Phase 1)

- [ ] Technology stack selected (OQ-001)
- [ ] Monorepo tooling selected (OQ-019)
- [ ] Lint and format tools configured
- [ ] Unit test framework configured
- [ ] `ci-pr.yml` created and passing
- [ ] Branch protection enabled on `main`
- [ ] Dependency audit workflow active
- [ ] Secret scanning active

---

## Related Documents

- [Testing Strategy](TESTING-STRATEGY.md)
- [Dependency Policy](DEPENDENCY-POLICY.md)
- [Repository Standards](REPOSITORY-STANDARDS.md)
- [Performance Budgets](../02-Architecture/PERFORMANCE-BUDGETS.md)
