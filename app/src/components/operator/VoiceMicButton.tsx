import { useCallback, useEffect, useState } from "react";
import {
  cancelListening,
  ensureVoiceListeningBridge,
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

function phaseLabel(phase: VoicePhase, available: boolean): string {
  if (!available) {
    return "Voice unavailable";
  }
  switch (phase) {
    case "preparing":
      return "Getting ready…";
    case "listening":
      return "Listening — speak now";
    case "recognizing":
      return "Recognizing…";
    case "processing":
      return "Processing…";
    case "finished":
      return "Done";
    case "error":
      return "Voice error";
    default:
      return "Speak to Workspace";
  }
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
  const [warmed, setWarmed] = useState(false);

  useEffect(() => {
    let active = true;
    void ensureVoiceListeningBridge();
    void warmUpVoice().then((status) => {
      if (!active) {
        return;
      }
      setAvailable(status.available || status.recognitionAvailable);
      setWarmed(Boolean(status.warmed || status.available));
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
    setPhase("preparing");

    // Finish warm-up before RecognizeAsync so the first spoken words are not
    // lost during cold SpeechRecognizer create/compile.
    if (!warmed) {
      const status = await warmUpVoice();
      setAvailable(status.available || status.recognitionAvailable);
      setWarmed(Boolean(status.warmed || status.available));
      if (!status.available && !status.recognitionAvailable) {
        setPhase("error");
        onVoiceMessage(status.message);
        setPhase("idle");
        return;
      }
    }

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

    setPhase("recognizing");
    await new Promise((r) => setTimeout(r, 40));
    setPhase("processing");
    onTranscript(result.transcript.trim());
    setPhase("finished");
    await new Promise((r) => setTimeout(r, 180));
    setPhase("idle");
  }, [onTranscript, onVoiceMessage, warmed]);

  const onToggle = () => {
    if (disabled) {
      return;
    }
    if (
      phase === "preparing" ||
      phase === "listening" ||
      phase === "recognizing" ||
      phase === "processing"
    ) {
      void stop();
      return;
    }
    void start();
  };

  const listening = phase === "listening";
  const preparing = phase === "preparing";
  const busy =
    preparing ||
    listening ||
    phase === "recognizing" ||
    phase === "processing";
  const label = phaseLabel(phase, available);

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
      aria-busy={busy}
    >
      <span className="op-shell__mic-icon" aria-hidden="true">
        {listening
          ? "●"
          : preparing
            ? "◌"
            : phase === "recognizing" || phase === "processing"
              ? "◎"
              : phase === "finished"
                ? "✓"
                : "◉"}
      </span>
      {listening && (
        <>
          <span className="op-shell__mic-pulse" aria-hidden="true" />
          <span className="op-shell__mic-wave" aria-hidden="true">
            <i />
            <i />
            <i />
            <i />
          </span>
        </>
      )}
    </button>
  );
}
