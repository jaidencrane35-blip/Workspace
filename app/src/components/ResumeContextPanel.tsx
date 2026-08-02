import { useCallback, useEffect, useState } from "react";
import { invokeIpc } from "../lib/ipc";
import type {
  ActionOperationResult,
  ActionPlanItem,
  ResumePlanPreview,
  SavedContext,
  Workspace,
} from "../types/domain";

type Step = "browse" | "preview" | "done";

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

  const backToBrowse = () => {
    setStep("browse");
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
        Choose a saved context, review exactly what Workspace intends to restore,
        then approve. Nothing runs before you approve.
      </p>

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
                  <button
                    type="button"
                    disabled={busy}
                    onClick={() => openPreview(context.id)}
                  >
                    Preview restore
                  </button>
                </li>
              ))}
            </ul>
          )}
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
