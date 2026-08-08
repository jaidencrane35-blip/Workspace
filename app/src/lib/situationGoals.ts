/**
 * Situation Goals (P16.35 / P16.39) — deterministic Intent Layer.
 *
 * Owns high-level desktop *situations* and *operator activity states*
 * (resume work, start day, work modes, end/break, screenshots, discovery)
 * without growing Intent Grammar verb tables or entity alias lists.
 *
 * Does NOT own: session pronouns (Context), single-turn underspecify (Goal Resolution),
 * Kernel composition, or inventing multi-app layouts from a work-mode label.
 *
 * Truthful only: never invents multi-app layouts or session memory.
 */

import {
  generateCapabilityDiscovery,
  generateRecoveryGuidance,
  resolveDiscoveryScope,
} from "./capabilityRegistry";
import type { IntentAction } from "./intentBridge";

function norm(raw: string): string {
  return raw
    .trim()
    .toLowerCase()
    .replace(/[.!?]+$/g, "")
    .replace(/,/g, " ")
    .replace(/\s+/g, " ")
    .replace(/^(please|can you|could you|would you|i think)\s+/i, "")
    .replace(/^let'?s\s+/i, "")
    .replace(/\s+(please|for me|now|thanks|thank you)$/i, "")
    .trim();
}

function continueForActivity(activityLabel: string): IntentAction {
  return {
    kind: "navigate",
    view: "resume",
    reply: `Opening Continue for your ${activityLabel} session — restore a saved Moment when you’re ready.`,
  };
}

function continueForSession(reason: string): IntentAction {
  return {
    kind: "navigate",
    view: "resume",
    reply: `${reason} Restore only runs after you approve a saved Moment.`,
  };
}

function endSessionClarify(): IntentAction {
  const recovery = generateRecoveryGuidance("close window");
  return {
    kind: "unknown",
    reply:
      "Stepping away — should I close a window, minimize something, or open Continue so you can leave this for later?",
    suggestion: recovery.suggestion,
  };
}

/**
 * Resolve situation-level / operator-activity goals.
 * Returns null when entity/command cognition should continue.
 */
export function resolveSituationGoal(raw: string): IntentAction | null {
  const text = norm(raw);
  if (!text) {
    return null;
  }

  // Capability discovery — broader than command catalogues (Owner phrasing).
  if (
    /^(show|tell)\s+me\s+everything\s+you\s+(know\s+how\s+to\s+control|can\s+(control|do))$/i.test(
      text,
    ) ||
    /^everything\s+you\s+(know\s+how\s+to\s+control|can\s+(control|do))$/i.test(
      text,
    )
  ) {
    const discovery = generateCapabilityDiscovery(resolveDiscoveryScope(text));
    return {
      kind: "capabilityExplain",
      reply: discovery.reply,
      suggestion: discovery.suggestion,
    };
  }

  // Screenshots with optional time — open Pictures; never fake date filters.
  if (
    /\b(screenshots?|screen\s*shots?)\b/.test(text) &&
    /\b(find|looking\s+for|show|open|locate|where)\b/.test(text)
  ) {
    return {
      kind: "appLaunch",
      query: "shell:My Pictures",
      reply:
        "Opening Pictures — that’s where screenshots usually land. I can’t filter to yesterday from Conversation yet.",
    };
  }

  // Operator work modes — Continue for that activity; never invent app sets.
  const workMode = text.match(
    /^i'?m\s+(coding|debugging|researching|reviewing)(?:\s+code)?$/i,
  );
  if (workMode?.[1]) {
    const label = workMode[1].toLowerCase();
    return continueForActivity(label);
  }
  if (/^i'?m\s+writing\s+documentation$/i.test(text)) {
    return continueForActivity("documentation");
  }
  if (/^i'?m\s+doing\s+(research|code\s+review)$/i.test(text)) {
    return continueForActivity(
      /review/i.test(text) ? "review" : "research",
    );
  }

  // Session resume / start — operator wants the desk back, not a command.
  if (
    /^(i\s+need\s+to\s+)?get\s+back\s+(into|to)\s+work$/i.test(text) ||
    /^set\s+me\s+up$/i.test(text) ||
    /^i'?m\s+starting\s+my\s+day$/i.test(text) ||
    /^get\s+started$/i.test(text) ||
    /^take\s+me\s+back$/i.test(text) ||
    /^back\s+to\s+work$/i.test(text)
  ) {
    return continueForSession(
      "Let’s get you back into work through Continue.",
    );
  }

  // Resume / continue / environment setup — Continue Moments (Owner approves).
  if (
    /\b(continue where i left off|where i left off|pick up where i left off)\b/.test(
      text,
    ) ||
    /\b(development|coding|dev|work)\s+(setup|environment|space)\b/.test(text) ||
    /\b(my\s+)?(coding|development|dev)\s+environment\b/.test(text) ||
    /\bi\s+was\s+working\s+on\s+something\b/.test(text) ||
    /\bi\s+need\s+everything\s+ready\b/.test(text) ||
    /\beverything\s+ready\b/.test(text)
  ) {
    return continueForSession(
      "Let’s get your development setup back through Continue.",
    );
  }

  // End / break / done — ask for a concrete desktop action (never invent close-all).
  if (
    /^(i'?m\s+done(\s+with\s+this)?|done with this|i'?m\s+finished(\s+with\s+this)?)$/i.test(
      text,
    ) ||
    /^i'?m\s+taking\s+a\s+break$/i.test(text) ||
    /^taking\s+a\s+break$/i.test(text) ||
    /^close\s+everything$|^put\s+everything\s+away$/i.test(text)
  ) {
    return endSessionClarify();
  }

  return null;
}

/** Operator-activity family for evidence / batteries (Situation Goals ownership). */
export type OperatorActivityFamily =
  | "session_resume"
  | "work_mode"
  | "session_end"
  | "screenshots"
  | "discovery"
  | "none";

export function classifyOperatorActivity(raw: string): OperatorActivityFamily {
  const text = norm(raw);
  if (!text) return "none";
  if (
    /^(show|tell)\s+me\s+everything\s+you\s+(know\s+how\s+to\s+control|can\s+(control|do))$/i.test(
      text,
    ) ||
    /^everything\s+you\s+(know\s+how\s+to\s+control|can\s+(control|do))$/i.test(
      text,
    )
  ) {
    return "discovery";
  }
  if (
    /\b(screenshots?|screen\s*shots?)\b/.test(text) &&
    /\b(find|looking\s+for|show|open|locate|where)\b/.test(text)
  ) {
    return "screenshots";
  }
  if (
    /^i'?m\s+(coding|debugging|researching|reviewing)/i.test(text) ||
    /^i'?m\s+writing\s+documentation$/i.test(text) ||
    /^i'?m\s+doing\s+(research|code\s+review)$/i.test(text)
  ) {
    return "work_mode";
  }
  if (
    /^(i\s+need\s+to\s+)?get\s+back\s+(into|to)\s+work$/i.test(text) ||
    /^set\s+me\s+up$/i.test(text) ||
    /^i'?m\s+starting\s+my\s+day$/i.test(text) ||
    /^get\s+started$/i.test(text) ||
    /^take\s+me\s+back$/i.test(text) ||
    /^back\s+to\s+work$/i.test(text) ||
    /\b(continue where i left off|where i left off|pick up where i left off)\b/.test(
      text,
    ) ||
    /\b(development|coding|dev|work)\s+(setup|environment|space)\b/.test(text) ||
    /\bi\s+was\s+working\s+on\s+something\b/.test(text)
  ) {
    return "session_resume";
  }
  if (
    /^(i'?m\s+done|i'?m\s+finished|i'?m\s+taking\s+a\s+break|taking\s+a\s+break|close\s+everything)/i.test(
      text,
    )
  ) {
    return "session_end";
  }
  return "none";
}
