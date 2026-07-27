# Testing Strategy

| Field | Value |
|-------|-------|
| **Purpose** | Define testing layers, expectations, and responsibilities for Workspace |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [Engineering Principles](ENGINEERING-PRINCIPLES.md), [Definition of Done](DEFINITION-OF-DONE.md), [CI/CD Plan](CI-CD-PLAN.md), [Threat Model](../07-Security/THREAT-MODEL.md) |
| **Update Process** | Update when test tooling is selected and when new subsystems require test patterns. |

---

## 1. Testing Philosophy

- Test **behaviour**, not implementation details
- Critical paths require automated tests before merge
- AI permission flows are security-critical — they require dedicated tests
- External dependencies (Windows APIs, devices) are mocked in unit tests
- Tests must run in CI on every PR

Current baseline frameworks:

- Rust crates: `cargo test`
- Frontend/contracts: `vitest` in `tests/`

---

## 2. Test Layers

```
┌─────────────────────────────────────┐
│  End-to-End (E2E)                   │  Few, high-value user flows
├─────────────────────────────────────┤
│  Integration                        │  Cross-module interactions
├─────────────────────────────────────┤
│  Unit                               │  Individual module behaviour
├─────────────────────────────────────┤
│  Architectural / Policy             │  Boundary and permission enforcement
└─────────────────────────────────────┘
```

---

## 3. Unit Testing

### Scope

Individual packages and modules in isolation.

### Requirements

| Area | Unit Test Expectation |
|------|----------------------|
| Platform Kernel | Event bus publish/subscribe, permission validation logic |
| Domain Services | Business logic with mocked OS interfaces |
| AI Subsystem | Pattern detection, confidence scoring, suggestion formatting |
| Shell | Layout state management (when extractable from UI) |
| Utilities | All shared helpers |

### Rules

- Mock all external dependencies (Windows APIs, filesystem, network)
- No unit test requires Windows UI or installed applications
- Tests colocated with source or in parallel `tests/` directory per package
- Fast execution — entire unit suite target: under 2 minutes

---

## 4. Integration Testing

### Scope

Cross-module interactions within the Workspace process.

### Requirements

| Integration | Test Focus |
|-------------|------------|
| Shell ↔ Domain Services | App launch flow, layout save/restore |
| AI ↔ Platform Kernel | Event observation, permission request flow |
| Automation ↔ Domain Services | Approved automation executes correctly |
| Plugin Runtime ↔ Kernel | Permission enforcement, sandbox boundaries (Phase 3) |
| Windows Integration Layer | API abstraction with test doubles |

### Rules

- Use test doubles for Windows Integration Layer
- Integration tests may use temporary local storage
- Target: under 5 minutes for full integration suite

---

## 5. End-to-End Testing

### Scope

Full user flows through the running application.

### MVP E2E Flows (Required Before MVP Complete)

| Flow | Validates |
|------|-----------|
| Open Workspace → arrange panels → save → close → reopen → restore | Layout persistence |
| Launch application from Workspace | App integration |
| AI detects pattern → suggests → user approves → automation runs | Full AI sequence |
| User revokes automation → automation stops | Permission revocation |
| User clears AI memory → patterns deleted | Memory policy |

### Rules

- E2E tests run on CI for `main` branch merges
- E2E may require Windows runner (GitHub Actions `windows-latest`)
- Keep E2E suite minimal — cover critical paths only
- Target: under 10 minutes for E2E suite

---

## 6. AI Behaviour Testing

AI tests are **security-critical** and required before any AI feature ships.

### Required Test Scenarios

| Scenario | Expected Result |
|----------|-----------------|
| AI attempts action without approval | Blocked by Permission Gateway |
| User dismisses suggestion | No automation created; cooldown applied |
| User approves one-time automation | Executes once; does not repeat |
| User revokes persistent automation | Stops immediately |
| Observation domain disabled | No data collected from that domain |
| Confidence below L3 | No suggestion generated |
| Prohibited data in event | Not stored (Memory Policy) |
| AI subsystem crash | Workspace continues without AI |
| Permission Gateway unavailable | No automations execute |

### Test Type

- Unit tests for confidence scoring and pattern detection
- Integration tests for permission flow end-to-end
- Architectural tests verifying AI cannot call domain services directly

---

## 7. Regression Testing

### Approach

- All fixed bugs include a regression test unless impractical (document why)
- CI runs full test suite on every PR — regressions block merge
- Snapshot tests for layout serialization (when format decided)

### Release Regression

Before each release:

- Full test suite passes
- Manual smoke test of MVP flows
- Performance smoke test against [Performance Budgets](../02-Architecture/PERFORMANCE-BUDGETS.md)

---

## 8. Security Testing

Derived from [Threat Model](../07-Security/THREAT-MODEL.md):

| Test | Layer | Priority |
|------|-------|----------|
| Permission Gateway enforcement | Integration + Architectural | Critical |
| AI direct domain service call blocked | Architectural | Critical |
| Plugin sandbox isolation | Integration | High (Phase 3) |
| Secret scan in CI | CI | Critical |
| Dependency vulnerability scan | CI | Critical |

---

## 9. Coverage Expectations

Coverage targets remain guidance (not hard merge gates) until dedicated tooling is added:

| Layer | Target |
|-------|--------|
| Platform Kernel | High — 80%+ |
| AI Subsystem (permission flows) | High — 90%+ for permission/confidence logic |
| Domain Services | Medium — 70%+ |
| Shell UI | Medium — focus on state logic |
| E2E | Critical paths only — not coverage-driven |

Coverage is a guide, not a goal. Tests must verify behaviour, not chase percentages.

---

## 10. Test Data

- Use synthetic data only — no real user data in tests
- No secrets in test fixtures
- Temporary directories cleaned up after tests
- AI pattern test fixtures use fabricated event sequences

---

## Related Documents

- [CI/CD Plan](CI-CD-PLAN.md)
- [Definition of Done](DEFINITION-OF-DONE.md)
- [Threat Model](../07-Security/THREAT-MODEL.md)
- [AI Operating Model](../05-AI/AI-OPERATING-MODEL.md)
