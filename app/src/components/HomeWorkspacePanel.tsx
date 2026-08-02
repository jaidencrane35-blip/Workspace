import { BookmarkPlus } from "lucide-react";
import { useEffect, useState } from "react";
import { ICON } from "../lib/icons";
import { invokeIpc } from "../lib/ipc";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";
import type { SavedContext, Workspace } from "../types/domain";
import { EmptyStructure } from "./EmptyStructure";
import { MomentCard } from "./MomentCard";
import { IntentionObject } from "./objects/IntentionObject";
import { QuickActionObject } from "./objects/QuickActionObject";
import { useIntentEngine } from "./IntentEngine";
import { useWorkspaceComposition } from "./WorkspaceComposition";
import { WorkspaceObject } from "./WorkspaceObject";

interface HomeWorkspacePanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onCreateWorkspace: () => void;
  onGoToSave: () => void;
  onContinueContext: (contextId: string) => void;
}

export function HomeWorkspacePanel({
  workspace,
  busy,
  onCreateWorkspace,
  onGoToSave,
  onContinueContext,
}: HomeWorkspacePanelProps) {
  const {
    density,
    setPrimaryObject,
    setSecondaryObjects,
    setAttentionScene,
    setAmbient,
  } = useWorkspaceComposition();
  const { setEmpty } = useIntentEngine();
  const [recent, setRecent] = useState<SavedContext[]>([]);
  const [loadError, setLoadError] = useState<string | null>(null);

  useEffect(() => {
    if (!workspace) {
      setRecent([]);
      setEmpty(true);
      setAttentionScene("empty");
      setPrimaryObject("home-create");
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
        if (sorted[0]) {
          setEmpty(false);
          setAttentionScene("default");
          setPrimaryObject(sorted[0].id);
          setSecondaryObjects([
            "quick-save",
            ...sorted.slice(1, 5).map((context) => context.id),
          ]);
          setAmbient("moment");
        } else {
          setEmpty(true);
          setAttentionScene("empty");
          setPrimaryObject("first-moment");
          setSecondaryObjects(["intention-empty", "quick-save"]);
        }
      })
      .catch((err: unknown) => {
        setLoadError(err instanceof Error ? err.message : String(err));
      });
  }, [
    workspace,
    setEmpty,
    setAttentionScene,
    setPrimaryObject,
    setSecondaryObjects,
    setAmbient,
  ]);

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
        <div className="ws-compose ws-compose--empty attention-field">
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
          <EmptyStructure />
        </div>
        <p className="trust-strip muted">{RESTORE_LIMITS_SUMMARY}</p>
      </section>
    );
  }

  const latest = recent[0] ?? null;
  const satellites = recent.slice(1);

  return (
    <section
      className="spatial-frame ws-canvas ws-canvas--home"
      data-testid="workspace-home"
      data-density={density}
    >
      <div className="place__identity place__identity--quiet">
        <p className="exp-kicker">{workspace.name}</p>
        <h1 className="place__title place__title--continue">
          Continue your work
        </h1>
        <button
          type="button"
          className="home-quiet-save"
          disabled={busy}
          onClick={onGoToSave}
        >
          <BookmarkPlus
            size={ICON.md}
            strokeWidth={ICON.stroke}
            aria-hidden="true"
          />
          Quick save
        </button>
      </div>

      {loadError && <p className="error">{loadError}</p>}

      {latest ? (
        <>
          <div className="home-hero-band attention-field">
            <MomentCard
              variant="hero"
              state="expanded"
              context={latest}
              busy={busy}
              sparseMeta
              onContinue={() => onContinueContext(latest.id)}
              onSelect={() => {
                setPrimaryObject(latest.id);
                setAmbient("moment");
              }}
            />
          </div>

          {density !== "focus" && satellites.length > 0 && (
            <div className="home-field dash-grid" aria-label="Earlier moments">
              {satellites.map((context, index) => (
                <MomentCard
                  key={context.id}
                  variant="compact"
                  state="collapsed"
                  sparse
                  attentionWeight={1}
                  className={`home-satellite home-satellite--${index % 3}`}
                  context={context}
                  busy={busy}
                  onSelect={() => {
                    setPrimaryObject(context.id);
                    setAmbient("moment");
                    onContinueContext(context.id);
                  }}
                  onContinue={() => onContinueContext(context.id)}
                />
              ))}
            </div>
          )}
        </>
      ) : (
        <div className="ws-compose ws-compose--invite attention-field">
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
                  <BookmarkPlus
                    size={ICON.md}
                    strokeWidth={ICON.stroke}
                    aria-hidden="true"
                  />
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
            <div className="ws-compose__orbit">
              <EmptyStructure />
            </div>
          )}
        </div>
      )}
    </section>
  );
}
