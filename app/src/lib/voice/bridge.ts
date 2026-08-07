/**
 * Voice Input bridge — Conversation input device (P16).
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
  message: "Voice is ready.",
  inputState: "idle",
  warmed: true,
};

const SPEECH_PRIVACY_MESSAGE =
  "Windows needs speech privacy turned on before I can listen. Open Settings → Privacy & security → Speech, turn on Online speech recognition, then try again.";

const MICROPHONE_PERMISSION_MESSAGE =
  "Workspace can’t use the microphone yet. Open Settings → Privacy & security → Microphone, allow access for Workspace, then try again.";

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
    return "I couldn’t listen just now. Check that a microphone is connected and try again.";
  }
  return raw;
}

/**
 * Hoisted off the listen hot path — subscribing to events before each invoke
 * previously delayed RecognizeAsync and dropped leading speech.
 */
let listeningBridge: Promise<(() => void) | null> | null = null;
const listeningCallbacks = new Set<() => void>();

export async function ensureVoiceListeningBridge(): Promise<void> {
  if (!isTauriRuntime()) {
    return;
  }
  if (!listeningBridge) {
    listeningBridge = listen("voice-listening", () => {
      for (const cb of listeningCallbacks) {
        try {
          cb();
        } catch {
          // Ignore listener faults — recognition must continue.
        }
      }
    })
      .then((unlisten) => unlisten)
      .catch(() => null);
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
 * Single-utterance listen. `onListening` fires only when capture has started
 * (never during engine create/compile).
 */
export async function listenOnce(
  onListening?: () => void,
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
  if (onListening) {
    listeningCallbacks.add(onListening);
  }

  try {
    // Hot path: invoke only — event subscription is already mounted.
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
    if (status === "permission_denied") {
      void openVoiceSettings(
        message === SPEECH_PRIVACY_MESSAGE ? "speech" : "microphone",
      );
    }
    return {
      ok: false,
      transcript: null,
      status,
      message,
    };
  } finally {
    if (onListening) {
      listeningCallbacks.delete(onListening);
    }
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

export async function openVoiceSettings(
  target: "microphone" | "speech" = "microphone",
): Promise<void> {
  if (!isTauriRuntime()) {
    return;
  }
  try {
    await invokeIpc<void>("voice_open_settings", { target });
  } catch {
    // Best-effort settings launch.
  }
}
