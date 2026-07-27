# CI/CD Plan

| Field | Value |
|-------|-------|
| **Purpose** | Define current and planned CI/CD governance for Workspace |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [Repository Standards](REPOSITORY-STANDARDS.md), [Testing Strategy](TESTING-STRATEGY.md), [Definition of Done](DEFINITION-OF-DONE.md), [Dependency Policy](DEPENDENCY-POLICY.md) |
| **Update Process** | Update when workflow steps, branch protection, or release policy changes. |

---

## 1. Current CI Status

Active workflow: [`.github/workflows/ci-pr.yml`](../../.github/workflows/ci-pr.yml)

Current CI runs on `windows-latest` and executes:

- `pnpm install`
- `pnpm typecheck`
- `pnpm build`
- `cargo check --workspace`
- `cargo build --workspace`
- `cargo test --workspace`
- `pnpm test`

Installer packaging and release publishing are not automated yet.

---

## 2. Pipeline Goals

- Keep `main` buildable
- Enforce typed, tested, and audited PRs
- Keep architecture contracts validated in CI
- Expand security checks without destabilizing delivery

---

## 3. Validation Layers

### Layer A — Implemented PR Gates

| Check | Status | Blocks Merge |
|-------|--------|--------------|
| TypeScript typecheck | Implemented | Yes |
| Frontend build | Implemented | Yes |
| Rust workspace check/build/test | Implemented | Yes |
| Frontend contract tests (`pnpm test`) | Implemented | Yes |

### Layer B — Planned Additions

| Check | Status | Blocks Merge |
|-------|--------|--------------|
| Lint | Planned | Yes |
| Formatter check | Planned | Yes |
| Dependency vulnerability scan | Planned | Yes (Critical/High) |
| Secret scan | Planned | Yes |
| Documentation link check | Planned | No (warn) |

### Layer C — Main/Release Extensions

| Check | Status | Blocks Merge |
|-------|--------|--------------|
| E2E smoke suite | Planned | Yes (main/release) |
| Performance smoke | Planned | Warn initially |
| Packaging verification | Planned | Yes (release) |

---

## 4. Branch Strategy Integration

| Branch | Pipeline |
|--------|----------|
| Feature branches | Layer A via PR |
| `main` | Layer A (+ Layer C as adopted) |
| Release branches | Full release validation set |

See [Repository Standards](REPOSITORY-STANDARDS.md).

---

## 5. Branch Protection Intentions

Target repository settings for `main`:

- Pull request required before merge
- At least one approval
- Required status checks must pass
- No force pushes
- No direct commits

---

## 6. Release Validation

Before release tags:

| Validation | Requirement |
|------------|-------------|
| Required CI checks pass | Required |
| Definition of Done met | Required |
| No unresolved Critical/High dependency vulnerabilities | Required |
| Threat model reviewed for release scope | Required |
| Changelog updated | Required |
| Version updated according to semver policy | Required |

---

## 7. Environment Strategy

| Environment | Purpose | Deployment |
|-------------|---------|------------|
| Local | Developer validation | Manual |
| CI | PR and merge validation | Automated |
| Staging | Pre-release verification | Planned |
| Production | User releases | Planned |

Distribution automation remains aligned with OQ-009 resolution.

---

## 8. Workflow Inventory

| File | Purpose | State |
|------|---------|-------|
| `ci-pr.yml` | PR and push validation | Implemented |
| `ci-main.yml` | Main-specific checks | Planned |
| `dependency-audit.yml` | Scheduled vulnerability scans | Planned |
| `secret-scan.yml` | Secret detection | Planned |

---

## 9. Governance Checklist

- [x] Technology stack selected (DEC-007)
- [x] Monorepo tooling selected (DEC-012)
- [x] Unit/integration framework selected (Cargo + Vitest)
- [x] `ci-pr.yml` active
- [ ] Lint and format gates in CI
- [ ] Dependency audit workflow active
- [ ] Secret scanning workflow active
- [ ] Branch protection enforced in repository settings

---

## Related Documents

- [Testing Strategy](TESTING-STRATEGY.md)
- [Dependency Policy](DEPENDENCY-POLICY.md)
- [Repository Standards](REPOSITORY-STANDARDS.md)
- [Performance Budgets](../02-Architecture/PERFORMANCE-BUDGETS.md)
