import { useCallback, useEffect, useState } from "react";
import {
  cancelListening,
  ensureVoiceListeningBridge,
  getVoiceStatus,
  listenOnce,
  openVoiceSettings,
  warmUpVoice,
  type VoicePhase,
} from "../../lib/voice";
import {
  clearRememberedVoicePermissionGranted,
  hasRememberedVoicePermissionGranted,
  markSettingsGuidanceOffered,
  permissionGuidanceMessage,
  rememberVoicePermissionGranted,
  voiceReadyMessage,
  wasSettingsGuidanceOffered,
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
 * Never auto-opens Windows Settings (Permission Guidance Principle).
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
  const [needsSettingsOffer, setNeedsSettingsOffer] = useState(false);
  const [settingsKind, setSettingsKind] = useState<"microphone" | "speech">(
    "microphone",
  );

  const applyStatus = useCallback(
    (status: Awaited<ReturnType<typeof warmUpVoice>>, announceReady: boolean) => {
      const ok = status.available || status.recognitionAvailable;
      setAvailable(ok);
      setWarmed(Boolean(status.warmed || status.available));
      const granted =
        status.permission === "granted" ||
        (ok && status.permission !== "denied");
      if (granted) {
        const wasNew = !hasRememberedVoicePermissionGranted();
        rememberVoicePermissionGranted();
        setNeedsSettingsOffer(false);
        if (announceReady && wasNew) {
          onVoiceMessage(voiceReadyMessage());
        }
        return;
      }
      if (status.permission === "denied") {
        clearRememberedVoicePermissionGranted();
        setNeedsSettingsOffer(true);
        setSettingsKind(
          status.message.toLowerCase().includes("speech privacy")
            ? "speech"
            : "microphone",
        );
        if (!wasSettingsGuidanceOffered()) {
          markSettingsGuidanceOffered();
          onVoiceMessage(
            permissionGuidanceMessage(
              status.message.toLowerCase().includes("speech privacy")
                ? "speech"
                : "microphone",
            ),
          );
        }
      }
    },
    [onVoiceMessage],
  );

  useEffect(() => {
    let active = true;
    void ensureVoiceListeningBridge();
    void warmUpVoice().then((status) => {
      if (!active) {
        return;
      }
      applyStatus(status, false);
    });

    const onVis = () => {
      if (document.visibilityState !== "visible") {
        return;
      }
      void getVoiceStatus().then((status) => {
        if (!active) {
          return;
        }
        const wasDenied = needsSettingsOffer || !hasRememberedVoicePermissionGranted();
        const nowOk =
          status.permission === "granted" ||
          (status.available && status.permission !== "denied");
        applyStatus(status, Boolean(wasDenied && nowOk));
      });
    };
    document.addEventListener("visibilitychange", onVis);
    window.addEventListener("focus", onVis);
    return () => {
      active = false;
      document.removeEventListener("visibilitychange", onVis);
      window.removeEventListener("focus", onVis);
    };
  }, [applyStatus, needsSettingsOffer]);

  const finish = useCallback(async () => {
    await cancelListening();
  }, []);

  const start = useCallback(async () => {
    // Permission Guidance: second click while denied opens Settings once (user intent).
    if (needsSettingsOffer && !available) {
      await openVoiceSettings(settingsKind);
      onVoiceMessage(
        settingsKind === "speech"
          ? "I opened Windows Speech settings. Turn on speech recognition, then return here — I’ll check again."
          : "I opened Windows Microphone settings. Allow Workspace, then return here — I’ll check again.",
      );
      return;
    }

    setPhase("preparing");
    setSoundActive(false);

    if (!warmed) {
      const status = await warmUpVoice();
      applyStatus(status, false);
      if (!status.available && !status.recognitionAvailable) {
        setPhase("error");
        setNeedsSettingsOffer(true);
        setSettingsKind(
          status.message.toLowerCase().includes("speech privacy")
            ? "speech"
            : "microphone",
        );
        onVoiceMessage(
          permissionGuidanceMessage(
            status.message.toLowerCase().includes("speech privacy")
              ? "speech"
              : "microphone",
          ),
        );
        markSettingsGuidanceOffered();
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
        // Promote to listening shortly after speech energy.
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
        setNeedsSettingsOffer(true);
        setSettingsKind(speechPrivacy ? "speech" : "microphone");
        onVoiceMessage(
          permissionGuidanceMessage(speechPrivacy ? "speech" : "microphone"),
        );
        markSettingsGuidanceOffered();
      } else {
        onVoiceMessage(result.message);
      }
      setPhase("idle");
      return;
    }

    rememberVoicePermissionGranted();
    setNeedsSettingsOffer(false);
    setPhase("recognizing");
    await new Promise((r) => setTimeout(r, 40));
    setPhase("processing");
    onTranscript(result.transcript.trim());
    setPhase("finished");
    await new Promise((r) => setTimeout(r, 180));
    setPhase("idle");
  }, [
    applyStatus,
    available,
    needsSettingsOffer,
    onTranscript,
    onVoiceMessage,
    settingsKind,
    warmed,
  ]);

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
  const label = phaseLabel(phase, available);

  return (
    <button
      type="button"
      className="op-shell__mic"
      data-phase={phase}
      data-listening={listening ? "true" : "false"}
      data-ready={phase === "ready" ? "true" : "false"}
      data-preparing={preparing ? "true" : "false"}
      data-sound={soundActive ? "true" : "false"}
      data-available={available ? "true" : "false"}
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
                  : phase === "error" || !available
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
