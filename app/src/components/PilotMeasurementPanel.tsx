import { AnimatePresence, motion, useReducedMotion } from "motion/react";
import { useCallback, useEffect, useState } from "react";
import { spring } from "../design-system";
import { invokeIpc } from "../lib/ipc";
import type {
  PilotMeasurementScope,
  PilotMeasurementSnapshot,
} from "../types/domain";
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
  const {
    density,
    setAttentionScene,
    setPrimaryObject,
    setSecondaryObjects,
  } = useWorkspaceComposition();
  const { setReflecting } = useIntentEngine();
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
  const [completed, setCompleted] = useState<number[]>([]);
  const [historySeeded, setHistorySeeded] = useState(false);

  useEffect(() => {
    setReflecting(true);
    setAttentionScene("checkin");
    const primary =
      chapter === 0
        ? "checkin-baseline"
        : chapter === 1
          ? "checkin-leave"
          : "checkin-median";
    setPrimaryObject(primary);
    setSecondaryObjects(
      ["checkin-baseline", "checkin-leave", "checkin-median"].filter(
        (id) => id !== primary,
      ),
    );
    return () => setReflecting(false);
  }, [
    chapter,
    setReflecting,
    setAttentionScene,
    setPrimaryObject,
    setSecondaryObjects,
  ]);

  const advanceChapter = (from: number) => {
    setCompleted((prev) =>
      prev.includes(from) ? prev : [...prev, from],
    );
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
    setCompleted(done);
    setChapter(done.includes(1) ? 1 : done[done.length - 1]!);
    setHistorySeeded(true);
  }, [snapshot, historySeeded]);

  const consented =
    snapshot?.consent != null &&
    snapshot.consent.withdrawn_at == null &&
    snapshot.consent.scope_id === snapshot.scope.id;

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
      <section className="spatial-frame spatial-frame--center">
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
        className="spatial-frame checkin-dash"
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

  return (
    <section
      className="spatial-frame checkin-dash"
      data-testid="pilot-measurement-active"
      data-density={density}
    >
      <header className="spatial-header">
        <p className="exp-kicker">Check-in</p>
        <h1 className="spatial-title">How’s the return feeling?</h1>
        <p className="spatial-summary">
          Local pilot pulse only — they are not saved contexts and are not sent
          anywhere.
        </p>
      </header>

      <div className="checkin-metrics ws-compose__float">
        <CheckInSummaryObject
          id="checkin-baseline"
          label="Baseline"
          value={
            snapshot.baseline
              ? `${snapshot.baseline.return_minutes}`
              : "—"
          }
          unit="min"
          hero
          state="expanded"
        />
        <CheckInSummaryObject
          id="checkin-leave"
          label="Leave → resume"
          value={`${snapshot.leave_resume.length}`}
          unit={`${snapshot.distinct_resume_days} days`}
          state="idle"
        />
        <CheckInSummaryObject
          id="checkin-median"
          label="Median"
          value={
            snapshot.median_return_minutes != null
              ? `${snapshot.median_return_minutes}`
              : "n/a"
          }
          unit="min"
          state="idle"
        />
      </div>

      {snapshot.leave_resume.length > 0 && (
        <WorkspaceSurface
          tone="soft"
          padding="md"
          className="checkin-trend"
          data-testid="checkin-trend"
        >
          <p className="exp-kicker">Return trend</p>
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
                  12,
                  Math.round((record.return_minutes / max) * 72),
                );
                return (
                  <div key={record.id} className="checkin-trend__col">
                    <span
                      className="checkin-trend__bar"
                      style={{ height }}
                      title={`${record.return_minutes} min · ${record.local_day}`}
                    />
                    <span className="checkin-trend__day">
                      {record.local_day.slice(5)}
                    </span>
                  </div>
                );
              })}
          </div>
          <ul className="list compact checkin-trend__history">
            {snapshot.leave_resume.slice(0, 4).map((record) => (
              <li key={`hist-${record.id}`}>
                <strong>{record.return_minutes} min</strong>
                <span className="muted">
                  {" "}
                  · {record.local_day}
                  {record.correction_needed
                    ? ` · corrected: ${record.correction_note || "yes"}`
                    : ""}
                </span>
              </li>
            ))}
          </ul>
          {snapshot.interview_baseline && (
            <div className="checkin-trend__reflection">
              <p className="exp-kicker">Earlier reflection</p>
              <p className="quote-pane__text">
                {snapshot.interview_baseline.responses}
              </p>
            </div>
          )}
        </WorkspaceSurface>
      )}

      <div className="checkin-narrative pilot-forms attention-field">
        <div
          className="checkin-spatial-trail"
          role="tablist"
          aria-label="Check-in chapters"
        >
          {["Baseline", "Return", "Reflect", "Week four"].map((label, index) => {
            const done = completed.includes(index);
            const active = chapter === index;
            return (
              <button
                key={label}
                type="button"
                role="tab"
                aria-selected={active}
                className={[
                  "checkin-spatial-chip",
                  active ? "is-active" : "",
                  done ? "is-done" : "",
                ]
                  .filter(Boolean)
                  .join(" ")}
                onClick={() => setChapter(index)}
              >
                <span className="checkin-spatial-chip__mark" aria-hidden="true" />
                <span className="checkin-spatial-chip__label">{label}</span>
              </button>
            );
          })}
        </div>

        <AnimatePresence mode="wait">
          {chapter === 0 && (
            <motion.div
              key="ch-0"
              className="checkin-chat"
              initial={reduceMotion ? false : { opacity: 0, x: 24 }}
              animate={{ opacity: 1, x: 0 }}
              exit={reduceMotion ? undefined : { opacity: 0, x: -16 }}
              transition={spring.soft}
            >
              <WorkspaceSurface tone="soft" padding="md" className="checkin-bubble">
                <p className="quote-pane__text">
                  Before Workspace — about how many minutes to get back?
                </p>
              </WorkspaceSurface>
              <WorkspaceSurface tone="solid" padding="md" className="checkin-bubble--you">
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
              </WorkspaceSurface>
            </motion.div>
          )}


          {chapter === 1 && (
            <motion.div
              key="ch-1"
              className="checkin-chat"
              initial={reduceMotion ? false : { opacity: 0, x: 24 }}
              animate={{ opacity: 1, x: 0 }}
              exit={reduceMotion ? undefined : { opacity: 0, x: -16 }}
              transition={spring.soft}
            >
              <WorkspaceSurface tone="soft" padding="md" className="checkin-bubble">
                <p className="quote-pane__text">
                  After a Continue — how many minutes to feel back?
                </p>
              </WorkspaceSurface>
              <WorkspaceSurface tone="solid" padding="md" className="checkin-bubble--you">
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
              </WorkspaceSurface>
            </motion.div>
          )}

          {chapter === 2 && (
            <motion.div
              key="ch-2"
              className="checkin-chat"
              initial={reduceMotion ? false : { opacity: 0, x: 24 }}
              animate={{ opacity: 1, x: 0 }}
              exit={reduceMotion ? undefined : { opacity: 0, x: -16 }}
              transition={spring.soft}
            >
              <WorkspaceSurface tone="soft" padding="md" className="checkin-bubble">
                <p className="quote-pane__text">
                  A few reflections — whenever you’re ready.
                </p>
                <ul className="list compact">
                  {BASELINE_PROMPTS.map((prompt) => (
                    <li key={prompt}>{prompt}</li>
                  ))}
                </ul>
              </WorkspaceSurface>
              <WorkspaceSurface tone="solid" padding="md" className="checkin-bubble--you">
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
              </WorkspaceSurface>
            </motion.div>
          )}

          {chapter === 3 && (
            <motion.div
              key="ch-3"
              className="checkin-chat"
              initial={reduceMotion ? false : { opacity: 0, x: 24 }}
              animate={{ opacity: 1, x: 0 }}
              exit={reduceMotion ? undefined : { opacity: 0, x: -16 }}
              transition={spring.soft}
            >
              <WorkspaceSurface tone="soft" padding="md" className="checkin-bubble">
                <p className="quote-pane__text">
                  Week four — still useful?
                </p>
                <ul className="list compact">
                  {WEEK_FOUR_PROMPTS.map((prompt) => (
                    <li key={prompt}>{prompt}</li>
                  ))}
                </ul>
              </WorkspaceSurface>
              <WorkspaceSurface tone="solid" padding="md" className="checkin-bubble--you">
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
              </WorkspaceSurface>
            </motion.div>
          )}
        </AnimatePresence>
      </div>

      <WorkspaceSurface tone="default" padding="md">
        <h3>Withdraw consent</h3>
        <div className="exp-actions">
          <button
            type="button"
            className="exp-btn"
            disabled={busy}
            onClick={() => withdraw(false)}
          >
            Withdraw consent (keep records)
          </button>
          <button
            type="button"
            className="exp-btn ghost"
            disabled={busy}
            onClick={() => withdraw(true)}
          >
            Withdraw and clear pilot records
          </button>
        </div>
      </WorkspaceSurface>
    </section>
  );
}
