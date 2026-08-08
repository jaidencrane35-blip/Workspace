/**
 * P24.S2 — Language Faculty Boundary (scaffold).
 *
 * ADR-P24-CONFLICT-A Option C:
 *   Faculty → MeaningProposal → deterministic validator → GoalContract
 *
 * This module is the seam only. The sole Faculty implementation today is the
 * existing deterministic `comprehend()` — not a model. Future backends must
 * replace `proposeMeaning` without gaining capability selection, IPC, memory
 * writes, or Kernel authority.
 *
 * Forbidden here (enforced by verify-language-faculty): capability registry,
 * execution planner, CapabilityIntent, providers, Tauri/IPC, permission,
 * completion claims, context/memory writes.
 */

import {
  comprehend,
  type GoalContract,
  type GoalDomain,
  type GoalMode,
  type GoalOutcome,
  type GoalReference,
  type GoalTarget,
} from "./goalContract";

/**
 * Untrusted Faculty output. Same semantic shape as GoalContract — ADR §6:
 * GoalContract is sufficient; no parallel Meaning type with execution fields.
 *
 * Accepted only after `validateMeaningProposal`. Callers must not treat a
 * proposal as an authorized GoalContract.
 */
export type MeaningProposal = GoalContract;

/** Read-only Faculty port. Implementations propose meaning; they never act. */
export interface LanguageFaculty {
  /**
   * Propose meaning for an Owner utterance.
   * Must not perform IPC, select capabilities, or write context/memory.
   */
  proposeMeaning(utterance: string): MeaningProposal;
}

const OUTCOMES: ReadonlySet<string> = new Set([
  "SOCIAL",
  "KNOW",
  "COMPUTE",
  "PERCEIVE_MACHINE",
  "PERCEIVE_WORLD",
  "REACH_STATE",
  "META",
  "UNDETERMINED",
]);

const MODES: ReadonlySet<string> = new Set([
  "conversation",
  "information",
  "computation",
  "observation",
  "show",
  "action",
  "hybrid",
  "meta",
  "undetermined",
]);

const DOMAINS: ReadonlySet<string> = new Set([
  "conversation",
  "knowledge",
  "time",
  "desktop",
  "application",
  "web",
  "content",
  "workspace",
  "unknown",
]);

const TARGET_ROLES: ReadonlySet<string> = new Set([
  "subject",
  "place",
  "application",
  "site",
  "container",
  "state",
]);

/** Fields that would turn Meaning into Plan / Effect / Authority. */
const FORBIDDEN_PROPOSAL_KEYS = [
  "steps",
  "plan",
  "capabilities",
  "capabilityId",
  "capabilityIds",
  "provider",
  "providers",
  "operation",
  "operations",
  "domainOperation",
  "intent",
  "capabilityIntent",
  "ipc",
  "permission",
  "granted",
  "completed",
  "verified",
  "execution",
] as const;

/**
 * Authority-leak and invented-identity patterns that must never appear in
 * Faculty meaning (evidence, subjects, target phrases/canonicals).
 */
const AUTHORITY_LEAK =
  /\b(C-[A-Z]{2,}-\d{3}|CapabilityIntent|execute_capability_intent|capabilityRegistry|executionPlanner|ProviderInvoke|invokeIpc)\b/i;

const PROVIDER_LEAK =
  /\b(WindowProvider|ApplicationProvider|ClipboardProvider|BrowserProvider)\b/;

export type MeaningValidationResult =
  | { ok: true; goal: GoalContract }
  | { ok: false; reason: string };

function isPlainObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isStringArray(value: unknown): value is string[] {
  return Array.isArray(value) && value.every((item) => typeof item === "string");
}

function validateTarget(value: unknown, index: number): string | null {
  if (!isPlainObject(value)) return `targets[${index}] must be an object`;
  if (typeof value.phrase !== "string") return `targets[${index}].phrase must be a string`;
  if (typeof value.role !== "string" || !TARGET_ROLES.has(value.role)) {
    return `targets[${index}].role is not a GoalTargetRole`;
  }
  if (!(typeof value.canonical === "string" || value.canonical === null)) {
    return `targets[${index}].canonical must be string | null`;
  }
  if (typeof value.recognized !== "boolean") {
    return `targets[${index}].recognized must be a boolean`;
  }
  // Invented identity: Faculty must not claim recognition without a canonical,
  // and must not invent a canonical while claiming unrecognized.
  if (value.recognized === true && value.canonical === null) {
    return `targets[${index}] claims recognized without a canonical`;
  }
  return null;
}

function validateReference(value: unknown, index: number): string | null {
  if (!isPlainObject(value)) return `references[${index}] must be an object`;
  if (typeof value.phrase !== "string") return `references[${index}].phrase must be a string`;
  if (typeof value.resolved !== "boolean") {
    return `references[${index}].resolved must be a boolean`;
  }
  if (!(typeof value.referent === "string" || value.referent === null)) {
    return `references[${index}].referent must be string | null`;
  }
  if (value.resolved === true && value.referent === null) {
    return `references[${index}] claims resolved without a referent`;
  }
  // Insufficient context must not invent a target.
  if (value.resolved === false && value.referent !== null) {
    return `references[${index}] invents a referent while unresolved`;
  }
  return null;
}

function scanLeak(label: string, text: string): string | null {
  if (AUTHORITY_LEAK.test(text)) {
    return `${label} contains capability/authority vocabulary`;
  }
  if (PROVIDER_LEAK.test(text)) {
    return `${label} contains provider identity`;
  }
  return null;
}

/**
 * Deterministic validation boundary (ADR Option C).
 *
 * Accepts unknown input so malformed Faculty output can be rejected rather than
 * trusted by TypeScript structural typing alone.
 */
export function validateMeaningProposal(
  proposal: unknown,
): MeaningValidationResult {
  if (!isPlainObject(proposal)) {
    return { ok: false, reason: "proposal must be an object" };
  }

  for (const key of FORBIDDEN_PROPOSAL_KEYS) {
    if (key in proposal) {
      return { ok: false, reason: `proposal must not carry authority field "${key}"` };
    }
  }

  if (typeof proposal.utterance !== "string") {
    return { ok: false, reason: "utterance must be a string" };
  }
  if (typeof proposal.normalized !== "string") {
    return { ok: false, reason: "normalized must be a string" };
  }
  if (typeof proposal.mode !== "string" || !MODES.has(proposal.mode)) {
    return { ok: false, reason: "mode is not a GoalMode" };
  }
  if (typeof proposal.outcome !== "string" || !OUTCOMES.has(proposal.outcome)) {
    return { ok: false, reason: "outcome is not a GoalOutcome" };
  }
  if (typeof proposal.domain !== "string" || !DOMAINS.has(proposal.domain)) {
    return { ok: false, reason: "domain is not a GoalDomain" };
  }
  if (!(typeof proposal.subject === "string" || proposal.subject === null)) {
    return { ok: false, reason: "subject must be string | null" };
  }
  if (!Array.isArray(proposal.targets)) {
    return { ok: false, reason: "targets must be an array" };
  }
  if (!Array.isArray(proposal.references)) {
    return { ok: false, reason: "references must be an array" };
  }
  if (
    !(typeof proposal.requestedResult === "string" || proposal.requestedResult === null)
  ) {
    return { ok: false, reason: "requestedResult must be string | null" };
  }
  if (typeof proposal.compound !== "boolean") {
    return { ok: false, reason: "compound must be a boolean" };
  }
  if (!isStringArray(proposal.clauses)) {
    return { ok: false, reason: "clauses must be string[]" };
  }
  if (typeof proposal.clarificationNeeded !== "boolean") {
    return { ok: false, reason: "clarificationNeeded must be a boolean" };
  }
  if (!isStringArray(proposal.uncertainties)) {
    return { ok: false, reason: "uncertainties must be string[]" };
  }
  if (!isStringArray(proposal.evidence)) {
    return { ok: false, reason: "evidence must be string[]" };
  }

  for (let i = 0; i < proposal.targets.length; i++) {
    const err = validateTarget(proposal.targets[i], i);
    if (err) return { ok: false, reason: err };
  }
  for (let i = 0; i < proposal.references.length; i++) {
    const err = validateReference(proposal.references[i], i);
    if (err) return { ok: false, reason: err };
  }

  const textSurfaces = [
    proposal.utterance,
    proposal.normalized,
    proposal.subject ?? "",
    proposal.requestedResult ?? "",
    ...proposal.clauses,
    ...proposal.uncertainties,
    ...proposal.evidence,
    ...proposal.targets.map(
      (t) =>
        `${(t as GoalTarget).phrase}\0${(t as GoalTarget).canonical ?? ""}`,
    ),
    ...proposal.references.map(
      (r) =>
        `${(r as GoalReference).phrase}\0${(r as GoalReference).referent ?? ""}`,
    ),
  ];
  for (const surface of textSurfaces) {
    const leak = scanLeak("proposal", surface);
    if (leak) return { ok: false, reason: leak };
  }

  // Semantic consistency: answer-only outcomes must not claim action mode alone.
  const outcome = proposal.outcome as GoalOutcome;
  const mode = proposal.mode as GoalMode;
  if (
    (outcome === "KNOW" || outcome === "COMPUTE" || outcome === "SOCIAL") &&
    mode === "action"
  ) {
    return {
      ok: false,
      reason: `${outcome} must not be paired with action-only mode`,
    };
  }

  const goal: GoalContract = {
    utterance: proposal.utterance,
    normalized: proposal.normalized,
    mode: mode,
    outcome: outcome,
    domain: proposal.domain as GoalDomain,
    subject: proposal.subject as string | null,
    targets: proposal.targets as GoalTarget[],
    references: proposal.references as GoalReference[],
    requestedResult: proposal.requestedResult as string | null,
    compound: proposal.compound,
    clauses: proposal.clauses,
    clarificationNeeded: proposal.clarificationNeeded,
    uncertainties: proposal.uncertainties,
    // Preserve Faculty evidence as proposed — do not annotate the happy path
    // so the deterministic Faculty remains behaviourally identical to comprehend().
    evidence: proposal.evidence,
  };

  return { ok: true, goal };
}

/**
 * Sole Faculty implementation for P24.S2: the existing deterministic
 * comprehend(). Not a model. Does not rewrite comprehension.
 */
export const deterministicLanguageFaculty: LanguageFaculty = {
  proposeMeaning(utterance: string): MeaningProposal {
    return comprehend(utterance);
  },
};

let activeFaculty: LanguageFaculty = deterministicLanguageFaculty;

/** Test / future seam: replace the Faculty without changing the validator. */
export function setLanguageFaculty(faculty: LanguageFaculty): void {
  activeFaculty = faculty;
}

export function getLanguageFaculty(): LanguageFaculty {
  return activeFaculty;
}

export function resetLanguageFaculty(): void {
  activeFaculty = deterministicLanguageFaculty;
}

/**
 * Comprehend through the Faculty boundary.
 *
 * propose → validate → GoalContract; on rejection, fall back to deterministic
 * comprehend() exactly once (ADR: no unbounded retries).
 *
 * Context grounding (groundGoalInContext) remains outside this function — it
 * is deterministic session grounding, not Faculty authority.
 */
export function comprehendViaFaculty(
  utterance: string,
  faculty: LanguageFaculty = activeFaculty,
): GoalContract {
  const proposal = faculty.proposeMeaning(utterance);
  const validated = validateMeaningProposal(proposal);
  if (validated.ok) {
    return validated.goal;
  }
  // Deterministic fallback — the cascade/comprehend path remains the safety net.
  const fallback = comprehend(utterance);
  return {
    ...fallback,
    evidence: [
      ...fallback.evidence,
      `faculty_rejected=${validated.reason}`,
      "faculty=deterministic_fallback",
    ],
  };
}
