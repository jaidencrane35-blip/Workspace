import { useCallback, useEffect, useState } from "react";
import { createPortal } from "react-dom";
import { invokeIpc } from "../lib/ipc";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";
import type {
  ActionOperationResult,
  ActionPlanItem,
  ResumePlanPreview,
  SavedContext,
  SavedContextWindow,
  Workspace,
} from "../types/domain";
import { useActiveMoment } from "./ActiveMoment";
import { useCognitiveEngine } from "./CognitiveEngine";
import { EmptyStructure } from "./EmptyStructure";
import { ContinuePreviewBody } from "./objects/ContinuePreviewObject";
import { RestoreLimitsNotice } from "./RestoreLimitsNotice";
import { useIntentEngine } from "./IntentEngine";
import { WorkspaceSurface } from "./WorkspaceSurface";

type Step = "browse" | "inspect" | "confirm_delete" | "preview" | "done";

interface ResumeContextPanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
  onGoToPilot?: () => void;
  onGoHome?: () => void;
  focusContextId?: string | null;
}

function formatError(err: unknown): string {
  return err instanceof Error ? err.message : String(err);
}

function formatMoment(iso: string): string {
  const at = new Date(iso);
  return Number.isNaN(at.getTime()) ? iso : at.toLocaleString();
}

function describeWindow(window: SavedContextWindow): string {
  const parts = [
    window.minimized ? "minimised" : `${window.width}×${window.height}`,
  ];
  if (window.focused) {
    parts.push("you were working here");
  }
  if (window.monitor_index !== null) {
    parts.push(`monitor ${window.monitor_index + 1}`);
  }
  parts.push(`process ${window.process_id}`);
  return parts.join(" · ");
}

function describeDisposition(item: ActionPlanItem): string {
  switch (item.projected_disposition) {
    case "will_attempt":
      return "";
    case "will_skip_unsupported":
      return "Stays as-is";
    case "will_skip_unresolvable":
      return "Needs a click";
    default:
      return "";
  }
}

function describeOutcome(disposition: string): string {
  switch (disposition) {
    case "completed":
      return "Restored";
    case "failed":
      return "Failed";
    case "skipped_unsupported":
      return "Unsupported";
    case "skipped_unresolvable":
      return "Could not restore";
    case "refused_changed":
      return "Changed since preview";
    case "not_attempted":
      return "Not attempted";
    case "outcome_unknown":
      return "Outcome unknown";
    default:
      return disposition;
  }
}

export function ResumeContextPanel({
  workspace,
  busy,
  onBusy,
  onError,
  onMessage,
  onGoToPilot,
  onGoHome,
  focusContextId = null,
}: ResumeContextPanelProps) {
  const { setRestoring } = useIntentEngine();
  const { noteResume } = useCognitiveEngine();
  const {
    primary,
    expandHost,
    selectMoment,
    setPresence,
    setExpanding,
    reloadMoments,
  } = useActiveMoment();
  const [step, setStep] = useState<Step>("browse");
  const [inspected, setInspected] = useState<SavedContext | null>(null);
  const [preview, setPreview] = useState<ResumePlanPreview | null>(null);
  const [result, setResult] = useState<ActionOperationResult | null>(null);

  const openInspect = (contextId: string) => {
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        const context = await invokeIpc<SavedContext>("get_saved_context", {
          savedContextId: contextId,
        });
        setInspected(context);
        setPreview(null);
        setResult(null);
        setStep("inspect");
        setExpanding(false);
        setPresence("presence");
      } catch (err: unknown) {
        onError(formatError(err));
      } finally {
        onBusy(false);
      }
    })();
  };

  const openPreview = useCallback(
    (contextId: string) => {
      onBusy(true);
      onError(null);
      selectMoment(contextId);
      noteResume(contextId);
      void (async () => {
        try {
          const next = await invokeIpc<ResumePlanPreview>(
            "resolve_resume_plan",
            { savedContextId: contextId },
          );
          setPreview(next);
          setResult(null);
          setStep("preview");
          setRestoring(true);
          setPresence("restoring");
          setExpanding(true);
        } catch (err: unknown) {
          onError(formatError(err));
        } finally {
          onBusy(false);
        }
      })();
    },
    [
      onBusy,
      onError,
      selectMoment,
      noteResume,
      setRestoring,
      setPresence,
      setExpanding,
    ],
  );

  useEffect(() => {
    if (!focusContextId || !workspace) {
      return;
    }
    openPreview(focusContextId);
  }, [focusContextId, workspace, openPreview]);

  const approveAndRestore = () => {
    if (!preview) {
      return;
    }
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        const outcome = await invokeIpc<ActionOperationResult>(
          "execute_resume_plan",
          {
            plan: preview.plan,
            approvedPlanDigest: preview.plan.plan_digest,
          },
        );
        setResult(outcome);
        setStep("done");
        setExpanding(false);
        setPresence("presence");
        onMessage(`Resume finished: ${outcome.outcome.replace(/_/g, " ")}`);
      } catch (err: unknown) {
        onError(formatError(err));
      } finally {
        onBusy(false);
      }
    })();
  };

  const confirmDelete = () => {
    if (!inspected) {
      return;
    }
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        const deletedName = inspected.name;
        await invokeIpc<null>("delete_saved_context", {
          savedContextId: inspected.id,
        });
        setInspected(null);
        setPreview(null);
        setResult(null);
        setStep("browse");
        reloadMoments();
        onMessage(`Deleted “${deletedName}”. It can no longer be restored.`);
      } catch (err: unknown) {
        onError(formatError(err));
      } finally {
        onBusy(false);
      }
    })();
  };

  const backToBrowse = () => {
    setStep("browse");
    setInspected(null);
    setPreview(null);
    setResult(null);
    setRestoring(false);
    setPresence("presence");
    setExpanding(false);
    reloadMoments();
  };

  useEffect(() => {
    const open = step === "preview" && Boolean(preview);
    setExpanding(open);
    if (open) {
      setPresence("restoring");
    } else if (step === "browse") {
      setPresence("presence");
    }
    return () => setExpanding(false);
  }, [step, preview, setExpanding, setPresence]);

  if (!workspace) {
    return (
      <section className="ws-region continue-place continue-place--empty">
        <WorkspaceSurface level="floating" tone="hero" padding="xl" className="focus-card">
          <p className="exp-kicker">Continue</p>
          <h2 className="focus-card__title">Open your workspace</h2>
          <p className="muted">Then pick up where you left off.</p>
          {onGoHome && (
            <div className="exp-actions">
              <button type="button" className="exp-btn primary" onClick={onGoHome}>
                Go to Home
              </button>
            </div>
          )}
        </WorkspaceSurface>
      </section>
    );
  }

  if (!primary && step === "browse") {
    return (
      <section className="ws-region continue-place">
        <EmptyStructure
          title="Nothing to continue yet"
          hint="Save a moment — then it waits here for you."
        />
      </section>
    );
  }

  return (
    <section
      className={[
        "ws-region",
        "continue-place",
        step === "preview" ? "continue-place--inside" : "",
      ]
        .filter(Boolean)
        .join(" ")}
    >
      {(step === "browse" || step === "preview") && (
        <>
          {step === "browse" && (
            <p className="sr-only">
              Continue — step back in through the Moment above. Remember this
              place via Continue on the object.
            </p>
          )}
          {step === "browse" && primary && (
            <details className="exp-inspect continue-inspect-recess continue-inspect-recess--quiet">
              <summary>Inspect</summary>
              <button
                type="button"
                className="exp-btn ghost continue-inspect-entry"
                disabled={busy}
                onClick={() => openInspect(primary.id)}
              >
                Open details
              </button>
              <p className="muted">{RESTORE_LIMITS_SUMMARY}</p>
            </details>
          )}
          {step === "preview" && preview && expandHost
            ? createPortal(
                <ContinuePreviewBody
                  preview={preview}
                  busy={busy}
                  onApprove={approveAndRestore}
                  onCancel={backToBrowse}
                  onInspect={() => openInspect(preview.saved_context_id)}
                  describeDisposition={describeDisposition}
                />,
                expandHost,
              )
            : null}
        </>
      )}

      {step === "inspect" && inspected && (
        <WorkspaceSurface
          level="overlay"
          tone="hero"
          padding="xl"
          className="focus-card focus-card--center"
        >
          <h2 className="focus-card__title">{inspected.name}</h2>
          <p className="exp-intention">
            {inspected.handoff_note.trim()
              ? inspected.handoff_note
              : "No handoff was recorded with this context."}
          </p>
          <p className="muted">
            Shown exactly as you wrote it. Saved{" "}
            {formatMoment(inspected.created_at)}.
          </p>
          <details className="exp-inspect">
            <summary>Window and monitor details</summary>
            <p className="muted">Scope {inspected.approved_scope}</p>
            <ul className="list compact">
              {inspected.windows.map((window) => (
                <li key={window.id}>
                  <div>{window.title}</div>
                  <div className="muted">{describeWindow(window)}</div>
                </li>
              ))}
            </ul>
            <h4>
              {inspected.monitors.length}{" "}
              {inspected.monitors.length === 1 ? "monitor" : "monitors"}
            </h4>
            <ul className="list compact">
              {inspected.monitors.map((monitor) => (
                <li key={monitor.id}>
                  <div>
                    {monitor.name || `Monitor ${monitor.monitor_index + 1}`}
                    {monitor.is_primary ? " · main" : ""}
                  </div>
                  <div className="muted">
                    {monitor.width}×{monitor.height} at {monitor.x},{monitor.y}
                  </div>
                </li>
              ))}
            </ul>
          </details>
          <RestoreLimitsNotice />
          <div className="exp-actions">
            <button
              type="button"
              className="exp-btn primary"
              disabled={busy}
              onClick={() => openPreview(inspected.id)}
            >
              Preview restore
            </button>
            <button
              type="button"
              className="exp-btn"
              disabled={busy}
              onClick={backToBrowse}
            >
              Back
            </button>
            <button
              type="button"
              className="exp-btn ghost"
              disabled={busy}
              onClick={() => setStep("confirm_delete")}
            >
              Delete this context
            </button>
          </div>
        </WorkspaceSurface>
      )}

      {step === "confirm_delete" && inspected && (
        <WorkspaceSurface tone="hero" padding="lg" className="focus-card focus-card--center">
          <h2 className="focus-card__title">Delete “{inspected.name}”?</h2>
          <p className="exp-lede">
            This removes the saved context and its restore identities from this
            computer. It cannot be undone.
          </p>
          <div className="exp-actions">
            <button
              type="button"
              className="exp-btn"
              disabled={busy}
              onClick={() => setStep("inspect")}
            >
              Cancel
            </button>
            <button
              type="button"
              className="exp-btn primary"
              disabled={busy}
              onClick={confirmDelete}
            >
              Delete permanently
            </button>
          </div>
        </WorkspaceSurface>
      )}

      {step === "done" && result && preview && (
        <WorkspaceSurface tone="hero" padding="lg" className="focus-card focus-card--center">
          <p className="exp-kicker">Done</p>
          <h2 className="focus-card__title">You’re back</h2>
          <p>
            Outcome: <strong>{result.outcome.replace(/_/g, " ")}</strong>
          </p>
          <RestoreLimitsNotice compact />
          <details className="exp-inspect">
            <summary>Per-window outcomes</summary>
            <ul className="resume-plan">
              {result.items.map((item) => (
                <li key={item.item_id}>
                  <strong>{item.target_summary}</strong>
                  <div>
                    {describeOutcome(item.disposition)} · {item.what}
                  </div>
                </li>
              ))}
            </ul>
          </details>
          <div className="exp-actions">
            <button type="button" className="exp-btn" onClick={backToBrowse}>
              Back to saved moments
            </button>
            {onGoToPilot && (
              <button
                type="button"
                className="exp-btn ghost"
                onClick={onGoToPilot}
              >
                Record leave→resume for the pilot
              </button>
            )}
          </div>
        </WorkspaceSurface>
      )}
    </section>
  );
}
