/**
 * Sprint 65 — Shared development-only persistence access.
 * No domain imports. Keys and schemas owned by callers.
 */

import type { ExperienceStoreAdapter } from "./experienceStore";

export type { ExperienceStoreAdapter } from "./experienceStore";

/**
 * Load a schemaVersion:1 JSON bundle. On miss/invalid parse, returns empty().
 * Does not change key names or on-disk shape — callers own serialization shape.
 */
export function loadJsonBundleOrNull<T extends { schemaVersion: 1 }>(
  store: ExperienceStoreAdapter,
  key: string,
  isValid: (parsed: unknown) => parsed is T,
): T | null {
  const raw = store.getItem(key);
  if (!raw) {
    return null;
  }
  try {
    const parsed: unknown = JSON.parse(raw);
    if (!isValid(parsed)) {
      return null;
    }
    return parsed;
  } catch {
    return null;
  }
}

export function loadJsonBundle<T extends { schemaVersion: 1 }>(
  store: ExperienceStoreAdapter,
  key: string,
  empty: () => T,
  isValid: (parsed: unknown) => parsed is T,
): T {
  return loadJsonBundleOrNull(store, key, isValid) ?? empty();
}

export function saveJsonBundle(
  store: ExperienceStoreAdapter,
  key: string,
  value: unknown,
): void {
  store.setItem(key, JSON.stringify(value));
}

export function clearJsonKey(
  store: ExperienceStoreAdapter,
  key: string,
): void {
  store.removeItem(key);
}
