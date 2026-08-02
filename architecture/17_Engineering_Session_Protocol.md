# Engineering Session Protocol v1.0

Status: Active
Authority: Authoritative engineering-session governance for Workspace
Version: 1.0

This document governs how engineering sessions begin, decide, stop, validate,
and record work. It does not alter capability ownership, accepted architecture,
contracts, Product Proof behaviour, or runtime functionality.

Authority order for conflicts involving this document:

1. `00_Workspace_Blueprint.md`
2. Accepted ADR bodies in `architecture/decisions/`
3. Authoritative capability, interaction, contract, and schema documents
4. `04_Cursor_Protocol.md` for the implementation process steps
5. This document for session startup, model responsibilities, stop conditions,
   validation, and git discipline of engineering practice
6. `01_Current_State.md`, `02_Engineering_Ledger.md`, and other operational
   records for mission state and history

If this document conflicts with a higher authority, the higher authority wins.
If two same-level authorities conflict, stop and report; do not invent a
resolution.

---

## 1. Purpose

Every engineering session—regardless of AI model or human contributor—must
begin from repository authority rather than conversational context.

This protocol exists so that:

- the repository remains the sole engineering authority
- sessions are replaceable (ADR-0008)
- incomplete or contradictory authority produces a stop, not invention
- architecture, implementation, and review roles remain distinct
- Product Proof and capability boundaries are not silently widened

This is an engineering governance document only.

---

## 2. Repository authority hierarchy

### Authoritative

| Order | Authority | Role |
|---|---|---|
| 1 | Blueprint | Highest product and principle authority |
| 2 | Accepted ADR bodies | Binding decisions |
| 3 | Capability Architecture, Interaction Matrix, Capability Contracts, Contract Schema | Ownership, communication, public contracts, conceptual acceptance |
| 4 | Specialized authoritative specs (for example Action Desktop Mutation Contract, Saved Context Restore Identity Specification) when in scope | Narrow contract bindings subordinate to higher documents |
| 5 | Cursor Protocol | Implementation process steps |
| 6 | This Engineering Session Protocol | Session practice governance |
| 7 | Current State | Active mission, completed work, next task |
| 8 | Engineering Ledger | Decision and change history |
| 9 | Research Catalogue and accepted research records | Evidence and research status; not runtime design by themselves |
| 10 | Code, tests, migrations, and package manifests | Implementation truth for what exists |

### Never authoritative

- Conversation history, chat summaries, or prior agent transcripts
- Uncommitted local notes outside the repository
- Proposed-but-unaccepted architecture documents
- Model memory, preferences, or “what we discussed last time”
- External blog posts or vendor docs unless cited by an accepted repository
  research or ADR artifact

Conversation may supply a task objective. It may not supply missing
architecture, invent contracts, or override repository records.

---

## 3. Mandatory session startup procedure

Before planning or mutating the repository, every engineering session must:

1. Read `04_Cursor_Protocol.md`
2. Read this document (`17_Engineering_Session_Protocol.md`)
3. Complete the required reading order in §4 for the session’s mode
4. State the session mode (architecture, implementation, review, research,
   documentation, or product-strategy as declared by the task)
5. State the exact objective and the repository authorities that bind it
6. Confirm mutation permissions from the task and higher authorities
7. Stop if authority is incomplete, contradictory, or insufficient for the
   requested mutation class

No implementation, architecture redesign, or contract invention may begin
before steps 1–6 succeed.

---

## 4. Required reading order

### Deterministic base order (every engineering session)

1. `04_Cursor_Protocol.md`
2. `17_Engineering_Session_Protocol.md` (this document)
3. `00_Workspace_Blueprint.md`
4. `01_Current_State.md`
5. `02_Engineering_Ledger.md` (recent and task-relevant entries)
6. `03_Research_Catalogue.md` (entries relevant to the task)

### Mode-specific additions (read after the base order, before planning)

**Architecture session**

- Accepted ADRs relevant to the change
- `08_Workspace_Capability_Architecture.md`
- `09_Capability_Interaction_Matrix.md`
- `10_Capability_Contracts.md`
- `11_Contract_Schema_and_Acceptance_Specification.md`
- Any specialized authoritative specification named by the task

**Implementation session**

- Current State active/next task and known unknowns
- Authoritative contracts and specs that bind the task
- Existing code and tests in the owned surfaces only
- Engineering Ledger entries that unlocked or blocked the task

**Review session**

- The claimed objective, change set, and validation evidence
- The same authorities the implementer was bound by
- Do not expand scope into redesign unless the review finds a contradiction
  that requires architecture referral

**Research session**

- Capability Technology Research Framework and Research Roadmap when applicable
- Catalogue entry and any existing research record for the capability

Reading is complete when the session can cite the binding authorities for the
objective without relying on chat memory.

---

## 5. Model responsibilities

### Architecture

- Consumes and, when authorized, amends architecture documentation only
- Must not implement runtime behaviour in an architecture-only session
- Must not invent capabilities, ownership, or contracts to unblock work
- Stops and reports when implementation is requested but architecture is not
  ready

### Implementation

- Consumes approved architecture exactly
- Prefers extending existing owned code
- Must not move logic across capability boundaries
- Must not weaken permissions, explainability, or trust guarantees
- Must not redesign architecture when a contradiction appears; stop and report

### Review

- Audits conformance to repository authority
- Does not silently “fix forward” by inventing missing authority
- Distinguishes product/strategy gaps from architecture defects from
  implementation defects
- May recommend stop, referral, or the smallest conformance correction

A single session may be declared as one primary mode. Mixed mode is allowed
only when the task explicitly authorizes each mutation class.

---

## 6. Prompt generation standard

Engineering prompts and session briefs should:

1. Name the mode and mutation permissions
2. Name the objective and out-of-scope items
3. Require the Cursor Protocol and this protocol to be read first
4. Point to binding repository authorities by path
5. Forbid inventing contracts, ownership, or Product Proof behaviour
6. Define stop conditions and validation commands
7. Define documentation and git expectations

Prompts must not instruct a model to treat conversation history as authority,
bypass Current State, or “just make it work” when architecture is incomplete.

Reusable prompt shapes may be recorded in `07_Prompt_Pattern_Library.md`
without changing this protocol’s requirements.

---

## 7. Implementation stop conditions

Stop immediately and report, without inventing a fix, when any of the following
is true:

- Required authoritative documents are missing, unread, or mutually contradictory
- The task requires a contract, action type, ownership assignment, or permission
  scope that does not exist in the repository
- Implementation would bypass capability ownership or move responsibility
- Validation cannot be run, or required acceptance evidence cannot be produced
- The task asks for runtime behaviour that Current State or the Ledger marks
  blocked or not unlocked
- Product Proof or trust rules would be weakened (ambient capture, silent
  retry, unpreviewed mutation, dishonest success)
- Architecture mutation is forbidden by the task but appears necessary

Stopping is success when it prevents unauthorized invention.

---

## 8. Validation requirements

Before declaring implementation work complete:

1. Run the validation commands required by the task, or the repository’s
   standard known-good checks when the task does not narrow them
2. Ensure acceptance evidence required by binding contracts is present
3. Confirm no unauthorized architecture or ownership change occurred
4. Confirm documentation updates required by the Cursor Protocol were made
5. Confirm git discipline in §9 was followed

Documentation-only sessions validate by consistency of reading order, links,
and non-contradiction with higher authorities. They do not require runtime
test suites unless the task says otherwise.

---

## 9. Git discipline

- Commit only when the task asks for a commit, or when the session’s declared
  completion rules require one after successful validation
- Do not commit secrets, unrelated work, or mixed product-proof and
  unrelated research changes in one commit without explicit instruction
- Prefer separate commits for implementation and required documentation when
  the task so specifies
- Do not amend, force-push, or rewrite shared history unless the user
  explicitly requests it
- Push only when the task explicitly requests a push
- Conversation claims of “already committed” are irrelevant; `git` status in
  the repository is authoritative

---

## 10. Architecture review cadence

- Architecture-changing work requires an architecture session (or explicit
  architecture authority) before implementation
- After architecture acceptance that unblocks an implementation task, an
  implementation session may proceed against that accepted text only
- If implementation reveals a genuine contradiction with accepted architecture,
  stop; do not redesign in place
- Product Proof readiness and sequencing decisions are recorded in the
  Engineering Ledger and Current State; they do not silently amend contracts
- Proposed documents (for example a proposed Architecture Guardian) are not
  session gates until accepted

---

## 11. Engineering invariants

1. **Repository is the authority.** Chat is not.
2. **Sessions are replaceable.** A new session must be able to continue from
   artifacts alone (ADR-0008).
3. **No ambient authority.** Missing facts are reported, not inferred into
   contracts or ownership.
4. **Capability ownership is frozen during implementation** unless an
   architecture session explicitly changes it.
5. **Permission before automation** remains binding for product behaviour.
6. **Explainability and honest outcomes** are not optional quality nits.
7. **Documentation never lives in runtime** (ADR-0007); this protocol is an
   architecture-pack artifact, not application payload.
8. **Complexity must justify itself**; session process must not invent new
   subsystems to feel complete.
9. **Stop over invent.** Contradiction reporting is preferred to speculative
   repair.
10. **Cursor Protocol still governs implementation steps**; this document
    governs how sessions start and when they must refuse to proceed.

---

## 12. Relationship to other pack documents

| Document | Relationship |
|---|---|
| `04_Cursor_Protocol.md` | Entry process; reads this protocol next; then Blueprint and operational records |
| ADR-0008 | Sessions replaceable; reinforces conversation non-authority |
| ADR-0007 | This file remains outside runtime |
| `07_Prompt_Pattern_Library.md` | May store prompt examples that obey this protocol |
| `14_Architecture_Guardian.md` | Proposed only; not a gate until accepted |
| Current State / Ledger | Record mission progress; do not replace architecture |

---

## 13. Acceptance of this protocol

Adding this protocol is complete when:

- it is present as `architecture/17_Engineering_Session_Protocol.md`
- `04_Cursor_Protocol.md` includes it in the mandatory reading order before
  Current State
- no capability, contract, or runtime file was modified to invent behaviour
- a new session can determine startup order from the repository alone
