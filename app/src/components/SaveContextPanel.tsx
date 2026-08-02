import { motion, useReducedMotion } from "motion/react";
import { useCallback, useEffect, useState } from "react";
import { invokeIpc } from "../lib/ipc";
import type {
  SavedContext,
  SavedContextCaptureScope,
  SavedContextWindow,
  Workspace,
} from "../types/domain";
import { ElevatedCard } from "./ElevatedCard";
import { RestoreLimitsNotice } from "./RestoreLimitsNotice";
import { useWorkspaceComposition } from "./WorkspaceComposition";

type Step = "naming" | "reviewing" | "saved";

interface SaveContextPanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
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
  const { density } = useWorkspaceComposition();
  const reduceMotion = useReducedMotion();
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

  const enter = reduceMotion
    ? undefined
    : { initial: { opacity: 0, y: 12 }, animate: { opacity: 1, y: 0 } };

  if (!workspace) {
    return (
      <section
        className="spatial-frame spatial-frame--center save-env"
        data-density={density}
      >
        <ElevatedCard tone="hero" elevation={3} padding="xl" className="focus-card">
          <p className="exp-kicker">Save</p>
          <h2 className="focus-card__title">Start your workspace</h2>
          <p className="muted">Then leave yourself a note.</p>
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
        </ElevatedCard>
      </section>
    );
  }

  if (scopeError) {
    return (
      <section className="spatial-frame spatial-frame--center save-env">
        <ElevatedCard tone="hero" elevation={3} padding="xl" className="focus-card">
          <p className="exp-kicker">Save</p>
          <h2 className="focus-card__title">Saving is unavailable</h2>
          <p className="error">{scopeError}</p>
        </ElevatedCard>
      </section>
    );
  }

  if (step === "saved" && saved) {
    return (
      <section className="spatial-frame spatial-frame--center save-env">
        <motion.div {...enter} transition={{ type: "spring", stiffness: 300, damping: 30 }}>
          <ElevatedCard
            tone="hero"
            elevation={3}
            padding="xl"
            className="focus-card save-success"
          >
            <p className="exp-kicker">Saved</p>
            <h2 className="focus-card__title">{saved.name}</h2>
            <p className="exp-intention">{saved.handoff_note}</p>
            <p className="muted">
              Kept on this computer · {formatMoment(saved.created_at)}
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
          </ElevatedCard>
        </motion.div>
      </section>
    );
  }

  if (step === "reviewing" && scope) {
    return (
      <section className="spatial-frame spatial-frame--center save-env">
        <ElevatedCard tone="hero" elevation={3} padding="xl" className="focus-card">
          <p className="exp-kicker">Review</p>
          <h2 className="focus-card__title">Bookmark “{trimmedName}”?</h2>
          <div className="exp-intention-block">
            <p className="exp-intention">{trimmedHandoff}</p>
            <p className="muted">Exactly as you wrote it</p>
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
        </ElevatedCard>
      </section>
    );
  }

  return (
    <section
      className="spatial-frame spatial-frame--center save-env save-env--write"
      data-density={density}
    >
      <ElevatedCard
        tone="hero"
        elevation={3}
        padding="xl"
        className="focus-card write-card"
        layout
      >
        <p className="exp-kicker">Save</p>
        <h2 className="focus-card__title">Leave a note</h2>

        <label className="exp-field write-card__name" htmlFor="saved-context-name">
          <span>Name</span>
          <input
            id="saved-context-name"
            className="input-wide"
            value={name}
            placeholder="Tuesday review"
            disabled={busy}
            onChange={(event) => setName(event.target.value)}
          />
        </label>

        <label
          className="exp-field write-card__anchor"
          htmlFor="saved-context-handoff"
        >
          <span>What next?</span>
          <textarea
            id="saved-context-handoff"
            className="input-wide note-textarea write-textarea"
            rows={density === "focus" ? 8 : 6}
            value={handoffNote}
            placeholder="Finish the client proposal outline…"
            disabled={busy}
            autoFocus
            onChange={(event) => setHandoffNote(event.target.value)}
          />
        </label>
        <p className="muted write-card__hint">
          You write this. Workspace will not invent or rewrite it.
        </p>

        <div className="exp-actions write-card__actions">
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
          <p className="muted skeleton-shimmer" style={{ textAlign: "center" }}>
            Checking capture scope…
          </p>
        )}
      </ElevatedCard>
    </section>
  );
}
