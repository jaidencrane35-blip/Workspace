import { useEffect, useState } from "react";
import { invokeIpc } from "../lib/ipc";
import type { SavedContext, Workspace } from "../types/domain";

interface HomeWorkspacePanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onCreateWorkspace: () => void;
  onGoToSave: () => void;
  onGoToContinue: () => void;
  onContinueContext: (contextId: string) => void;
}

function formatMoment(iso: string): string {
  const at = new Date(iso);
  return Number.isNaN(at.getTime()) ? iso : at.toLocaleString();
}

/**
 * Product home — “this is my workspace.”
 * Presentation only; Save/Resume ownership unchanged.
 */
export function HomeWorkspacePanel({
  workspace,
  busy,
  onCreateWorkspace,
  onGoToSave,
  onGoToContinue,
  onContinueContext,
}: HomeWorkspacePanelProps) {
  const [recent, setRecent] = useState<SavedContext[]>([]);
  const [loadError, setLoadError] = useState<string | null>(null);

  useEffect(() => {
    if (!workspace) {
      setRecent([]);
      return;
    }
    void invokeIpc<SavedContext[]>("list_saved_contexts", {
      workspaceId: workspace.id,
    })
      .then((contexts) => {
        const sorted = [...contexts].sort((a, b) =>
          b.created_at.localeCompare(a.created_at),
        );
        setRecent(sorted.slice(0, 4));
        setLoadError(null);
      })
      .catch((err: unknown) => {
        setLoadError(err instanceof Error ? err.message : String(err));
      });
  }, [workspace]);

  if (!workspace) {
    return (
      <section className="exp-stage">
        <div className="exp-hero-card">
          <p className="exp-kicker">Welcome</p>
          <h2>This is your Workspace</h2>
          <p className="exp-lede">
            A calm place to leave work and return to it — with your own note of
            what comes next. Nothing is read from the desktop until you ask.
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
        <div className="exp-card-grid placeholder-grid">
          <article className="exp-card placeholder">
            <h3>Recent work</h3>
            <p>Saved moments will appear here.</p>
          </article>
          <article className="exp-card placeholder">
            <h3>Your intention</h3>
            <p>Handoff notes you write stay exactly as you left them.</p>
          </article>
          <article className="exp-card placeholder">
            <h3>Continue</h3>
            <p>Restore still-open windows in this same Windows session.</p>
          </article>
        </div>
      </section>
    );
  }

  const latest = recent[0] ?? null;

  return (
    <section className="exp-stage">
      <header className="exp-home-header">
        <div>
          <p className="exp-kicker">Workspace</p>
          <h2>{workspace.name}</h2>
          <p className="exp-lede">
            {latest
              ? "Pick up where you left off, or mark a new place before you step away."
              : "Mark where you are before you leave — then continue when you return."}
          </p>
        </div>
        <div className="exp-actions">
          <button
            type="button"
            className="exp-btn primary"
            onClick={onGoToSave}
            disabled={busy}
          >
            Save this moment
          </button>
          <button
            type="button"
            className="exp-btn"
            onClick={onGoToContinue}
            disabled={busy}
          >
            Continue work
          </button>
        </div>
      </header>

      {loadError && <p className="error">{loadError}</p>}

      {latest ? (
        <article className="exp-card featured">
          <p className="exp-kicker">Most recent</p>
          <h3>{latest.name}</h3>
          <p className="exp-intention">
            {latest.handoff_note.trim()
              ? latest.handoff_note
              : "No handoff was recorded."}
          </p>
          <p className="muted">
            {formatMoment(latest.created_at)} · {latest.windows.length}{" "}
            {latest.windows.length === 1 ? "window" : "windows"}
          </p>
          <div className="exp-actions">
            <button
              type="button"
              className="exp-btn primary"
              disabled={busy}
              onClick={() => onContinueContext(latest.id)}
            >
              Continue this
            </button>
            <button
              type="button"
              className="exp-btn ghost"
              disabled={busy}
              onClick={onGoToContinue}
            >
              See all
            </button>
          </div>
        </article>
      ) : (
        <article className="exp-card featured empty-invite">
          <p className="exp-kicker">Ready when you are</p>
          <h3>Nothing saved yet</h3>
          <p className="exp-lede">
            When you step away, save a short note about what you intend next.
            That becomes the way back.
          </p>
          <button
            type="button"
            className="exp-btn primary"
            onClick={onGoToSave}
            disabled={busy}
          >
            Save your first moment
          </button>
        </article>
      )}

      {recent.length > 1 && (
        <div className="exp-card-grid">
          {recent.slice(1).map((context) => (
            <article key={context.id} className="exp-card">
              <h3>{context.name}</h3>
              <p className="exp-intention compact">
                {context.handoff_note.trim() || "No handoff recorded."}
              </p>
              <p className="muted">{formatMoment(context.created_at)}</p>
              <button
                type="button"
                className="exp-btn ghost"
                disabled={busy}
                onClick={() => onContinueContext(context.id)}
              >
                Continue
              </button>
            </article>
          ))}
        </div>
      )}
    </section>
  );
}
