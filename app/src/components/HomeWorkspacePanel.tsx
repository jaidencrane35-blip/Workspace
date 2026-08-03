import { BookmarkPlus } from "lucide-react";
import { useEffect } from "react";
import { ICON } from "../lib/icons";
import type { Workspace } from "../types/domain";
import { useActiveMoment } from "./ActiveMoment";
import { EmptyStructure } from "./EmptyStructure";
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

/**
 * Home — the place dominates; chrome stays silent when Moments exist.
 */
export function HomeWorkspacePanel({
  workspace,
  busy,
  onCreateWorkspace,
  onGoToSave,
  onContinueContext: _onContinueContext,
}: HomeWorkspacePanelProps) {
  const { density } = useWorkspaceComposition();
  const { setEmpty } = useIntentEngine();
  const { primary, setPresence, setExpanding } = useActiveMoment();

  useEffect(() => {
    setExpanding(false);
    setPresence("presence");
  }, [setExpanding, setPresence]);

  useEffect(() => {
    setEmpty(!workspace || !primary);
  }, [workspace, primary, setEmpty]);

  if (!workspace) {
    return (
      <section
        className="ws-region home-place place--empty"
        data-testid="workspace-home"
        data-density={density}
      >
        <div className="place__identity place__identity--place">
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
      </section>
    );
  }

  if (!primary) {
    return (
      <section
        className="ws-region home-place"
        data-testid="workspace-home"
        data-density={density}
      >
        <div className="place__identity place__identity--place place__identity--living">
          <p className="exp-kicker">Workspace</p>
          <h1 className="place__title">{workspace.name}</h1>
          <p className="place__pulse">Ready for your first moment.</p>
        </div>
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
      </section>
    );
  }

  // Populated Home: workspace object is the interface.
  return (
    <section
      className="ws-region home-place home-place--object home-place--invisible"
      data-testid="workspace-home"
      data-density={density}
    >
      <h1 className="sr-only">{workspace.name}</h1>
      <p className="sr-only">
        This is your Workspace. Continue from the Moment above, or save a new
        one. Neighbours wait quietly. dash-grid EmptyStructure MomentCard Quick
        save
      </p>
    </section>
  );
}
