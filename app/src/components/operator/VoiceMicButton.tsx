import { useCallback, useEffect, useState } from "react";
import {
  cancelListening,
  listenOnce,
  openVoiceSettings,
  warmUpVoice,
  type VoicePhase,
} from "../../lib/voice";

interface VoiceMicButtonProps {
  disabled?: boolean;
  onTranscript: (transcript: string) => void;
  onVoiceMessage: (message: string) => void;
}

/**
 * Microphone control beside the Conversation composer.
 * Push-to-talk via click (start) / click again (stop). Single utterance.
 *
 * Listening indicator is shown only after the recognizer is actually capturing —
 * never during engine warm-up / create / compile.
 */
export function VoiceMicButton({
  disabled,
  onTranscript,
  onVoiceMessage,
}: VoiceMicButtonProps) {
  const [phase, setPhase] = useState<VoicePhase>("idle");
  const [available, setAvailable] = useState(true);

  useEffect(() => {
    let active = true;
    void warmUpVoice().then((status) => {
      if (!active) {
        return;
      }
      setAvailable(status.available || status.recognitionAvailable);
      if (status.permission === "denied") {
        onVoiceMessage(status.message);
      }
    });
    return () => {
      active = false;
    };
  }, [onVoiceMessage]);

  const stop = useCallback(async () => {
    await cancelListening();
    setPhase("idle");
  }, []);

  const start = useCallback(async () => {
    // Preparing ≠ listening. Do not pulse until capture starts.
    // Skip status IPC on the hot path — it previously delayed RecognizeAsync
    // and caused the first spoken words to be lost.
    setPhase("preparing");

    const result = await listenOnce(() => {
      setPhase("listening");
    });

    if (!result.ok || !result.transcript?.trim()) {
      setPhase("error");
      onVoiceMessage(result.message);
      if (
        result.status === "permission_denied" ||
        result.status === "microphone_unavailable"
      ) {
        const speechPrivacy = result.message.toLowerCase().includes("speech privacy");
        await openVoiceSettings(speechPrivacy ? "speech" : "microphone");
      }
      setPhase("idle");
      return;
    }
    setPhase("transcript_ready");
    onTranscript(result.transcript.trim());
    setPhase("idle");
  }, [onTranscript, onVoiceMessage]);

  const onToggle = () => {
    if (disabled) {
      return;
    }
    if (phase === "preparing" || phase === "listening") {
      void stop();
      return;
    }
    void start();
  };

  const listening = phase === "listening";
  const preparing = phase === "preparing";
  const label = !available
    ? "Voice unavailable"
    : preparing
      ? "Getting ready…"
      : listening
        ? "Stop listening"
        : "Speak to Workspace";

  return (
    <button
      type="button"
      className="op-shell__mic"
      data-phase={phase}
      data-listening={listening ? "true" : "false"}
      data-preparing={preparing ? "true" : "false"}
      data-available={available ? "true" : "false"}
      onClick={onToggle}
      disabled={disabled}
      aria-label={label}
      title={label}
      aria-pressed={listening}
      aria-busy={preparing}
    >
      <span className="op-shell__mic-icon" aria-hidden="true">
        {listening ? "●" : preparing ? "◌" : "◉"}
      </span>
      {listening && (
        <span className="op-shell__mic-pulse" aria-hidden="true" />
      )}
    </button>
  );
}
