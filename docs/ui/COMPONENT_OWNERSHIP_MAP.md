# Component Ownership Map

| Path | Layer | Track | Notes |
| --- | --- | --- | --- |
| `app/src/components/operator/DesktopOperator.tsx` | 1 | A | Form A UI |
| `app/src/components/operator/OperatorRoot.tsx` | 2 | A | Form B shell + dock host |
| `app/src/components/operator/RepositoryHealthPanel.tsx` | Dev | A | Developer only |
| `app/src/lib/shellRuntime.ts` | 1–2 | A | Durable mode/size/pos |
| `app/src/lib/shellWindows.ts` | 1–2 | A | Native window apply |
| `app/src/lib/shellStateMachine.ts` | 1–2 | A | Zero-Trap graph |
| `app/src/lib/intentBridge.ts` | 2→3 | A/B | Deterministic intents |
| `app/src/lib/capabilityEvolution.ts` | 4 | C | Proposals only |
| `app/src/App.tsx` | 2 host | A | Mounts OperatorRoot + PP satellites |
| Product Proof views (`home`/`save`/…) | 3 satellite | B hosted | Conversation-opened only |
| `docs/ui/UI_ARCHITECTURE_SPECIFICATION.md` | Law | A | Presentation authority |

**Rule:** New UI files must declare Layer + Track in PR/milestone notes.
