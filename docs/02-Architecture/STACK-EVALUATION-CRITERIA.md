# Technology Stack Evaluation Criteria

| Field | Value |
|-------|-------|
| **Purpose** | Define evaluation criteria for future technology stack decisions without selecting a stack |
| **Owner** | Project Owner |
| **Dependencies** | [Architecture Principles](ARCHITECTURE-PRINCIPLES.md), [System Overview](SYSTEM-OVERVIEW.md), [Performance Budgets](PERFORMANCE-BUDGETS.md), [Threat Model](../07-Security/THREAT-MODEL.md) |
| **Update Process** | Update when evaluation criteria change. Stack selection recorded in Decision Log (resolves OQ-001). |

---

## 1. Status

**Technology stack selected:** Tauri + React + TypeScript + Rust + SQLite (DEC-007, 2026-07-23).

This document records the evaluation framework used to reach that decision. For the recorded outcome, see [Decision Log](../09-Decisions/DECISION-LOG.md) DEC-007.

---

## 2. Evaluation Process

1. List candidate stacks with brief rationale for inclusion
2. Score each candidate against criteria below (1–5 scale)
3. Weight scores by priority
4. Document trade-offs and risks per candidate
5. Present recommendation to Project Owner
6. Record decision in [Decision Log](../09-Decisions/DECISION-LOG.md)

---

## 3. Evaluation Criteria

### 3.1 Windows Integration (Weight: Critical)

| Question | Score Guide |
|----------|-------------|
| Can it access Windows Shell, window management, and audio APIs? | 1 = poor/no access; 5 = native/full access |
| Can it coexist with Windows without replacing the shell? | 1 = requires shell replacement; 5 = clean coexistence |
| Does it support UAC/elevation patterns correctly? | 1 = problematic; 5 = well-supported |
| Windows Store distribution viable? | 1 = not viable; 5 = fully supported |

See [Windows Integration Model](WINDOWS-INTEGRATION-MODEL.md) for coexistence approaches.

### 3.2 Performance (Weight: Critical)

| Question | Score Guide |
|----------|-------------|
| Cold startup time achievable within budget? | See [Performance Budgets](PERFORMANCE-BUDGETS.md) — target ≤ 3s |
| UI responsiveness under load? | 1 = frequent jank; 5 = consistently smooth |
| Background resource usage when idle? | See Performance Budgets — target ≤ 100 MB RAM idle |
| Event processing latency for AI observation? | 1 = noticeable delay; 5 = sub-100ms |

### 3.3 Memory Usage (Weight: High)

| Question | Score Guide |
|----------|-------------|
| Baseline memory footprint? | 1 = >300 MB idle; 5 = <100 MB idle |
| Memory growth over extended sessions? | 1 = significant leaks/growth; 5 = stable |
| Impact of multiple panels/plugins loaded? | 1 = linear explosion; 5 = manageable scaling |

### 3.4 Background Operation (Weight: High)

| Question | Score Guide |
|----------|-------------|
| Can it run persistently without disrupting user workflow? | 1 = intrusive; 5 = seamless |
| System tray / background mode support? | 1 = poor; 5 = native |
| Impact on battery life (laptops)? | 1 = significant drain; 5 = minimal |
| Behaviour when user is not interacting with Workspace? | 1 = high resource use; 5 = low footprint |

### 3.5 Plugin Isolation (Weight: High)

| Question | Score Guide |
|----------|-------------|
| Sandboxing options for untrusted plugin code? | 1 = none; 5 = strong isolation (WASM, separate process) |
| Plugin crash isolation from core app? | 1 = shared process; 5 = fully isolated |
| Plugin API surface definable and enforceable? | 1 = difficult; 5 = natural fit |

See [Plugin Architecture Vision](../06-Plugins/PLUGIN-ARCHITECTURE-VISION.md).

### 3.6 AI Integration (Weight: High)

| Question | Score Guide |
|----------|-------------|
| Local model inference support? | 1 = none; 5 = strong (ONNX, llama.cpp, etc.) |
| Event-driven architecture natural fit? | 1 = awkward; 5 = idiomatic |
| Pattern storage and processing locally? | 1 = limited; 5 = straightforward |
| Remote AI API integration possible with opt-in? | 1 = difficult; 5 = standard HTTP/SDK |

See [AI Operating Model](../05-AI/AI-OPERATING-MODEL.md).

### 3.7 Local Model Support (Weight: Medium)

| Question | Score Guide |
|----------|-------------|
| Can run inference without cloud dependency? | 1 = no; 5 = yes, performant |
| GPU acceleration available? | 1 = none; 5 = CUDA/DirectML/Metal |
| Model size and loading time acceptable? | 1 = impractical; 5 = reasonable for desktop |
| Fallback to rule-based AI if models unavailable? | 1 = all-or-nothing; 5 = graceful degradation |

Note: MVP may use rule-based AI only (OQ-004). This criterion affects Phase 2+.

### 3.8 Development Speed (Weight: Medium)

| Question | Score Guide |
|----------|-------------|
| Time to first running shell prototype? | 1 = months; 5 = days |
| Ecosystem maturity (libraries, tooling)? | 1 = immature; 5 = rich |
| AI contributor (Cursor) effectiveness with stack? | 1 = poor support; 5 = excellent |
| Hiring / contributor availability? | 1 = rare skills; 5 = common skills |

### 3.9 Maintainability (Weight: Critical)

| Question | Score Guide |
|----------|-------------|
| Supports modular monorepo architecture? | 1 = poor; 5 = excellent |
| Type safety and static analysis? | 1 = none; 5 = strong |
| Testability (unit, integration, E2E)? | 1 = difficult; 5 = well-supported |
| Long-term viability at 100k+ LOC? | 1 = risky; 5 = proven at scale |
| Debugging and diagnostics tooling? | 1 = poor; 5 = excellent |

---

## 4. Scoring Template

Use this template when evaluating candidates:

```markdown
## Candidate: [Name]

| Criterion | Weight | Score (1-5) | Weighted | Notes |
|-----------|--------|-------------|----------|-------|
| Windows integration | Critical | | | |
| Performance | Critical | | | |
| Memory usage | High | | | |
| Background operation | High | | | |
| Plugin isolation | High | | | |
| AI integration | High | | | |
| Local model support | Medium | | | |
| Development speed | Medium | | | |
| Maintainability | Critical | | | |
| **Total** | | | **/45** | |

### Strengths


### Weaknesses


### Risks


### Recommendation

```

---

## 5. Disqualifiers

A candidate is **disqualified** regardless of score if:

- It requires replacing the Windows shell (violates constitution)
- It cannot enforce plugin sandboxing at all
- It has no viable path to Windows API access
- Its license is incompatible with project license (OQ-010)
- It cannot meet minimum performance budgets even with optimisation

---

## 6. Relationship to Other Decisions

| Decision | Status |
|----------|--------|
| OQ-001 (stack) | **Resolved** — DEC-007 (Tauri) |
| OQ-002 (process model) | **Resolved** — DEC-011 (multi-process) |
| OQ-014 (Windows integration) | **Resolved** — DEC-008 (hybrid) |
| OQ-019 (monorepo tooling) | **Resolved** — DEC-012 (pnpm) |
| OQ-007 (plugin runtime) | Open — Phase 3 |
| CI/CD tooling | Phase 1 — Tauri + pnpm + Cargo |

---

## Related Documents

- [Windows Integration Model](WINDOWS-INTEGRATION-MODEL.md)
- [Architecture Principles](ARCHITECTURE-PRINCIPLES.md)
- [Decision Log](../09-Decisions/DECISION-LOG.md) — DEC-007
- [Repository Structure](REPOSITORY-STRUCTURE.md)
