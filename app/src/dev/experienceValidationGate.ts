/**
 * Development-only Experience Validation gate.
 * Production builds stay off unless explicitly forced for local validation.
 */

export function isExperienceValidationEnabled(): boolean {
  if (import.meta.env.PROD) {
    return import.meta.env.VITE_ENABLE_EXPERIENCE_VALIDATION === "1";
  }
  if (import.meta.env.VITE_DISABLE_EXPERIENCE_VALIDATION === "1") {
    return false;
  }
  return true;
}
