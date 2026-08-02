import { useCallback, useEffect, useState } from "react";
import { invokeIpc } from "../lib/ipc";
import type {
  PilotMeasurementScope,
  PilotMeasurementSnapshot,
} from "../types/domain";
import { ElevatedCard } from "./ElevatedCard";
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
  const { density } = useWorkspaceComposition();
  const [scope, setScope] = useState<PilotMeasurementScope | null>(null);
  const [snapshot, setSnapshot] = useState<PilotMeasurementSnapshot | null>(null);
  const [baselineMinutes, setBaselineMinutes] = useState("");
  const [baselineNotes, setBaselineNotes] = useState("");
  const [returnMinutes, setReturnMinutes] = useState("");
  const [correctionNeeded, setCorrectionNeeded] = useState(false);
  const [correctionNote, setCorrectionNote] = useState("");
  const [interviewBaseline, setInterviewBaseline] = useState("");
  const [interviewWeekFour, setInterviewWeekFour] = useState("");

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
        <ElevatedCard tone="hero" padding="lg" className="focus-card">
          <p className="exp-kicker">Check-in</p>
          <h2 className="focus-card__title">One moment…</h2>
          <p className="muted">Loading…</p>
        </ElevatedCard>
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
          <ElevatedCard tone="soft" padding="md" className="checkin-bubble">
            <p className="exp-kicker">Check-in</p>
            <p className="quote-pane__text">
              How does returning to work feel after an interruption?
            </p>
            <p className="muted" style={{ marginTop: "0.5rem" }}>
              {scope.purpose}
            </p>
          </ElevatedCard>
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
          These records evaluate the Product Proof hypothesis. They are not saved
          contexts and are not sent anywhere.
        </p>
      </header>

      <div className="checkin-metrics">
        <ElevatedCard
          tone="hero"
          elevation={3}
          padding="lg"
          className="metric-orb"
          layout
        >
          <p className="exp-kicker">Baseline</p>
          <p className="exp-stat metric-orb__value">
            {snapshot.baseline
              ? `${snapshot.baseline.return_minutes}`
              : "—"}
          </p>
          <p className="muted">min</p>
        </ElevatedCard>
        <ElevatedCard tone="soft" elevation={2} padding="lg" className="metric-orb">
          <p className="exp-kicker">Leave → resume</p>
          <p className="exp-stat metric-orb__value">
            {snapshot.leave_resume.length}
          </p>
          <p className="muted">{snapshot.distinct_resume_days} days</p>
        </ElevatedCard>
        <ElevatedCard tone="soft" elevation={2} padding="lg" className="metric-orb">
          <p className="exp-kicker">Median</p>
          <p className="exp-stat metric-orb__value">
            {snapshot.median_return_minutes != null
              ? `${snapshot.median_return_minutes}`
              : "n/a"}
          </p>
          <p className="muted">min</p>
        </ElevatedCard>
      </div>

      <div className="checkin-chat pilot-forms">
        <ElevatedCard tone="soft" padding="md" className="checkin-bubble">
          <p className="quote-pane__text" style={{ fontSize: "1rem" }}>
            Before Workspace — about how many minutes to get back?
          </p>
        </ElevatedCard>
        <ElevatedCard tone="solid" padding="md" className="checkin-bubble--you">
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
          <button
            type="button"
            className="exp-btn primary"
            disabled={busy}
            onClick={saveBaseline}
          >
            Save baseline
          </button>
        </ElevatedCard>

        <ElevatedCard tone="soft" padding="md" className="checkin-bubble">
          <p className="quote-pane__text" style={{ fontSize: "1rem" }}>
            After a Continue — how many minutes to feel back?
          </p>
        </ElevatedCard>
        <ElevatedCard tone="solid" padding="md" className="checkin-bubble--you">
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
          <button
            type="button"
            className="exp-btn primary"
            disabled={busy}
            onClick={saveLeaveResume}
          >
            Record this leave→resume
          </button>
        </ElevatedCard>

        <ElevatedCard tone="soft" padding="md" className="checkin-bubble">
          <p className="quote-pane__text" style={{ fontSize: "1rem" }}>
            A few reflections — whenever you’re ready.
          </p>
          <ul className="list compact">
            {BASELINE_PROMPTS.map((prompt) => (
              <li key={prompt}>{prompt}</li>
            ))}
          </ul>
        </ElevatedCard>
        <ElevatedCard tone="solid" padding="md" className="checkin-bubble--you">
          <textarea
            className="input-wide"
            rows={4}
            value={interviewBaseline}
            disabled={busy}
            placeholder="Write your answers here. Stored only on this computer."
            onChange={(event) => setInterviewBaseline(event.target.value)}
          />
          <button
            type="button"
            className="exp-btn"
            disabled={busy}
            onClick={() => saveInterview("baseline", interviewBaseline)}
          >
            Save baseline interview
          </button>
        </ElevatedCard>

        <ElevatedCard tone="soft" padding="md" className="checkin-bubble">
          <p className="quote-pane__text" style={{ fontSize: "1rem" }}>
            Week four — still useful?
          </p>
          <ul className="list compact">
            {WEEK_FOUR_PROMPTS.map((prompt) => (
              <li key={prompt}>{prompt}</li>
            ))}
          </ul>
        </ElevatedCard>
        <ElevatedCard tone="solid" padding="md" className="checkin-bubble--you">
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
        </ElevatedCard>
      </div>

      <ElevatedCard tone="default" padding="md">
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
      </ElevatedCard>
    </section>
  );
}
