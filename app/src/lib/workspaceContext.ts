/**
 * Workspace Context Model (P16.37) — Intent Layer.
 *
 * Owns session continuity only:
 * - current objective / last action / plan referents
 * - pronoun resolution (it / that / there)
 * - "again" / continue-what-I-was-doing
 * - unresolved clarification
 *
 * Does NOT own: grammar, Situation Goals, Goal Resolution ranking, Kernel composition.
 * Deterministic. No ML. No hidden AI.
 */

import type { IntentAction } from "./intentBridge";
import type { ExecutionPlan } from "./executionPlanner";

export interface WorkspaceContextState {
  currentObjective: string | null;
  lastAction: IntentAction | null;
  lastUtterance: string | null;
  lastWindowQuery: string | null;
  lastAppQuery: string | null;
  lastUrl: string | null;
  lastBeside: string | null;
  lastPlanSummary: string | null;
  unresolvedClarification: string | null;
  completedKinds: string[];
  pendingKinds: string[];
  windowsReferenced: string[];
  appsLaunched: string[];
  turn: number;
}

const MAX_HISTORY = 12;

let state: WorkspaceContextState = emptyState();

function emptyState(): WorkspaceContextState {
  return {
    currentObjective: null,
    lastAction: null,
    lastUtterance: null,
    lastWindowQuery: null,
    lastAppQuery: null,
    lastUrl: null,
    lastBeside: null,
    lastPlanSummary: null,
    unresolvedClarification: null,
    completedKinds: [],
    pendingKinds: [],
    windowsReferenced: [],
    appsLaunched: [],
    turn: 0,
  };
}

export function resetWorkspaceContext(): void {
  state = emptyState();
}

export function getWorkspaceContext(): Readonly<WorkspaceContextState> {
  return state;
}

function norm(raw: string): string {
  return raw
    .trim()
    .toLowerCase()
    .replace(/[.!?]+$/g, "")
    .replace(/,/g, " ")
    .replace(/\s+/g, " ")
    .replace(/^(please|can you|could you|would you|i think)\s+/i, "")
    .replace(/\s+(please|for me|now|thanks|thank you)$/i, "")
    .trim();
}

function pushUnique(list: string[], value: string): string[] {
  const next = [...list.filter((x) => x !== value), value];
  return next.slice(-MAX_HISTORY);
}

function referentLabel(): string | null {
  return state.lastWindowQuery ?? state.lastAppQuery ?? null;
}

function cloneAction(action: IntentAction): IntentAction {
  return { ...action };
}

/**
 * Context-owned intents (continuity / again / pronoun close).
 * Returns null when Grammar → Situation → Goal Resolution should proceed.
 */
export function resolveFromWorkspaceContext(raw: string): {
  action: IntentAction;
  evidence: string[];
} | null {
  const text = norm(raw);
  if (!text) {
    return null;
  }
  const evidence: string[] = [`turn=${state.turn}`];

  // Replay last action
  if (
    /^(do that again|do it again|again|once more|repeat that)$/i.test(text)
  ) {
    if (state.lastAction) {
      evidence.push("again→replay_last_action");
      return {
        action: {
          ...cloneAction(state.lastAction),
          reply:
            state.lastAction.reply ||
            "Doing that again from the current session context.",
        },
        evidence,
      };
    }
    evidence.push("again→no_prior_action");
    return {
      action: {
        kind: "unknown",
        reply:
          "There’s nothing in this session to repeat yet. Ask for a desktop action first.",
        suggestion: 'Try “open Chrome” or “open ChatGPT beside Cursor”.',
      },
      evidence,
    };
  }

  // Session continuity → Continue (Context owns still working / back / previous / resume)
  if (
    /^(i'?m still working|still working|finish this|finish up|go back|back|previous)$/i.test(
      text,
    ) ||
    /^(bring everything back|continue what i was doing|resume what i was doing)$/i.test(
      text,
    )
  ) {
    evidence.push("continuity→continue_surface");
    return {
      action: {
        kind: "navigate",
        view: "resume",
        reply:
          "Let’s get you back through Continue — restore only runs after you approve a saved Moment. I won’t invent a layout.",
      },
      evidence,
    };
  }

  // Find / locate it — Context binds pronouns; Goal Resolution clarifies only when unbound
  if (
    /^(find it|locate it|where (is|did) it(?: go)?|show it|bring it back)$/i.test(
      text,
    )
  ) {
    const ref = referentLabel();
    if (ref) {
      evidence.push(`find_pronoun→${ref}`);
      return {
        action: {
          kind: "winFocus",
          query: ref,
          reply: `Looking for “${ref}” from session context.`,
        },
        evidence,
      };
    }
    return null;
  }

  // Close that / it — requires a referent
  const closeRef = text.match(/^(close|quit|dismiss)\s+(that|it|this)(?:\s+window)?$/i);
  if (closeRef) {
    const ref = referentLabel();
    if (ref) {
      evidence.push(`close_pronoun→${ref}`);
      return {
        action: {
          kind: "appClose",
          query: ref,
          reply: `Closing “${ref}”.`,
        },
        evidence,
      };
    }
    evidence.push("close_pronoun→clarify");
    return {
      action: {
        kind: "unknown",
        reply:
          "Close what — which window or app? Name it; I won’t invent a target.",
        suggestion: 'Try “close Chrome” or open something first.',
      },
      evidence,
    };
  }

  // Focus / bring / show that window (bare "move this window" stays Intent clarify)
  const thatWindow = text.match(
    /^(focus|bring|show)\s+(that|it|this)(?:\s+window)?(?:\s+to\s+the\s+front)?$/i,
  );
  if (thatWindow) {
    const ref = referentLabel();
    if (ref) {
      evidence.push(`window_pronoun→${ref}`);
      return {
        action: {
          kind: "winFocus",
          query: ref,
          reply: `Looking for “${ref}”.`,
        },
        evidence,
      };
    }
    return {
      action: {
        kind: "unknown",
        reply: "Which window? Name it, or open something first so I have a referent.",
      },
      evidence: [...evidence, "window_pronoun→clarify"],
    };
  }

  // Put / open it beside Y
  const beside = text.match(
    /^(?:put|place|open|move)\s+(it|that|this)\s+(?:beside|next\s+to|alongside)\s+(.+)$/i,
  );
  if (beside?.[2]) {
    const besideTarget = beside[2].trim();
    if (state.lastUrl) {
      evidence.push(`it_beside→url+${besideTarget}`);
      return {
        action: {
          kind: "browserOpenBeside",
          url: state.lastUrl,
          beside: besideTarget,
          reply: `Opening beside “${besideTarget}” from session context.`,
        },
        evidence,
      };
    }
    if (state.lastAppQuery) {
      // App beside → open browser surface beside (same Kernel composition path as browser beside)
      evidence.push(`it_beside→app_as_browser_surface+${besideTarget}`);
      return {
        action: {
          kind: "browserOpenBeside",
          url: "https://www.google.com",
          beside: besideTarget,
          reply: `Opening your recent app surface beside “${besideTarget}”.`,
        },
        evidence,
      };
    }
    evidence.push("it_beside→clarify");
    return {
      action: {
        kind: "unknown",
        reply:
          "Put what beside that — which app or site? Open it first, or name it.",
        suggestion: 'Try “open Chrome” then “put it beside Cursor”.',
      },
      evidence,
    };
  }

  // Open the other one — needs a prior beside/app pair; otherwise clarify
  if (/^(open|show|focus)\s+(the\s+)?other\s+one$/i.test(text)) {
    if (state.lastBeside) {
      evidence.push(`other_one→${state.lastBeside}`);
      return {
        action: {
          kind: "winFocus",
          query: state.lastBeside,
          reply: `Looking for “${state.lastBeside}”.`,
        },
        evidence,
      };
    }
    if (state.lastAppQuery && state.appsLaunched.length > 1) {
      const other =
        state.appsLaunched[state.appsLaunched.length - 2] ?? state.lastAppQuery;
      evidence.push(`other_one→prior_app_${other}`);
      return {
        action: {
          kind: "winFocus",
          query: other,
          reply: `Looking for “${other}”.`,
        },
        evidence,
      };
    }
    return {
      action: {
        kind: "unknown",
        reply:
          "The other one — which? I need two things in this session first.",
        suggestion: 'Try “open ChatGPT beside Cursor”, then “open the other one”.',
      },
      evidence: [...evidence, "other_one→clarify"],
    };
  }

  return null;
}

/**
 * Record outcome after Intent resolution for the next turn.
 */
export function commitWorkspaceContext(
  utterance: string,
  action: IntentAction,
  plan?: ExecutionPlan | null,
): void {
  state = {
    ...state,
    turn: state.turn + 1,
    lastUtterance: utterance.trim(),
    lastAction: cloneAction(action),
    currentObjective: action.reply || state.currentObjective,
    lastPlanSummary: plan
      ? plan.steps.map((s) => s.goal).join(" → ")
      : state.lastPlanSummary,
    unresolvedClarification:
      action.kind === "unknown"
        ? action.reply
        : null,
    completedKinds: pushUnique(state.completedKinds, action.kind),
    pendingKinds:
      action.kind === "unknown"
        ? pushUnique(state.pendingKinds, "clarification")
        : state.pendingKinds.filter((k) => k !== "clarification"),
  };

  if ("query" in action && typeof action.query === "string" && action.query) {
    const q = action.query;
    if (
      action.kind === "winFocus" ||
      action.kind === "winMaximize" ||
      action.kind === "winMinimize" ||
      action.kind === "winRestore" ||
      action.kind === "appClose" ||
      action.kind === "winFocusMinimize"
    ) {
      if (!/^(it|that|this|something|one|everything back)$/i.test(q)) {
        state.lastWindowQuery = q;
        state.windowsReferenced = pushUnique(state.windowsReferenced, q);
      }
    }
    if (
      action.kind === "appOpen" ||
      action.kind === "appLaunch" ||
      action.kind === "appOpenMaximize" ||
      action.kind === "appFocus"
    ) {
      state.lastAppQuery = q;
      state.appsLaunched = pushUnique(state.appsLaunched, q);
      state.lastWindowQuery = state.lastWindowQuery ?? q;
      state.windowsReferenced = pushUnique(state.windowsReferenced, q);
    }
  }

  if ("url" in action && typeof action.url === "string") {
    state.lastUrl = action.url;
  }
  if ("beside" in action && typeof action.beside === "string") {
    state.lastBeside = action.beside;
    state.windowsReferenced = pushUnique(state.windowsReferenced, action.beside);
  }
  if ("focusQuery" in action && typeof action.focusQuery === "string") {
    state.lastWindowQuery = action.focusQuery;
  }
}
