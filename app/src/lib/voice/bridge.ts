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
  message: "Voice input is available.",
  inputState: "idle",
};

export async function getVoiceStatus(): Promise<VoiceStatus> {
  if (!isTauriRuntime()) {
    return DEMO_STATUS;
  }
  try {
    return await invokeIpc<VoiceStatus>("voice_status");
  } catch (error) {
    const message =
      error instanceof IpcCommandError
        ? error.message
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
    return await invokeIpc<VoiceListenResult>("voice_listen_once");
  } catch (error) {
    return {
      ok: false,
      transcript: null,
      status: "recognition_failed",
      message:
        error instanceof IpcCommandError
          ? error.message
          : "I couldn’t listen just now.",
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
