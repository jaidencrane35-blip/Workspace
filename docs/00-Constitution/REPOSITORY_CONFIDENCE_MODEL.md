# Repository Confidence Model

| Field | Value |
| --- | --- |
| **Kind** | Engineering Governance — not constitutional law |
| **Status** | Binding for audits and architectural claims |
| **Related** | Evidence Before Architectural Confidence (constitutional principle); Engineering Execution Standard v1 |

---

## Purpose

Repository audits and engineering claims MUST distinguish **evidence of compliance in examined scope** from **universal certainty**. This model prevents epistemic drift.

---

## Confidence levels

| Confidence | Meaning |
| --- | --- |
| **Verified** | Direct repository evidence examined for the stated scope |
| **Supported** | Strong evidence; not exhaustively proven across all related surfaces |
| **Hypothesis** | Plausible; awaiting repository evidence |
| **Unknown** | Not yet audited |

---

## Rules

1. Every audit conclusion MUST carry a confidence level and a stated scope.  
2. “No evidence of non-compliance” MUST NOT be reported as “proof of universal compliance.”  
3. Expanding scope (new provider, dormant IPC, new OS) resets relevant claims to Supported, Hypothesis, or Unknown until re-examined.  
4. Constitutional redesign claims require Verified non-compliance or a Review Trigger — never Hypothesis alone.

---

## Use in audits

Each scorecard row and executive claim SHOULD note:

- **Scope** (what was examined)  
- **Confidence** (Verified / Supported / Hypothesis / Unknown)  
- **Evidence** (paths / behaviours)
