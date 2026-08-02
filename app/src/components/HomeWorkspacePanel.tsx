import { BookmarkPlus, Compass, Play } from "lucide-react";
import { useEffect, useState } from "react";
import { invokeIpc } from "../lib/ipc";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";
import { formatRelativeTime } from "../lib/time";
import type { SavedContext, Workspace } from "../types/domain";
import { EmptyStructure } from "./EmptyStructure";
import { MomentCard } from "./MomentCard";

interface HomeWorkspacePanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onCreateWorkspace: () => void;
  onGoToSave: () => void;
  onGoToContinue: () => void;
  onContinueContext: (contextId: string) => void;
}

/**
 * Dashboard-first Home — Experience Phase 2.
 * Owned data only; no fabricated activity.
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
        setRecent(sorted.slice(0, 7));
        setLoadError(null);
      })
      .catch((err: unknown) => {
        setLoadError(err instanceof Error ? err.message : String(err));
      });
  }, [workspace]);

  if (!workspace) {
    return (
      <section className="dash" data-testid="workspace-home">
        <div className="dash-hero dash-hero--welcome">
          <div className="dash-hero__atmosphere" aria-hidden="true" />
          <div className="dash-hero__content">
            <p className="exp-kicker">Workspace</p>
            <h2>This is your Workspace</h2>
            <p className="exp-lede short">
              Leave a note. Come back. Continue — still-open windows, same
              session.
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
        <EmptyStructure />
        <p className="trust-strip muted">{RESTORE_LIMITS_SUMMARY}</p>
      </section>
    );
  }

  const latest = recent[0] ?? null;
  const rest = recent.slice(1);
  const summary =
    recent.length === 0
      ? "No saved moments yet"
      : recent.length === 1
        ? `1 saved moment · last ${formatRelativeTime(recent[0].created_at)}`
        : `${recent.length} recent moments · last ${formatRelativeTime(recent[0].created_at)}`;

  return (
    <section className="dash" data-testid="workspace-home">
      <header className="dash-chrome">
        <div>
          <p className="exp-kicker">Workspace</p>
          <h1 className="dash-title">{workspace.name}</h1>
          <p className="dash-summary">{summary}</p>
        </div>
        <div className="dash-quick">
          <button
            type="button"
            className="exp-btn primary"
            onClick={onGoToSave}
            disabled={busy}
          >
            <BookmarkPlus size={16} aria-hidden="true" />
            Quick save
          </button>
          <button
            type="button"
            className="exp-btn"
            onClick={onGoToContinue}
            disabled={busy || recent.length === 0}
          >
            <Play size={16} aria-hidden="true" />
            Continue
          </button>
          <button
            type="button"
            className="exp-btn ghost"
            onClick={onGoToContinue}
            disabled={busy}
          >
            <Compass size={16} aria-hidden="true" />
            All moments
          </button>
        </div>
      </header>

      {loadError && <p className="error">{loadError}</p>}

      {latest ? (
        <div className="dash-grid">
          <MomentCard
            variant="hero"
            className="span-8"
            context={latest}
            busy={busy}
            onContinue={() => onContinueContext(latest.id)}
            onInspect={onGoToContinue}
          />
          <aside className="dash-rail span-4">
            <article className="action-card">
              <p className="exp-kicker">Next</p>
              <h3>Step away cleanly</h3>
              <p className="muted">
                Name the moment and write what you intend next. Nothing is read
                until you confirm.
              </p>
              <button
                type="button"
                className="exp-btn"
                onClick={onGoToSave}
                disabled={busy}
              >
                Save this moment
              </button>
            </article>
            <article className="action-card action-card--soft">
              <p className="exp-kicker">Memory</p>
              <h3>Owned by you</h3>
              <p className="muted">
                Handoffs stay exactly as you wrote them. No invented activity.
              </p>
            </article>
          </aside>
          {rest.map((context, index) => (
            <MomentCard
              key={context.id}
              variant={index === 0 ? "standard" : "compact"}
              className={index === 0 ? "span-6" : "span-3"}
              context={context}
              busy={busy}
              onContinue={() => onContinueContext(context.id)}
            />
          ))}
          {rest.length < 3 &&
            Array.from({ length: 3 - rest.length }).map((_, index) => (
              <MomentCard
                key={`ph-${index}`}
                variant="placeholder"
                className="span-3"
                placeholderLabel="Open slot"
                placeholderHint="Fills when you save again"
              />
            ))}
        </div>
      ) : (
        <>
          <div className="dash-grid">
            <article className="moment-card moment-card--hero span-8 empty-invite">
              <p className="exp-kicker">Ready when you are</p>
              <h3>Save your first moment</h3>
              <p className="exp-lede short">
                Before you leave, leave yourself a note. That becomes the way
                back.
              </p>
              <button
                type="button"
                className="exp-btn primary"
                onClick={onGoToSave}
                disabled={busy}
              >
                <BookmarkPlus size={16} aria-hidden="true" />
                Save your first moment
              </button>
            </article>
            <aside className="dash-rail span-4">
              <article className="action-card">
                <p className="exp-kicker">Continue</p>
                <h3>After you save</h3>
                <p className="muted">
                  Moments you keep will show here for one-click continuation.
                </p>
              </article>
            </aside>
          </div>
          <EmptyStructure />
        </>
      )}

      <p className="trust-strip muted">{RESTORE_LIMITS_SUMMARY}</p>
    </section>
  );
}
