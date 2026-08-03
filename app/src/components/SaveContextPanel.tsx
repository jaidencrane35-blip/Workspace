import { AnimatePresence, motion, useReducedMotion } from "motion/react";
import { useCallback, useEffect, useState } from "react";
import { createPortal } from "react-dom";
import { spring } from "../design-system";
import { invokeIpc } from "../lib/ipc";
import type {
  SavedContext,
  SavedContextCaptureScope,
  SavedContextWindow,
  Workspace,
} from "../types/domain";
import { useActiveMoment } from "./ActiveMoment";
import { RestoreLimitsNotice } from "./RestoreLimitsNotice";
import { useIntentEngine } from "./IntentEngine";
import { useWorkspaceComposition } from "./WorkspaceComposition";
import { WorkspaceSurface } from "./WorkspaceSurface";

type Step = "naming" | "saved";

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

function MomentWriteAttach({
  name,
  handoffNote,
  busy,
  density,
  scope,
  canReview,
  onName,
  onHandoff,
  onEnter,
  onLeave,
  onSave,
  onClear,
}: {
  name: string;
  handoffNote: string;
  busy: boolean;
  density: string;
  scope: SavedContextCaptureScope | null;
  canReview: boolean;
  onName: (value: string) => void;
  onHandoff: (value: string) => void;
  onEnter: () => void;
  onLeave: () => void;
  onSave: () => void;
  onClear: () => void;
}) {
  const reduceMotion = useReducedMotion();
  const active = Boolean(name.trim() || handoffNote.trim() || canReview);
  return (
    <div
      className="moment-write moment-attach"
      data-writing={active ? "on" : "idle"}
    >
      <button
        type="button"
        className="moment-write__invite"
        aria-hidden={active}
        tabIndex={active ? -1 : 0}
        onClick={() => {
          onEnter();
          document.getElementById("saved-context-handoff")?.focus();
        }}
      >
        Touch to leave a note
      </button>
      <div className="moment-write__tools">
        <label className="exp-field write-field" htmlFor="saved-context-name">
          <span className="sr-only">Name</span>
          <input
            id="saved-context-name"
            className="input-ghost write-name"
            value={name}
            placeholder="Name this moment"
            disabled={busy}
            onFocus={onEnter}
            onBlur={onLeave}
            onChange={(event) => {
              onEnter();
              onName(event.target.value);
            }}
          />
        </label>
        <label
          className="exp-field write-field write-card__anchor"
          htmlFor="saved-context-handoff"
        >
          <span className="sr-only">What next?</span>
          <textarea
            id="saved-context-handoff"
            className="input-ghost write-textarea"
            rows={density === "focus" ? 8 : 6}
            value={handoffNote}
            placeholder="What should future-you know?"
            disabled={busy}
            onFocus={onEnter}
            onBlur={onLeave}
            onChange={(event) => {
              onEnter();
              onHandoff(event.target.value);
            }}
          />
        </label>
        <p className="muted write-card__hint">
          You write this. Workspace will not invent or rewrite it.
        </p>
        {scope === null && (
          <p className="muted text-center">Checking capture scope…</p>
        )}
        <AnimatePresence>
          {canReview && scope && (
            <motion.div
              className="write-review"
              initial={reduceMotion ? false : { opacity: 0, height: 0 }}
              animate={{ opacity: 1, height: "auto" }}
              exit={reduceMotion ? undefined : { opacity: 0, height: 0 }}
              transition={spring.soft}
            >
              <details className="exp-inspect">
                <summary>What will be saved</summary>
                <p className="muted">{scope.purpose}</p>
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
              <div className="exp-actions write-card__actions">
                <button
                  type="button"
                  className="exp-btn primary"
                  onClick={onSave}
                  disabled={busy || !canReview}
                >
                  Save this context
                </button>
                <button
                  type="button"
                  className="exp-btn ghost"
                  onClick={onClear}
                  disabled={busy}
                >
                  Clear
                </button>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
}

export function SaveContextPanel({
  workspace,
  busy,
  onBusy,
  onError,
  onMessage,
  onCreateWorkspace,
}: SaveContextPanelProps) {
  const { density, setWritingMode, setAmbient } = useWorkspaceComposition();
  const { setWriting } = useIntentEngine();
  const {
    primary,
    expandHost,
    setPresence,
    setExpanding,
    reloadMoments,
    selectMoment,
  } = useActiveMoment();

  const leaveWriting = useCallback(() => {
    setWritingMode(false);
    setWriting(false);
    setAmbient("moment");
  }, [setWritingMode, setWriting, setAmbient]);

  const [scope, setScope] = useState<SavedContextCaptureScope | null>(null);
  const [scopeError, setScopeError] = useState<string | null>(null);
  const [name, setName] = useState("");
  const [handoffNote, setHandoffNote] = useState("");
  const [step, setStep] = useState<Step>("naming");
  const [saved, setSaved] = useState<SavedContext | null>(null);
  const [toolsOpen, setToolsOpen] = useState(false);

  useEffect(() => {
    invokeIpc<SavedContextCaptureScope>("get_saved_context_capture_scope")
      .then(setScope)
      .catch((err: unknown) => setScopeError(formatError(err)));
  }, []);

  const trimmedName = name.trim();
  const trimmedHandoff = handoffNote.trim();
  const canReview =
    scope !== null && trimmedName.length > 0 && trimmedHandoff.length > 0;
  const editing = toolsOpen || trimmedName.length > 0 || trimmedHandoff.length > 0;

  const attach =
    Boolean(workspace) &&
    !scopeError &&
    !(step === "saved" && saved) &&
    Boolean(primary);

  useEffect(() => {
    setExpanding(attach);
    if (attach) {
      setPresence(editing ? "writing" : "presence");
    }
    return () => setExpanding(false);
  }, [attach, editing, setExpanding, setPresence]);

  const enterWriting = useCallback(() => {
    setToolsOpen(true);
    setWritingMode(true);
    setWriting(true);
    setAmbient("input");
    setPresence("writing");
  }, [setWritingMode, setWriting, setAmbient, setPresence]);

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
        reloadMoments();
        selectMoment(context.id);
        setPresence("presence");
        setExpanding(false);
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
    reloadMoments,
    selectMoment,
    setPresence,
    setExpanding,
  ]);

  const startAgain = () => {
    setName("");
    setHandoffNote("");
    setSaved(null);
    setStep("naming");
    onMessage(null);
    onError(null);
  };

  const clearDraft = () => {
    setName("");
    setHandoffNote("");
    onError(null);
  };

  const writeForm = (
    <MomentWriteAttach
      name={name}
      handoffNote={handoffNote}
      busy={busy}
      density={density}
      scope={scope}
      canReview={canReview}
      onName={setName}
      onHandoff={setHandoffNote}
      onEnter={enterWriting}
      onLeave={leaveWriting}
      onSave={save}
      onClear={clearDraft}
    />
  );

  if (!workspace) {
    return (
      <section className="ws-region save-place save-env">
        <WorkspaceSurface level="floating" tone="hero" padding="xl" className="focus-card">
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
        </WorkspaceSurface>
      </section>
    );
  }

  if (scopeError) {
    return (
      <section className="ws-region save-place save-env">
        <WorkspaceSurface level="floating" tone="hero" padding="xl" className="focus-card">
          <p className="exp-kicker">Save</p>
          <h2 className="focus-card__title">Saving is unavailable</h2>
          <p className="error">{scopeError}</p>
        </WorkspaceSurface>
      </section>
    );
  }

  if (step === "saved" && saved) {
    return (
      <section className="ws-region save-place save-env">
        <p className="place__pulse text-center">
          Saved into this place · {formatMoment(saved.created_at)}
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
          <RestoreLimitsNotice />
        </details>
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
      </section>
    );
  }

  if (!primary) {
    return (
      <section
        className="ws-region save-place save-env save-env--write attention-field"
        data-density={density}
      >
        <header className="place__identity place__identity--quiet-region">
          <p className="exp-kicker">In {workspace.name}</p>
          <h2 className="place__title place__title--region">Leave a note</h2>
        </header>
        {writeForm}
      </section>
    );
  }

  return (
    <section
      className="ws-region save-place save-env save-env--attach"
      data-density={density}
      data-writing="ready"
    >
      <p className="sr-only">Writing expands inside the active Moment.</p>
      {expandHost ? createPortal(writeForm, expandHost) : null}
    </section>
  );
}
