# Product Proof Rule
## Permanent engineering completion criterion

| Field | Value |
| --- | --- |
| **Status** | Authoritative (P12.5) |
| **Applies to** | Every Capability Provider execution program |
| **Authority** | Also recorded in `.cursor/rules/constitutional-execution-protocol.mdc` |

---

## Rule

A provider is **not Product Complete** until all of the following are true:

1. The provider exists and is independently testable.
2. It is connected to Conversation through the frozen Capability Runtime pipeline.
3. The Product Owner can naturally discover and use it without menus, dashboards, or documentation.
4. Conversation returns **truthful** success and failure feedback (never invented success).
5. Users never see Provider / Router / Registry terminology.
6. **Natural Language Robustness** (below) is satisfied.

Infrastructure alone is insufficient.

---

## Natural Language Robustness Rule (permanent — P14.5)

A provider is **not Product Complete** merely because commands execute correctly.  
A provider becomes Product Complete only when ordinary users can express the same intent naturally.

Natural language improvements belong **only** inside:

- Intent Layer  
- Kernel Operator (planning / compose)  
- Conversation reply generation  

They do **not** belong inside providers.  
Providers remain deterministic.  
Intent resolution remains deterministic (no probabilistic AI matching).

Future execution programs must satisfy **both**:

- Engineering Completion  
- Product Proof  

before they are considered complete.

---

## Frozen pipeline (no shortcuts)

```
Conversation
    ↓
Intent Layer → CapabilityIntent
    ↓
execute_capability_intent
    ↓
Kernel Operator
    ↓
Capability Runtime → Capability Router
    ↓
Provider Registry
    ↓
Capability Provider
    ↓
Desktop Service
    ↓
Kernel Operator (response)
    ↓
Conversation Response
```

---

## Required Product Proof harness (every provider)

Each provider ships a harness under `docs/capability-runtime/product-proof/`:

| Artifact | Purpose |
| --- | --- |
| `{provider}.proof.json` | Machine-checkable utterances → expected intent kinds |
| `{PROVIDER}_PRODUCT_PROOF.md` | Owner review checklist + failure / permission notes |

Harness contents (minimum):

- Conversation examples  
- Expected routing (intent kind → operation)  
- Expected execution surface (IPC)  
- Expected response character (truthful, user language)  
- Failure examples  
- Permission notes  
- Validation examples (wired into Vitest / verifier)

---

## Ownership

| Concern | Owner |
| --- | --- |
| Language understanding | Conversation / Intent Layer |
| Execution | Capability Provider |
| Permissions / audit | Capability Runtime (unchanged) |
| Product Proof acceptance | Product Owner experience |

Providers never call each other. Conversation never bypasses the runtime.
