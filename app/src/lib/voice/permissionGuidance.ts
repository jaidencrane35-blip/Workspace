/**
 * Permission Guidance Principle (P16.9) + permanent permission architecture (P16.12).
 *
 * OS permissions belong to Windows. Workspace:
 *   detect → explain once → open Settings once (user click) → remain running →
 *   re-check on return → remember grant → never interrupt future launches.
 *
 * Never auto-open Settings on launch, warm-up, or focus.
 */

const GRANTED_KEY = "workspace.voice.permissionGranted";
const DENIED_GUIDANCE_KEY = "workspace.voice.deniedGuidanceOffered";

export type VoicePermissionGate =
  | "idle"
  | "explain"
  | "awaiting_return"
  | "still_denied";

/** Session-only gate — Settings open is per deny cycle. */
let gate: VoicePermissionGate = "idle";
let explainAnnounced = false;
let settingsKind: "microphone" | "speech" = "microphone";

function storage(): Storage | null {
  try {
    return typeof localStorage !== "undefined" ? localStorage : null;
  } catch {
    return null;
  }
}

export function rememberVoicePermissionGranted(): void {
  storage()?.setItem(GRANTED_KEY, "1");
  storage()?.removeItem(DENIED_GUIDANCE_KEY);
  gate = "idle";
  explainAnnounced = false;
}

export function hasRememberedVoicePermissionGranted(): boolean {
  return storage()?.getItem(GRANTED_KEY) === "1";
}

export function clearRememberedVoicePermissionGranted(): void {
  storage()?.removeItem(GRANTED_KEY);
}

export function rememberVoicePermissionDenied(): void {
  clearRememberedVoicePermissionGranted();
}

export function currentSettingsKind(): "microphone" | "speech" {
  return settingsKind;
}

/** Call when OS reports denied / unavailable mic or speech privacy. */
export function notePermissionDenied(kind: "microphone" | "speech"): {
  announce: boolean;
  message: string;
} {
  settingsKind = kind;
  rememberVoicePermissionDenied();
  if (gate === "awaiting_return") {
    // User returned from Settings but still denied — one fresh explain, then one more open.
    gate = "still_denied";
    explainAnnounced = false;
  } else if (gate !== "still_denied") {
    gate = "explain";
  }
  const alreadyGuided = storage()?.getItem(DENIED_GUIDANCE_KEY) === "1";
  const announce = !explainAnnounced && !alreadyGuided;
  explainAnnounced = true;
  if (announce) {
    storage()?.setItem(DENIED_GUIDANCE_KEY, "1");
  }
  return { announce, message: permissionGuidanceMessage(kind) };
}

/** User clicked mic to open Settings — only once per cycle. */
export function noteSettingsOpened(): void {
  gate = "awaiting_return";
}

export function shouldOpenSettingsOnMicClick(): boolean {
  return gate === "explain" || gate === "still_denied";
}

export function isAwaitingSettingsReturn(): boolean {
  return gate === "awaiting_return";
}

/** Successful grant from recheck or successful listen. */
export function notePermissionGranted(): void {
  rememberVoicePermissionGranted();
}

/**
 * Settings return with MediaCapture/warm looking OK — but listen must still prove
 * speech privacy / real capture. Clears awaiting without stamping a grant (P16.20).
 */
export function noteSettingsReturnNeedsListenConfirm(): void {
  gate = "idle";
  explainAnnounced = false;
}

export function settingsReturnConfirmMessage(): string {
  return "If you updated permissions, click the microphone to confirm I can listen.";
}

export function permissionGuidanceMessage(kind: "microphone" | "speech"): string {
  if (kind === "speech") {
    return "Windows needs speech privacy turned on before I can listen. Click the microphone once and I’ll open the right Settings page — then come back here.";
  }
  return "Workspace can’t use the microphone yet. Click the microphone once and I’ll open Windows Settings so you can allow access — then come back here.";
}

export function settingsOpenedMessage(kind: "microphone" | "speech"): string {
  if (kind === "speech") {
    return "I opened Windows Speech settings. Turn on online speech recognition, then return here — I’ll check again automatically.";
  }
  return "I opened Windows Microphone settings. Allow Workspace, then return here — I’ll check again automatically.";
}

export function awaitingReturnMessage(): string {
  return "Finish the permission in Windows Settings, then return here — I’ll check again. I won’t keep reopening Settings.";
}

/**
 * After permission is remembered — mic is still Idle until Capturing Ready.
 * Never claim "Voice ready" while the control is not in Ready phase (P16.24).
 */
export function voicePermissionSetMessage(): string {
  return "Permission set — click the microphone to speak.";
}
