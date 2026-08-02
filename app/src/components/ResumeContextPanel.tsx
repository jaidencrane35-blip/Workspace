import { useCallback, useEffect, useState } from "react";
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
import { EmptyStructure } from "./EmptyStructure";
import { MomentCard } from "./MomentCard";
import { RestoreLimitsNotice } from "./RestoreLimitsNotice";

type Step = "browse" | "inspect" | "confirm_delete" | "preview" | "done";

interface ResumeContextPanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
  /** Optional navigation to the consented pilot measurement surface (PP-P01E). */
  onGoToPilot?: () => void;
  onGoHome?: () => void;
  /** When set, open preview for this saved context after browse loads. */
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
      return "Will restore";
    case "will_skip_unsupported":
      return "Unsupported";
    case "will_skip_unresolvable":
      return "Cannot restore";
    default:
      return item.projected_disposition;
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
  const [step, setStep] = useState<Step>("browse");
  const [contexts, setContexts] = useState<SavedContext[]>([]);
  const [inspected, setInspected] = useState<SavedContext | null>(null);
  const [preview, setPreview] = useState<ResumePlanPreview | null>(null);
  const [result, setResult] = useState<ActionOperationResult | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);

  const reload = useCallback(() => {
    if (!workspace) {
      setContexts([]);
      return;
    }
    void invokeIpc<SavedContext[]>("list_saved_contexts", {
      workspaceId: workspace.id,
    })
      .then(setContexts)
      .catch((err: unknown) => setLoadError(formatError(err)));
  }, [workspace]);

  useEffect(() => {
    reload();
  }, [reload]);

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
        onMessage(`Inspecting “${context.name}”`);
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
      void (async () => {
        try {
          const next = await invokeIpc<ResumePlanPreview>("resolve_resume_plan", {
            savedContextId: contextId,
          });
          setPreview(next);
          setResult(null);
          setStep("preview");
          onMessage(`Preview ready for “${next.saved_context_name}”`);
        } catch (err: unknown) {
          onError(formatError(err));
        } finally {
          onBusy(false);
        }
      })();
    },
    [onBusy, onError, onMessage],
  );

  useEffect(() => {
    if (!focusContextId || !workspace || contexts.length === 0) {
      return;
    }
    if (!contexts.some((context) => context.id === focusContextId)) {
      return;
    }
    openPreview(focusContextId);
  }, [focusContextId, workspace, contexts, openPreview]);

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
        const deletedId = inspected.id;
        await invokeIpc<null>("delete_saved_context", {
          savedContextId: deletedId,
        });
        setInspected(null);
        setPreview(null);
        setResult(null);
        setStep("browse");
        reload();
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
    reload();
  };

  if (!workspace) {
    return (
      <section className="exp-stage">
        <div className="exp-hero-card">
          <p className="exp-kicker">Continue</p>
          <h2>Let’s continue your work</h2>
          <p className="exp-lede">
            Create or open a workspace first — then your saved moments will be
            ready to continue.
          </p>
          {onGoHome && (
            <button type="button" className="exp-btn primary" onClick={onGoHome}>
              Go to Home
            </button>
          )}
        </div>
      </section>
    );
  }

  const sorted = [...contexts].sort((a, b) =>
    b.created_at.localeCompare(a.created_at),
  );
  const featured = sorted[0] ?? null;
  const others = sorted.slice(1);

  return (
    <section className="dash continue-dash">
      <header className="dash-chrome">
        <div>
          <p className="exp-kicker">Continue</p>
          <h1 className="dash-title">What were you doing?</h1>
          <p className="dash-summary">
            Pick up your note. Nothing moves until you approve a restore.
          </p>
        </div>
      </header>
      <p className="trust-strip muted">{RESTORE_LIMITS_SUMMARY}</p>

      {loadError && <p className="error">{loadError}</p>}

      {step === "browse" && (
        <>
          {contexts.length === 0 ? (
            <EmptyStructure
              title="Nothing to continue yet"
              hint="Save a moment from Home — then it appears here as a place you can return to."
            />
          ) : (
            <div className="dash-grid">
              {featured && (
                <MomentCard
                  variant="hero"
                  className="span-8"
                  context={featured}
                  busy={busy}
                  onContinue={() => openPreview(featured.id)}
                  onInspect={() => openInspect(featured.id)}
                />
              )}
              <aside className="dash-rail span-4">
                <article className="action-card">
                  <p className="exp-kicker">Next</p>
                  <h3>Can I continue?</h3>
                  <p className="muted">
                    Continue previews the restore plan. Inspect is only for
                    details.
                  </p>
                </article>
              </aside>
              {others.map((context, index) => (
                <MomentCard
                  key={context.id}
                  variant={index < 2 ? "standard" : "compact"}
                  className={index < 2 ? "span-6" : "span-4"}
                  context={context}
                  busy={busy}
                  onContinue={() => openPreview(context.id)}
                  onInspect={() => openInspect(context.id)}
                />
              ))}
            </div>
          )}
        </>
      )}

      {step === "inspect" && inspected && (
        <article className="exp-card featured">
          <p className="exp-kicker">Inspect</p>
          <h3>{inspected.name}</h3>
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
            <h4>
              {inspected.windows.length}{" "}
              {inspected.windows.length === 1 ? "window" : "windows"}
            </h4>
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
        </article>
      )}

      {step === "confirm_delete" && inspected && (
        <article className="exp-card featured">
          <h3>Delete “{inspected.name}”?</h3>
          <p className="exp-lede">
            This removes the saved context and its restore identities from this
            computer. It cannot be undone. Windows already open on your desktop
            are not closed.
          </p>
          <p className="muted">
            After deletion, this context cannot be inspected or restored.
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
        </article>
      )}

      {step === "preview" && preview && (
        <article className="exp-card featured">
          <p className="exp-kicker">Preview</p>
          <h3>Continue “{preview.saved_context_name}”</h3>
          <p className="exp-intention">
            {preview.handoff_note.trim()
              ? preview.handoff_note
              : "No handoff was recorded with this context."}
          </p>
          <p className="muted">
            Shown exactly as you wrote it. Expires{" "}
            {formatMoment(preview.plan.expires_at)}.
          </p>
          <RestoreLimitsNotice />
          <details className="exp-inspect" open>
            <summary>Restore plan</summary>
            <ul className="resume-plan">
              {preview.plan.items.map((item) => (
                <li key={item.item_id}>
                  <strong>{item.target_summary}</strong>
                  <div>
                    {item.action_type} · {describeDisposition(item)}
                  </div>
                  {item.reason && <div className="muted">{item.reason}</div>}
                </li>
              ))}
            </ul>
          </details>
          <div className="exp-actions">
            <button
              type="button"
              className="exp-btn primary"
              disabled={busy}
              onClick={approveAndRestore}
            >
              Approve and restore
            </button>
            <button
              type="button"
              className="exp-btn ghost"
              disabled={busy}
              onClick={backToBrowse}
            >
              Cancel
            </button>
          </div>
        </article>
      )}

      {step === "done" && result && preview && (
        <article className="exp-card featured">
          <p className="exp-kicker">Done</p>
          <h3>You’re back</h3>
          <p className="exp-intention">
            {preview.handoff_note.trim()
              ? preview.handoff_note
              : "No handoff was recorded with this context."}
          </p>
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
                  {item.reason && <div className="muted">{item.reason}</div>}
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
          {onGoToPilot && (
            <p className="muted">
              Pilot timing is only recorded if you have consented under Check-in
              and enter the minutes yourself. Nothing is measured in the
              background.
            </p>
          )}
        </article>
      )}
    </section>
  );
}
