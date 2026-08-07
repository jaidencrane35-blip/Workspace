# Constitutional Operations Audit — Repository Truth vs Constitutional Intent

| Field | Value |
| --- | --- |
| **Kind** | Engineering Governance evidence — not constitutional law |
| **Date** | 2026-08-07 |
| **Normative** | Workspace Constitutional Specification v2.1 + Engineering Execution Standard v1 |
| **Stance** | Spec and EES assumed correct; repository is evidence |
| **Program type** | Governance / Documentation |
| **Max layer** | Documentation (no Spec/runtime redesign) |
| **Spec modified?** | No |
| **Runtime modified?** | No |

---

## 1. Constitutional Compliance Report (executive)

**Verdict:** The **Conversation → desktop Effect path** faithfully implements the Constitutional Transformation Chain and Capability Integration model. Providers do not call providers. Conversation does not invoke banned provider IPC. Kernel Operator plans and composes; Permission is applied via `CommandPipeline` on steps.

**Latent risks (not proven Conversation violations):** Broad registered IPC surface (~197 commands) remains available at the Tauri boundary; Conversation Operator path bans direct provider commands. Historical UI / Blueprint docs remain archive candidates. These are **Track A / repository debt**, not demonstrated constitutional Effect-path failures.

**No objectively justified runtime correction** before Product Proof.

**Declaration:** No constitutional non-compliance was found within the audited operational scope. The audited operational path faithfully implements the Workspace Constitutional Specification v2 and the Workspace Engineering Execution Standard v1. Future constitutional review requires new objective evidence or an explicit Constitutional Review Trigger.

This is **no evidence of non-compliance within scope**, not proof of universal compliance. See §16 Confidence Boundary and `REPOSITORY_CONFIDENCE_MODEL.md`.

---

## 2. Subsystem compliance matrix

| Subsystem | Constitutional Owner | Owns / produces | Consumes | Authority | Compliance | Hidden authority risk | Drift risk |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Voice WRAP | Input | Signal → transcript | Mic/OS speech | None over Effects | **Pass** — review/Send, no auto-submit (`OperatorRoot` F10) | Low | Low |
| Composer / typed input | Input + Presentation | Signal text | User | None over Effects | **Pass** | Low | Low |
| Intent Layer (grammar, situation, semantic, context, goal, planner modules) | Understanding | Meaning (`IntentAction` / CapabilityIntent mapping) | Utterance | Must not Effect | **Pass** — `handleOperatorUtterance` → `resolveIntent` then Kernel IPC or shell UI | Med* | Med* |
| Capability Registry | Understanding (consult) | Discovery text | Utterance | None | **Pass** — consultative | Low | Low |
| Kernel Operator | Orchestration + Evidence compose | Plan; composed reply Facts | CapabilityIntent | Orders Execution | **Pass** — `execute_capability_intent` plan→steps→compose | Low | Low |
| Permission / CommandPipeline | Authority | Allow/deny on steps | Actor + step | Gate Plan→Effect | **Pass** — `execute_step` uses `CommandPipeline` | Low | Low |
| Capability Runtime / Router / Registry | Execution host | Route to provider | Plan steps | None (no own Effects) | **Pass** — `CapabilityRouter` single provider invoke | Low | Low |
| Providers (clipboard, app, window, notify, browser, screenshot) | Execution | Effect | Routed request | Must not authorize peers | **Pass** — no provider→provider in `capability_runtime/` | Low | Low |
| Moments / Continue shell | Presentation (+ consented restore UX) | Experience; opens approve-before-move | Intent shell actions | Restore Effects via separate approve path | **Pass** — shell opens review; “Approve the plan before anything moves” | Low | Low |
| Persistence / SQLite | Implementation under Persistence | Durable state | Kernel/services | N/A | **Pass** as local store | Low | Low |
| Conversation UI (`OperatorRoot`) | Presentation | Experience | Operator outcomes | Must not decide Effects | **Pass** — comments cite Operator Authority; capability IPC only via Operator | Low | Low |
| `runtimeBridge` | Presentation boundary | Single IPC | CapabilityIntent | Bans provider commands | **Pass** — `BANNED_PROVIDER_COMMANDS` | Low | Low |

\*Intent modules are **Understanding stages**, not new Information Owners. Parallel `executionPlanner` (TS) used for Product Proof evidence can look like dual Orchestration; **Conversation Effects use Kernel `plan_capability_intent`**. Treat TS planner as Understanding/evidence aid — Track A clarity, not Effect-path violation.

---

## 3. Information Ownership Graph (Conversation Effect path)

```
Signal (Input: Voice WRAP | composer)
    → Meaning (Understanding: resolveIntent / Intent stages → CapabilityIntent)
    → Plan (Orchestration: Kernel plan_capability_intent)
    → [Authority] (CommandPipeline / Permission on execute_step)
    → Effect (Execution: Router → one Provider → OS)
    → Fact (Evidence: ProviderInvokeResponse + compose_user_reply)
    → Experience (Presentation: OperatorRoot transcript)
```

| Transition | Illegal? | Hidden ownership? | Duplicate truth? | Circular? |
| --- | --- | --- | --- | --- |
| Signal → Meaning | No | No | No | No |
| Meaning → Plan | No | No | Conceptual dual planner (TS evidence vs Kernel) — Kernel authoritative for Effects | No |
| Plan → Effect via Authority | No | No | No | No |
| Effect → Fact | No | No | No | No |
| Fact → Experience | No | No | No | No |
| Provider → Provider | **Forbidden; not observed** | — | — | — |
| Conversation → Provider IPC | **Forbidden on Operator path; banned list** | Latent if other UI used raw IPC | — | — |

---

## 4. Transformation Chain Verification

| Law / stage | Evidence |
| --- | --- |
| Meaning ↛ Effect | `intelligence.ts`: capability actions go `toCapabilityIntent` → `executeCapabilityIntent`; no provider invoke in Intent |
| Experience ↛ Plan | UI applies shell navigate/save/open review; does not compose provider plans |
| Plan → Effect gated | `execute_step` → `CommandPipeline` |
| Effect ↛ Effect | Router invokes one provider; composition in Kernel Operator only (`browser.open_beside`, `app.open_or_focus`) |
| Facts from Effects | `compose_user_reply(intent, plan, results)` |
| Observability | Results + composed message; Voice status messages |

---

## 5. Engineering Execution Audit

| Stage | Repository support | Followed today? |
| --- | --- | --- |
| Classification | EES §5; AGENTS pre-flight | **Yes** for agent entry (AGENTS + protocol) |
| Constitutional Compliance | Checklist + Spec | **Yes** as required artifact |
| Lowest-layer | EES §3 | **Yes** as standing rule |
| Implementation / Verification | `pnpm test`, provider verifiers | **Yes** |
| Product Proof | PRODUCT_PROOF_RULE; P16 pending Owner | **Process active** — Owner gate open |
| Owner Acceptance / Health | handoff + project-health | **Yes** |

**Bypass?** No alternate lifecycle inventing architecture was found in active entry points. Historical sprint docs do not re-run EES (historical). Agents are instructed to refuse Spec redesign without Review Trigger.

---

## 6. Capability Integration Audit (sample)

| Capability | Integrates via Chain? | Bespoke architecture? | Evidence |
| --- | --- | --- | --- |
| Clipboard | Yes | No | Domain via Kernel → clipboard provider |
| Application | Yes | No | `app.open_or_focus` composed in Kernel |
| Window | Yes | No | Window provider via Router |
| Browser | Yes | No | Browser + Kernel compose open_beside |
| Notifications | Yes | No | Notification provider |
| Screenshot | Yes | No | Screenshot provider |
| Voice | Input device only | No | Not a Capability Provider |

All sampled closed providers use the same Kernel IPC entry. **No bespoke constitutional architecture** found for Product Proof capability path.

---

## 7. Architectural Drift Report

| Candidate | Evidence | Classification |
| --- | --- | --- |
| Provider→provider | Not found in `capability_runtime` | None |
| Conversation provider IPC | Only `execute_capability_intent` in Operator path; bans listed | None on path |
| UI authority over Effects | Shell opens Moment review; no silent restore | None |
| Dual Intent planners | TS `executionPlanner` + Kernel plan | **Conceptual** — Track A clarity; Kernel owns Effect Plans |
| Broad IPC registry | ~197 commands registered; Product Proof uses subset | **Latent surface** — Track A quarantine |
| Blueprint “highest authority” | Historical `architecture/00_*` | Historical — not active |
| Competing Spec claims | Fixed in A3.V | Cleared for active docs |

**Has architectural drift begun on the Effect path?** **No** (evidence).

---

## 8. Governance Audit

| Layer | Status |
| --- | --- |
| Spec sole architectural authority | Yes — AGENTS, handoff, protocol, health |
| EES subordinate governance | Yes |
| Product Proof / protocol subordinate | Yes |
| Arch Const V2 subordinate | Yes (preamble corrected A3.V) |
| Silent reassertion of authority | None in active entry docs |

---

## 9. Complexity / abstraction challenge

| Abstraction | Preserves ownership? | Action |
| --- | --- | --- |
| Intent stage modules | Yes (Understanding) | Keep |
| Kernel Operator | Yes (Orchestration) | Keep |
| Capability Runtime | Yes (Execution host) | Keep |
| TS executionPlanner | Evidence/Understanding aid | Keep; do not elevate to second Orchestration |
| Large IPC surface | Weakens Singular Truth of entry if misused | Track A quarantine — not Spec change |

---

## 10. Production separation assessment

| Concern | Separated? |
| --- | --- |
| Constitutional work | Closed (Operations) |
| Architecture Standard | Subordinate docs |
| Engineering / EES | Active |
| Capability development | P17 deferred |
| Production readiness | Track A |
| Product Proof | P16 pending Owner |
| Scale readiness | Track A / future |

No Spec pollution by production or Product Proof process (post A3.F).

---

## 11. Constitutional Compliance Scorecard

Scores: **Pass / Partial / Fail**. Partial requires cited residual risk without Effect-path proof of violation.

| Dimension | Score | Evidence |
| --- | --- | --- |
| Constitutional Compliance | **Pass** | Conversation Effect path matches Chain |
| Information Ownership Integrity | **Pass** | Owners map; Intent stages ≠ new Owners |
| Authority Integrity | **Pass** | CommandPipeline on steps; no hidden UI Authority for Effects |
| Traceability | **Pass** | Intent → IPC → plan → results → reply |
| Capability Integration | **Pass** | Sampled providers identical entry |
| Engineering Execution | **Pass** | EES wired in AGENTS/protocol/verify |
| Architectural Drift | **Pass** | No Effect-path drift; latent IPC Track A |
| Governance Separation | **Pass** | Spec vs governance docs |
| Repository Coherence | **Partial** | Active path coherent; historical Blueprint/sprint corpus still noisy |
| Long-term Maintainability | **Pass** | Spec + EES + checklist; Track A for IPC/docs |

---

## 12. Evidence Index

| Evidence | Location |
| --- | --- |
| Operator façade | `app/src/lib/operator/intelligence.ts` |
| Banned provider IPC | `app/src/lib/operator/runtimeBridge.ts` |
| Conversation shell vs capability | `app/src/components/operator/OperatorRoot.tsx` |
| Voice review/Send | `OperatorRoot` `onVoiceTranscript` |
| Kernel entry | `packages/kernel/src/commands/capability_intent.rs` |
| Router single provider | `packages/kernel/src/capability_runtime/router.rs` |
| Kernel Authority Rule | `docs/operator/KERNEL_AUTHORITY_RULE.md` |
| Spec / EES | `WORKSPACE_CONSTITUTIONAL_SPECIFICATION_V2.md`, `WORKSPACE_ENGINEERING_EXECUTION_STANDARD_V1.md` |

---

## 13. Runtime corrections

**None.** No objective, behavior-preserving runtime defect requiring change before Product Proof.

Recommended **Track A** (not this audit): IPC quarantine; historical doc banners; optional terminology map Intent modules → Understanding.

---

## 14. Explicit answers

| Question | Answer |
| --- | --- |
| Does the repository truly operate under the Constitution? | **Supported** for audited Conversation Operator Effect path (Verified within that scope) |
| Any runtime violate constitutional ownership? | **No evidence** on audited Effect path |
| Any subsystem possess hidden authority? | **No evidence** on audited path; latent IPC = Supported risk / Unknown exhaustiveness |
| Has architectural drift begun? | **No evidence** on Effect path |
| Is EES actually being followed? | **Verified** at repository entry points; ongoing sessions = Supported |
| Objectively justified runtime work before Product Proof? | **No** |

---

## 15. Repository health / branch

| Field | Value |
| --- | --- |
| Branch | `v2-dev` |
| Tag | `workspace-constitution-v2.1` |
| Health | Spec 2.1; P16 Product Proof pending |
| Confidence model | `REPOSITORY_CONFIDENCE_MODEL.md` |

---

## 16. Confidence Boundary (permanent section)

### Proven (Verified — direct repository evidence in this audit)

- Conversation → Effect path  
- Transformation Chain on that path  
- Information ownership mapping for audited subsystems  
- Authority Gate (`CommandPipeline` on steps)  
- Kernel composition (providers do not call providers)  
- Provider isolation (Router → single provider)  
- Engineering entry points (AGENTS, protocol, EES, checklist)

### Not Proven

- Every dormant subsystem  
- Future capabilities  
- Future providers  
- Future production programs  
- Third-party integrations  
- Future operating systems  
- Exhaustive IPC-surface misuse scenarios  

### Future evidence required

Constitutional review reopens only if:

1. Objective non-compliance appears within an examined or newly examined scope  
2. A Constitutional Review Trigger occurs  
3. A constitutional amendment is proposed with Product Owner approval  

---

## Declaration

No constitutional non-compliance was found within the audited operational scope. The audited operational path faithfully implements the Workspace Constitutional Specification v2 and the Workspace Engineering Execution Standard v1. Future constitutional review requires new objective evidence or an explicit Constitutional Review Trigger.

Future engineering SHOULD proceed under the existing constitutional framework:

1. Product Proof  
2. Track A production  
3. Capability delivery  
4. UX / performance / reliability  

—not constitutional evolution. Meta-engineering on the governance stack SHOULD stop unless new objective evidence appears.
