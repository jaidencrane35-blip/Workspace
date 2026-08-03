import { AnimatePresence, motion, useReducedMotion } from "motion/react";
import { useCallback, useEffect, useState } from "react";
import { createPortal } from "react-dom";
import { spring } from "../design-system";
import { invokeIpc } from "../lib/ipc";
import type {
  PilotMeasurementScope,
  PilotMeasurementSnapshot,
} from "../types/domain";
import { useActiveMoment } from "./ActiveMoment";
import { useCognitiveEngine } from "./CognitiveEngine";
import { CheckInSummaryObject } from "./objects/CheckInSummaryObject";
import { useIntentEngine } from "./IntentEngine";
import { WorkspaceSurface } from "./WorkspaceSurface";
import { useWorkspaceComposition } from "./WorkspaceComposition";

interface PilotMeasurementPanelProps {
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
}

function formatError(err: unknown): string {
  return err instanceof Error ? err.message : String(err);
}

const BASELINE_PROMPTS = [
  "About how many minutes does reconstructing a client context usually take after an interruption?",
  "What do you use today (Windows, PowerToys Workspaces, notes, something else)?",
  "Roughly how many times per day do you switch between recurring contexts?",
];

const WEEK_FOUR_PROMPTS = [
  "On how many days in week four did you use Resume?",
  "Did the handoff note help you continue, or was layout enough?",
  "Would you keep using Workspace after the pilot? Why or why not?",
];

const CHECKIN_CHAPTERS = [
  "Baseline",
  "Return",
  "Reflect",
  "Week four",
] as const;

/**
 * PP-P01E — Consented local pilot measurement and interview kit.
 * Evaluation data only; never ambient; never uploaded.
 * Presentation redesigned; measurement behaviour unchanged.
 */
export function PilotMeasurementPanel({
  busy,
  onBusy,
  onError,
  onMessage,
}: PilotMeasurementPanelProps) {
  const { density, setAttentionScene } = useWorkspaceComposition();
  const { setReflecting } = useIntentEngine();
  const { noteReflection } = useCognitiveEngine();
  const {
    primary: activeMoment,
    expandHost,
    setPresence,
    setExpanding,
  } = useActiveMoment();
  const reduceMotion = useReducedMotion();
  const [scope, setScope] = useState<PilotMeasurementScope | null>(null);
  const [snapshot, setSnapshot] = useState<PilotMeasurementSnapshot | null>(null);
  const [baselineMinutes, setBaselineMinutes] = useState("");
  const [baselineNotes, setBaselineNotes] = useState("");
  const [returnMinutes, setReturnMinutes] = useState("");
  const [correctionNeeded, setCorrectionNeeded] = useState(false);
  const [correctionNote, setCorrectionNote] = useState("");
  const [interviewBaseline, setInterviewBaseline] = useState("");
  const [interviewWeekFour, setInterviewWeekFour] = useState("");
  const [chapter, setChapter] = useState(0);
  const [historySeeded, setHistorySeeded] = useState(false);

  useEffect(() => {
    setReflecting(true);
    setAttentionScene("checkin");
    setPresence("reflecting");
    return () => setReflecting(false);
  }, [setReflecting, setAttentionScene, setPresence]);

  const advanceChapter = (from: number) => {
    setChapter(Math.min(3, from + 1));
  };

  const reload = useCallback(() => {
    onBusy(true);
    void (async () => {
      try {
        const [nextScope, nextSnapshot] = await Promise.all([
          invokeIpc<PilotMeasurementScope>("get_pilot_measurement_scope"),
          invokeIpc<PilotMeasurementSnapshot>("get_pilot_measurement_snapshot"),
        ]);
        setScope(nextScope);
        setSnapshot(nextSnapshot);
        if (nextSnapshot.baseline) {
          setBaselineMinutes(String(nextSnapshot.baseline.return_minutes));
          setBaselineNotes(nextSnapshot.baseline.notes);
        }
        if (nextSnapshot.interview_baseline) {
          setInterviewBaseline(nextSnapshot.interview_baseline.responses);
        }
        if (nextSnapshot.interview_week_four) {
          setInterviewWeekFour(nextSnapshot.interview_week_four.responses);
        }
      } catch (err: unknown) {
        onError(formatError(err));
      } finally {
        onBusy(false);
      }
    })();
  }, [onBusy, onError]);

  useEffect(() => {
    reload();
  }, [reload]);

  useEffect(() => {
    if (!snapshot) {
      return;
    }
    if (snapshot.interview_week_four) {
      noteReflection(0.88);
    } else if (snapshot.interview_baseline) {
      noteReflection(0.72);
    } else if (snapshot.leave_resume.length > 0) {
      noteReflection(0.5);
    } else if (snapshot.baseline) {
      noteReflection(0.4);
    }
  }, [snapshot, noteReflection]);

  useEffect(() => {
    if (!snapshot || historySeeded) {
      return;
    }
    const done: number[] = [];
    if (snapshot.baseline) {
      done.push(0);
    }
    if (snapshot.leave_resume.length > 0) {
      done.push(1);
    }
    if (snapshot.interview_baseline) {
      done.push(2);
    }
    if (snapshot.interview_week_four) {
      done.push(3);
    }
    if (done.length === 0) {
      return;
    }
    setChapter(done.includes(1) ? 1 : done[done.length - 1]!);
    setHistorySeeded(true);
  }, [snapshot, historySeeded]);

  const consented =
    snapshot?.consent != null &&
    snapshot.consent.withdrawn_at == null &&
    snapshot.consent.scope_id === snapshot.scope.id;

  const attachConversation = Boolean(activeMoment) && consented;

  useEffect(() => {
    setExpanding(attachConversation);
    if (attachConversation) {
      setPresence("reflecting");
    }
    return () => setExpanding(false);
  }, [attachConversation, setExpanding, setPresence]);

  const grantConsent = () => {
    if (!scope) {
      return;
    }
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        await invokeIpc("grant_pilot_consent", {
          approvedScope: scope.id,
        });
        onMessage("Pilot measurement consent recorded. Data stays on this computer.");
        reload();
      } catch (err: unknown) {
        onError(formatError(err));
        onBusy(false);
      }
    })();
  };

  const withdraw = (clearRecords: boolean) => {
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        await invokeIpc("withdraw_pilot_consent", { clearRecords });
        onMessage(
          clearRecords
            ? "Consent withdrawn and pilot records cleared."
            : "Consent withdrawn. Existing records kept until you clear them.",
        );
        reload();
      } catch (err: unknown) {
        onError(formatError(err));
        onBusy(false);
      }
    })();
  };

  const saveBaseline = () => {
    const minutes = Number(baselineMinutes);
    if (!Number.isFinite(minutes) || minutes <= 0) {
      onError("Enter baseline return-to-work minutes greater than zero.");
      return;
    }
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        await invokeIpc("record_pilot_baseline", {
          returnMinutes: Math.round(minutes),
          notes: baselineNotes,
        });
        noteReflection(0.45);
        onMessage("Baseline recorded locally.");
        reload();
      } catch (err: unknown) {
        onError(formatError(err));
        onBusy(false);
      }
    })();
  };

  const saveLeaveResume = () => {
    const minutes = Number(returnMinutes);
    if (!Number.isFinite(minutes) || minutes <= 0) {
      onError("Enter leave→resume minutes greater than zero.");
      return;
    }
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        await invokeIpc("record_pilot_leave_resume", {
          returnMinutes: Math.round(minutes),
          correctionNeeded,
          correctionNote,
          localDay: "",
        });
        setReturnMinutes("");
        setCorrectionNeeded(false);
        setCorrectionNote("");
        noteReflection(0.62);
        onMessage("Leave→resume record saved locally.");
        reload();
      } catch (err: unknown) {
        onError(formatError(err));
        onBusy(false);
      }
    })();
  };

  const saveInterview = (phase: "baseline" | "week_four", responses: string) => {
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        await invokeIpc("record_pilot_interview", { phase, responses });
        noteReflection(phase === "week_four" ? 0.9 : 0.75);
        onMessage(
          phase === "baseline"
            ? "Baseline interview notes saved locally."
            : "Week-four interview notes saved locally.",
        );
        reload();
      } catch (err: unknown) {
        onError(formatError(err));
        onBusy(false);
      }
    })();
  };

  if (!scope || !snapshot) {
    return (
      <section className="ws-region checkin-place">
        <WorkspaceSurface tone="hero" padding="lg" className="focus-card">
          <p className="exp-kicker">Check-in</p>
          <h2 className="focus-card__title">One moment…</h2>
          <p className="muted">Loading…</p>
        </WorkspaceSurface>
      </section>
    );
  }

  if (!consented) {
    return (
      <section
        className="ws-region checkin-place checkin-dash"
        data-testid="pilot-measurement-consent"
      >
        <div className="checkin-chat">
          <WorkspaceSurface tone="soft" padding="md" className="checkin-bubble">
            <p className="exp-kicker">Check-in</p>
            <p className="quote-pane__text">
              How does returning to work feel after an interruption?
            </p>
            <p className="muted">{scope.purpose}</p>
          </WorkspaceSurface>
          <details className="exp-inspect" open>
            <summary>What’s in this pulse</summary>
            <ul className="list compact">
              {scope.measured.map((item) => (
                <li key={item.key}>{item.summary}</li>
              ))}
            </ul>
            <h3>What will not be measured</h3>
            <ul className="list compact">
              {scope.not_measured.map((item) => (
                <li key={item.key}>{item.summary}</li>
              ))}
            </ul>
          </details>
          <p className="muted">
            Scope {scope.id}. Nothing is recorded until you consent. You can
            withdraw later and clear the records.
          </p>
          <div className="exp-actions">
            <button
              type="button"
              className="exp-btn primary"
              disabled={busy}
              onClick={grantConsent}
            >
              I consent to local pilot measurement
            </button>
          </div>
        </div>
      </section>
    );
  }

  const chapterLabel = CHECKIN_CHAPTERS[chapter] ?? CHECKIN_CHAPTERS[0];

  const conversation = (
    <div
      id="checkin-form"
      className="moment-attach moment-checkin checkin-narrative pilot-forms"
      role="region"
      aria-label={`${chapterLabel} form`}
      tabIndex={-1}
    >
      <p className="sr-only">{chapterLabel}</p>
      <h2 className="sr-only">{chapterLabel}</h2>
      <AnimatePresence mode="wait">
        {chapter === 0 && (
          <motion.div
            key="ch-0"
            className="checkin-chat checkin-chat--live"
            initial={reduceMotion ? false : { opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={reduceMotion ? undefined : { opacity: 0.4 }}
            transition={spring.soft}
          >
            <p className="checkin-prompt">
              Before Workspace — about how many minutes to get back?
            </p>
            <div className="checkin-bubble--you checkin-bubble--object">
              <label className="exp-field" htmlFor="pilot-baseline-minutes">
                <span>Minutes</span>
                <input
                  id="pilot-baseline-minutes"
                  className="input-wide"
                  inputMode="numeric"
                  value={baselineMinutes}
                  disabled={busy}
                  onChange={(event) => setBaselineMinutes(event.target.value)}
                />
              </label>
              <label className="exp-field" htmlFor="pilot-baseline-notes">
                <span>Notes (optional)</span>
                <textarea
                  id="pilot-baseline-notes"
                  className="input-wide"
                  rows={2}
                  value={baselineNotes}
                  disabled={busy}
                  onChange={(event) => setBaselineNotes(event.target.value)}
                />
              </label>
              <div className="exp-actions">
                <button
                  type="button"
                  className="exp-btn primary"
                  disabled={busy}
                  onClick={saveBaseline}
                >
                  Save baseline
                </button>
                <button
                  type="button"
                  className="exp-btn ghost"
                  onClick={() => advanceChapter(0)}
                >
                  Next
                </button>
              </div>
            </div>
          </motion.div>
        )}

        {chapter === 1 && (
          <motion.div
            key="ch-1"
            className="checkin-chat checkin-chat--live"
            initial={reduceMotion ? false : { opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={reduceMotion ? undefined : { opacity: 0.4 }}
            transition={spring.soft}
          >
            <p className="checkin-prompt">
              After a Continue — how many minutes to feel back?
            </p>
            <div className="checkin-bubble--you checkin-bubble--object">
              <label className="exp-field" htmlFor="pilot-return-minutes">
                <span>Minutes to return</span>
                <input
                  id="pilot-return-minutes"
                  className="input-wide"
                  inputMode="numeric"
                  value={returnMinutes}
                  disabled={busy}
                  onChange={(event) => setReturnMinutes(event.target.value)}
                />
              </label>
              <label className="exp-check">
                <input
                  type="checkbox"
                  checked={correctionNeeded}
                  disabled={busy}
                  onChange={(event) => setCorrectionNeeded(event.target.checked)}
                />
                <span>I needed to correct something after restore</span>
              </label>
              {correctionNeeded && (
                <label className="exp-field" htmlFor="pilot-correction-note">
                  <span>What did you correct?</span>
                  <textarea
                    id="pilot-correction-note"
                    className="input-wide"
                    rows={2}
                    value={correctionNote}
                    disabled={busy}
                    onChange={(event) => setCorrectionNote(event.target.value)}
                  />
                </label>
              )}
              <div className="exp-actions">
                <button
                  type="button"
                  className="exp-btn primary"
                  disabled={busy}
                  onClick={saveLeaveResume}
                >
                  Record this leave→resume
                </button>
                <button
                  type="button"
                  className="exp-btn ghost"
                  onClick={() => advanceChapter(1)}
                >
                  Next
                </button>
              </div>
            </div>
          </motion.div>
        )}

        {chapter === 2 && (
          <motion.div
            key="ch-2"
            className="checkin-chat checkin-chat--live"
            initial={reduceMotion ? false : { opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={reduceMotion ? undefined : { opacity: 0.4 }}
            transition={spring.soft}
          >
            <div className="checkin-prompt">
              <p>A few reflections — whenever you’re ready.</p>
              <ul className="list compact">
                {BASELINE_PROMPTS.map((prompt) => (
                  <li key={prompt}>{prompt}</li>
                ))}
              </ul>
            </div>
            <div className="checkin-bubble--you checkin-bubble--object">
              <textarea
                className="input-wide"
                rows={4}
                value={interviewBaseline}
                disabled={busy}
                placeholder="Write your answers here. Stored only on this computer."
                onChange={(event) => setInterviewBaseline(event.target.value)}
              />
              <div className="exp-actions">
                <button
                  type="button"
                  className="exp-btn"
                  disabled={busy}
                  onClick={() => saveInterview("baseline", interviewBaseline)}
                >
                  Save baseline interview
                </button>
                <button
                  type="button"
                  className="exp-btn ghost"
                  onClick={() => advanceChapter(2)}
                >
                  Next
                </button>
              </div>
            </div>
          </motion.div>
        )}

        {chapter === 3 && (
          <motion.div
            key="ch-3"
            className="checkin-chat checkin-chat--live"
            initial={reduceMotion ? false : { opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={reduceMotion ? undefined : { opacity: 0.4 }}
            transition={spring.soft}
          >
            <div className="checkin-prompt">
              <p>Week four — still useful?</p>
              <ul className="list compact">
                {WEEK_FOUR_PROMPTS.map((prompt) => (
                  <li key={prompt}>{prompt}</li>
                ))}
              </ul>
            </div>
            <div className="checkin-bubble--you checkin-bubble--object">
              <textarea
                className="input-wide"
                rows={4}
                value={interviewWeekFour}
                disabled={busy}
                placeholder="Write your answers here. Stored only on this computer."
                onChange={(event) => setInterviewWeekFour(event.target.value)}
              />
              <button
                type="button"
                className="exp-btn"
                disabled={busy}
                onClick={() => saveInterview("week_four", interviewWeekFour)}
              >
                Save week-four interview
              </button>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );

  return (
    <section
      className="ws-region checkin-place checkin-dash checkin-dash--story checkin-dash--conversation checkin-place--object"
      data-testid="pilot-measurement-active"
      data-density={density}
      aria-labelledby="checkin-title"
    >
      <header className="sr-only">
        <p>Check-in</p>
        <h1 id="checkin-title">How’s the return feeling?</h1>
        <p>
          Local pilot pulse only — they are not saved contexts and are not sent
          anywhere. Section {chapter + 1} of {CHECKIN_CHAPTERS.length}:{" "}
          {chapterLabel}.
        </p>
      </header>

      <nav className="sr-only" aria-label="Check-in sections">
        <ol>
          {CHECKIN_CHAPTERS.map((label, index) => (
            <li key={label}>
              <a
                href="#checkin-form"
                aria-current={chapter === index ? "step" : undefined}
                onClick={(event) => {
                  event.preventDefault();
                  setChapter(index);
                  document.getElementById("checkin-form")?.focus();
                }}
              >
                {label}
                {chapter === index ? " (current)" : ""}
              </a>
            </li>
          ))}
        </ol>
        <a href="#checkin-evidence">Skip to pulse evidence</a>
      </nav>

      {attachConversation
        ? expandHost
          ? createPortal(conversation, expandHost)
          : null
        : conversation}

      <aside
        id="checkin-evidence"
        className="checkin-evidence checkin-evidence--annotation"
        aria-label="Pulse evidence"
        tabIndex={-1}
      >
        <details className="checkin-evidence__fold">
          <summary>Pulse</summary>
          {(snapshot.interview_baseline || snapshot.interview_week_four) && (
            <p className="checkin-story checkin-story--settled checkin-story--inline">
              {(
                snapshot.interview_week_four?.responses ||
                snapshot.interview_baseline?.responses ||
                ""
              ).trim()}
            </p>
          )}
          <div className="checkin-metrics checkin-metrics--quiet checkin-metrics--ambient">
          <CheckInSummaryObject
            id="checkin-baseline"
            label="Was"
            value={
              snapshot.baseline
                ? `${snapshot.baseline.return_minutes}`
                : "—"
            }
            unit="min"
            state="idle"
          />
          <CheckInSummaryObject
            id="checkin-leave"
            label="Notes"
            value={`${snapshot.leave_resume.length}`}
            unit="returns"
            state="idle"
          />
          <CheckInSummaryObject
            id="checkin-median"
            label="Now"
            value={
              snapshot.median_return_minutes != null
                ? `${snapshot.median_return_minutes}`
                : "—"
            }
            unit="min"
            hero
            state="expanded"
          />
          </div>
        {snapshot.leave_resume.length > 0 && (
          <div
            className="checkin-trend checkin-trend--quiet"
            data-testid="checkin-trend"
          >
            <div
              className="checkin-trend__bars"
              role="img"
              aria-label="Minutes to return across recent leave→resume records"
            >
              {[...snapshot.leave_resume]
                .slice()
                .reverse()
                .map((record) => {
                  const max = Math.max(
                    ...snapshot.leave_resume.map((r) => r.return_minutes),
                    snapshot.baseline?.return_minutes ?? 0,
                    1,
                  );
                  const height = Math.max(
                    8,
                    Math.round((record.return_minutes / max) * 40),
                  );
                  return (
                    <div key={record.id} className="checkin-trend__col">
                      <span
                        className="checkin-trend__bar"
                        style={{ height }}
                        title={`${record.return_minutes} min · ${record.local_day}`}
                      />
                    </div>
                  );
                })}
            </div>
          </div>
        )}
        </details>
      </aside>

      <details className="exp-inspect checkin-withdraw checkin-withdraw--quiet">
        <summary>Withdraw</summary>
        <div className="exp-actions">
          <button
            type="button"
            className="exp-btn ghost"
            disabled={busy}
            onClick={() => withdraw(false)}
          >
            Keep records
          </button>
          <button
            type="button"
            className="exp-btn ghost"
            disabled={busy}
            onClick={() => withdraw(true)}
          >
            Clear records
          </button>
        </div>
      </details>
    </section>
  );
}
