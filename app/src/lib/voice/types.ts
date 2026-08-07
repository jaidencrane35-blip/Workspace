/** Voice Input states (P16) — Conversation input device only. */

export type VoicePhase =
  | "idle"
  | "listening"
  | "recognizing"
  | "transcript_ready"
  | "error";

export interface VoiceStatus {
  available: boolean;
  microphoneAvailable: boolean;
  recognitionAvailable: boolean;
  permission: string;
  message: string;
  inputState: string;
}

export interface VoiceListenResult {
  ok: boolean;
  transcript?: string | null;
  status: string;
  message: string;
}
