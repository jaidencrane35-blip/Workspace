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
      return "Preparing microphone…";
    case "ready":
      return "Ready — Workspace is listening";
    case "listening":
      return "Listening — speak naturally";
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
 * Listening / Ready indicators appear only after WinRT Capturing —
 * never during warm-up or before the audio contract is established.
 */
export function VoiceMicButton({
  disabled,
  onTranscript,
  onVoiceMessage,
}: VoiceMicButtonProps) {
  const [phase, setPhase] = useState<VoicePhase>("idle");
  const [available, setAvailable] = useState(true);
  const [warmed, setWarmed] = useState(false);
  const [soundActive, setSoundActive] = useState(false);

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

  /** Mic toggle while listening = finish turn (like submitting typed text). */
  const finish = useCallback(async () => {
    await cancelListening();
  }, []);

  const start = useCallback(async () => {
    setPhase("preparing");
    setSoundActive(false);

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

    const result = await listenOnce({
      onReady: () => {
        setPhase("ready");
      },
      onListening: () => {
        setPhase("listening");
      },
      onSoundStarted: () => {
        setSoundActive(true);
      },
    });

    setSoundActive(false);

    if (!result.ok || !result.transcript?.trim()) {
      if (result.status === "cancelled") {
        setPhase("idle");
        return;
      }
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
      phase === "ready" ||
      phase === "listening" ||
      phase === "recognizing" ||
      phase === "processing"
    ) {
      void finish();
      return;
    }
    void start();
  };

  const capturing = phase === "ready" || phase === "listening";
  const preparing = phase === "preparing";
  const busy =
    preparing ||
    capturing ||
    phase === "recognizing" ||
    phase === "processing";
  const label = phaseLabel(phase, available);

  return (
    <button
      type="button"
      className="op-shell__mic"
      data-phase={phase}
      data-listening={capturing ? "true" : "false"}
      data-preparing={preparing ? "true" : "false"}
      data-sound={soundActive ? "true" : "false"}
      data-available={available ? "true" : "false"}
      onClick={onToggle}
      disabled={disabled}
      aria-label={label}
      title={label}
      aria-pressed={capturing}
      aria-busy={busy}
    >
      <span className="op-shell__mic-icon" aria-hidden="true">
        {capturing
          ? "●"
          : preparing
            ? "◌"
            : phase === "recognizing" || phase === "processing"
              ? "◎"
              : phase === "finished"
                ? "✓"
                : "◉"}
      </span>
      {capturing && (
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
