import { useCallback, useEffect, useState } from "react";
import { invokeIpc } from "../lib/ipc";
import type {
  SavedContext,
  SavedContextCaptureScope,
  SavedContextWindow,
  Workspace,
} from "../types/domain";
import { RestoreLimitsNotice } from "./RestoreLimitsNotice";

/**
 * Saving a bounded workspace context (Product Proof PP-M1-01 / PP-P01A).
 *
 * Naming and an explicit handoff note come first. The capture scope is reviewed
 * next. Only confirmation causes the desktop to be read. Workspace never
 * invents the intended next action.
 */

type Step = "naming" | "reviewing" | "saved";

interface SaveContextPanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
  /** Pilot chrome creates a workspace without routing through Canvas. */
  onCreateWorkspace: () => void;
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

export function SaveContextPanel({
  workspace,
  busy,
  onBusy,
  onError,
  onMessage,
  onCreateWorkspace,
}: SaveContextPanelProps) {
  const [scope, setScope] = useState<SavedContextCaptureScope | null>(null);
  const [scopeError, setScopeError] = useState<string | null>(null);
  const [name, setName] = useState("");
  const [handoffNote, setHandoffNote] = useState("");
  const [step, setStep] = useState<Step>("naming");
  const [saved, setSaved] = useState<SavedContext | null>(null);

  useEffect(() => {
    invokeIpc<SavedContextCaptureScope>("get_saved_context_capture_scope")
      .then(setScope)
      .catch((err: unknown) => setScopeError(formatError(err)));
  }, []);

  const trimmedName = name.trim();
  const trimmedHandoff = handoffNote.trim();
  const canReview =
    scope !== null && trimmedName.length > 0 && trimmedHandoff.length > 0;

  const save = useCallback(() => {
    if (!workspace || !scope || !trimmedHandoff) {
      return;
    }
    onBusy(true);
    onError(null);
    void (async () => {
      try {
        const context = await invokeIpc<SavedContext>("save_workspace_context", {
          workspaceId: workspace.id,
          name: trimmedName,
          approvedScope: scope.id,
          handoffNote: trimmedHandoff,
        });
        setSaved(context);
        setStep("saved");
        onMessage(`Saved “${context.name}”`);
      } catch (err: unknown) {
        onError(formatError(err));
      } finally {
        onBusy(false);
      }
    })();
  }, [
    workspace,
    scope,
    trimmedName,
    trimmedHandoff,
    onBusy,
    onError,
    onMessage,
  ]);

  const startAgain = () => {
    setName("");
    setHandoffNote("");
    setSaved(null);
    setStep("naming");
    onMessage(null);
    onError(null);
  };

  if (!workspace) {
    return (
      <section className="dash save-dash">
        <div className="dash-hero dash-hero--welcome">
          <div className="dash-hero__atmosphere" aria-hidden="true" />
          <div className="dash-hero__content">
            <p className="exp-kicker">Save</p>
            <h2>Start your workspace</h2>
            <p className="exp-lede short">
              Create a place for moments. Nothing from the desktop is read until
              you review and confirm.
            </p>
            <div className="exp-actions">
              <button
                type="button"
                className="exp-btn primary"
                onClick={onCreateWorkspace}
                disabled={busy}
              >
                Create a workspace
              </button>
            </div>
          </div>
        </div>
      </section>
    );
  }

  if (scopeError) {
    return (
      <section className="exp-stage">
        <div className="exp-hero-card">
          <p className="exp-kicker">Save</p>
          <h2>Saving is unavailable</h2>
          <p className="exp-lede">
            Workspace could not confirm what a capture would include, so it will
            not capture anything.
          </p>
          <p className="error">{scopeError}</p>
        </div>
      </section>
    );
  }

  if (step === "saved" && saved) {
    return (
      <section className="exp-stage">
        <article className="exp-card featured">
          <p className="exp-kicker">Saved</p>
          <h2>{saved.name}</h2>
          <p className="exp-intention">{saved.handoff_note}</p>
          <p className="muted">
            Kept on this computer at {formatMoment(saved.created_at)}. You wrote
            the intention — Workspace did not invent or rewrite it.
          </p>
          <details className="exp-inspect">
            <summary>Inspect what was kept</summary>
            <h3>
              {saved.windows.length}{" "}
              {saved.windows.length === 1 ? "window" : "windows"}
            </h3>
            <ul className="list compact">
              {saved.windows.map((window) => (
                <li key={window.id}>
                  <div>{window.title}</div>
                  <div className="muted">{describeWindow(window)}</div>
                </li>
              ))}
            </ul>
            <h3>
              {saved.monitors.length}{" "}
              {saved.monitors.length === 1 ? "monitor" : "monitors"}
            </h3>
            <ul className="list compact">
              {saved.monitors.map((monitor) => (
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
              className="exp-btn"
              onClick={startAgain}
              disabled={busy}
            >
              Save another moment
            </button>
          </div>
        </article>
      </section>
    );
  }

  if (step === "reviewing" && scope) {
    return (
      <section className="exp-stage">
        <article className="exp-card featured">
          <p className="exp-kicker">Review</p>
          <h2>Ready to bookmark “{trimmedName}”?</h2>
          <p className="exp-lede">
            Nothing has been looked at yet. Confirm only if this matches what you
            want kept.
          </p>

          <div className="exp-intention-block">
            <h3>What you intend next</h3>
            <p className="exp-intention">{trimmedHandoff}</p>
            <p className="muted">Kept exactly as you wrote it.</p>
          </div>

          <details className="exp-inspect" open>
            <summary>What will be saved</summary>
            <p className="muted">{scope.purpose}</p>
            <h3>From the desktop</h3>
            <ul className="list compact">
              {scope.captured.map((item) => (
                <li key={item.key}>{item.summary}</li>
              ))}
            </ul>
            <h3>Not saved</h3>
            <ul className="list compact">
              {scope.excluded.map((item) => (
                <li key={item.key}>{item.summary}</li>
              ))}
            </ul>
          </details>

          <RestoreLimitsNotice />

          <div className="exp-actions">
            <button
              type="button"
              className="exp-btn primary"
              onClick={save}
              disabled={busy}
            >
              Save this context
            </button>
            <button
              type="button"
              className="exp-btn ghost"
              onClick={() => setStep("naming")}
              disabled={busy}
            >
              Cancel
            </button>
          </div>
          <p className="muted">
            Cancelling captures nothing, because nothing has been captured.
          </p>
        </article>
      </section>
    );
  }

  return (
    <section className="dash save-dash">
      <div className="dash-grid">
        <article className="moment-card moment-card--hero span-8 note-card">
          <p className="exp-kicker">Save</p>
          <h2>Leave a note for yourself</h2>
          <p className="exp-lede short">
            Before you step away — name the moment and write what comes next.
          </p>

          <label className="exp-field" htmlFor="saved-context-name">
            <span>Name this moment</span>
            <input
              id="saved-context-name"
              className="input-wide"
              value={name}
              placeholder="Tuesday review"
              disabled={busy}
              onChange={(event) => setName(event.target.value)}
            />
          </label>

          <label className="exp-field" htmlFor="saved-context-handoff">
            <span>What do you intend to do next?</span>
            <textarea
              id="saved-context-handoff"
              className="input-wide note-textarea"
              rows={4}
              value={handoffNote}
              placeholder="Finish the client proposal outline…"
              disabled={busy}
              onChange={(event) => setHandoffNote(event.target.value)}
            />
          </label>
          <p className="muted">
            You write this. Workspace will not invent or rewrite it.
          </p>

          <div className="exp-actions">
            <button
              type="button"
              className="exp-btn primary"
              onClick={() => setStep("reviewing")}
              disabled={busy || !canReview}
            >
              Review what will be saved
            </button>
          </div>
          {scope === null && (
            <p className="muted">Checking what a capture would include…</p>
          )}
        </article>
        <aside className="dash-rail span-4">
          <article className="action-card">
            <p className="exp-kicker">Remember</p>
            <h3>Nothing read yet</h3>
            <p className="muted">
              Desktop capture waits until you review and confirm. Cancel means
              zero observation.
            </p>
          </article>
        </aside>
      </div>
    </section>
  );
}
