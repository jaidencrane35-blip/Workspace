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
import type { GoalContract } from "./goalContract";

export interface WorkspaceContextState {
  currentObjective: string | null;
  /** P23.S1 — comprehended meaning for this turn (never execution detail). */
  currentGoal: GoalContract | null;
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
    currentGoal: null,
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
 * P23.S5 — ground an elliptical question against what was just discussed.
 *
 * "Which one am I using?" means nothing on its own, and means something exact
 * straight after a question about the Owner's windows. Context already knows
 * which it is, because it holds the meaning of the previous turn.
 *
 * Grounding refines *meaning* only, and only into perception: it can turn an
 * unanswerable question into an answerable one, never into a desktop effect.
 * An ungrounded "which one?" is returned untouched, so nothing is invented —
 * the Owner still gets the honest limitation.
 *
 * It runs once per turn, before anything reads the goal, so every later reader
 * sees the same meaning.
 */
export function groundGoalInContext(goal: GoalContract): GoalContract {
  const elliptical = /^(which|what) one (am i|are we) (using|working in|in|on)\b/;
  if (!elliptical.test(goal.normalized)) {
    return goal;
  }
  const prior = state.currentGoal;
  const priorWasDesktopObservation =
    prior?.outcome === "PERCEIVE_MACHINE" &&
    prior.mode === "observation" &&
    prior.domain === "desktop";
  if (!priorWasDesktopObservation) {
    return goal;
  }
  return {
    ...goal,
    outcome: "PERCEIVE_MACHINE",
    mode: "observation",
    domain: "desktop",
    evidence: [...goal.evidence, "grounded_reference=prior_desktop_observation"],
  };
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
    /^(do that again|do it again|again|once more|repeat that|do the previous(?: one)?|previous one)$/i.test(
      text,
    )
  ) {
    if (state.lastAction) {
      evidence.push("again→replay_last_action");
      return {
        action: {
          ...cloneAction(state.lastAction),
          reply:
            state.lastAction.reply ||
            "Doing that again from what we just did.",
        },
        evidence,
      };
    }
    evidence.push("again→no_prior_action");
    return {
      action: {
        kind: "unknown",
        reply:
          "There’s nothing in this session to repeat yet — what should we do on the desktop?",
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
          "Let’s get you back through Continue — restore only runs after you approve a saved Moment.",
      },
      evidence,
    };
  }

  // Open / launch it — bind to last app or site when known
  if (/^(open|launch|show)\s+(it|that|this)$/i.test(text)) {
    if (state.lastUrl) {
      evidence.push("open_pronoun→url");
      return {
        action: {
          kind: "browserOpen",
          url: state.lastUrl,
          reply: "Opening that again from this session.",
        },
        evidence,
      };
    }
    if (state.lastAppQuery) {
      evidence.push(`open_pronoun→${state.lastAppQuery}`);
      return {
        action: {
          kind: "appOpen",
          query: state.lastAppQuery,
          reply: `Opening “${state.lastAppQuery}” again.`,
        },
        evidence,
      };
    }
    evidence.push("open_pronoun→clarify");
    return {
      action: {
        kind: "unknown",
        reply: "Open what — which app or site? Name it and I’ll continue from there.",
      },
      evidence,
    };
  }

  // Find / locate / bring it back — Context binds pronouns
  if (
    /^(find it|locate it|where (is|did) it(?: go)?|show it|bring it back|bring that back)$/i.test(
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
          reply: `Looking for “${ref}” from what we were just doing.`,
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
        reply: "Close what — which window or app? Name it, or open something first.",
      },
      evidence,
    };
  }

  // Minimize / maximize / restore that|it|this
  const winState = text.match(
    /^(minimize|minimise|maximize|maximise|restore)\s+(that|it|this)(?:\s+window)?$/i,
  );
  if (winState) {
    const verb = winState[1]!.toLowerCase();
    const ref = referentLabel();
    if (ref) {
      const kind =
        verb.startsWith("min")
          ? "winMinimize"
          : verb.startsWith("max")
            ? "winMaximize"
            : "winRestore";
      evidence.push(`${kind}_pronoun→${ref}`);
      return {
        action: {
          kind,
          query: ref,
          reply:
            kind === "winMinimize"
              ? `Minimizing “${ref}”.`
              : kind === "winMaximize"
                ? `Maximizing “${ref}”.`
                : `Restoring “${ref}”.`,
        },
        evidence,
      };
    }
    evidence.push("win_state_pronoun→clarify");
    return {
      action: {
        kind: "unknown",
        reply: "Which window should I change? Name it, or ask what’s open.",
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
        reply: "Which window? Name it, or open something first so I know what you mean.",
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
          reply: `Opening beside “${besideTarget}” from this session.`,
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
  goal?: GoalContract | null,
): void {
  state = {
    ...state,
    turn: state.turn + 1,
    lastUtterance: utterance.trim(),
    lastAction: cloneAction(action),
    currentGoal: goal ?? state.currentGoal,
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

  if (
    action.kind === "compoundOpen" ||
    action.kind === "prepareCodingWorkspace"
  ) {
    for (const t of action.targets) {
      if (t.kind === "app") {
        state.lastAppQuery = t.query;
        state.appsLaunched = pushUnique(state.appsLaunched, t.query);
        state.lastWindowQuery = state.lastWindowQuery ?? t.query;
        state.windowsReferenced = pushUnique(state.windowsReferenced, t.query);
      } else {
        state.lastUrl = t.url;
      }
    }
  }

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
