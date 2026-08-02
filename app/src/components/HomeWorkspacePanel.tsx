import { BookmarkPlus, Play } from "lucide-react";
import { LayoutGroup, motion, useReducedMotion } from "motion/react";
import { useEffect, useState } from "react";
import { spring } from "../design-system";
import { invokeIpc } from "../lib/ipc";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";
import { formatRelativeTime } from "../lib/time";
import type { SavedContext, Workspace } from "../types/domain";
import { EmptyStructure } from "./EmptyStructure";
import { MomentCard } from "./MomentCard";
import { useWorkspaceComposition } from "./WorkspaceComposition";
import { WorkspaceSurface } from "./WorkspaceSurface";

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
  const { density } = useWorkspaceComposition();
  const reduceMotion = useReducedMotion();
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
        <div className="place__fab-row">
          <button
            type="button"
            className="exp-btn primary"
            onClick={onCreateWorkspace}
            disabled={busy}
          >
            Create a workspace
          </button>
        </div>
        <div className="dash-grid">
          <EmptyStructure />
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

      <LayoutGroup>
        {latest ? (
          <div className="ws-canvas__stage">
            <motion.div
              className="ws-canvas__anchor"
              layout={!reduceMotion}
              transition={spring.layout}
            >
              <MomentCard
                variant="hero"
                state="expanded"
                context={latest}
                busy={busy}
                onContinue={() => onContinueContext(latest.id)}
                onInspect={onGoToContinue}
              />
            </motion.div>

            <aside className="ws-canvas__float">
              <WorkspaceSurface
                level="floating"
                tone="soft"
                padding="md"
                className="quote-pane"
                layout
              >
                <div className="quote-pane__mark" aria-hidden="true">
                  “
                </div>
                <p className="quote-pane__text">
                  {latest.handoff_note.trim() || "No handoff was recorded."}
                </p>
                <p className="quote-pane__meta">
                  Your note · {formatRelativeTime(latest.created_at)}
                </p>
              </WorkspaceSurface>
              <button
                type="button"
                className="exp-btn primary"
                onClick={onGoToSave}
                disabled={busy}
              >
                <BookmarkPlus size={16} aria-hidden="true" />
                Quick save
              </button>
            </aside>

            {density !== "focus" && (
              <div className="dash-grid ws-canvas__orbit">
                {orbit.map((context, index) => (
                  <MomentCard
                    key={context.id}
                    variant={index === 0 ? "standard" : "compact"}
                    state="collapsed"
                    className={index === 0 ? "span-6" : "span-3"}
                    context={context}
                    busy={busy}
                    onContinue={() => onContinueContext(context.id)}
                    onSelect={() => onContinueContext(context.id)}
                  />
                ))}
                {orbit.length < (density === "flow" ? 4 : 2) &&
                  Array.from({
                    length: Math.max(
                      1,
                      (density === "flow" ? 4 : 2) - orbit.length,
                    ),
                  }).map((_, index) => (
                    <MomentCard
                      key={`ph-${index}`}
                      variant="placeholder"
                      className="span-3"
                      placeholderLabel="Open"
                      placeholderHint="Fills when you save"
                    />
                  ))}
              </div>
            )}
          </div>
        ) : (
          <div className="ws-canvas__stage ws-canvas__stage--invite">
            <div className="ws-canvas__anchor">
              <WorkspaceSurface
                level="floating"
                tone="hero"
                padding="xl"
                interactive
                className="moment-card moment-card--hero empty-invite"
                layout
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
              </WorkspaceSurface>
            </div>
            <aside className="ws-canvas__float">
              <WorkspaceSurface
                level="floating"
                tone="soft"
                padding="md"
                className="quote-pane"
              >
                <div className="quote-pane__mark" aria-hidden="true">
                  “
                </div>
                <p className="quote-pane__text muted">
                  Your next intention will live here.
                </p>
                <p className="quote-pane__meta">Nothing invented</p>
              </WorkspaceSurface>
              <button
                type="button"
                className="exp-btn"
                onClick={onGoToSave}
                disabled={busy}
              >
                <BookmarkPlus size={16} aria-hidden="true" />
                Quick save
              </button>
            </aside>
            {density !== "focus" && (
              <div className="dash-grid">
                <EmptyStructure />
              </div>
            )}
            <div className="place__fab-row">
              <button
                type="button"
                className="exp-btn"
                onClick={onGoToContinue}
                disabled={busy || recent.length === 0}
              >
                <Play size={16} aria-hidden="true" />
                Continue
              </button>
            </div>
          </div>
        )}
      </LayoutGroup>

      <p className="trust-strip muted">{RESTORE_LIMITS_SUMMARY}</p>
    </section>
  );
}
