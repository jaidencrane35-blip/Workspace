import { useCallback, useEffect, useState } from "react";
import { invokeIpc } from "../lib/ipc";
import type {
  PilotMeasurementScope,
  PilotMeasurementSnapshot,
} from "../types/domain";

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
 */
export function PilotMeasurementPanel({
  busy,
  onBusy,
  onError,
  onMessage,
}: PilotMeasurementPanelProps) {
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
      <section className="assistant-hero">
        <p className="assistant-kicker">Pilot</p>
        <h2>Pilot measurement</h2>
        <p className="muted">Loading…</p>
      </section>
    );
  }

  if (!consented) {
    return (
      <section className="assistant-hero" data-testid="pilot-measurement-consent">
        <p className="assistant-kicker">Pilot</p>
        <h2>Measure the pilot — only with your consent</h2>
        <p className="lede">{scope.purpose}</p>

        <section>
          <h3>What will be measured</h3>
          <ul className="list compact">
            {scope.measured.map((item) => (
              <li key={item.key}>{item.summary}</li>
            ))}
          </ul>
        </section>

        <section>
          <h3>What will not be measured</h3>
          <ul className="list compact">
            {scope.not_measured.map((item) => (
              <li key={item.key}>{item.summary}</li>
            ))}
          </ul>
        </section>

        <p className="muted">
          Scope {scope.id}. Nothing is recorded until you consent. You can withdraw
          later and clear the records.
        </p>

        <div className="row">
          <button type="button" disabled={busy} onClick={grantConsent}>
            I consent to local pilot measurement
          </button>
        </div>
      </section>
    );
  }

  return (
    <section className="assistant-hero" data-testid="pilot-measurement-active">
      <p className="assistant-kicker">Pilot</p>
      <h2>Local pilot evidence</h2>
      <p className="lede">
        These records evaluate the Product Proof hypothesis. They are not saved
        contexts and are not sent anywhere.
      </p>

      <section>
        <h3>Summary</h3>
        <ul className="list compact">
          <li>
            Baseline minutes:{" "}
            {snapshot.baseline
              ? snapshot.baseline.return_minutes
              : "not recorded yet"}
          </li>
          <li>
            Leave→resume records: {snapshot.leave_resume.length} · distinct days:{" "}
            {snapshot.distinct_resume_days}
          </li>
          <li>
            Median leave→resume minutes:{" "}
            {snapshot.median_return_minutes ?? "n/a"}
          </li>
        </ul>
        <p className="muted">
          LEDGER-0013 looks for median return-to-work reduction versus baseline,
          and week-four habit (Resume on at least three distinct days).
        </p>
      </section>

      <section>
        <h3>Baseline (before Workspace)</h3>
        <p className="muted">
          Your estimate of minutes to return to work without Workspace.
        </p>
        <div className="row">
          <label htmlFor="pilot-baseline-minutes">Minutes</label>
          <input
            id="pilot-baseline-minutes"
            className="input-wide"
            inputMode="numeric"
            value={baselineMinutes}
            disabled={busy}
            onChange={(event) => setBaselineMinutes(event.target.value)}
          />
        </div>
        <div className="row">
          <label htmlFor="pilot-baseline-notes">Notes (optional)</label>
          <textarea
            id="pilot-baseline-notes"
            className="input-wide"
            rows={2}
            value={baselineNotes}
            disabled={busy}
            onChange={(event) => setBaselineNotes(event.target.value)}
          />
        </div>
        <button type="button" disabled={busy} onClick={saveBaseline}>
          Save baseline
        </button>
      </section>

      <section>
        <h3>Leave→resume record</h3>
        <p className="muted">
          After you use Resume, enter how many minutes it took to get back to
          work, and whether you needed to correct anything. Nothing is inferred.
        </p>
        <div className="row">
          <label htmlFor="pilot-return-minutes">Minutes to return</label>
          <input
            id="pilot-return-minutes"
            className="input-wide"
            inputMode="numeric"
            value={returnMinutes}
            disabled={busy}
            onChange={(event) => setReturnMinutes(event.target.value)}
          />
        </div>
        <div className="row">
          <label>
            <input
              type="checkbox"
              checked={correctionNeeded}
              disabled={busy}
              onChange={(event) => setCorrectionNeeded(event.target.checked)}
            />{" "}
            I needed to correct something after restore
          </label>
        </div>
        {correctionNeeded && (
          <div className="row">
            <label htmlFor="pilot-correction-note">What did you correct?</label>
            <textarea
              id="pilot-correction-note"
              className="input-wide"
              rows={2}
              value={correctionNote}
              disabled={busy}
              onChange={(event) => setCorrectionNote(event.target.value)}
            />
          </div>
        )}
        <button type="button" disabled={busy} onClick={saveLeaveResume}>
          Record this leave→resume
        </button>
      </section>

      <section>
        <h3>Baseline interview kit</h3>
        <ul className="list compact">
          {BASELINE_PROMPTS.map((prompt) => (
            <li key={prompt}>{prompt}</li>
          ))}
        </ul>
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
          disabled={busy}
          onClick={() => saveInterview("baseline", interviewBaseline)}
        >
          Save baseline interview
        </button>
      </section>

      <section>
        <h3>Week-four interview kit</h3>
        <ul className="list compact">
          {WEEK_FOUR_PROMPTS.map((prompt) => (
            <li key={prompt}>{prompt}</li>
          ))}
        </ul>
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
          disabled={busy}
          onClick={() => saveInterview("week_four", interviewWeekFour)}
        >
          Save week-four interview
        </button>
      </section>

      <section>
        <h3>Withdraw consent</h3>
        <div className="button-row">
          <button type="button" disabled={busy} onClick={() => withdraw(false)}>
            Withdraw consent (keep records)
          </button>
          <button type="button" disabled={busy} onClick={() => withdraw(true)}>
            Withdraw and clear pilot records
          </button>
        </div>
      </section>
    </section>
  );
}
