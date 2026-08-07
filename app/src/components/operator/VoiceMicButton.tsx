import { useCallback, useEffect, useRef, useState } from "react";
import {
  cancelListening,
  ensureVoiceListeningBridge,
  listenOnce,
  openVoiceSettings,
  recheckVoicePermission,
  warmUpVoice,
  type VoicePhase,
} from "../../lib/voice";
import {
  awaitingReturnMessage,
  currentSettingsKind,
  hasRememberedVoicePermissionGranted,
  isAwaitingSettingsReturn,
  notePermissionDenied,
  notePermissionGranted,
  noteSettingsOpened,
  settingsOpenedMessage,
  shouldOpenSettingsOnMicClick,
  voiceReadyMessage,
} from "../../lib/voice/permissionGuidance";

interface VoiceMicButtonProps {
  disabled?: boolean;
  onTranscript: (transcript: string) => void;
  onVoiceMessage: (message: string) => void;
}

function phaseLabel(phase: VoicePhase, available: boolean): string {
  if (!available) {
    return "Voice unavailable — click for Settings help";
  }
  switch (phase) {
    case "preparing":
      return "Preparing…";
    case "ready":
      return "Ready — speak when you like";
    case "speechDetected":
      return "Speech detected";
    case "listening":
      return "Listening";
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
 * Ready = Capturing contract. Listening = speech detected.
 * Permission architecture: explain once → Settings once → recheck on return → remember.
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
  const [deniedUi, setDeniedUi] = useState(false);
  const onVoiceMessageRef = useRef(onVoiceMessage);
  onVoiceMessageRef.current = onVoiceMessage;

  const applyStatus = useCallback(
    (
      status: Awaited<ReturnType<typeof warmUpVoice>>,
      announceReady: boolean,
    ) => {
      const ok = status.available || status.recognitionAvailable;
      setAvailable(ok);
      setWarmed(Boolean(status.warmed || status.available));
      const granted =
        status.permission === "granted" ||
        (ok && status.permission !== "denied");
      if (granted) {
        const wasNew = !hasRememberedVoicePermissionGranted();
        notePermissionGranted();
        setDeniedUi(false);
        if (announceReady && wasNew) {
          onVoiceMessageRef.current(voiceReadyMessage());
        }
        return;
      }
      if (status.permission === "denied") {
        const kind = status.message.toLowerCase().includes("speech privacy")
          ? "speech"
          : "microphone";
        const { announce, message } = notePermissionDenied(kind);
        setDeniedUi(true);
        if (announce) {
          onVoiceMessageRef.current(message);
        }
      }
    },
    [],
  );

  useEffect(() => {
    let active = true;
    void ensureVoiceListeningBridge();
    void warmUpVoice().then((status) => {
      if (!active) {
        return;
      }
      // Remembered grant: warm silently; never re-open Settings on launch.
      applyStatus(status, false);
    });

    const onVis = () => {
      if (document.visibilityState !== "visible") {
        return;
      }
      if (!isAwaitingSettingsReturn()) {
        return;
      }
      void recheckVoicePermission().then((status) => {
        if (!active) {
          return;
        }
        applyStatus(status, true);
      });
    };
    document.addEventListener("visibilitychange", onVis);
    window.addEventListener("focus", onVis);
    return () => {
      active = false;
      document.removeEventListener("visibilitychange", onVis);
      window.removeEventListener("focus", onVis);
    };
  }, [applyStatus]);

  const finish = useCallback(async () => {
    await cancelListening();
  }, []);

  const start = useCallback(async () => {
    // Already opened Settings — do not spam; wait for return + recheck.
    if (isAwaitingSettingsReturn()) {
      onVoiceMessage(awaitingReturnMessage());
      return;
    }

    // Permission architecture: one Settings open per deny cycle (user click).
    if (shouldOpenSettingsOnMicClick()) {
      const kind = currentSettingsKind();
      noteSettingsOpened();
      setDeniedUi(true);
      await openVoiceSettings(kind);
      onVoiceMessage(settingsOpenedMessage(kind));
      return;
    }

    setPhase("preparing");
    setSoundActive(false);

    if (!warmed) {
      const status = await warmUpVoice();
      applyStatus(status, false);
      if (status.permission === "denied" || (!status.available && !status.recognitionAvailable)) {
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
        setPhase((prev) =>
          prev === "ready" || prev === "preparing" ? "speechDetected" : prev,
        );
        window.setTimeout(() => {
          setPhase((prev) =>
            prev === "speechDetected" || prev === "ready" ? "listening" : prev,
          );
        }, 40);
      },
    });

    setSoundActive(false);

    if (!result.ok || !result.transcript?.trim()) {
      if (result.status === "cancelled") {
        setPhase("idle");
        return;
      }
      setPhase("error");
      if (
        result.status === "permission_denied" ||
        result.status === "microphone_unavailable"
      ) {
        const speechPrivacy = result.message.toLowerCase().includes("speech privacy");
        const { announce, message } = notePermissionDenied(
          speechPrivacy ? "speech" : "microphone",
        );
        setDeniedUi(true);
        if (announce) {
          onVoiceMessage(message);
        }
      } else {
        onVoiceMessage(result.message);
      }
      setPhase("idle");
      return;
    }

    notePermissionGranted();
    setDeniedUi(false);
    setPhase("recognizing");
    await new Promise((r) => setTimeout(r, 40));
    setPhase("processing");
    onTranscript(result.transcript.trim());
    setPhase("finished");
    await new Promise((r) => setTimeout(r, 180));
    setPhase("idle");
  }, [applyStatus, onTranscript, onVoiceMessage, warmed]);

  const onToggle = () => {
    if (disabled) {
      return;
    }
    if (
      phase === "preparing" ||
      phase === "ready" ||
      phase === "speechDetected" ||
      phase === "listening" ||
      phase === "recognizing" ||
      phase === "processing"
    ) {
      void finish();
      return;
    }
    void start();
  };

  const activeCapture =
    phase === "ready" ||
    phase === "speechDetected" ||
    phase === "listening";
  const listening =
    phase === "speechDetected" || phase === "listening";
  const preparing = phase === "preparing";
  const busy =
    preparing ||
    activeCapture ||
    phase === "recognizing" ||
    phase === "processing";
  const label = phaseLabel(phase, available && !deniedUi);

  return (
    <button
      type="button"
      className="op-shell__mic"
      data-phase={phase}
      data-listening={listening ? "true" : "false"}
      data-ready={phase === "ready" ? "true" : "false"}
      data-preparing={preparing ? "true" : "false"}
      data-sound={soundActive ? "true" : "false"}
      data-available={available && !deniedUi ? "true" : "false"}
      onClick={onToggle}
      disabled={disabled}
      aria-label={label}
      title={label}
      aria-pressed={activeCapture}
      aria-busy={busy}
    >
      <span className="op-shell__mic-icon" aria-hidden="true">
        {listening
          ? "●"
          : phase === "ready"
            ? "◉"
            : preparing
              ? "◌"
              : phase === "recognizing" || phase === "processing"
                ? "◎"
                : phase === "finished"
                  ? "✓"
                  : phase === "error" || deniedUi || !available
                    ? "!"
                    : "◉"}
      </span>
      {activeCapture && (
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
