/**
 * Permission Guidance Principle (P16.9) — OS permissions belong to Windows.
 * Workspace detects, explains, guides, verifies — never auto-spams Settings.
 */

const GRANTED_KEY = "workspace.voice.permissionGranted";
const OFFERED_KEY = "workspace.voice.settingsGuidanceOffered";

function storage(): Storage | null {
  try {
    return typeof localStorage !== "undefined" ? localStorage : null;
  } catch {
    return null;
  }
}

export function rememberVoicePermissionGranted(): void {
  storage()?.setItem(GRANTED_KEY, "1");
}

export function hasRememberedVoicePermissionGranted(): boolean {
  return storage()?.getItem(GRANTED_KEY) === "1";
}

export function clearRememberedVoicePermissionGranted(): void {
  storage()?.removeItem(GRANTED_KEY);
}

export function markSettingsGuidanceOffered(): void {
  storage()?.setItem(OFFERED_KEY, "1");
}

export function wasSettingsGuidanceOffered(): boolean {
  return storage()?.getItem(OFFERED_KEY) === "1";
}

export function permissionGuidanceMessage(kind: "microphone" | "speech"): string {
  if (kind === "speech") {
    return "Windows needs speech privacy turned on before I can listen. Click the microphone again and I’ll open the right Settings page for you.";
  }
  return "Workspace can’t use the microphone yet. Click the microphone again and I’ll open Windows Settings so you can allow access — then come back here.";
}

export function voiceReadyMessage(): string {
  return "✓ Voice ready";
}
