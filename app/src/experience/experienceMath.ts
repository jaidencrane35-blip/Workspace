/**
 * Sprint 72 — Shared deterministic math for Experience presentation pipeline.
 * Single source of truth for clamp / round / lerp / variance helpers.
 */

export function clamp01(n: number): number {
  return Math.min(1, Math.max(0, n));
}

export function round4(n: number): number {
  return Number(n.toFixed(4));
}

export function lerp(a: number, b: number, t: number): number {
  return a + (b - a) * clamp01(t);
}

export function populationVariance(values: readonly number[]): number {
  if (values.length === 0) {
    return 0;
  }
  const mean = values.reduce((a, b) => a + b, 0) / values.length;
  const sumSq = values.reduce((a, b) => a + (b - mean) ** 2, 0);
  return sumSq / values.length;
}

export function meanOf(values: readonly number[]): number | null {
  if (values.length === 0) {
    return null;
  }
  return round4(values.reduce((a, b) => a + b, 0) / values.length);
}
