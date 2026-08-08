# T2 — Product Proof Rejection Audit  
## Companion Identity & Conversational Experience

| Field | Value |
| --- | --- |
| **Program** | Owner Product Proof — Rejection Audit (Release Hold Trigger **T2**) |
| **Date** | 2026-08-08 |
| **Branch** | `v2-dev` |
| **Kind** | Product Proof audit — **no implementation** |
| **Primary evidence** | Live Owner Product Proof session (rejected) |
| **Secondary evidence** | Repository conversation / intent surfaces (identity mechanisms only) |
| **Posture** | Release Hold remains; this audit does not authorize engineering until Owner chooses T1 |

**Binding:** Owner observations are accepted as truth. This document explains *why* the rejection occurred as systemic product behaviour — not as a defect punch-list.

---

## 1. Executive Summary

The Owner rejected Workspace not because providers fail to exist, but because **the product does not yet behave as a Conversational Desktop Operator**. In live use it presents as a **truthful command help system**: greetings and recoveries advertise recipe phrases; refusals repeat defensive “I won’t invent…” language; each turn is interpreted as an isolated command; pronouns and goals across turns collapse; multi-step desktop asks shrink to the first verb; visual chrome reads unfinished.

**Verdict:** Architecture and capability plumbing are ahead of **companion identity**. Conversation currently optimizes for *honest non-invention* and *discoverable commands* more than for *ongoing collaboration about the desktop*.

**Smallest systemic explanation (three root causes):**

1. **Help-shaped Conversation voice** — recovery and greeting language trains “type a command.”  
2. **Command-turn Intent model** — one utterance ≈ one mapped operation; weak goal/anaphora continuity.  
3. **Operator presence gap** — desktop agency and premium surface do not yet match “companion” claims.

**Single recommended vertical slice (if Owner continues):**  
**Companion Conversation Voice** — rewrite greeting, unknown, and recovery replies so Workspace speaks as a collaborator, not a command catalogue (Intent/Conversation layer only; no architecture redesign).

---

## 2. Companion Identity Assessment

| Stated identity | Live Owner experience |
| --- | --- |
| Conversational Desktop Operator | Feels like **Windows Help** |
| Desktop companion | Feels like a **command interpreter** with honesty disclaimers |
| Conversation is the product | Conversation is a **prompt to recite supported commands** |

**Assessment:** Identity mismatch is decisive. Product Gravity and Interaction Language describe a calm companion; live replies teach a limited command vocabulary. Excellence of Spec/pipeline does not transfer into felt companionship when every soft failure returns a “Try …” menu.

---

## 3. Conversation Assessment

| Question | Finding |
| --- | --- |
| Matches CDO identity? | **No** — Help/catalog posture dominates |
| Recovery language for end users? | **Inappropriate as primary voice** — defensive, implementation-aware (“won’t invent”), repetitive |
| Memory / continuity? | **Insufficient** — pronouns (`that`, `it`, `again`, `the folder`) fail across turns |
| Greeting behaviour | Immediately attaches **command suggestions** after a short companion line |
| Churn | Same capability suggestions and refusal tropes recur almost every soft miss |

**Mechanism (supporting, not contradicting Owner):** Intent Layer greets with a suggestion list; unknown/near-miss guidance rotates a small pool of “won’t invent / won’t pretend” lines plus near-identical “Try …” capability recipes (`conversationGuidance.ts`, greeting path in `intentBridge.ts`). Kernel sanitize paths also normalize engineering failures into the same “won’t invent / won’t pretend” family (`compose.rs`). Truthfulness was engineered; **companionship was not**.

---

## 4. Desktop Operator Assessment

| Question | Finding |
| --- | --- |
| Intent level vs command level? | **Primarily command-level** — multi-step goals often execute only the first action |
| Desktop awareness | **Weaker than Owner expectation** for a desktop operator |
| Collaborative recovery | Recovery offers **command recipes**, not joint problem-solving (“what are we trying to finish?”) |
| Control of own window vs target | Opening Notepad **minimized Workspace** — operator appears to act on itself, not the desktop task |

**Assessment:** Operator authority is incomplete in *felt* desktop agency. Even when individual providers work, the product does not consistently plan or narrate **user-shaped** desktop work, so the Owner experiences a thin command runner.

---

## 5. Interaction Quality Assessment

| Dimension | Owner-facing result |
| --- | --- |
| Predictability | Predictably Help-like — same suggestions return |
| Trust | Honesty exists, but sounds **defensive**, not trustworthy-calm |
| Cognitive load | Owner must remember exact phrases; software does not adapt |
| Learnability | Learns the **command list**, not a relationship with the desktop |
| Responsiveness | Not the rejection core; identity and continuity dominate |

User Adaptation Prohibition is violated in spirit: the Owner must adapt speech to rigid command forms and re-state referents every turn.

---

## 6. Visual Product Assessment

Owner evidence: large scrollbars, oversized controls, weak spacing, little hierarchy, low premium feel.

**Assessment:** Presentation **supports** the rejection (“unfinished product”) but is **secondary**. A polished Help surface would still fail Product Proof if Conversation remains a command catalogue. Visual work should not be the first slice unless Owner explicitly prioritizes it after identity is fixed.

---

## 7. Root Cause Analysis

Grouped into the **smallest** systemic causes that explain the majority of observations.

### RC-A — Help-shaped Conversation voice (Critical)

Conversation treats soft failure, greeting, and near-miss as **capability advertising**. Defensive truth tropes (“I won’t invent…”) and recurring “Try …” lists produce Windows Help energy.

**Explains:** Help feel · greeting advertises commands · repeated suggestions · “I won’t invent…” churn · defensive tone · recovery as command suggestions · limitations communicated as catalog, not collaboration.

### RC-B — Command-turn Intent model (Critical)

Each Owner utterance is resolved largely as an **isolated command**, not as a turn in an ongoing desktop goal. Anaphora and “again / that / the folder” lack durable conversational binding; multi-step asks collapse to the first executable verb.

**Explains:** Pronoun failure · multi-step only first action · thinks in commands not goals · weak collaborative recovery · “command interpreter” overall.

### RC-C — Operator presence gap (High)

Felt desktop agency (what Workspace controls, what it knows, what happens to its own window) does not match “companion operating the desktop.” Visual unfinishedness amplifies the gap but is not the root.

**Explains:** Notepad / self-minimize surprise · weak desktop awareness · unfinished premium feel · “excellent architecture, weak product identity.”

**Not root causes for this rejection:** Missing File Provider alone; unsigned installer; IPC quarantine; constitutional architecture.

---

## 8. Ranked Product Proof Opportunities

| Rank | Opportunity | Owner impact | Effort | Risk |
| --- | --- | --- | --- | --- |
| **1** | **Companion Conversation Voice** — greetings, unknown, near-miss, and failure replies as collaborator language; stop default command-menu append; vary/retire “won’t invent” churn | **Critical** | S–M | Low–Med (copy + Intent/Conversation only) |
| 2 | **Turn continuity for referents** — bind that/it/again/the folder to recent desktop entities within session | Critical | M | Med (determinism) |
| 3 | **Goal-shaped multi-step planning** — one Owner ask → ordered desktop plan with honest partial progress | High | M–L | Med–High |
| 4 | **Operator window discipline** — never sacrifice Workspace presence incorrectly when acting on other apps | High | S–M | Med |
| 5 | **Premium Conversation chrome** — spacing, scrollbars, hierarchy | Med | S–M | Low (taste) |

---

## 9. Recommend — one vertical slice

### Companion Conversation Voice

**Problem:** Workspace tells the truth like a refusals engine and teaches commands like Help. The Owner cannot feel a companion.

**Outcome:** Empty and soft-miss Conversation sounds like a calm desktop partner: acknowledges the situation, states limits plainly **once**, invites the next natural step without a rotating command catalogue, and never opens with “here are three commands to memorize.”

**In scope (smallest):**
1. Greeting path — companion line only; no automatic “Try …” capability strip.  
2. Unknown / near-miss / generic guidance — collaborative phrasing; eliminate repetitive “won’t invent / won’t pretend” as the default chorus.  
3. Preserve constitutional honesty (no invented success, no fake capabilities).  
4. Intent Layer + Conversation reply composition only — **not** providers, not Spec, not new frameworks.  
5. Verifier: greetings and unknowns must not attach the canned multi-command suggestion list; refusal language must not churn the same invent/pretend lines.

**Out of scope for this slice:** Pronoun memory engine; full multi-step planner; visual redesign; File Provider; A2 signing.

**Success test (Owner):** After the slice, a greeting and a soft miss feel like talking to a desktop companion — not like opening Windows Help.

---

## Explicit answers

| Audit objective | Answer |
| --- | --- |
| Why command interpreter not companion? | Help-shaped voice + command-turn Intent (RC-A, RC-B) |
| Conversation model vs CDO identity? | **Mismatch** — Help/catalog vs companion |
| Recovery language appropriate? | **No** as primary voice |
| Conversation memory sufficient? | **No** |
| Intent vs command level? | **Command level** dominates |
| UI supports premium vision? | **No** — secondary |
| Limitations communicated trustworthily? | Honest but **not trustworthy-calm** — defensive catalog |

| Question | Answer |
| --- | --- |
| Why did Owner reject? | Weak companion identity over strong architecture |
| Implement now? | **No** |
| Release Hold? | **Remains** until Owner chooses T1 (or other T3–T5) |

---

## Compliance

| Constraint | Posture |
| --- | --- |
| No implementation | Observed |
| No architecture redesign | Observed |
| No new frameworks / Spec work | Observed |
| T2 audit only | Observed |
| Owner evidence primary | Observed |

---

## Stop

T2 rejection audit complete. **Do not implement.** Await Owner direction (typically T1: one bounded slice — Companion Conversation Voice).
