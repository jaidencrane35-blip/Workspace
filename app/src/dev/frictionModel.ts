/**
 * Cognitive friction model — derived only from interaction traces.
 * Reproducible: same session → same FrictionScore.
 */

import type { ExperienceEvent, ExperienceSession } from "./experienceEvents";

export interface FrictionComponents {
  /** ms from session_start to first_meaningful_interaction (lower better). */
  timeToConfidenceMs: number;
  /** navigations that did not lead to save/continue success. */
  unnecessaryNavigation: number;
  /** repeated_action events. */
  interactionRedundancy: number;
  /** distinct destination switches. */
  attentionSwitching: number;
  /** flow_abandon count. */
  flowInterruption: number;
  /** continue_success after any prior abandon or backtrack in session. */
  successfulRecovery: number;
}

export interface FrictionScore {
  /** 0 = frictionless, 1 = high friction. */
  score: number;
  components: FrictionComponents;
  /** Evidence counts for audit. */
  evidence: {
    eventCount: number;
    saveSuccess: number;
    continueSuccess: number;
    firstInteractionAt: number | null;
    pointerEvents: number;
    keyboardEvents: number;
  };
}

function eventsOf(
  session: ExperienceSession,
  type: ExperienceEvent["type"],
): ExperienceEvent[] {
  return session.events.filter((e) => e.type === type);
}

/**
 * Pure friction calculation — no I/O, no clocks.
 */
export function computeFrictionScore(session: ExperienceSession): FrictionScore {
  const first = eventsOf(session, "first_meaningful_interaction")[0];
  const navigates = eventsOf(session, "navigate");
  const saves = eventsOf(session, "save_success");
  const continues = eventsOf(session, "continue_success");
  const abandons = eventsOf(session, "flow_abandon");
  const repeats = eventsOf(session, "repeated_action");
  const backtracks = eventsOf(session, "backtrack");
  const modalities = eventsOf(session, "modality");

  const lastEvent = session.events[session.events.length - 1];
  const timeToConfidenceMs = first ? first.t : lastEvent?.t ?? 0;

  const successNavTargets = new Set<string>();
  for (const e of [...saves, ...continues]) {
    if (e.destination) {
      successNavTargets.add(e.destination);
    }
  }
  let unnecessaryNavigation = 0;
  for (const nav of navigates) {
    const dest = nav.destination ?? "unknown";
    if (dest === "home" || dest === "help" || dest === "pilot") {
      // Supporting destinations — count only rapid thrashing later via repeats.
      continue;
    }
    if (!successNavTargets.has(dest) && dest !== "unknown") {
      unnecessaryNavigation += 1;
    }
  }
  // If user never succeeded, all task navigations count.
  if (saves.length + continues.length === 0) {
    unnecessaryNavigation = navigates.filter(
      (n) => n.destination === "save" || n.destination === "resume",
    ).length;
  }

  const destinations = navigates
    .map((n) => n.destination)
    .filter(Boolean) as string[];
  let attentionSwitching = 0;
  for (let i = 1; i < destinations.length; i++) {
    if (destinations[i] !== destinations[i - 1]) {
      attentionSwitching += 1;
    }
  }

  const recovered =
    continues.length > 0 && (abandons.length > 0 || backtracks.length > 0)
      ? 1
      : 0;

  const components: FrictionComponents = {
    timeToConfidenceMs,
    unnecessaryNavigation,
    interactionRedundancy: repeats.length,
    attentionSwitching,
    flowInterruption: abandons.length,
    successfulRecovery: recovered,
  };

  // Normalize into 0–1 composite (weights sum to 1).
  const timeNorm = Math.min(1, timeToConfidenceMs / 15000);
  const navNorm = Math.min(1, unnecessaryNavigation / 6);
  const redNorm = Math.min(1, repeats.length / 8);
  const switchNorm = Math.min(1, attentionSwitching / 10);
  const interruptNorm = Math.min(1, abandons.length / 3);
  const recoveryRelief = recovered * 0.08;

  const score = Math.max(
    0,
    Math.min(
      1,
      timeNorm * 0.22 +
        navNorm * 0.2 +
        redNorm * 0.18 +
        switchNorm * 0.18 +
        interruptNorm * 0.22 -
        recoveryRelief,
    ),
  );

  return {
    score: Number(score.toFixed(4)),
    components,
    evidence: {
      eventCount: session.events.length,
      saveSuccess: saves.length,
      continueSuccess: continues.length,
      firstInteractionAt: first?.t ?? null,
      pointerEvents: modalities.filter((m) => m.modality === "pointer").length,
      keyboardEvents: modalities.filter((m) => m.modality === "keyboard").length,
    },
  };
}
