# Performance Budgets

| Field | Value |
|-------|-------|
| **Purpose** | Define future performance targets for Workspace subsystems |
| **Owner** | Lead Software Engineer |
| **Dependencies** | [Architecture Principles](ARCHITECTURE-PRINCIPLES.md), [Stack Evaluation Criteria](STACK-EVALUATION-CRITERIA.md), [CI/CD Plan](../03-Engineering/CI-CD-PLAN.md) |
| **Update Process** | Update when targets are validated or revised. Changes require Decision Log entry if targets are relaxed. |

---

## 1. Status

**Targets defined; not yet validated.** These budgets will be verified during Phase 1 prototyping and enforced via CI smoke tests when tooling is available.

---

## 2. Budget Philosophy

- Measure before optimising
- Fail visibly — performance regressions warn in CI before they reach users
- Background operation must not degrade the user's PC experience
- Budgets apply to MVP hardware: mid-range Windows 10/11 laptop (16 GB RAM, integrated GPU)

---

## 3. Startup Performance

| Metric | Target | Measurement |
|--------|--------|-------------|
| Cold start (first launch after boot) | ≤ 3 seconds to interactive shell | App process start → shell accepts input |
| Warm start (subsequent launch) | ≤ 1.5 seconds to interactive shell | App process start → shell accepts input |
| Layout restore | ≤ 500 ms after shell interactive | Saved layout fully rendered |
| Background service start | ≤ 2 seconds (if separate process) | Service ready to receive events |

Cold start includes framework initialisation (Electron/Tauri/.NET runtime). Stack selection (OQ-001) affects achievability.

---

## 4. UI Responsiveness

| Metric | Target | Measurement |
|--------|--------|-------------|
| Panel resize/move | ≤ 16 ms frame time (60 fps) | User drag operation |
| Panel save | ≤ 200 ms | User action → confirmation |
| App launch from Workspace | ≤ 300 ms to initiate + OS launch time | Click → process start |
| Navigation transition | ≤ 100 ms | Panel/tab switch |
| Suggestion prompt display | ≤ 500 ms after pattern detected | Internal — not user-perceived delay |

If frame time exceeds 16 ms for more than 3 consecutive frames during interaction, it is a performance bug.

---

## 5. Memory Usage

| State | Target | Maximum |
|-------|--------|---------|
| Idle (shell open, no active interaction) | ≤ 80 MB | 100 MB |
| Active (user interacting, apps monitored) | ≤ 150 MB | 200 MB |
| AI observing (pattern processing) | ≤ 200 MB | 250 MB |
| With local AI model loaded (Phase 2+, if applicable) | ≤ 500 MB | 750 MB |
| Per plugin (when plugins exist) | ≤ 30 MB | 50 MB |

Memory measured as private working set of all Workspace processes combined.

---

## 6. Background Resource Usage

| Metric | Target | Maximum |
|--------|--------|---------|
| CPU (idle, observing) | ≤ 1% average | 3% peak |
| CPU (AI pattern processing) | ≤ 5% average | 10% peak |
| Disk I/O (idle) | ≤ 1 MB/min | 5 MB/min |
| Event bus throughput | ≤ 100 events/sec sustained | 500 events/sec peak |
| Battery impact (laptop) | Negligible — no measurable drain when idle | Must not appear in Windows battery report as significant |

Workspace must not appear in Windows Task Manager as a top resource consumer when the user is not interacting with it.

---

## 7. AI Subsystem Budgets

| Metric | Target |
|--------|--------|
| Event observation latency | ≤ 50 ms (event occurs → stored for processing) |
| Pattern detection cycle | ≤ 5 seconds (batch processing, not per-event) |
| Suggestion generation | ≤ 1 second after confidence threshold met |
| Permission prompt display | ≤ 500 ms after suggestion approved internally |
| Pattern store size | ≤ 10 MB for typical user (6 months) |

See [Memory Policy](../05-AI/MEMORY-POLICY.md) for retention impact on store size.

---

## 8. Persistence Performance

| Operation | Target |
|-----------|--------|
| Save layout | ≤ 100 ms |
| Load layout | ≤ 200 ms |
| Save preference | ≤ 50 ms |
| Export AI memory | ≤ 2 seconds |
| Clear AI memory | ≤ 1 second |

Dependent on persistence format decision (OQ-003).

---

## 9. CI Performance Smoke Tests

When CI is configured, run on `main` merges:

| Test | Pass Criteria |
|------|---------------|
| Cold start smoke | Shell interactive within 5 seconds (generous CI threshold) |
| Memory smoke | Process memory ≤ 300 MB after startup (CI environment allowance) |
| Layout round-trip | Save and restore completes without error |

CI thresholds are more generous than user-facing targets to account for CI runner variability. User-facing targets in sections 3–8 are authoritative.

---

## 10. Regression Policy

| Scenario | Action |
|----------|--------|
| Budget exceeded in development | Fix before merge or log tech-debt with plan |
| Budget exceeded in CI smoke test | Warn initially; block merge once stable |
| Budget exceeded in production | Performance bug — prioritise fix |
| Budget relaxation needed | Decision Log entry with justification |

---

## Related Documents

- [Stack Evaluation Criteria](STACK-EVALUATION-CRITERIA.md)
- [CI/CD Plan](../03-Engineering/CI-CD-PLAN.md)
- [Architecture Principles](ARCHITECTURE-PRINCIPLES.md)
- [Testing Strategy](../03-Engineering/TESTING-STRATEGY.md)
