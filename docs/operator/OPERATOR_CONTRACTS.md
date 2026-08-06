# Operator Contracts

## OperatorTurnRequest

| Field | Type | Notes |
| --- | --- | --- |
| `utterance` | string | Raw Conversation input |
| `contextId?` | string | Optional turn context (future) |

## OperatorOutcome

| Kind | Meaning |
| --- | --- |
| `reply` | User-facing text (+ optional suggestion). No further UI effect. |
| `shell` | Presentation directive only (navigate, collapse, developer, …). Never a provider invoke. |

## OperatorPlanStep

| Field | Type |
| --- | --- |
| `domain` | `clipboard` \| `application` \| `window` \| … |
| `operation` | string |
| `args` | record |

## OperatorPlan

| Field | Type |
| --- | --- |
| `steps` | OperatorPlanStep[] |
| `compositionId?` | string — from composition catalogue when multi-domain |

## Failure contract

Operator never invents success. Runtime/provider errors become truthful `reply` text without Provider jargon.
