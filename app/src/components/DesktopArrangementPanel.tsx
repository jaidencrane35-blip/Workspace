/**
 * Purpose: Optional remember/restore control for observed desktop layouts.
 * Owner: Frontend product shell (Product Contract V3)
 * Inputs: Active profile, busy/banner callbacks
 * Outputs: capture_desktop_arrangement / restore / list IPC
 * Dependencies: desktopArrangementUi helpers, list/details/diagnostics views
 * Non-goals: Competing with Stage as a co-primary column; setup-first forms
 */

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
  /** Notify Stage (or other observers) after restore mutates the desktop. */
  onDesktopChanged?: () => void;
}

export function DesktopArrangementPanel({
  workspace,
  busy,
  onBusy,
  onError,
  onMessage,
  onDesktopChanged,
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
        "Save and restore need the desktop app runtime.",
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
      onError("Create a named profile under Profiles to save this layout.");
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
    onBusy(true);
    onError(null);
    setLocalHint(null);
    void (async () => {
      try {
        const result = await invokeIpc<DesktopArrangementRestoreResult>(
          "restore_desktop_arrangement",
          {
            arrangementId: selected.id,
            focusFirst: true,
          },
        );
        setRestoreResult(result);
        onDesktopChanged?.();
        onMessage(
          result.outcomes.some((outcome) => outcome.simulated)
            ? "Restore finished (simulated)"
            : "Restore finished",
        );
      } catch (err: unknown) {
        const message = err instanceof Error ? err.message : String(err);
        onError(message);
        setLocalHint(permissionHintForError(message));
      } finally {
        onBusy(false);
      }
    })();
  };

  const summaryLabel = !workspace
    ? "Remember layout"
    : arrangements.length === 0
      ? "Remember layout"
      : arrangements.length === 1
        ? "Remember layout · 1 saved"
        : `Remember layout · ${arrangements.length} saved`;

  return (
    <details className="desktop-arrangement-panel compact-control">
      <summary>{summaryLabel}</summary>

      {!workspace ? (
        <p className="muted arrangement-empty-line">{empty.body}</p>
      ) : (
        <>
          <section aria-label="Saved arrangements">
            <div className="row section-heading-row">
              <h3>Saved</h3>
              <button
                type="button"
                className="ghost"
                disabled={busy || loading || !isIpcRuntimeAvailable()}
                onClick={() => {
                  void run("Arrangements refreshed", refreshList);
                }}
              >
                Refresh
              </button>
            </div>
            {loading && arrangements.length === 0 ? (
              <p className="muted">Loading…</p>
            ) : arrangements.length === 0 ? (
              <p className="muted arrangement-empty-line">{empty.body}</p>
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
                  Restore
                </button>
              </div>
            </section>
          ) : null}

          <details className="arrangement-save-details">
            <summary>Save current windows</summary>
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
            <button
              type="button"
              disabled={busy || !isIpcRuntimeAvailable()}
              onClick={captureArrangement}
            >
              Save
            </button>
          </details>

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
    </details>
  );
}
