# Constitutional Gap Analysis

| Field | Value |
| --- | --- |
| **Purpose** | Evidence-backed gaps between constitutional intent and implementation |
| **Authority** | Constitution V2 + knowledge maps + repository evidence |
| **Date** | 2026-08-07 |

Evidence sources: `docs/ipc-surface-map.md`, `docs/domain-contract-assessment.md`, `docs/state-authority-map.md`, `docs/governance-map.md`, `architecture/ARCHITECTURAL_CONSTITUTION_V2.md`, `app/src/types/domain.ts`, `app/src-tauri/src/lib.rs`, `scripts/verify-explanation-catalog.mjs`.

---

## Findings

| ID | Finding | Severity | Effort | Arch risk | Long-term value | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| G1 | Hand-maintained TS domain (~4888 lines) not generated from Rust | **Critical** → **Reduced (PP slice)** | L full; M PP done | High if big-bang rewrite | Very high | PP Experience types generated + `pnpm verify:contracts`; remainder of `domain.ts` still manual — see milestone report |
| G2 | IPC tiers (Product/Developer/Diagnostic/Experimental) not machine-enforced | **Critical** → **Closed (enforcement)** | S–M | Low (labels only) | High | Closed by Phase A #2 — `pnpm verify:ipc-tiers`; see `docs/engineering-milestone-report.md` |
| G3 | Documentation authority fragmented (architecture/, docs/, Decision Log) | **High** | M | Medium (process) | High | governance-map; Law XI |
| G4 | `WorkspaceState` name collision (kernel lifecycle vs domain desktop) | **High** | S (rename + aliases) | Medium if careless rename | Medium–high | state-authority-map |
| G5 | No AgentToolGate; agent tools not uniformly outside-model gated | **High** | M (design + integrate) | Medium | High when AI expands | kiro-adoption; Law IV |
| G6 | Audit lacks tamper-evident integrity chain | **Medium** | M | Low–medium | High for trust claims | Law V; blind-spots |
| G7 | No provider-neutral ModelProvider boundary | **Medium** | M | Low if interface-first | High before vendor lock | Law X |
| G8 | No BackgroundWorkerSupervisor; observation scheduler disabled but no formal AI worker host | **Medium** | M | High if ambient enabled | Medium | Law XII; runtime-service-map |
| G9 | No formal RFC workflow beyond Decision Log | **Low** | S | Low | Medium | governance-map; kiro RFC study |
| G10 | Quarantined / Operator surfaces not labelled in CI | **High** → **Closed (labelling)** | S | Low | High | Closed by Phase A #2 quarantine registry + CI |

Effort: S ≤ 1 week · M 1–3 weeks · L > 3 weeks (one engineer).

---

## Ranking notes

- **Critical for next execution:** G2 is completable without behaviour change and unblocks surface governance. G1 is higher absolute value but incomplete codegen is worse than deferred full program.
- Ambient capture remains constitutionally off — not a gap to “fix” by enabling.
