/**
 * Deterministic Moment name matching for the intent bridge.
 * Never fabricates a Moment — returns null when ambiguous or missing.
 */

function normalize(input: string): string {
  return input
    .trim()
    .toLowerCase()
    .replace(/[.!?]+$/g, "")
    .replace(/[·•|_/\\]+/g, " ")
    .replace(/\s+/g, " ");
}

export interface MomentRef {
  id: string;
  name: string;
}

/**
 * Match a user-provided name fragment to a saved Moment.
 * Requires a unique best hit — ambiguity returns null.
 */
export function matchMomentByName(
  contexts: MomentRef[],
  query: string,
): MomentRef | null {
  const q = normalize(query);
  if (!q || contexts.length === 0) {
    return null;
  }

  const scored = contexts
    .map((ctx) => {
      const name = normalize(ctx.name);
      let score = 0;
      if (name === q) {
        score = 100;
      } else if (name.startsWith(q)) {
        score = 80;
      } else if (q.length >= 5 && name.includes(q)) {
        // Short fragments are too loose (e.g. "deck") — require length ≥ 5.
        score = 60;
      } else {
        const tokens = q.split(" ").filter((t) => t.length >= 5);
        if (tokens.length > 0 && tokens.every((t) => name.includes(t))) {
          score = 50;
        }
      }
      return { ctx, score };
    })
    .filter((row) => row.score > 0)
    .sort((a, b) => b.score - a.score);

  if (scored.length === 0) {
    return null;
  }
  if (scored.length === 1 || scored[0].score > scored[1].score) {
    return scored[0].ctx;
  }
  return null;
}
