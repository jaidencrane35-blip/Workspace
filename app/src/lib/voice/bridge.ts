/**
 * Voice Input bridge — Conversation input device (P16.7).
 * Never calls Capability Providers or Kernel Operator for recognition.
 */

import { listen } from "@tauri-apps/api/event";
import { IpcCommandError, invokeIpc } from "../ipc";
import { isTauriRuntime } from "../shellRuntime";
import type { VoiceListenResult, VoiceStatus } from "./types";

const DEMO_STATUS: VoiceStatus = {
  available: true,
  microphoneAvailable: true,
  recognitionAvailable: true,
  permission: "granted",
  message: "Voice is set up — click the microphone to speak.",
  inputState: "idle",
  warmed: true,
};

const SPEECH_PRIVACY_MESSAGE =
  "Windows needs speech privacy turned on before I can listen. Click the microphone once and I’ll open the right Settings page — then come back here.";

const MICROPHONE_PERMISSION_MESSAGE =
  "Workspace can’t use the microphone yet. Click the microphone once and I’ll open Windows Settings so you can allow access — then come back here.";

const LISTEN_RETRY_MESSAGE =
  "I couldn’t listen just now. Try the microphone again.";

/** Strip technical IPC / OS detail before Conversation shows a voice error. */
export function desktopVoiceMessage(raw: string): string {
  const lower = raw.toLowerCase();
  if (
    lower.includes("privacy policy") ||
    lower.includes("privacy statement") ||
    lower.includes("0x80045509") ||
    (lower.includes("speech privacy") && lower.includes("0x"))
  ) {
    return SPEECH_PRIVACY_MESSAGE;
  }
  if (
    lower.includes("microphone") &&
    (lower.includes("denied") || lower.includes("can’t use") || lower.includes("can't use"))
  ) {
    return MICROPHONE_PERMISSION_MESSAGE;
  }
  if (
    /0x[0-9a-f]{8}/i.test(raw) ||
    /recognize\s*:|winrt|speechrecognizer|hresult|provider/i.test(raw)
  ) {
    return LISTEN_RETRY_MESSAGE;
  }
  return raw.trim() || LISTEN_RETRY_MESSAGE;
}

/**
 * Hoisted off the listen hot path — subscribing to events before each invoke
 * previously delayed ContinuousRecognitionSession start and dropped leading speech.
 * P16.27: voice-ready + voice-sound only (voice-listening removed — unused dual path).
 */
let listeningBridge: Promise<(() => void) | null> | null = null;
const readyCallbacks = new Set<() => void>();
const soundCallbacks = new Set<() => void>();

export async function ensureVoiceListeningBridge(): Promise<void> {
  if (!isTauriRuntime()) {
    return;
  }
  if (!listeningBridge) {
    listeningBridge = (async () => {
      const unsubs: Array<() => void> = [];
      try {
        unsubs.push(
          await listen("voice-ready", () => {
            for (const cb of readyCallbacks) {
              try {
                cb();
              } catch {
                /* ignore */
              }
            }
          }),
        );
        unsubs.push(
          await listen("voice-sound", () => {
            for (const cb of soundCallbacks) {
              try {
                cb();
              } catch {
                /* ignore */
              }
            }
          }),
        );
        return () => {
          for (const u of unsubs) u();
        };
      } catch {
        return null;
      }
    })();
  }
  await listeningBridge;
}

export async function getVoiceStatus(): Promise<VoiceStatus> {
  if (!isTauriRuntime()) {
    return DEMO_STATUS;
  }
  try {
    const status = await invokeIpc<VoiceStatus>("voice_status");
    return { ...status, message: desktopVoiceMessage(status.message) };
  } catch (error) {
    const message =
      error instanceof IpcCommandError
        ? desktopVoiceMessage(error.message)
        : "Voice input isn’t available right now.";
    return {
      available: false,
      microphoneAvailable: false,
      recognitionAvailable: false,
      permission: "unavailable",
      message,
      inputState: "idle",
      warmed: false,
    };
  }
}

/** Pre-warm the speech engine so the next mic click can listen immediately. */
export async function warmUpVoice(): Promise<VoiceStatus> {
  if (!isTauriRuntime()) {
    return DEMO_STATUS;
  }
  try {
    await ensureVoiceListeningBridge();
    const status = await invokeIpc<VoiceStatus>("voice_warm_up");
    return { ...status, message: desktopVoiceMessage(status.message) };
  } catch {
    return getVoiceStatus();
  }
}

/**
 * After the user returns from Windows Settings — clear peek cache and re-probe once.
 * Never use peek-only status here (stale Denied would block grant paths).
 */
export async function recheckVoicePermission(): Promise<VoiceStatus> {
  if (!isTauriRuntime()) {
    return DEMO_STATUS;
  }
  try {
    const status = await invokeIpc<VoiceStatus>("voice_recheck_permission");
    return { ...status, message: desktopVoiceMessage(status.message) };
  } catch {
    return getVoiceStatus();
  }
}

export type ListenOnceHooks = {
  onReady?: () => void;
  onSoundStarted?: () => void;
};

/**
 * Continuous listen turn (Conversation Continuity).
 * Ends when the user finishes speaking (long silence), toggles the mic (Stop),
 * or a genuine recognition error occurs — never on a short mid-speech pause.
 * `onReady` fires only after WinRT Capturing (trustworthy contract).
 * `onSoundStarted` fires when WinRT reports speech energy (SoundStarted).
 */
export async function listenOnce(
  hooks: ListenOnceHooks = {},
): Promise<VoiceListenResult> {
  if (!isTauriRuntime()) {
    return {
      ok: false,
      transcript: null,
      status: "unavailable",
      message:
        "Voice input needs the Workspace app. Open Workspace to speak to Conversation.",
    };
  }

  await ensureVoiceListeningBridge();
  if (hooks.onReady) readyCallbacks.add(hooks.onReady);
  if (hooks.onSoundStarted) soundCallbacks.add(hooks.onSoundStarted);

  try {
    const result = await invokeIpc<VoiceListenResult>("voice_listen_once");
    return { ...result, message: desktopVoiceMessage(result.message) };
  } catch (error) {
    const raw =
      error instanceof IpcCommandError
        ? error.message
        : "I couldn’t listen just now.";
    const message = desktopVoiceMessage(raw);
    const status =
      message === SPEECH_PRIVACY_MESSAGE || message === MICROPHONE_PERMISSION_MESSAGE
        ? "permission_denied"
        : "recognition_failed";
    // Permission Guidance: never auto-open Settings from the bridge.
    return {
      ok: false,
      transcript: null,
      status,
      message,
    };
  } finally {
    // Defer teardown so late Tauri emits after IPC return still paint Ready/Listening (R36).
    const ready = hooks.onReady;
    const sound = hooks.onSoundStarted;
    globalThis.setTimeout(() => {
      if (ready) readyCallbacks.delete(ready);
      if (sound) soundCallbacks.delete(sound);
    }, 120);
  }
}

export async function cancelListening(): Promise<void> {
  if (!isTauriRuntime()) {
    return;
  }
  try {
    await invokeIpc<void>("voice_cancel");
  } catch {
    // Best-effort cancel.
  }
}

/** Returns true only when Settings URI launch succeeded (P16.27 — never lie). */
export async function openVoiceSettings(
  target: "microphone" | "speech" = "microphone",
): Promise<boolean> {
  if (!isTauriRuntime()) {
    return false;
  }
  try {
    await invokeIpc<void>("voice_open_settings", { target });
    return true;
  } catch {
    return false;
  }
}
