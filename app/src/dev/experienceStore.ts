/**
 * Local-only ring buffer for Experience Validation sessions.
 * Never transmits. Never stores user content.
 */

import type { ExperienceSession } from "./experienceEvents";

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
  const raw = store.getItem(STORAGE_KEY);
  if (!raw) {
    return emptyBundle();
  }
  try {
    const parsed = JSON.parse(raw) as StoredBundle;
    if (parsed?.schemaVersion !== 1 || !Array.isArray(parsed.sessions)) {
      return emptyBundle();
    }
    return {
      schemaVersion: 1,
      sessions: parsed.sessions.slice(-MAX_SESSIONS),
    };
  } catch {
    return emptyBundle();
  }
}

export function saveBundle(
  store: ExperienceStoreAdapter,
  bundle: StoredBundle,
): void {
  const next: StoredBundle = {
    schemaVersion: 1,
    sessions: bundle.sessions.slice(-MAX_SESSIONS).map((session) => ({
      ...session,
      events: session.events.slice(0, MAX_EVENTS_PER_SESSION),
    })),
  };
  store.setItem(STORAGE_KEY, JSON.stringify(next));
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
  store.removeItem(STORAGE_KEY);
}

export function listSessions(store: ExperienceStoreAdapter): ExperienceSession[] {
  return loadBundle(store).sessions;
}
