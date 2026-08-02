import { BookmarkPlus, Play } from "lucide-react";
import { motion, useReducedMotion } from "motion/react";
import { useEffect, useState } from "react";
import { invokeIpc } from "../lib/ipc";
import { RESTORE_LIMITS_SUMMARY } from "../lib/restoreLimits";
import { formatRelativeTime } from "../lib/time";
import type { SavedContext, Workspace } from "../types/domain";
import { ElevatedCard } from "./ElevatedCard";
import { EmptyStructure } from "./EmptyStructure";
import { MomentCard } from "./MomentCard";
import { useWorkspaceComposition } from "./WorkspaceComposition";

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

  const stagger = reduceMotion
    ? undefined
    : {
        hidden: { opacity: 0 },
        show: {
          opacity: 1,
          transition: { staggerChildren: 0.06, delayChildren: 0.04 },
        },
      };
  const item = reduceMotion
    ? undefined
    : {
        hidden: { opacity: 0, y: 14, scale: 0.98 },
        show: {
          opacity: 1,
          y: 0,
          scale: 1,
          transition: { type: "spring" as const, stiffness: 340, damping: 30 },
        },
      };

  if (!workspace) {
    return (
      <section
        className="spatial-frame place place--empty"
        data-testid="workspace-home"
        data-density={density}
      >
        <motion.div
          className="place__identity"
          initial={reduceMotion ? false : { opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ type: "spring", stiffness: 280, damping: 28 }}
        >
          <p className="exp-kicker">Workspace</p>
          <h2 className="place__title">This is your Workspace</h2>
          <p className="place__pulse">Where your work lives.</p>
        </motion.div>
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
  const rest = recent.slice(1);
  const pulse =
    recent.length === 0
      ? "Ready for your first moment"
      : recent.length === 1
        ? `Last note · ${formatRelativeTime(recent[0].created_at)}`
        : `${recent.length} moments · last ${formatRelativeTime(recent[0].created_at)}`;

  return (
    <section
      className="spatial-frame place"
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
        <motion.div
          className="place__composition"
          variants={stagger}
          initial={reduceMotion ? false : "hidden"}
          animate="show"
        >
          <motion.div className="place__primary" variants={item} layout>
            <MomentCard
              variant="hero"
              context={latest}
              busy={busy}
              onContinue={() => onContinueContext(latest.id)}
              onInspect={onGoToContinue}
            />
          </motion.div>

          <aside className="place__context">
            <motion.div variants={item} layout>
              <ElevatedCard
                tone="float"
                elevation={3}
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
              </ElevatedCard>
            </motion.div>
            <motion.div variants={item}>
              <button
                type="button"
                className="exp-btn primary"
                onClick={onGoToSave}
                disabled={busy}
              >
                <BookmarkPlus size={16} aria-hidden="true" />
                Quick save
              </button>
            </motion.div>
          </aside>

          {density !== "focus" && (
            <motion.div
              className="dash-grid place__constellation"
              variants={stagger}
            >
              {rest.map((context, index) => (
                <motion.div
                  key={context.id}
                  className={index === 0 ? "span-6" : "span-3"}
                  variants={item}
                  layout
                >
                  <MomentCard
                    variant={
                      density === "flow"
                        ? index < 2
                          ? "standard"
                          : "compact"
                        : index === 0
                          ? "standard"
                          : "compact"
                    }
                    context={context}
                    busy={busy}
                    onContinue={() => onContinueContext(context.id)}
                  />
                </motion.div>
              ))}
              {rest.length < (density === "flow" ? 4 : 2) &&
                Array.from({
                  length: Math.max(
                    1,
                    (density === "flow" ? 4 : 2) - rest.length,
                  ),
                }).map((_, index) => (
                  <motion.div
                    key={`ph-${index}`}
                    className="span-3"
                    variants={item}
                  >
                    <MomentCard
                      variant="placeholder"
                      placeholderLabel="Open"
                      placeholderHint="Fills when you save"
                    />
                  </motion.div>
                ))}
            </motion.div>
          )}
        </motion.div>
      ) : (
        <motion.div
          className="place__composition place__composition--invite"
          variants={stagger}
          initial={reduceMotion ? false : "hidden"}
          animate="show"
        >
          <motion.div className="place__primary" variants={item} layout>
            <ElevatedCard
              tone="hero"
              elevation={3}
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
            </ElevatedCard>
          </motion.div>
          <aside className="place__context">
            <motion.div variants={item}>
              <ElevatedCard
                tone="float"
                elevation={3}
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
              </ElevatedCard>
            </motion.div>
            <motion.div variants={item}>
              <button
                type="button"
                className="exp-btn"
                onClick={onGoToSave}
                disabled={busy}
              >
                <BookmarkPlus size={16} aria-hidden="true" />
                Quick save
              </button>
            </motion.div>
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
        </motion.div>
      )}

      <p className="trust-strip muted">{RESTORE_LIMITS_SUMMARY}</p>
    </section>
  );
}
