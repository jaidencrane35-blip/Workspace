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

/** Registered sources. Order within a rung is declaration order. */
const SOURCES: readonly AnswerSource[] = [temporalAnswerSource];

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
