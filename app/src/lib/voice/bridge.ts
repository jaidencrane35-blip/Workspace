/**
 * Voice Input bridge — Conversation input device (P16).
 * Never calls Capability Providers or Kernel Operator for recognition.
 */

import { IpcCommandError, invokeIpc } from "../ipc";
import { isTauriRuntime } from "../shellRuntime";
import type { VoiceListenResult, VoiceStatus } from "./types";

const DEMO_STATUS: VoiceStatus = {
  available: true,
  microphoneAvailable: true,
  recognitionAvailable: true,
  permission: "granted",
  message: "Voice can listen after Windows speech privacy is allowed.",
  inputState: "idle",
};

const SPEECH_PRIVACY_MESSAGE =
  "Windows needs speech privacy turned on before I can listen. Open Settings → Privacy & security → Speech, turn on Online speech recognition, then try again.";

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
    /0x[0-9a-f]{8}/i.test(raw) ||
    /recognize\s*:|winrt|speechrecognizer|hresult|provider/i.test(raw)
  ) {
    return "I couldn’t listen just now. Check that a microphone is connected and try again.";
  }
  return raw;
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
    };
  }
}

export async function listenOnce(): Promise<VoiceListenResult> {
  if (!isTauriRuntime()) {
    return {
      ok: false,
      transcript: null,
      status: "unavailable",
      message:
        "Voice input needs the Workspace app. Open Workspace to speak to Conversation.",
    };
  }
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
      message === SPEECH_PRIVACY_MESSAGE
        ? "permission_denied"
        : "recognition_failed";
    return {
      ok: false,
      transcript: null,
      status,
      message,
    };
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
