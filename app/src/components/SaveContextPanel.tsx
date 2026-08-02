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
  onGoToCanvas: () => void;
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
  onGoToCanvas,
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
      <section className="assistant-hero">
        <p className="assistant-kicker">Save</p>
        <h2>Nowhere to keep this yet</h2>
        <p className="lede">
          Saved contexts belong to a workspace, and there is no active one. Create
          a workspace first, then come back here.
        </p>
        <div className="row">
          <button type="button" onClick={onGoToCanvas}>
            Go to Canvas
          </button>
        </div>
      </section>
    );
  }

  if (scopeError) {
    return (
      <section className="assistant-hero">
        <p className="assistant-kicker">Save</p>
        <h2>Saving is unavailable</h2>
        <p className="lede">
          Workspace could not confirm what a capture would include, so it will not
          capture anything. Nothing has been looked at.
        </p>
        <p className="error">{scopeError}</p>
      </section>
    );
  }

  if (step === "saved" && saved) {
    return (
      <section className="assistant-hero">
        <p className="assistant-kicker">Saved</p>
        <h2>{saved.name}</h2>
        <p className="lede">
          Saved on this computer at {formatMoment(saved.created_at)}. Here is
          everything that was kept — nothing else was recorded.
        </p>

        <section>
          <h3>What you intend to do next</h3>
          <p>{saved.handoff_note}</p>
          <p className="muted">
            You wrote this. Workspace did not invent or rewrite it.
          </p>
        </section>

        <section>
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
        </section>

        <section>
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
        </section>

        <RestoreLimitsNotice />

        <div className="row">
          <button type="button" onClick={startAgain} disabled={busy}>
            Save another context
          </button>
        </div>
      </section>
    );
  }

  if (step === "reviewing" && scope) {
    return (
      <section className="assistant-hero">
        <p className="assistant-kicker">Review</p>
        <h2>Nothing has been looked at yet</h2>
        <p className="lede">
          This is what saving “{trimmedName}” would record. Read it, then decide.
        </p>

        <section>
          <h3>What you intend to do next</h3>
          <p>{trimmedHandoff}</p>
          <p className="muted">
            Kept exactly as you wrote it. Not generated, not inferred.
          </p>
        </section>

        <section>
          <h3>Why</h3>
          <p>{scope.purpose}</p>
        </section>

        <section>
          <h3>What will be saved from the desktop</h3>
          <ul className="list compact">
            {scope.captured.map((item) => (
              <li key={item.key}>{item.summary}</li>
            ))}
          </ul>
        </section>

        <section>
          <h3>What will not be saved</h3>
          <ul className="list compact">
            {scope.excluded.map((item) => (
              <li key={item.key}>{item.summary}</li>
            ))}
          </ul>
        </section>

        <RestoreLimitsNotice />

        <div className="row">
          <button type="button" onClick={save} disabled={busy}>
            Save this context
          </button>
          <button
            type="button"
            className="linkish"
            onClick={() => setStep("naming")}
            disabled={busy}
          >
            Cancel
          </button>
        </div>
        <p className="muted">
          Cancelling captures nothing, because nothing has been captured.
        </p>
      </section>
    );
  }

  return (
    <section className="assistant-hero">
      <p className="assistant-kicker">Save</p>
      <h2>Save what you are working on</h2>
      <p className="lede">
        Name the context and write what you intend to do next. Workspace looks
        at nothing until you have read what would be captured and confirmed it.
      </p>

      <div className="row">
        <label htmlFor="saved-context-name">Name</label>
        <input
          id="saved-context-name"
          className="input-wide"
          value={name}
          placeholder="Tuesday review"
          disabled={busy}
          onChange={(event) => setName(event.target.value)}
        />
      </div>

      <div className="row">
        <label htmlFor="saved-context-handoff">
          What do you intend to do next?
        </label>
        <textarea
          id="saved-context-handoff"
          className="input-wide"
          rows={3}
          value={handoffNote}
          placeholder="e.g. Finish the client proposal outline"
          disabled={busy}
          onChange={(event) => setHandoffNote(event.target.value)}
        />
      </div>
      <p className="muted">
        You write this. Workspace will not invent or rewrite it.
      </p>

      <div className="row">
        <button
          type="button"
          onClick={() => setStep("reviewing")}
          disabled={busy || !canReview}
        >
          Review what will be saved
        </button>
      </div>
      {scope === null && (
        <p className="muted">Checking what a capture would include…</p>
      )}
    </section>
  );
}
