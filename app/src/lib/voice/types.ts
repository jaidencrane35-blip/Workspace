/** Voice Input states (P16.9 Ready contract) — Conversation input device only. */

export type VoicePhase =
  | "idle"
  | "preparing"
  | "ready"
  | "speechDetected"
  | "listening"
  | "recognizing"
  /** Owner is reviewing transcript in the composer (F10) — not Kernel processing. */
  | "reviewing"
  | "processing"
  | "finished"
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
