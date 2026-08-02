import { BookmarkPlus, Play } from "lucide-react";
import { useEffect, useState } from "react";
import { invokeIpc } from "../lib/ipc";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";
import { formatRelativeTime } from "../lib/time";
import type { SavedContext, Workspace } from "../types/domain";
import { EmptyStructure } from "./EmptyStructure";
import { MomentCard } from "./MomentCard";
import { IntentionObject } from "./objects/IntentionObject";
import { QuickActionObject } from "./objects/QuickActionObject";
import { useWorkspaceComposition } from "./WorkspaceComposition";
import { WorkspaceObject } from "./WorkspaceObject";

interface HomeWorkspacePanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onCreateWorkspace: () => void;
  onGoToSave: () => void;
  onGoToContinue: () => void;
  onContinueContext: (contextId: string) => void;
}

export function HomeWorkspacePanel({
  workspace,
  busy,
  onCreateWorkspace,
  onGoToSave,
  onGoToContinue,
  onContinueContext,
}: HomeWorkspacePanelProps) {
  const { density, setSelectedObjectId, setAmbient } = useWorkspaceComposition();
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
      <section
        className="spatial-frame ws-canvas place--empty"
        data-testid="workspace-home"
        data-density={density}
      >
        <div className="place__identity">
          <p className="exp-kicker">Workspace</p>
          <h2 className="place__title">This is your Workspace</h2>
          <p className="place__pulse">Where your work lives.</p>
        </div>
        <div className="ws-compose ws-compose--empty">
          <WorkspaceObject
            objectId="home-create"
            kind="quick-action"
            slot="anchor"
            state="expanded"
            className="empty-invite"
          >
            <p className="moment-card__kicker">Begin</p>
            <h3>Create your Workspace</h3>
            <p className="moment-card__handoff">
              One place for moments, notes, and return.
            </p>
            <div className="moment-card__actions">
              <button
                type="button"
                className="exp-btn primary"
                onClick={onCreateWorkspace}
                disabled={busy}
              >
                Create a workspace
              </button>
            </div>
          </WorkspaceObject>
          <div className="dash-grid">
            <EmptyStructure />
          </div>
        </div>
        <p className="trust-strip muted">{RESTORE_LIMITS_SUMMARY}</p>
      </section>
    );
  }

  const latest = recent[0] ?? null;
  const orbit = recent.slice(1);
  const pulse =
    recent.length === 0
      ? "Ready for your first moment"
      : recent.length === 1
        ? `Last note · ${formatRelativeTime(recent[0].created_at)}`
        : `${recent.length} moments · last ${formatRelativeTime(recent[0].created_at)}`;

  return (
    <section
      className="spatial-frame ws-canvas"
      data-testid="workspace-home"
      data-density={density}
    >
      <div className="place__identity">
        <p className="exp-kicker">Workspace</p>
        <h1 className="place__title">{workspace.name}</h1>
        <p className="place__pulse">{pulse}</p>
      </div>

      {loadError && <p className="error">{loadError}</p>}

      {latest ? (
        <div className="ws-compose">
          <div className="ws-compose__anchor">
            <MomentCard
              variant="hero"
              state="expanded"
              context={latest}
              busy={busy}
              onContinue={() => onContinueContext(latest.id)}
              onInspect={onGoToContinue}
              onSelect={() => {
                setSelectedObjectId(latest.id);
                setAmbient("moment");
              }}
            />
          </div>

          <aside className="ws-compose__float">
            <IntentionObject
              id={`intention-${latest.id}`}
              text={
                latest.handoff_note.trim() || "No handoff was recorded."
              }
              meta={`Your note · ${formatRelativeTime(latest.created_at)}`}
              state="expanded"
            />
            <div className="ws-compose__utilities">
              <QuickActionObject
                id="quick-save"
                label="Quick save"
                icon={BookmarkPlus}
                primary
                disabled={busy}
                onClick={onGoToSave}
              />
              <QuickActionObject
                id="quick-continue"
                label="Continue"
                icon={Play}
                disabled={busy}
                onClick={onGoToContinue}
              />
            </div>
          </aside>

          {density !== "focus" && (
            <div className="ws-compose__orbit dash-grid">
              {orbit.map((context) => (
                <MomentCard
                  key={context.id}
                  variant="compact"
                  state="collapsed"
                  context={context}
                  busy={busy}
                  onContinue={() => onContinueContext(context.id)}
                  onSelect={() => {
                    setSelectedObjectId(context.id);
                    setAmbient("moment");
                    onContinueContext(context.id);
                  }}
                />
              ))}
              {orbit.length === 0 && <EmptyStructure />}
            </div>
          )}
        </div>
      ) : (
        <div className="ws-compose ws-compose--invite">
          <div className="ws-compose__anchor">
            <WorkspaceObject
              objectId="first-moment"
              kind="moment"
              slot="anchor"
              state="expanded"
              className="empty-invite"
            >
              <p className="moment-card__kicker">Start here</p>
              <h3>Save your first moment</h3>
              <p className="moment-card__handoff">
                One note. That’s the way back.
              </p>
              <div className="moment-card__actions">
                <button
                  type="button"
                  className="exp-btn primary"
                  onClick={onGoToSave}
                  disabled={busy}
                >
                  <BookmarkPlus size={16} aria-hidden="true" />
                  Save your first moment
                </button>
              </div>
            </WorkspaceObject>
          </div>
          <aside className="ws-compose__float">
            <IntentionObject
              id="intention-empty"
              text="Your next intention will live here."
              meta="Nothing invented"
              state="idle"
            />
            <QuickActionObject
              id="quick-save"
              label="Quick save"
              icon={BookmarkPlus}
              primary
              disabled={busy}
              onClick={onGoToSave}
            />
          </aside>
          {density !== "focus" && (
            <div className="dash-grid ws-compose__orbit">
              <EmptyStructure />
            </div>
          )}
        </div>
      )}

      <p className="trust-strip muted">{RESTORE_LIMITS_SUMMARY}</p>
    </section>
  );
}
