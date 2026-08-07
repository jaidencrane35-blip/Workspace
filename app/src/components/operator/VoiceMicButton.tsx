import { useCallback, useEffect, useState } from "react";
import {
  cancelListening,
  getVoiceStatus,
  listenOnce,
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
    void getVoiceStatus().then((status) => {
      if (active) {
        setAvailable(status.available || status.recognitionAvailable);
      }
    });
    return () => {
      active = false;
    };
  }, []);

  const stop = useCallback(async () => {
    await cancelListening();
    setPhase("idle");
  }, []);

  const start = useCallback(async () => {
    setPhase("listening");
    const status = await getVoiceStatus();
    if (!status.available && !status.recognitionAvailable) {
      setPhase("error");
      onVoiceMessage(status.message);
      setPhase("idle");
      return;
    }
    if (status.permission === "denied") {
      setPhase("error");
      onVoiceMessage(status.message);
      setPhase("idle");
      return;
    }

    setPhase("recognizing");
    const result = await listenOnce();
    if (!result.ok || !result.transcript?.trim()) {
      setPhase("error");
      onVoiceMessage(result.message);
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
    if (phase === "listening" || phase === "recognizing") {
      void stop();
      return;
    }
    void start();
  };

  const listening = phase === "listening" || phase === "recognizing";
  const label = !available
    ? "Voice unavailable"
    : listening
      ? "Stop listening"
      : "Speak to Workspace";

  return (
    <button
      type="button"
      className="op-shell__mic"
      data-phase={phase}
      data-listening={listening ? "true" : "false"}
      data-available={available ? "true" : "false"}
      onClick={onToggle}
      disabled={disabled}
      aria-label={label}
      title={label}
      aria-pressed={listening}
    >
      <span className="op-shell__mic-icon" aria-hidden="true">
        {listening ? "●" : "◉"}
      </span>
      {listening && (
        <span className="op-shell__mic-pulse" aria-hidden="true" />
      )}
    </button>
  );
}
