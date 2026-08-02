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
import { RestoreLimitsNotice } from "./RestoreLimitsNotice";

type Step = "browse" | "inspect" | "confirm_delete" | "preview" | "done";

interface ResumeContextPanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
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

  const openPreview = (contextId: string) => {
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
  };

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
      <section className="panel">
        <h2>Resume</h2>
        <p className="muted">Create or open a workspace before resuming a saved context.</p>
      </section>
    );
  }

  return (
    <section className="panel">
      <h2>Resume</h2>
      <p className="lede">
        Choose a saved context to inspect, delete, or restore. Nothing runs before
        you approve a restore, and nothing is deleted before you confirm.
      </p>
      <p className="muted">{RESTORE_LIMITS_SUMMARY}</p>

      {loadError && <p className="error">{loadError}</p>}

      {step === "browse" && (
        <>
          {contexts.length === 0 ? (
            <p className="muted">No saved contexts in this workspace yet.</p>
          ) : (
            <ul className="resume-list">
              {contexts.map((context) => (
                <li key={context.id}>
                  <div>
                    <strong>{context.name}</strong>
                    <div className="muted">
                      {formatMoment(context.created_at)} · {context.windows.length}{" "}
                      windows · scope {context.approved_scope}
                    </div>
                    {context.handoff_note.trim() ? (
                      <div className="resume-handoff">
                        Next: {context.handoff_note}
                      </div>
                    ) : (
                      <div className="muted">No handoff was recorded.</div>
                    )}
                  </div>
                  <div className="resume-actions">
                    <button
                      type="button"
                      disabled={busy}
                      onClick={() => openInspect(context.id)}
                    >
                      Inspect
                    </button>
                    <button
                      type="button"
                      disabled={busy}
                      onClick={() => openPreview(context.id)}
                    >
                      Preview restore
                    </button>
                  </div>
                </li>
              ))}
            </ul>
          )}
        </>
      )}

      {step === "inspect" && inspected && (
        <>
          <h3>Saved context “{inspected.name}”</h3>
          <p className="muted">
            Saved on this computer at {formatMoment(inspected.created_at)}. Scope{" "}
            {inspected.approved_scope}. This is everything Workspace retained —
            nothing else.
          </p>

          <section className="resume-handoff-block">
            <h4>What you intended to do next</h4>
            {inspected.handoff_note.trim() ? (
              <p>{inspected.handoff_note}</p>
            ) : (
              <p className="muted">No handoff was recorded with this context.</p>
            )}
            <p className="muted">
              Shown exactly as you wrote it. Workspace did not invent or rewrite
              it.
            </p>
          </section>

          <section>
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
          </section>

          <section>
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
          </section>

          <RestoreLimitsNotice />

          <div className="button-row">
            <button type="button" disabled={busy} onClick={backToBrowse}>
              Back
            </button>
            <button
              type="button"
              disabled={busy}
              onClick={() => openPreview(inspected.id)}
            >
              Preview restore
            </button>
            <button
              type="button"
              disabled={busy}
              onClick={() => setStep("confirm_delete")}
            >
              Delete this context
            </button>
          </div>
        </>
      )}

      {step === "confirm_delete" && inspected && (
        <>
          <h3>Delete “{inspected.name}”?</h3>
          <p className="lede">
            This removes the saved context and its restore identities from this
            computer. It cannot be undone. Windows already open on your desktop
            are not closed.
          </p>
          <p className="muted">
            After deletion, this context cannot be inspected or restored.
          </p>
          <div className="button-row">
            <button
              type="button"
              disabled={busy}
              onClick={() => setStep("inspect")}
            >
              Cancel
            </button>
            <button type="button" disabled={busy} onClick={confirmDelete}>
              Delete permanently
            </button>
          </div>
        </>
      )}

      {step === "preview" && preview && (
        <>
          <h3>Restore plan for “{preview.saved_context_name}”</h3>
          <p className="muted">
            Expires {formatMoment(preview.plan.expires_at)}. Approve only if this
            matches what you want restored.
          </p>
          <section className="resume-handoff-block">
            <h4>What you intended to do next</h4>
            {preview.handoff_note.trim() ? (
              <p>{preview.handoff_note}</p>
            ) : (
              <p className="muted">No handoff was recorded with this context.</p>
            )}
            <p className="muted">
              Shown exactly as you wrote it. Window restore does not change this
              text.
            </p>
          </section>
          <RestoreLimitsNotice />
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
          <div className="button-row">
            <button type="button" disabled={busy} onClick={backToBrowse}>
              Cancel
            </button>
            <button type="button" disabled={busy} onClick={approveAndRestore}>
              Approve and restore
            </button>
          </div>
        </>
      )}

      {step === "done" && result && preview && (
        <>
          <h3>Restore result</h3>
          <p>
            Outcome: <strong>{result.outcome.replace(/_/g, " ")}</strong>
          </p>
          <section className="resume-handoff-block">
            <h4>What you intended to do next</h4>
            {preview.handoff_note.trim() ? (
              <p>{preview.handoff_note}</p>
            ) : (
              <p className="muted">No handoff was recorded with this context.</p>
            )}
          </section>
          <RestoreLimitsNotice compact />
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
          <button type="button" onClick={backToBrowse}>
            Back to saved contexts
          </button>
        </>
      )}
    </section>
  );
}
