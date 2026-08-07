/**
 * Situation Goals (P16.35) — deterministic Intent Layer.
 *
 * Reasons about high-level desktop *situations* (resume work, end session,
 * find screenshots, discover capabilities) without growing Intent Grammar
 * verb tables or entity alias lists.
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
    .replace(/^(please|can you|could you|would you)\s+/i, "")
    .replace(/\s+(please|for me|now|thanks|thank you)$/i, "")
    .trim();
}

/**
 * Resolve situation-level goals. Returns null when entity/command cognition
 * should continue.
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
    return {
      kind: "navigate",
      view: "resume",
      reply:
        "Opening Continue. I don’t invent a multi-app layout from a vague setup — restore only runs after you approve a saved Moment.",
    };
  }

  // End / done — ask for a concrete desktop action (never invent close-all).
  if (
    /^(i'?m\s+done(\s+with\s+this)?|done with this|i'?m\s+finished(\s+with\s+this)?)$/i.test(
      text,
    ) ||
    /^close\s+everything$|^put\s+everything\s+away$/i.test(text)
  ) {
    const recovery = generateRecoveryGuidance("close window");
    return {
      kind: "unknown",
      reply:
        "What should I put away — close a window, minimize something, or open Continue to leave this for later? I won’t invent a close-everything action.",
      suggestion: recovery.suggestion,
    };
  }

  return null;
}
