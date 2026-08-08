/**
 * P23.S1 — Goal Contract (Outcome-First Comprehension). Intent Layer, meaning only.
 *
 * Answers one question: what would have to be true for the Owner to consider
 * this request fulfilled? It never answers "which capability", "which provider",
 * or "in what order" — those are Kernel Operator authority.
 *
 * Deterministic (no probabilistic AI). Signals are extracted from the whole
 * utterance first, and the outcome is then derived from the accumulated
 * signals. No branch returns an action, and nothing here produces an Effect.
 *
 * Constitutional position: Understanding owns Meaning and MUST NOT produce
 * Effect (Spec v2 forbidden transition: Meaning → Effect).
 *
 * Distinct from Goal Resolution (P16.36), which runs *after* an IntentAction
 * has been chosen, derives its outcome from that action, and carries an
 * ExecutionPlan. This runs before, and carries no plan.
 */

import { resolveDesktopEntity } from "./semanticIntentEngine";
import { getWorkspaceContext } from "./workspaceContext";

/** How the Owner is communicating — derived, never a routing decision. */
export type GoalMode =
  | "conversation"
  | "information"
  | "computation"
  | "observation"
  | "show"
  | "action"
  | "hybrid"
  | "meta"
  | "undetermined";

/** What the Owner wants to be true when the interaction ends. Primary key. */
export type GoalOutcome =
  | "SOCIAL"
  | "KNOW"
  | "COMPUTE"
  | "PERCEIVE_MACHINE"
  | "PERCEIVE_WORLD"
  | "REACH_STATE"
  | "META"
  | "UNDETERMINED";

/** Subject matter of the request (Owner-facing domain, never a provider). */
export type GoalDomain =
  | "conversation"
  | "knowledge"
  | "time"
  | "desktop"
  | "application"
  | "web"
  | "content"
  | "workspace"
  | "unknown";

export type GoalTargetRole =
  | "subject"
  | "place"
  | "application"
  | "site"
  | "container"
  | "state";

/** A semantic thing the Owner named. Never a provider or capability. */
export interface GoalTarget {
  /** The Owner's own words for this target. */
  phrase: string;
  role: GoalTargetRole;
  /** Canonical display label when Workspace recognises the entity. */
  canonical: string | null;
  recognized: boolean;
}

/** A referring expression and whether existing context could bind it. */
export interface GoalReference {
  phrase: string;
  resolved: boolean;
  referent: string | null;
}

/**
 * The comprehension result. Meaning only.
 *
 * Deliberately absent: capability ids, provider names, domain/operation pairs,
 * execution steps, and anything resembling a plan.
 */
export interface GoalContract {
  utterance: string;
  normalized: string;
  mode: GoalMode;
  outcome: GoalOutcome;
  domain: GoalDomain;
  /** Primary thing the request is about, in the Owner's words. */
  subject: string | null;
  targets: GoalTarget[];
  references: GoalReference[];
  /** What the Owner expects to receive back, when they expect something. */
  requestedResult: string | null;
  compound: boolean;
  /** The Owner's own clauses — linguistic decomposition, never execution steps. */
  clauses: string[];
  clarificationNeeded: boolean;
  uncertainties: string[];
  evidence: string[];
}

/**
 * Verbs that request a change to machine state. Matched only in clause-initial
 * position, so "what windows do I have open" is not read as a request to open.
 */
const EFFECT_VERBS =
  "open|launch|start|run|close|quit|play|pause|focus|switch|bring|move|snap|maximi[sz]e|minimi[sz]e|restore|centre|center|resize|type|click|press|copy|paste|prepare|put|make|set|go|navigate|arrange|place|search|capture|save|take";

/** Verbs that request knowledge without necessarily changing state. */
const OBSERVATION_VERBS = "find|locate|list|enumerate|count|check|look|read|see";

/** "show" is deliberately separate: it can be perceptual or observational. */
const SHOW_VERBS = "show|display";

const CLAUSE_LEAD =
  "(?:please\\s+)?(?:can you\\s+|could you\\s+|would you\\s+|i(?:'| a)?m going to\\s+)?";

const BARE_VERB = new RegExp(
  `^(?:${EFFECT_VERBS}|${OBSERVATION_VERBS}|${SHOW_VERBS})$`,
);

const EFFECT_CLAUSE = new RegExp(`^${CLAUSE_LEAD}(?:${EFFECT_VERBS})\\b`);
const OBSERVATION_CLAUSE = new RegExp(`^${CLAUSE_LEAD}(?:${OBSERVATION_VERBS})\\b`);
const SHOW_CLAUSE = new RegExp(`^${CLAUSE_LEAD}(?:${SHOW_VERBS})\\b`);

const MACHINE_OBJECTS =
  /\b(window|windows|screen|screens|monitor|monitors|clipboard|notification|notifications|screenshot|screenshots|desktop|taskbar|app|apps|application|applications|tab|tabs)\b/;

const CONTENT_OBJECTS =
  /\b(file|files|folder|folders|directory|directories|document|documents|picture|pictures|image|images|photo|photos|video|videos|download|downloads)\b/;

const SPATIAL_MARKERS = /\b(where|map|location|located|whereabouts)\b/;

const STATE_MARKERS: Array<{ test: RegExp; label: string }> = [
  { test: /\bfull ?screen(ed)?\b/, label: "fullscreen" },
  {
    test: /\b(volume|audio|sound)\b[^.]*\b(on|up|enabled|unmuted)\b/,
    label: "audio enabled",
  },
  { test: /\b(play it|playing|start playing)\b/, label: "playing" },
  { test: /\bmaximi[sz]ed\b/, label: "maximized" },
  { test: /\bminimi[sz]ed\b/, label: "minimized" },
  { test: /\bbeside\b/, label: "side by side" },
];

/**
 * Referring expressions. Each pattern requires a genuine referring context so
 * that expletive pronouns ("what time is it") are not mistaken for references.
 */
const REFERENCE_PATTERNS: RegExp[] = [
  /\bthe one i just opened\b/,
  /\bthe (?:latest|last) one\b/,
  /\b(?:that|this) (?:folder|window|file|app|application|one|tab|site|page)\b/,
  new RegExp(`\\b(?:${EFFECT_VERBS}|${OBSERVATION_VERBS}|${SHOW_VERBS})\\s+it\\b`),
  new RegExp(`\\b(?:${EFFECT_VERBS}|${OBSERVATION_VERBS}|${SHOW_VERBS})\\s+that\\b`),
  /\b(?:in|on|with|about|from|into)\s+it\b/,
  /\b(?:go|take me|head)\s+there\b/,
  /\bdo (?:it|that) again\b/,
  /^again$/,
];

const SOCIAL_PATTERNS: RegExp[] = [
  /^(hi|hello|hey|yo|howdy)\b/,
  /\bhow are you\b/,
  /\bhow'?s it going\b/,
  /\bhow are things\b/,
  /\bhow'?s (?:my|your) day\b/,
  /^(thanks|thank you|cheers|ty)\b/,
  /^(bye|goodbye|good night)\b/,
  /^good (morning|afternoon|evening)\b/,
];

const META_PATTERNS: RegExp[] = [
  /^(stop|cancel|wait|never ?mind|forget it)\b/,
  /\bwhat are you doing\b/,
  /\bwhat can you do\b/,
];

function normalize(raw: string): string {
  return raw
    .trim()
    .toLowerCase()
    .replace(/[.!?]+$/g, "")
    .replace(/\s+/g, " ")
    .trim();
}

function any(patterns: RegExp[], text: string): boolean {
  return patterns.some((p) => p.test(text));
}

function isVerbClause(clause: string): boolean {
  return (
    EFFECT_CLAUSE.test(clause) ||
    OBSERVATION_CLAUSE.test(clause) ||
    SHOW_CLAUSE.test(clause)
  );
}

/** Split into the Owner's own clauses, keeping only those that request something. */
function verbClauses(text: string): string[] {
  return text
    .split(/\s*,\s*|\s+and then\s+|\s+then\s+|\s+and\s+/)
    .map((part) => part.trim().replace(/^(?:and|then)\s+/, "").trim())
    .filter((part) => part.length > 0 && isVerbClause(part));
}

/**
 * Remove referring expressions from a clause so the remainder is only the
 * things the Owner actually named. "open it in Paint" keeps Paint; "open that
 * folder" keeps nothing, because the Owner named nothing.
 */
function stripReferences(clause: string): string {
  let remainder = clause;
  for (const pattern of REFERENCE_PATTERNS) {
    if (!pattern.test(remainder)) continue;
    remainder = remainder.replace(pattern, " ").replace(/\s+/g, " ").trim();
  }
  return remainder;
}

/** Strip the leading verb and filler from a clause, leaving the named thing. */
function clauseObject(clause: string): string {
  let object = clause
    .replace(
      new RegExp(
        `^${CLAUSE_LEAD}(?:go\\s+to|navigate\\s+to|switch\\s+to|bring\\s+up|take\\s+me\\s+to|search\\s+for)\\s+`,
      ),
      "",
    )
    .replace(
      new RegExp(
        `^${CLAUSE_LEAD}(?:${EFFECT_VERBS}|${OBSERVATION_VERBS}|${SHOW_VERBS})\\s+`,
      ),
      "",
    );
  let previous = "";
  while (previous !== object) {
    previous = object;
    object = object
      .replace(/^(?:me|up|the|a|an|my|to|in|into|with|on|at|it|that|this)\s+/, "")
      .trim();
  }
  object = object.replace(/\s+(?:for me|please|now)$/, "").trim();
  // A clause reduced to its bare verb named nothing.
  if (BARE_VERB.test(object)) return "";
  return object;
}

function entityRole(kind: string, fallback: GoalTargetRole): GoalTargetRole {
  if (kind === "site") return "site";
  if (kind === "shell") return "container";
  if (kind === "application" || kind === "browser" || kind === "protocol") {
    return "application";
  }
  return fallback;
}

function pushTarget(
  targets: GoalTarget[],
  phrase: string,
  role: GoalTargetRole,
): void {
  const trimmed = phrase.trim();
  if (!trimmed) return;
  if (targets.some((t) => t.phrase.toLowerCase() === trimmed.toLowerCase())) {
    return;
  }
  const entity = role === "state" ? null : resolveDesktopEntity(trimmed);
  targets.push({
    phrase: trimmed,
    role: entity ? entityRole(entity.kind, role) : role,
    // Display label only — never the entity's open value, URL, or query.
    canonical: entity ? entity.label : null,
    recognized: Boolean(entity),
  });
}

/** Place named after "where … is" or a trailing "in <place>". */
function extractPlace(text: string): string | null {
  const where =
    text.match(/\bwhere\s+(?:is\s+)?(.+?)\s+(?:is|are|located)\b/) ??
    text.match(/\bwhere\s+is\s+(.+)$/);
  if (where?.[1]) return where[1].trim();
  const inPlace = text.match(/\bin\s+([a-z][a-z\s,'-]*[a-z])$/);
  if (inPlace?.[1]) return inPlace[1].trim();
  return null;
}

/**
 * Ways an Owner asks for the clock. Phrasings, not a phrase list: each pattern
 * covers a shape ("what … time", "tell me the time", "the time in …") so
 * unseen wordings of the same request still comprehend as a clock question.
 */
const TIME_REQUEST_SHAPES = [
  /\bwhat time\b/,
  /\bwhat(?:'s| is)?\s+(?:the\s+)?(?:current\s+)?time\b/,
  /\b(?:tell me|know)\s+(?:the\s+)?(?:current\s+)?time\b/,
  /\bcurrent time\b/,
  /\btime right now\b/,
  /\bthe time (?:in|there|now)\b/,
];

const DATE_REQUEST_SHAPES = [
  /\bwhat date\b/,
  /\bwhat(?:'s| is)?\s+(?:the\s+)?(?:today'?s\s+)?date\b/,
  /\b(?:tell me|know)\s+(?:the\s+)?(?:today'?s\s+)?date\b/,
  /\bthe date (?:in|there|today)\b/,
  /\bwhat day is it\b/,
];

function requestedResultFor(text: string): string | null {
  if (/\bhow many\b/.test(text) || /\bcount\b/.test(text)) return "count";
  if (DATE_REQUEST_SHAPES.some((shape) => shape.test(text))) {
    return "current date";
  }
  if (TIME_REQUEST_SHAPES.some((shape) => shape.test(text))) {
    return "current time";
  }
  if (SPATIAL_MARKERS.test(text)) return "location";
  if (/\btell me what\b/.test(text) || /\bwhat does it (show|say)\b/.test(text)) {
    return "description";
  }
  if (/\bwhat\s+(windows|apps|applications|tabs)\b/.test(text)) return "list";
  return null;
}

/** Digits combined with an arithmetic or rate marker. */
function looksComputational(text: string): boolean {
  if (!/\d/.test(text)) return false;
  return (
    /[+\-*/×÷%]/.test(text) ||
    /\bper\b/.test(text) ||
    /\bhow much is\b/.test(text) ||
    /\bpercent\b/.test(text) ||
    /\b(convert|calculate|compute)\b/.test(text)
  );
}

/**
 * Bind referring expressions. A reference is resolved when session context
 * holds a referent, or when the Owner named its antecedent earlier in the same
 * utterance ("find the latest screenshot and open **it** in Paint").
 */
function extractReferences(text: string, targets: GoalTarget[]): GoalReference[] {
  const context = getWorkspaceContext();
  const sessionReferent =
    context.lastWindowQuery ?? context.lastAppQuery ?? context.lastUrl ?? null;
  const found: GoalReference[] = [];

  for (const pattern of REFERENCE_PATTERNS) {
    const match = text.match(pattern);
    if (!match) continue;
    const phrase = match[0];
    if (found.some((r) => r.phrase === phrase)) continue;

    const at = text.indexOf(phrase);
    const antecedent = targets.find((target) => {
      if (target.role === "state") return false;
      const position = text.indexOf(target.phrase);
      return position >= 0 && position < at;
    });

    found.push({
      phrase,
      resolved: Boolean(antecedent ?? sessionReferent),
      referent: antecedent?.canonical ?? antecedent?.phrase ?? sessionReferent,
    });
  }
  return found;
}

function emptyContract(utterance: string): GoalContract {
  return {
    utterance: utterance.trim(),
    normalized: "",
    mode: "undetermined",
    outcome: "UNDETERMINED",
    domain: "unknown",
    subject: null,
    targets: [],
    references: [],
    requestedResult: null,
    compound: false,
    clauses: [],
    clarificationNeeded: false,
    uncertainties: [],
    evidence: ["empty_utterance"],
  };
}

/**
 * Comprehend an utterance into a Goal Contract.
 *
 * Pure with respect to the desktop: reads session context, touches no provider,
 * performs no IPC, and returns meaning only.
 */
export function comprehend(utterance: string): GoalContract {
  const text = normalize(utterance);
  if (!text) {
    return emptyContract(utterance);
  }

  const evidence: string[] = [];
  const targets: GoalTarget[] = [];
  const uncertainties: string[] = [];

  // --- Signals. Everything is gathered before anything is decided.
  const clauses = verbClauses(text);
  const compound = clauses.length >= 2;
  const hasEffectVerb = clauses.some((c) => EFFECT_CLAUSE.test(c));
  const hasObservationVerb = clauses.some((c) => OBSERVATION_CLAUSE.test(c));
  const hasShowVerb = clauses.some((c) => SHOW_CLAUSE.test(c));
  const social = any(SOCIAL_PATTERNS, text);
  const meta = any(META_PATTERNS, text);
  const interrogative =
    /^(what|where|when|why|who|how|which|is|are|can|do|does|did)\b/.test(text) ||
    // "I want to know X" and "tell me X" seek knowledge without a question mark.
    /^(i (?:want|need|'?d like) to know|tell me|do you know|any idea)\b/.test(text) ||
    /\?$/.test(utterance.trim());
  const machineObject = MACHINE_OBJECTS.test(text);
  const contentObject = CONTENT_OBJECTS.test(text);
  const spatial = SPATIAL_MARKERS.test(text);
  const computational = looksComputational(text);
  const requestedResult = requestedResultFor(text);

  evidence.push(
    `clauses=${clauses.length}`,
    `effect_verb=${hasEffectVerb}`,
    `observation_verb=${hasObservationVerb}`,
    `show_verb=${hasShowVerb}`,
    `interrogative=${interrogative}`,
    `machine_object=${machineObject}`,
    `content_object=${contentObject}`,
    `spatial=${spatial}`,
  );

  // --- Outcome derivation from the accumulated signals.
  let outcome: GoalOutcome = "UNDETERMINED";
  let mode: GoalMode = "undetermined";

  if (meta) {
    outcome = "META";
    mode = "meta";
    evidence.push("meta_marker");
  } else if (social && !hasEffectVerb && !machineObject && !contentObject) {
    outcome = "SOCIAL";
    mode = "conversation";
    evidence.push("social_only");
  } else if (computational) {
    outcome = "COMPUTE";
    // "What's 15% of 80 and open Notepad" wants the number *and* the app.
    mode = hasEffectVerb ? "hybrid" : "computation";
    evidence.push(`computational_signal:${mode}`);
  } else if (
    (requestedResult === "current time" || requestedResult === "current date") &&
    !hasEffectVerb
  ) {
    outcome = "KNOW";
    mode = "information";
    evidence.push("clock_question");
  } else if (hasShowVerb && spatial && !machineObject) {
    // Perceptual and spatial: the Owner wants to see it, not be told about it.
    outcome = "PERCEIVE_WORLD";
    mode = "show";
    evidence.push("show_spatial");
  } else if (
    !hasEffectVerb &&
    (machineObject || contentObject) &&
    (interrogative || hasObservationVerb || hasShowVerb)
  ) {
    outcome = "PERCEIVE_MACHINE";
    mode = "observation";
    evidence.push("machine_observation");
  } else if (hasEffectVerb && requestedResult) {
    // Effects are requested, but the goal is the information at the end of them.
    outcome = requestedResult === "location" ? "PERCEIVE_WORLD" : "KNOW";
    mode = "hybrid";
    evidence.push(`effects_then_result:${requestedResult}`);
  } else if (hasEffectVerb && (hasObservationVerb || hasShowVerb)) {
    outcome = "REACH_STATE";
    mode = "hybrid";
    evidence.push("observe_then_act");
  } else if (hasEffectVerb) {
    outcome = "REACH_STATE";
    mode = "action";
    evidence.push("state_goal");
  } else if (hasObservationVerb || hasShowVerb) {
    outcome = "PERCEIVE_MACHINE";
    mode = "observation";
    evidence.push("bare_observation");
  } else if (interrogative) {
    outcome = "KNOW";
    mode = "information";
    evidence.push("bare_question");
  }

  // --- Targets: the Owner's named things, in the Owner's words.
  for (const marker of STATE_MARKERS) {
    if (marker.test.test(text)) {
      pushTarget(targets, marker.label, "state");
    }
  }
  const place = extractPlace(text);
  if (place && (outcome === "KNOW" || outcome === "PERCEIVE_WORLD")) {
    pushTarget(targets, place, "place");
  }
  if (mode === "action" || mode === "hybrid" || mode === "observation") {
    for (const clause of clauses) {
      const object = clauseObject(stripReferences(clause));
      if (!object) continue;
      pushTarget(targets, object, "subject");
    }
  }

  const references = extractReferences(text, targets);

  // --- Domain.
  let domain: GoalDomain = "unknown";
  if (mode === "conversation") domain = "conversation";
  else if (mode === "meta") domain = "workspace";
  else if (
    requestedResult === "current time" ||
    requestedResult === "current date"
  ) {
    domain = "time";
  } else if (contentObject) domain = "content";
  else if (targets.some((t) => t.role === "site")) domain = "web";
  else if (targets.some((t) => t.role === "application")) domain = "application";
  else if (machineObject) domain = "desktop";
  else if (
    outcome === "KNOW" ||
    outcome === "PERCEIVE_WORLD" ||
    outcome === "COMPUTE"
  ) {
    domain = "knowledge";
  }

  // --- Subject.
  const placeTarget = targets.find((t) => t.role === "place");
  const subject =
    (outcome === "KNOW" || outcome === "PERCEIVE_WORLD") && placeTarget
      ? placeTarget.phrase
      : (targets.find((t) => t.role !== "state")?.phrase ??
        placeTarget?.phrase ??
        null);

  // --- Uncertainty. Never invent a referent or a target.
  for (const reference of references) {
    if (!reference.resolved) {
      uncertainties.push(`unresolved reference: “${reference.phrase}”`);
    }
  }
  for (const target of targets) {
    if (!target.recognized && target.role === "subject") {
      uncertainties.push(`unrecognised target: “${target.phrase}”`);
    }
  }
  if (outcome === "UNDETERMINED") {
    uncertainties.push("desired outcome could not be determined");
  }

  const clarificationNeeded =
    references.some((r) => !r.resolved) || outcome === "UNDETERMINED";

  return {
    utterance: utterance.trim(),
    normalized: text,
    mode,
    outcome,
    domain,
    subject,
    targets,
    references,
    requestedResult,
    compound,
    clauses,
    clarificationNeeded,
    uncertainties,
    evidence,
  };
}
