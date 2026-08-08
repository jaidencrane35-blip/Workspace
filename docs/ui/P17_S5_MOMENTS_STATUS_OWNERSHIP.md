# P17.S5 — Moments Status Ownership

| Field | Value |
| --- | --- |
| **Program** | Product Optimization — Moments Status Ownership |
| **Prior audit** | `docs/ui/P17_A5_ATTENTION_ORCHESTRATION_AUDIT.md` |
| **Date** | 2026-08-08 |
| **Commit** | _(stamped on ship)_ |
| **Branch** | `v2-dev` |

---

## 1. Files modified

| File | Change |
| --- | --- |
| `app/src/components/ResumeContextPanel.tsx` | Inline delete ack; clear status chrome; Owner restore outcome |
| `app/src/components/SaveContextPanel.tsx` | Clear stale ok-banner on save / clear draft |
| `app/src/App.tsx` | Error Dismiss; ok auto-clear; clear message on view change |
| `app/src/App.css` | Dismissible error banner layout |
| `scripts/verify-moments-status-ownership.mjs` | Regression verifier |
| `scripts/verify-moments-success-cohesion.mjs` | Align with inline delete (S5) |
| `tests/moments-status-ownership.test.ts` | Source continuity tests |

**Unchanged:** S1 Conversation failure compose, S2 Save/Restore success cards, S3 agency keys, S4 working-state, voice/Soft Send, toast infrastructure.

---

## 2. Before / after interaction flow

### Delete
| Before | After |
| --- | --- |
| Confirm → sticky green `Deleted “…"` toast | Confirm → inline Moments `“… is gone…”` |
| Toast survives later Resume steps | Cleared on next Moments action / view change |

### Restore done
| Before | After |
| --- | --- |
| You’re back + `Outcome: partially completed` | You’re back + `Mostly restored.` (Owner language) |

### Error banner
| Before | After |
| --- | --- |
| Sticky until next error overwrite | **Dismiss** control; also cleared when Moments starts a new action |

---

## 3. Validation

```text
node scripts/verify-moments-status-ownership.mjs
node scripts/verify-moments-success-cohesion.mjs
pnpm typecheck
pnpm --filter @workspace/tests exec vitest run moments-status-ownership
```

---

## 4. Remaining follow-ups (not in this slice)

- Dock layout Conversation-still-primary contract
- ActiveMoment truthful browse on App path
- Post-hoc streamText tempering
- Notify success first-person wording

---

## 5. Stop

One vertical slice complete. Await Product Owner review.
