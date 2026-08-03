/**
 * Local-only ring buffer for Experience Validation sessions.
 * Never transmits. Never stores user content.
 */

import type { ExperienceSession } from "./experienceEvents";
import {
  clearJsonKey,
  loadJsonBundle,
  saveJsonBundle,
} from "./governanceStore";

export const STORAGE_KEY = "ws.dev.experience.validation.v1";
export const MAX_SESSIONS = 20;
export const MAX_EVENTS_PER_SESSION = 2000;

export interface ExperienceStoreAdapter {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
}

export function memoryStore(seed: Record<string, string> = {}): ExperienceStoreAdapter {
  const map = new Map(Object.entries(seed));
  return {
    getItem: (key) => map.get(key) ?? null,
    setItem: (key, value) => {
      map.set(key, value);
    },
    removeItem: (key) => {
      map.delete(key);
    },
  };
}

export function browserStore(): ExperienceStoreAdapter | null {
  if (typeof localStorage === "undefined") {
    return null;
  }
  return localStorage;
}

interface StoredBundle {
  schemaVersion: 1;
  sessions: ExperienceSession[];
}

function emptyBundle(): StoredBundle {
  return { schemaVersion: 1, sessions: [] };
}

export function loadBundle(store: ExperienceStoreAdapter): StoredBundle {
  const parsed = loadJsonBundle(
    store,
    STORAGE_KEY,
    emptyBundle,
    (value): value is StoredBundle =>
      !!value &&
      typeof value === "object" &&
      (value as StoredBundle).schemaVersion === 1 &&
      Array.isArray((value as StoredBundle).sessions),
  );
  return {
    schemaVersion: 1,
    sessions: parsed.sessions.slice(-MAX_SESSIONS),
  };
}

export function saveBundle(
  store: ExperienceStoreAdapter,
  bundle: StoredBundle,
): void {
  saveJsonBundle(store, STORAGE_KEY, {
    schemaVersion: 1,
    sessions: bundle.sessions.slice(-MAX_SESSIONS).map((session) => ({
      ...session,
      events: session.events.slice(0, MAX_EVENTS_PER_SESSION),
    })),
  });
}

export function upsertSession(
  store: ExperienceStoreAdapter,
  session: ExperienceSession,
): void {
  const bundle = loadBundle(store);
  const index = bundle.sessions.findIndex((s) => s.sessionId === session.sessionId);
  const trimmed: ExperienceSession = {
    ...session,
    events: session.events.slice(0, MAX_EVENTS_PER_SESSION),
  };
  if (index >= 0) {
    bundle.sessions[index] = trimmed;
  } else {
    bundle.sessions.push(trimmed);
  }
  saveBundle(store, bundle);
}

export function clearStore(store: ExperienceStoreAdapter): void {
  clearJsonKey(store, STORAGE_KEY);
}

export function listSessions(store: ExperienceStoreAdapter): ExperienceSession[] {
  return loadBundle(store).sessions;
}
