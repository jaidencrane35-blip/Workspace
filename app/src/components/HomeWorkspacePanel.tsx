import { BookmarkPlus } from "lucide-react";
import { ICON } from "../lib/icons";
import type { Workspace } from "../types/domain";
import { useActiveMoment } from "./ActiveMoment";

interface HomeWorkspacePanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onCreateWorkspace: () => void;
  onGoToSave: () => void;
  onContinueContext: (contextId: string) => void;
}

/**
 * Moments tool (Mode 3 specialized) — not an onboarding dashboard.
 * P18.S1: browse list comes from ActiveMoment.moments — same authority as Continue.
 */
export function HomeWorkspacePanel({
  workspace,
  busy,
  onCreateWorkspace,
  onGoToSave,
  onContinueContext,
}: HomeWorkspacePanelProps) {
  const { moments } = useActiveMoment();

  if (!workspace) {
    return (
      <section
        className="ws-region home-place place--empty op-moments-tool"
        data-testid="workspace-home"
      >
        <div className="place__identity place__identity--place">
          <p className="exp-kicker">Moments</p>
          <h2 className="place__title">No workspace yet</h2>
          <p className="place__pulse">
            Ask in conversation to save where you are — or create a place for
            Moments here.
          </p>
        </div>
        <div className="moment-card__actions">
          <button
            type="button"
            className="exp-btn primary"
            onClick={onCreateWorkspace}
            disabled={busy}
          >
            Create workspace
          </button>
        </div>
      </section>
    );
  }

  if (moments.length === 0) {
    return (
      <section
        className="ws-region home-place op-moments-tool"
        data-testid="workspace-home"
      >
        <div className="place__identity place__identity--place place__identity--living">
          <p className="exp-kicker">Moments</p>
          <h1 className="place__title">{workspace.name}</h1>
          <p className="place__pulse">
            No saved Moments yet. Say “save this” in conversation when you want
            to put work down.
          </p>
        </div>
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
            Save now
          </button>
        </div>
      </section>
    );
  }

  return (
    <section
      className="ws-region home-place op-moments-tool"
      data-testid="workspace-home"
    >
      <div className="place__identity place__identity--place">
        <p className="exp-kicker">Moments</p>
        <h1 className="place__title">{workspace.name}</h1>
        <p className="place__pulse">
          Choose a Moment to restore — or ask in conversation.
        </p>
      </div>
      <ul className="op-moments-list" data-moments-browse="home">
        {moments.map((moment) => (
          <li key={moment.id}>
            <button
              type="button"
              className="op-moments-list__item"
              disabled={busy}
              onClick={() => onContinueContext(moment.id)}
            >
              <span className="op-moments-list__name">{moment.name}</span>
              <span className="op-moments-list__meta">
                {moment.handoff_note?.trim() || "Restore review"}
              </span>
            </button>
          </li>
        ))}
      </ul>
    </section>
  );
}
