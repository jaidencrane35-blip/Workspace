/**
 * Experience Validation instrumentation — interaction events only.
 * Local storage. Development-only by default. No user content.
 */

import type { PilotPrimaryView } from "../lib/pilotChrome";
import {
  sanitizeEvent,
  type ExperienceCommandId,
  type ExperienceDestination,
  type ExperienceEvent,
  type ExperienceFlow,
  type ExperienceSession,
  type InputModality,
} from "./experienceEvents";
import {
  browserStore,
  clearStore,
  listSessions,
  memoryStore,
  upsertSession,
  type ExperienceStoreAdapter,
} from "./experienceStore";
import { computeFrictionScore, type FrictionScore } from "./frictionModel";
import { isExperienceValidationEnabled } from "./experienceValidationGate";
import { replaySession, type ReplayHandlers, type ReplayResult } from "./sessionReplay";

export type {
  ExperienceEvent,
  ExperienceSession,
  FrictionScore,
  ReplayResult,
};

let enabled = false;
let store: ExperienceStoreAdapter = memoryStore();
let session: ExperienceSession | null = null;
let seq = 0;
let startedAt = 0;
let lastDestination: ExperienceDestination = "unknown";
let activeFlow: ExperienceFlow = "none";
let lastEventKey = "";
let lastEventAt = 0;
let firstMeaningful = false;
let disposeListeners: (() => void) | null = null;
let persistTimer: ReturnType<typeof setTimeout> | null = null;

function nowOffset(): number {
  return Math.max(0, Date.now() - startedAt);
}

function toDestination(view: string): ExperienceDestination {
  if (
    view === "home" ||
    view === "save" ||
    view === "resume" ||
    view === "pilot" ||
    view === "help"
  ) {
    return view;
  }
  return "unknown";
}

function schedulePersist(): void {
  if (!session) {
    return;
  }
  if (persistTimer) {
    clearTimeout(persistTimer);
  }
  persistTimer = setTimeout(() => {
    if (session) {
      upsertSession(store, session);
    }
  }, 50);
}

function append(partial: Omit<ExperienceEvent, "seq" | "t"> & { t?: number }): void {
  if (!enabled || !session) {
    return;
  }
  const event = sanitizeEvent({
    seq: seq++,
    t: partial.t ?? nowOffset(),
    ...partial,
  });
  if (!event) {
    return;
  }
  session.events.push(event);
  schedulePersist();
}

function noteRepeated(type: string, destination?: ExperienceDestination): void {
  const key = `${type}:${destination ?? ""}`;
  const t = nowOffset();
  if (key === lastEventKey && t - lastEventAt < 1200) {
    append({ type: "repeated_action", destination, targetKind: type });
  }
  lastEventKey = key;
  lastEventAt = t;
}

export function isInstrumentationActive(): boolean {
  return enabled;
}

export function getActiveSession(): ExperienceSession | null {
  return session ? structuredClone(session) : null;
}

export function listStoredSessions(): ExperienceSession[] {
  return listSessions(store).map((s) => structuredClone(s));
}

export function clearInstrumentation(): void {
  clearStore(store);
  session = null;
  seq = 0;
  firstMeaningful = false;
  activeFlow = "none";
  lastDestination = "unknown";
}

export function frictionForSession(target?: ExperienceSession): FrictionScore | null {
  const s = target ?? session;
  if (!s) {
    return null;
  }
  return computeFrictionScore(s);
}

export function replayStoredSession(
  sessionId: string,
  handlers?: ReplayHandlers,
): ReplayResult | null {
  const found = listSessions(store).find((s) => s.sessionId === sessionId);
  if (!found) {
    return null;
  }
  return replaySession(found, handlers);
}

export function replayActiveSession(handlers?: ReplayHandlers): ReplayResult | null {
  if (!session) {
    return null;
  }
  return replaySession(session, handlers);
}

/**
 * Start instrumentation. Safe no-op when gate is closed.
 * `storeOverride` is for tests.
 */
export function initExperienceInstrumentation(options?: {
  store?: ExperienceStoreAdapter;
  sessionId?: string;
  force?: boolean;
}): void {
  const allow = options?.force || isExperienceValidationEnabled();
  if (!allow) {
    enabled = false;
    return;
  }
  enabled = true;
  store = options?.store ?? browserStore() ?? memoryStore();
  startedAt = Date.now();
  seq = 0;
  firstMeaningful = false;
  activeFlow = "none";
  lastDestination = "home";
  session = {
    schemaVersion: 1,
    sessionId: options?.sessionId ?? `ev-${startedAt.toString(36)}`,
    startedAt,
    events: [],
  };
  append({ type: "session_start", destination: "home" });
  upsertSession(store, session);

  if (typeof window !== "undefined" && !disposeListeners) {
    const onPointer = () => {
      markMeaningful("pointer", "pointer");
    };
    const onKey = (event: KeyboardEvent) => {
      if (event.metaKey || event.ctrlKey || event.altKey) {
        trackCommand("unknown", "keyboard");
      }
      markMeaningful("keyboard", "keyboard");
    };
    window.addEventListener("pointerdown", onPointer, true);
    window.addEventListener("keydown", onKey, true);
    disposeListeners = () => {
      window.removeEventListener("pointerdown", onPointer, true);
      window.removeEventListener("keydown", onKey, true);
      disposeListeners = null;
    };
  }
}

export function disposeExperienceInstrumentation(): void {
  disposeListeners?.();
  if (persistTimer) {
    clearTimeout(persistTimer);
    persistTimer = null;
  }
  if (session && enabled) {
    upsertSession(store, session);
  }
  enabled = false;
}

function markMeaningful(
  modality: InputModality,
  targetKind: string,
): void {
  if (!enabled) {
    return;
  }
  append({ type: "modality", modality, targetKind });
  if (!firstMeaningful) {
    firstMeaningful = true;
    append({
      type: "first_meaningful_interaction",
      modality,
      destination: lastDestination,
      targetKind,
    });
  }
}

export function trackNavigate(
  next: PilotPrimaryView | ExperienceDestination,
  options?: { modality?: InputModality; commandId?: ExperienceCommandId },
): void {
  if (!enabled) {
    return;
  }
  const destination = toDestination(next);
  const from = lastDestination;

  if (activeFlow !== "none" && destination !== flowDestination(activeFlow)) {
    append({
      type: "flow_abandon",
      flow: activeFlow,
      from,
      destination,
      modality: options?.modality,
    });
    activeFlow = "none";
  }

  if (destination === from && from !== "unknown") {
    // ignore no-op
  } else if (
    destinationsIndex(destination) < destinationsIndex(from) &&
    from !== "unknown"
  ) {
    append({
      type: "backtrack",
      from,
      destination,
      modality: options?.modality,
    });
  }

  append({
    type: "navigate",
    from,
    destination,
    modality: options?.modality ?? "unknown",
    commandId: options?.commandId ?? "dock_navigate",
  });
  noteRepeated("navigate", destination);
  lastDestination = destination;

  if (destination === "save") {
    trackFlowStart("save");
  }
}

function destinationsIndex(dest: ExperienceDestination): number {
  const order = ["home", "save", "resume", "pilot", "help", "unknown"];
  return order.indexOf(dest);
}

function flowDestination(flow: ExperienceFlow): ExperienceDestination {
  if (flow === "save") return "save";
  if (flow === "continue") return "resume";
  return "unknown";
}

export function trackFlowStart(flow: "save" | "continue"): void {
  if (!enabled) {
    return;
  }
  activeFlow = flow;
  append({
    type: "flow_start",
    flow,
    destination: flowDestination(flow),
  });
}

export function trackSaveSuccess(): void {
  if (!enabled) {
    return;
  }
  append({ type: "save_success", flow: "save", destination: "save" });
  activeFlow = "none";
  noteRepeated("save_success", "save");
}

export function trackContinueSuccess(): void {
  if (!enabled) {
    return;
  }
  append({
    type: "continue_success",
    flow: "continue",
    destination: "resume",
  });
  activeFlow = "none";
  noteRepeated("continue_success", "resume");
}

export function trackCommand(
  commandId: ExperienceCommandId,
  modality: InputModality = "unknown",
): void {
  if (!enabled) {
    return;
  }
  append({ type: "command", commandId, modality, destination: lastDestination });
  noteRepeated(`command:${commandId}`, lastDestination);
}

/** Test helper — build a friction score from an explicit session. */
export function computeFrictionFromSession(
  target: ExperienceSession,
): FrictionScore {
  return computeFrictionScore(target);
}
