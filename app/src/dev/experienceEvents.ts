/**
 * Interaction-only event schema for Experience Validation.
 * Never carries user content (handoff notes, names, titles, window text).
 */

export type ExperienceDestination =
  | "home"
  | "save"
  | "resume"
  | "pilot"
  | "help"
  | "unknown";

export type InputModality = "pointer" | "keyboard" | "unknown";

export type ExperienceEventType =
  | "session_start"
  | "first_meaningful_interaction"
  | "navigate"
  | "flow_start"
  | "save_success"
  | "continue_success"
  | "flow_abandon"
  | "command"
  | "repeated_action"
  | "backtrack"
  | "modality";

export type ExperienceFlow = "save" | "continue" | "none";

/** Allowlisted command ids — never free-text. */
export type ExperienceCommandId =
  | "dock_navigate"
  | "dock_keyboard"
  | "command_surface"
  | "go_continue"
  | "go_save"
  | "unknown";

export interface ExperienceEvent {
  /** Monotonic sequence within a session. */
  seq: number;
  /** Milliseconds from session start (deterministic for replay). */
  t: number;
  type: ExperienceEventType;
  destination?: ExperienceDestination;
  from?: ExperienceDestination;
  flow?: ExperienceFlow;
  modality?: InputModality;
  commandId?: ExperienceCommandId;
  /** Opaque target role/testid — never text content. */
  targetKind?: string;
}

export interface ExperienceSession {
  schemaVersion: 1;
  sessionId: string;
  startedAt: number;
  events: ExperienceEvent[];
}

export const EXPERIENCE_EVENT_TYPES: readonly ExperienceEventType[] = [
  "session_start",
  "first_meaningful_interaction",
  "navigate",
  "flow_start",
  "save_success",
  "continue_success",
  "flow_abandon",
  "command",
  "repeated_action",
  "backtrack",
  "modality",
] as const;

/** Fields that must never appear on stored events. */
export const FORBIDDEN_EVENT_KEYS = [
  "handoff",
  "handoffNote",
  "note",
  "name",
  "title",
  "label",
  "text",
  "content",
  "summary",
  "windowTitle",
  "userContent",
  "message",
] as const;

export function isDestination(value: string): value is ExperienceDestination {
  return (
    value === "home" ||
    value === "save" ||
    value === "resume" ||
    value === "pilot" ||
    value === "help" ||
    value === "unknown"
  );
}

export function sanitizeEvent(
  event: ExperienceEvent,
): ExperienceEvent | null {
  if (!EXPERIENCE_EVENT_TYPES.includes(event.type)) {
    return null;
  }
  const clean: ExperienceEvent = {
    seq: event.seq,
    t: event.t,
    type: event.type,
  };
  if (event.destination && isDestination(event.destination)) {
    clean.destination = event.destination;
  }
  if (event.from && isDestination(event.from)) {
    clean.from = event.from;
  }
  if (event.flow === "save" || event.flow === "continue" || event.flow === "none") {
    clean.flow = event.flow;
  }
  if (
    event.modality === "pointer" ||
    event.modality === "keyboard" ||
    event.modality === "unknown"
  ) {
    clean.modality = event.modality;
  }
  if (event.commandId) {
    clean.commandId = event.commandId;
  }
  if (event.targetKind && /^[a-z0-9_.:-]{1,64}$/i.test(event.targetKind)) {
    clean.targetKind = event.targetKind;
  }
  return clean;
}
