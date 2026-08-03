/**
 * Deterministic trace analysis — aggregates Sprint 51 sessions into metrics.
 * Never inspects or infers user content.
 */

import type {
  ExperienceDestination,
  ExperienceSession,
} from "./experienceEvents";
import { computeFrictionScore } from "./frictionModel";
import { frictionEquals, replaySession } from "./sessionReplay";

export interface HesitationHotspot {
  destination: ExperienceDestination;
  medianGapMs: number;
  samples: number;
}

export interface NavigationLoop {
  /** Allowlisted destinations joined by → (e.g. save→resume→save). */
  pattern: string;
  count: number;
}

export interface TraceAggregate {
  sessionCount: number;
  sessionIds: string[];
  medianTimeToConfidenceMs: number;
  meanFrictionScore: number;
  medianFrictionScore: number;
  frictionMin: number;
  frictionMax: number;
  frictionP25: number;
  frictionP75: number;
  frictionScores: number[];
  hesitationHotspots: HesitationHotspot[];
  navigationLoops: NavigationLoop[];
  abandonedFlows: {
    save: number;
    continue: number;
    total: number;
  };
  recovery: {
    recovered: number;
    interrupted: number;
    successRate: number;
  };
  replay: {
    sessionsReplayed: number;
    divergent: number;
    divergenceRate: number;
  };
  successes: {
    save: number;
    continue: number;
  };
}

function median(sorted: number[]): number {
  if (sorted.length === 0) {
    return 0;
  }
  const mid = Math.floor(sorted.length / 2);
  if (sorted.length % 2 === 0) {
    return Number(((sorted[mid - 1]! + sorted[mid]!) / 2).toFixed(4));
  }
  return sorted[mid]!;
}

function percentile(sorted: number[], p: number): number {
  if (sorted.length === 0) {
    return 0;
  }
  if (sorted.length === 1) {
    return sorted[0]!;
  }
  const idx = (sorted.length - 1) * p;
  const lo = Math.floor(idx);
  const hi = Math.ceil(idx);
  if (lo === hi) {
    return sorted[lo]!;
  }
  const w = idx - lo;
  return Number((sorted[lo]! * (1 - w) + sorted[hi]! * w).toFixed(4));
}

function mean(values: number[]): number {
  if (values.length === 0) {
    return 0;
  }
  const sum = values.reduce((a, b) => a + b, 0);
  return Number((sum / values.length).toFixed(4));
}

function isDest(value: string | undefined): value is ExperienceDestination {
  return (
    value === "home" ||
    value === "save" ||
    value === "resume" ||
    value === "pilot" ||
    value === "help" ||
    value === "unknown"
  );
}

/**
 * Gaps between consecutive events while remaining on a destination,
 * using navigate boundaries. Hesitation = gap ≥ 800ms before next navigate.
 */
function collectHesitationGaps(
  session: ExperienceSession,
): Map<ExperienceDestination, number[]> {
  const gaps = new Map<ExperienceDestination, number[]>();
  let current: ExperienceDestination = "home";
  let lastT = 0;

  for (const event of session.events) {
    if (event.type === "session_start" && isDest(event.destination)) {
      current = event.destination;
      lastT = event.t;
      continue;
    }
    if (event.type === "navigate" && isDest(event.destination)) {
      const gap = event.t - lastT;
      if (gap >= 800) {
        const list = gaps.get(current) ?? [];
        list.push(gap);
        gaps.set(current, list);
      }
      current = event.destination;
      lastT = event.t;
      continue;
    }
    lastT = event.t;
  }
  return gaps;
}

/** Detect A→B→A loops in navigate destination sequences. */
function collectLoops(session: ExperienceSession): Map<string, number> {
  const dests = session.events
    .filter((e) => e.type === "navigate" && isDest(e.destination))
    .map((e) => e.destination!);
  const counts = new Map<string, number>();
  for (let i = 0; i + 2 < dests.length; i++) {
    const a = dests[i]!;
    const b = dests[i + 1]!;
    const c = dests[i + 2]!;
    if (a === c && a !== b) {
      const pattern = `${a}→${b}→${c}`;
      counts.set(pattern, (counts.get(pattern) ?? 0) + 1);
    }
  }
  return counts;
}

function emptyAggregate(): TraceAggregate {
  return {
    sessionCount: 0,
    sessionIds: [],
    medianTimeToConfidenceMs: 0,
    meanFrictionScore: 0,
    medianFrictionScore: 0,
    frictionMin: 0,
    frictionMax: 0,
    frictionP25: 0,
    frictionP75: 0,
    frictionScores: [],
    hesitationHotspots: [],
    navigationLoops: [],
    abandonedFlows: { save: 0, continue: 0, total: 0 },
    recovery: { recovered: 0, interrupted: 0, successRate: 0 },
    replay: { sessionsReplayed: 0, divergent: 0, divergenceRate: 0 },
    successes: { save: 0, continue: 0 },
  };
}

/**
 * Aggregate one or more sessions. Pure and deterministic for the same input order.
 * Sessions are analyzed in the given array order; sessionIds are sorted for identity stability.
 */
export function analyzeTraces(sessions: ExperienceSession[]): TraceAggregate {
  if (sessions.length === 0) {
    return emptyAggregate();
  }

  const ordered = [...sessions].sort((a, b) =>
    a.sessionId.localeCompare(b.sessionId),
  );

  const times: number[] = [];
  const frictions: number[] = [];
  const hotspotGaps = new Map<ExperienceDestination, number[]>();
  const loopCounts = new Map<string, number>();
  let abandonSave = 0;
  let abandonContinue = 0;
  let recovered = 0;
  let interrupted = 0;
  let divergent = 0;
  let saveSuccess = 0;
  let continueSuccess = 0;

  for (const session of ordered) {
    const friction = computeFrictionScore(session);
    frictions.push(friction.score);
    times.push(friction.components.timeToConfidenceMs);
    saveSuccess += friction.evidence.saveSuccess;
    continueSuccess += friction.evidence.continueSuccess;

    abandonSave += session.events.filter(
      (e) => e.type === "flow_abandon" && e.flow === "save",
    ).length;
    abandonContinue += session.events.filter(
      (e) => e.type === "flow_abandon" && e.flow === "continue",
    ).length;

    if (friction.components.flowInterruption > 0) {
      interrupted += 1;
    }
    if (friction.components.successfulRecovery > 0) {
      recovered += 1;
    }

    const gaps = collectHesitationGaps(session);
    for (const [dest, values] of gaps) {
      const list = hotspotGaps.get(dest) ?? [];
      list.push(...values);
      hotspotGaps.set(dest, list);
    }

    const loops = collectLoops(session);
    for (const [pattern, count] of loops) {
      loopCounts.set(pattern, (loopCounts.get(pattern) ?? 0) + count);
    }

    const firstReplay = replaySession(session);
    const secondReplay = replaySession(session);
    const expectedDests = session.events
      .filter((e) => e.type === "navigate" && isDest(e.destination))
      .map((e) => e.destination!);
    const destMismatch =
      firstReplay.destinations.length !== expectedDests.length ||
      firstReplay.destinations.some((d, i) => d !== expectedDests[i]);
    if (
      destMismatch ||
      !frictionEquals(firstReplay.friction, secondReplay.friction) ||
      !frictionEquals(firstReplay.friction, friction)
    ) {
      divergent += 1;
    }
  }

  const sortedFriction = [...frictions].sort((a, b) => a - b);
  const sortedTimes = [...times].sort((a, b) => a - b);

  const hesitationHotspots: HesitationHotspot[] = [...hotspotGaps.entries()]
    .map(([destination, values]) => {
      const sorted = [...values].sort((a, b) => a - b);
      return {
        destination,
        medianGapMs: median(sorted),
        samples: values.length,
      };
    })
    .sort((a, b) => {
      if (b.samples !== a.samples) {
        return b.samples - a.samples;
      }
      return a.destination.localeCompare(b.destination);
    });

  const navigationLoops: NavigationLoop[] = [...loopCounts.entries()]
    .map(([pattern, count]) => ({ pattern, count }))
    .sort((a, b) => {
      if (b.count !== a.count) {
        return b.count - a.count;
      }
      return a.pattern.localeCompare(b.pattern);
    });

  const abandonTotal = abandonSave + abandonContinue;
  const sessionsReplayed = ordered.length;

  return {
    sessionCount: ordered.length,
    sessionIds: ordered.map((s) => s.sessionId),
    medianTimeToConfidenceMs: median(sortedTimes),
    meanFrictionScore: mean(frictions),
    medianFrictionScore: median(sortedFriction),
    frictionMin: sortedFriction[0] ?? 0,
    frictionMax: sortedFriction[sortedFriction.length - 1] ?? 0,
    frictionP25: percentile(sortedFriction, 0.25),
    frictionP75: percentile(sortedFriction, 0.75),
    frictionScores: sortedFriction,
    hesitationHotspots,
    navigationLoops,
    abandonedFlows: {
      save: abandonSave,
      continue: abandonContinue,
      total: abandonTotal,
    },
    recovery: {
      recovered,
      interrupted,
      successRate:
        interrupted === 0
          ? 0
          : Number((recovered / interrupted).toFixed(4)),
    },
    replay: {
      sessionsReplayed,
      divergent,
      divergenceRate:
        sessionsReplayed === 0
          ? 0
          : Number((divergent / sessionsReplayed).toFixed(4)),
    },
    successes: {
      save: saveSuccess,
      continue: continueSuccess,
    },
  };
}
