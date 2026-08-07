/** Voice Input states (P16 / P16.5) — Conversation input device only. */

export type VoicePhase =
  | "idle"
  | "preparing"
  | "listening"
  | "transcript_ready"
  | "error";

export interface VoiceStatus {
  available: boolean;
  microphoneAvailable: boolean;
  recognitionAvailable: boolean;
  permission: string;
  message: string;
  inputState: string;
  warmed?: boolean;
}

export interface VoiceListenResult {
  ok: boolean;
  transcript?: string | null;
  status: string;
  message: string;
}
