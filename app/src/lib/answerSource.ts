/**
 * P23.S3 — Answer Source Ladder. Intent Layer.
 *
 * Answers one question: when Workspace already knows what the Owner wants,
 * which trusted source can supply the answer?
 *
 * It does **not** answer "which desktop capability should execute". No source
 * may name a capability, a provider, or anything executable — the ladder deals
 * in meaning and text only, and the Kernel Operator remains the sole execution
 * authority.
 *
 * Sources are consulted in rung order and the first truthful answer wins. A
 * source that cannot answer honestly returns null; it never guesses, and it
 * never falls sideways into doing something on the desktop instead.
 */

import type { GoalContract } from "./goalContract";
import {
  hasPositiveOutcomeEvidence,
  isAnswerOnlyGoal,
} from "./substitutionProhibition";
import { temporalAnswerSource } from "./temporalAnswerSource";
import {
  activeWindowAnswerSource,
  openWindowsAnswerSource,
} from "./observationAnswerSource";

/**
 * Precedence for obtaining an answer. Lower rungs are preferred because they
 * are cheaper, offline, and verifiable.
 *
 * Only `deterministic-local` is implemented in this slice. The remaining rungs
 * are named so later sources slot in without reshaping the Intelligence Layer:
 *
 * - `workspace-system` — authoritative Workspace or OS state (settings, health)
 * - `capability-observation` — an existing observation capability's result
 * - `authorized-external` — an authorized external information mechanism. This
 *   remains C-REA-003's existing decision, reached by falling through this
 *   ladder rather than by a source implemented here.
 * - `honest-limitation` — P23.S2 already supplies this when nothing can answer
 */
export type AnswerRung =
  | "deterministic-local"
  | "workspace-system"
  | "capability-observation"
  | "authorized-external"
  | "honest-limitation";

export const ANSWER_RUNGS: readonly AnswerRung[] = [
  "deterministic-local",
  "workspace-system",
  "capability-observation",
  "authorized-external",
  "honest-limitation",
] as const;

export interface Answer {
  /** Conversational text for the Owner. Never internal terminology. */
  text: string;
  /** Which source produced it — evidence for audit, not a routing decision. */
  sourceId: string;
  rung: AnswerRung;
}

export interface AnswerSource {
  id: string;
  rung: AnswerRung;
  /** Cheap check: is this semantic request within the source's contract? */
  covers(goal: GoalContract): boolean;
  /** The answer, or null when this source cannot supply one truthfully. */
  answer(goal: GoalContract): Answer | null;
}

/**
 * A source at the `capability-observation` rung cannot answer on its own: the
 * data belongs to the desktop, and only the Kernel may observe it. Such a
 * source therefore splits in two — it declares *what* it needs observed, and it
 * composes an answer once an authorized observation comes back. It never
 * obtains the observation itself, so the authority boundary is structural
 * rather than a matter of discipline.
 *
 * `need` is a semantic description of the observation, never a capability,
 * provider, or operation identifier. Translating a need into the existing
 * authorized request is the Intent Layer's job, exactly as it is for an
 * utterance.
 *
 * P23.S5 adds the second need. Two needs are what distinguish a vocabulary of
 * information the Owner can ask for from a registry of things Workspace can
 * run: the members describe what is worth knowing, and no member can be
 * satisfied except through a request the Intent Layer already knew how to make.
 * Each need must be covered by exactly one source, so a request has one
 * meaning rather than a set of candidates to choose between.
 */
export type ObservationNeed = "open-windows" | "active-window";

/**
 * Structural view of an authorized observation. Not a transport type.
 *
 * `focused` is carried because the Kernel reports it: which window is active is
 * observed fact, not something composition may infer from ordering.
 */
export interface ObservationResult {
  ok: boolean;
  items?: Array<{ title: string; focused?: boolean }> | null;
}

export interface ObservationAnswerSource {
  id: string;
  rung: "capability-observation";
  need: ObservationNeed;
  covers(goal: GoalContract): boolean;
  compose(goal: GoalContract, observation: ObservationResult): Answer | null;
}

/** Registered sources. Order within a rung is declaration order. */
const SOURCES: readonly AnswerSource[] = [temporalAnswerSource];

const OBSERVATION_SOURCES: readonly ObservationAnswerSource[] = [
  openWindowsAnswerSource,
  activeWindowAnswerSource,
];

/** Walk the ladder and return the first truthful answer. */
export function resolveAnswer(goal: GoalContract): Answer | null {
  for (const rung of ANSWER_RUNGS) {
    for (const source of SOURCES) {
      if (source.rung !== rung || !source.covers(goal)) continue;
      const answer = source.answer(goal);
      if (answer) return answer;
    }
  }
  return null;
}

/**
 * Answer a goal that is fulfilled by knowledge rather than machine change.
 *
 * Gated by the same predicates as the Substitution Prohibition, so the ladder
 * can never pre-empt a genuine desktop request, and never acts on the
 * bare-question default that comprehension applies to unrecognised questions.
 */
export function answerForGoal(goal: GoalContract): Answer | null {
  if (!isAnswerOnlyGoal(goal) || !hasPositiveOutcomeEvidence(goal)) {
    return null;
  }
  return resolveAnswer(goal);
}

/**
 * Which observation, if any, this request needs before it can be answered.
 * Returns a semantic need — the caller decides how to obtain it through the
 * existing authorized path.
 */
export function observationNeededFor(goal: GoalContract): ObservationNeed | null {
  for (const source of OBSERVATION_SOURCES) {
    if (source.covers(goal)) return source.need;
  }
  return null;
}

/**
 * Turn an authorized observation into the Owner's answer.
 *
 * Returns null when no source covers the request or the observation carries
 * nothing to report, so the truthful result of the observation itself is used
 * instead of an invented one.
 */
export function composeObservationAnswer(
  goal: GoalContract,
  observation: ObservationResult,
): Answer | null {
  for (const source of OBSERVATION_SOURCES) {
    if (!source.covers(goal)) continue;
    const answer = source.compose(goal, observation);
    if (answer) return answer;
  }
  return null;
}
