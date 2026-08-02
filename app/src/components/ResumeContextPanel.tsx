import { useCallback, useEffect, useState } from "react";
import { isExperienceDemoActive } from "../demo/demoMode";
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
import { ContinuePreviewBody } from "./objects/ContinuePreviewObject";
import { RestoreLimitsNotice } from "./RestoreLimitsNotice";
import { useIntentEngine } from "./IntentEngine";
import { useWorkspaceComposition } from "./WorkspaceComposition";
import { WorkspaceSurface } from "./WorkspaceSurface";

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
  const {
    density,
    setPrimaryObject,
    setSecondaryObjects,
    setAttentionScene,
    setAmbient,
  } = useWorkspaceComposition();
  const { setRestoring } = useIntentEngine();
  const [step, setStep] = useState<Step>("browse");
  const [contexts, setContexts] = useState<SavedContext[]>([]);
  const [inspected, setInspected] = useState<SavedContext | null>(null);
  const [preview, setPreview] = useState<ResumePlanPreview | null>(null);
  const [result, setResult] = useState<ActionOperationResult | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);

  const selectMoment = (id: string) => {
    setSelectedId(id);
    setPrimaryObject(id);
    setAmbient("moment");
  };

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
        if (!isExperienceDemoActive()) {
          onMessage(`Inspecting “${context.name}”`);
        }
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
          setPrimaryObject(contextId);
          setAttentionScene("restore");
          setRestoring(true);
          setSecondaryObjects([]);
          if (!isExperienceDemoActive()) {
            onMessage(`Preview ready for “${next.saved_context_name}”`);
          }
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
      onMessage,
      setPrimaryObject,
      setAttentionScene,
      setRestoring,
      setSecondaryObjects,
    ],
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
    setAttentionScene("default");
    setRestoring(false);
    reload();
  };

  if (!workspace) {
    return (
      <section className="spatial-frame spatial-frame--center">
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

  const sorted = [...contexts].sort((a, b) =>
    b.created_at.localeCompare(a.created_at),
  );
  const featured = sorted[0] ?? null;
  const others = sorted.slice(1);

  const previewing = step === "preview" && preview != null;
  const satellitePool = others.slice(0, density === "flow" ? 4 : 3);

  return (
    <section
      className={[
        "spatial-frame",
        "continue-gallery",
        "continue-dash",
        previewing ? "continue-dash--previewing" : "",
      ]
        .filter(Boolean)
        .join(" ")}
      data-density={density}
    >
      {(step === "browse" || step === "preview") && (
        <>
              {!previewing && (
            <>
              <header className="spatial-header spatial-header--quiet">
                <h1 className="spatial-title">What were you doing?</h1>
              </header>
              <p className="trust-strip trust-strip--quiet muted">
                {RESTORE_LIMITS_SUMMARY}
              </p>
              {featured && (
                <button
                  type="button"
                  className="exp-btn ghost continue-inspect-entry"
                  disabled={busy}
                  onClick={() => openInspect(featured.id)}
                >
                  Inspect
                </button>
              )}
            </>
          )}

          {loadError && <p className="error">{loadError}</p>}

          {contexts.length === 0 ? (
            <EmptyStructure
              title="Nothing to continue yet"
              hint="Save a moment — then it waits here for you."
            />
          ) : (
            <div
              className={
                previewing
                  ? "continue-cinema continue-cinema--focus is-dimmed attention-field"
                  : selectedId
                    ? "continue-cinema is-dimmed attention-field"
                    : "continue-cinema attention-field"
              }
            >
              {featured && (
                <div
                  className={
                    previewing
                      ? "continue-cinema__stage continue-cinema__stage--solo"
                      : "continue-cinema__stage"
                  }
                >
                  <MomentCard
                    variant="hero"
                    sparseMeta
                    state={
                      preview?.saved_context_id === featured.id
                        ? "preview"
                        : selectedId === featured.id
                          ? "selected"
                          : "expanded"
                    }
                    context={featured}
                    busy={busy}
                    onSelect={() => selectMoment(featured.id)}
                    onContinue={() => {
                      selectMoment(featured.id);
                      openPreview(featured.id);
                    }}
                    expandContent={
                      preview?.saved_context_id === featured.id ? (
                        <ContinuePreviewBody
                          preview={preview}
                          busy={busy}
                          onApprove={approveAndRestore}
                          onCancel={backToBrowse}
                          describeDisposition={describeDisposition}
                        />
                      ) : undefined
                    }
                  />
                </div>
              )}
              {!previewing && density !== "focus" && satellitePool.length > 0 && (
                <div className="continue-satellites continue-recede">
                  {satellitePool.map((context, index) => (
                    <MomentCard
                      key={context.id}
                      variant="compact"
                      sparse
                      attentionWeight={0.68}
                      className={`home-satellite home-satellite--${index % 3}`}
                      state={
                        selectedId === context.id ? "selected" : "collapsed"
                      }
                      context={context}
                      busy={busy}
                      onSelect={() => {
                        selectMoment(context.id);
                        openPreview(context.id);
                      }}
                      onContinue={() => {
                        selectMoment(context.id);
                        openPreview(context.id);
                      }}
                    />
                  ))}
                </div>
              )}
            </div>
          )}
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
        </WorkspaceSurface>
      )}

      {step === "confirm_delete" && inspected && (
        <WorkspaceSurface
          tone="hero"
          padding="lg"
          className="focus-card focus-card--center"
        >
          <h2 className="focus-card__title">Delete “{inspected.name}”?</h2>
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
        </WorkspaceSurface>
      )}

      {step === "done" && result && preview && (
        <WorkspaceSurface
          tone="hero"
          padding="lg"
          className="focus-card focus-card--center"
        >
          <p className="exp-kicker">Done</p>
          <h2 className="focus-card__title">You’re back</h2>
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
        </WorkspaceSurface>
      )}
    </section>
  );
}
