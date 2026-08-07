/**
 * Intent pipeline evidence (P16.32).
 *
 * Every utterance resolution must be attributable to stages.
 * Conversation never bypasses Intent → CapabilityIntent for desktop effects.
 */

import { parseDesktopIntent } from "./intentGrammar";
import { resolveIntent, type IntentAction } from "./intentBridge";
import { resolveDesktopEntity, resolveSemanticIntent } from "./semanticIntentEngine";
import { isCapabilityDiscoveryUtterance } from "./capabilityRegistry";

export interface PipelineStageEvidence {
  stage:
    | "normalize"
    | "capability_discovery"
    | "intent_grammar"
    | "semantic_engine"
    | "resolve_intent"
    | "executable_guard";
  hit: boolean;
  detail: string;
}

export interface IntentPipelineEvidence {
  utterance: string;
  stages: PipelineStageEvidence[];
  action: IntentAction;
  /** True when Semantic Engine produced the final action (or discovery). */
  semanticOwned: boolean;
  /** True when an unknown open/launch was refused instead of inventing .exe. */
  refusedExecutableGuess: boolean;
}

/**
 * Resolve an utterance and record objective stage evidence for Product Proof.
 */
export function resolveIntentWithEvidence(raw: string): IntentPipelineEvidence {
  const utterance = raw.trim();
  const stages: PipelineStageEvidence[] = [];

  stages.push({
    stage: "normalize",
    hit: Boolean(utterance),
    detail: utterance ? `len=${utterance.length}` : "empty",
  });

  const discovery = isCapabilityDiscoveryUtterance(utterance);
  stages.push({
    stage: "capability_discovery",
    hit: discovery,
    detail: discovery ? "registry-generated discovery" : "not discovery",
  });

  const grammar = parseDesktopIntent(utterance);
  stages.push({
    stage: "intent_grammar",
    hit: Boolean(grammar),
    detail: grammar
      ? `${grammar.action}/${grammar.modifier}/${grammar.context}`
      : "no grammar match",
  });

  const semantic = resolveSemanticIntent(utterance);
  stages.push({
    stage: "semantic_engine",
    hit: Boolean(semantic),
    detail: semantic ? `kind=${semantic.kind}` : "semantic deferred",
  });

  const action = resolveIntent(utterance);
  stages.push({
    stage: "resolve_intent",
    hit: true,
    detail: `kind=${action.kind}`,
  });

  const looksLikeOpen =
    /^(open|launch|start)\b/i.test(utterance) ||
    /^(take me to|show me|go to|i want)\b/i.test(utterance);
  const entity = grammar?.target
    ? resolveDesktopEntity(grammar.target)
    : null;
  const refused =
    looksLikeOpen &&
    action.kind === "unknown" &&
    !entity &&
    /won’t invent a program name/i.test(action.reply);

  stages.push({
    stage: "executable_guard",
    hit: refused || Boolean(entity) || !looksLikeOpen || action.kind !== "unknown",
    detail: refused
      ? "unknown open refused (no .exe invent)"
      : entity
        ? `entity=${entity.kind}:${entity.label}`
        : "n/a or resolved",
  });

  const semanticOwned = Boolean(
    semantic &&
      semantic.kind === action.kind &&
      JSON.stringify(semantic) === JSON.stringify(action),
  );

  return {
    utterance,
    stages,
    action,
    semanticOwned: discovery || semanticOwned || Boolean(semantic && action.kind === semantic.kind),
    refusedExecutableGuess: refused,
  };
}

/** True when launch/open query looks like an invented executable name. */
export function isInventedExecutableQuery(query: string): boolean {
  const q = query.trim().toLowerCase();
  if (!q.endsWith(".exe")) {
    return false;
  }
  // Spaced or multi-word stems before .exe are always invented garbage.
  const stem = q.slice(0, -4);
  if (stem.includes(" ") || stem.includes(" and ") || stem.length > 40) {
    return true;
  }
  return false;
}
