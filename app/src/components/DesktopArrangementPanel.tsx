import { useCallback, useEffect, useState } from "react";
import { invokeIpc, isIpcRuntimeAvailable } from "../lib/ipc";
import {
  emptyArrangementsCopy,
  permissionHintForError,
} from "../lib/desktopArrangementUi";
import type {
  DesktopArrangement,
  DesktopArrangementRestoreResult,
} from "../types/desktopArrangement";
import type { Workspace } from "../types/domain";
import { DesktopArrangementDetails } from "./DesktopArrangementDetails";
import { DesktopArrangementList } from "./DesktopArrangementList";
import { RestoreDiagnosticsView } from "./RestoreDiagnosticsView";

interface DesktopArrangementPanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
}

export function DesktopArrangementPanel({
  workspace,
  busy,
  onBusy,
  onError,
  onMessage,
}: DesktopArrangementPanelProps) {
  const [arrangements, setArrangements] = useState<DesktopArrangement[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [loading, setLoading] = useState(false);
  const [restoreResult, setRestoreResult] =
    useState<DesktopArrangementRestoreResult | null>(null);
  const [localHint, setLocalHint] = useState<string | null>(null);

  const selected =
    arrangements.find((item) => item.id === selectedId) ?? null;

  const run = useCallback(
    async (okMessage: string, action: () => Promise<void>) => {
      onBusy(true);
      onError(null);
      setLocalHint(null);
      try {
        await action();
        onMessage(okMessage);
      } catch (err: unknown) {
        const message = err instanceof Error ? err.message : String(err);
        onError(message);
        setLocalHint(permissionHintForError(message));
      } finally {
        onBusy(false);
      }
    },
    [onBusy, onError, onMessage],
  );

  const refreshList = useCallback(async () => {
    if (!workspace) {
      setArrangements([]);
      setSelectedId(null);
      return;
    }
    if (!isIpcRuntimeAvailable()) {
      setArrangements([]);
      setLocalHint(
        "Desktop arrangements need the Workspace desktop runtime (Tauri). Browse UI still works; save and restore stay disabled until the app backend is available.",
      );
      return;
    }
    setLoading(true);
    try {
      const listed = await invokeIpc<DesktopArrangement[]>(
        "list_desktop_arrangements",
        {
          workspaceId: workspace.id,
          limit: 50,
        },
      );
      setArrangements(listed);
      setSelectedId((prev) => {
        if (prev && listed.some((item) => item.id === prev)) {
          return prev;
        }
        return listed[0]?.id ?? null;
      });
    } finally {
      setLoading(false);
    }
  }, [workspace]);

  useEffect(() => {
    void refreshList().catch((err: unknown) => {
      const message = err instanceof Error ? err.message : String(err);
      onError(message);
      setLocalHint(permissionHintForError(message));
    });
  }, [refreshList, onError]);

  const empty = emptyArrangementsCopy(Boolean(workspace));

  const captureArrangement = () => {
    if (!workspace) {
      onError("Create a workspace first.");
      return;
    }
    const trimmed = name.trim();
    if (!trimmed) {
      onError("Give this arrangement a name before saving.");
      return;
    }
    void run("Arrangement saved from current windows", async () => {
      const saved = await invokeIpc<DesktopArrangement>(
        "capture_desktop_arrangement",
        {
          workspaceId: workspace.id,
          name: trimmed,
          description: description.trim() || null,
          arrangementId: null,
          refreshObservation: true,
        },
      );
      setArrangements((prev) => {
        const without = prev.filter((item) => item.id !== saved.id);
        return [saved, ...without];
      });
      setSelectedId(saved.id);
      setRestoreResult(null);
      setName("");
      setDescription("");
    });
  };

  const restoreArrangement = () => {
    if (!selected) {
      onError("Select an arrangement to restore.");
      return;
    }
    void run("Restore finished", async () => {
      const result = await invokeIpc<DesktopArrangementRestoreResult>(
        "restore_desktop_arrangement",
        {
          arrangementId: selected.id,
          focusFirst: true,
        },
      );
      setRestoreResult(result);
    });
  };

  return (
    <aside
      className="desktop-arrangement-panel"
      aria-label="Desktop arrangements"
    >
      <header className="arrangement-panel-hero">
        <p className="arrangement-eyebrow">Desktop arrangements</p>
        <h2>Remember this desktop</h2>
        <p className="lede">
          Save open windows into a named arrangement, then restore when you
          want. Observation on Stage does not require this.
        </p>
      </header>

      {!workspace ? (
        <section className="arrangement-empty" aria-live="polite">
          <h3>{empty.title}</h3>
          <p className="muted">{empty.body}</p>
        </section>
      ) : (
        <>
          <section aria-label="Save arrangement">
            <h3>Save current windows</h3>
            <label className="arrangement-field">
              <span>Name</span>
              <input
                type="text"
                value={name}
                disabled={busy || !isIpcRuntimeAvailable()}
                placeholder="Focus coding"
                onChange={(event) => setName(event.target.value)}
              />
            </label>
            <label className="arrangement-field">
              <span>Description (optional)</span>
              <input
                type="text"
                value={description}
                disabled={busy || !isIpcRuntimeAvailable()}
                placeholder="Editor and browser"
                onChange={(event) => setDescription(event.target.value)}
              />
            </label>
            <div className="row">
              <button
                type="button"
                disabled={busy || !isIpcRuntimeAvailable()}
                onClick={captureArrangement}
              >
                Save arrangement
              </button>
              <button
                type="button"
                className="ghost"
                disabled={busy || loading || !isIpcRuntimeAvailable()}
                onClick={() => {
                  void run("Arrangements refreshed", refreshList);
                }}
              >
                Refresh list
              </button>
            </div>
          </section>

          <section aria-label="Saved arrangements">
            <h3>Saved arrangements</h3>
            {loading && arrangements.length === 0 ? (
              <p className="muted">Loading…</p>
            ) : arrangements.length === 0 ? (
              <div className="arrangement-empty" aria-live="polite">
                <h4>{empty.title}</h4>
                <p className="muted">{empty.body}</p>
              </div>
            ) : (
              <DesktopArrangementList
                arrangements={arrangements}
                selectedId={selectedId}
                busy={busy}
                onSelect={(id) => {
                  setSelectedId(id);
                  setRestoreResult(null);
                }}
              />
            )}
          </section>

          {selected ? (
            <section aria-label="Selected arrangement">
              <DesktopArrangementDetails arrangement={selected} />
              <div className="row arrangement-restore-row">
                <button
                  type="button"
                  disabled={busy || !isIpcRuntimeAvailable()}
                  onClick={restoreArrangement}
                >
                  Restore arrangement
                </button>
              </div>
              <p className="muted arrangement-permission-note">
                Restore runs through Permission Gateway (`desktop.restore`).
                Missing windows are reported — they are not replaced
                automatically.
              </p>
            </section>
          ) : null}

          {restoreResult ? (
            <RestoreDiagnosticsView result={restoreResult} />
          ) : null}
        </>
      )}

      {localHint ? (
        <p className="arrangement-hint muted" role="note">
          {localHint}
        </p>
      ) : null}
    </aside>
  );
}
