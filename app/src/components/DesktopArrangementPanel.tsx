/**
 * Purpose: Arrangements control — Save, Update, Restore + operational explanations.
 * Owner: Frontend product shell (Programme I IC6)
 * Inputs: Active profile, busy/banner callbacks; WorkspaceState for derived meta
 * Outputs: capture_desktop_arrangement / restore / list IPC
 * Dependencies: arrangementProductUi, operationalConfidenceUi, list/details/diagnostics
 * Non-goals: Competing with Desktop as primary surface; onboarding persistence;
 *   parallel arrangement models; notification/event engines; explanation caches
 */

import { useCallback, useEffect, useMemo, useState } from "react";
import {
  arrangementProductWorkflowHint,
  arrangementSavedFeedback,
  arrangementUpdatedFeedback,
  desktopFirstUseGuidance,
} from "../lib/arrangementProductUi";
import {
  emptyArrangementsCopy,
  permissionHintForError,
} from "../lib/desktopArrangementUi";
import { invokeIpc, isIpcRuntimeAvailable } from "../lib/ipc";
import {
  WORKSPACE_RESTORE_VERB,
  WORKSPACE_SAVE_VERB,
  WORKSPACE_UPDATE_VERB,
} from "../lib/layoutsStageUi";
import {
  arrangementRestoredExplanationMessage,
  explainArrangementCurrency,
  explainPreRestore,
} from "../lib/operationalConfidenceUi";
import { useObservedWorkspaceState } from "../lib/useObservedWorkspaceState";
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
  const { workspaceState } = useObservedWorkspaceState();
  const observedWindows = workspaceState?.windows ?? [];
  const [arrangements, setArrangements] = useState<DesktopArrangement[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [loading, setLoading] = useState(false);
  const [restoreResult, setRestoreResult] =
    useState<DesktopArrangementRestoreResult | null>(null);
  const [localHint, setLocalHint] = useState<string | null>(null);
  /** Interaction State only — never persisted (IC5). */
  const [guidanceDismissed, setGuidanceDismissed] = useState(false);

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
      setLocalHint("Save and Restore need the desktop app runtime.");
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

  useEffect(() => {
    if (arrangements.length > 0) {
      setGuidanceDismissed(false);
    }
  }, [arrangements.length]);

  const empty = emptyArrangementsCopy(Boolean(workspace));
  const guidance =
    !guidanceDismissed &&
    desktopFirstUseGuidance({
      hasProfile: Boolean(workspace),
      arrangementCount: arrangements.length,
      desktopReady: Boolean(workspaceState?.windows?.length),
    });

  const currencyExplanation = useMemo(() => {
    if (!selected) {
      return null;
    }
    return explainArrangementCurrency(selected, observedWindows);
  }, [selected, observedWindows]);

  const preRestoreExplanation = useMemo(() => {
    if (!selected) {
      return null;
    }
    return explainPreRestore(selected, observedWindows);
  }, [selected, observedWindows]);

  const captureArrangement = () => {
    if (!workspace) {
      onError("Choose a Profile to Save an Arrangement.");
      return;
    }
    const trimmed = name.trim();
    if (!trimmed) {
      onError("Give this Arrangement a name before Save.");
      return;
    }
    onBusy(true);
    onError(null);
    setLocalHint(null);
    void (async () => {
      try {
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
        onMessage(arrangementSavedFeedback(saved));
      } catch (err: unknown) {
        const message = err instanceof Error ? err.message : String(err);
        onError(message);
        setLocalHint(permissionHintForError(message));
      } finally {
        onBusy(false);
      }
    })();
  };

  const updateSelectedArrangement = () => {
    if (!workspace) {
      onError("Choose a Profile to Update an Arrangement.");
      return;
    }
    if (!selected) {
      onError("Select an Arrangement to Update.");
      return;
    }
    onBusy(true);
    onError(null);
    setLocalHint(null);
    void (async () => {
      try {
        const saved = await invokeIpc<DesktopArrangement>(
          "capture_desktop_arrangement",
          {
            workspaceId: workspace.id,
            name: selected.name,
            description: selected.description || null,
            arrangementId: selected.id,
            refreshObservation: true,
          },
        );
        setArrangements((prev) => {
          const without = prev.filter((item) => item.id !== saved.id);
          return [saved, ...without];
        });
        setSelectedId(saved.id);
        setRestoreResult(null);
        onMessage(arrangementUpdatedFeedback(saved));
      } catch (err: unknown) {
        const message = err instanceof Error ? err.message : String(err);
        onError(message);
        setLocalHint(permissionHintForError(message));
      } finally {
        onBusy(false);
      }
    })();
  };

  const restoreArrangement = () => {
    if (!selected) {
      onError("Select an Arrangement to Restore.");
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
          arrangementRestoredExplanationMessage(selected.name, result),
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
    ? "Arrangements"
    : arrangements.length === 0
      ? "Arrangements"
      : arrangements.length === 1
        ? "Arrangements · 1 saved"
        : `Arrangements · ${arrangements.length} saved`;

  return (
    <details className="desktop-arrangement-panel compact-control">
      <summary>{summaryLabel}</summary>

      {!workspace ? (
        <p className="muted arrangement-empty-line">{empty.body}</p>
      ) : (
        <>
          <p className="muted arrangement-workflow-hint">
            {arrangementProductWorkflowHint()}
          </p>

          {guidance && arrangements.length === 0 ? (
            <div className="arrangement-first-use" role="note">
              <p className="arrangement-first-use-title">{guidance.title}</p>
              <p className="muted arrangement-first-use-body">{guidance.body}</p>
              <button
                type="button"
                className="ghost"
                disabled={busy}
                onClick={() => {
                  setGuidanceDismissed(true);
                }}
              >
                Dismiss
              </button>
            </div>
          ) : null}

          <section aria-label="Saved Arrangements">
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
                observedWindows={observedWindows}
                onSelect={(id) => {
                  setSelectedId(id);
                  setRestoreResult(null);
                }}
              />
            )}
          </section>

          {selected ? (
            <section aria-label="Selected Arrangement">
              <DesktopArrangementDetails
                arrangement={selected}
                observedWindows={observedWindows}
              />
              {currencyExplanation ? (
                <div
                  className="arrangement-operational-explain"
                  data-arrangement-currency={currencyExplanation.currency}
                >
                  <p className="arrangement-details-meta">
                    {currencyExplanation.line}
                  </p>
                  <p className="muted arrangement-details-meta">
                    Since capture · {currencyExplanation.changeLine}
                  </p>
                  {preRestoreExplanation ? (
                    <>
                      <p className="arrangement-details-meta">
                        {preRestoreExplanation.line}
                      </p>
                      <ul className="arrangement-prerestore-list muted">
                        {preRestoreExplanation.bullets.map((bullet) => (
                          <li key={bullet}>{bullet}</li>
                        ))}
                      </ul>
                    </>
                  ) : null}
                </div>
              ) : null}
              <div className="row arrangement-restore-row">
                <button
                  type="button"
                  disabled={
                    busy ||
                    !isIpcRuntimeAvailable() ||
                    !(preRestoreExplanation?.available ?? false)
                  }
                  title={preRestoreExplanation?.summaryLine}
                  onClick={restoreArrangement}
                >
                  {WORKSPACE_RESTORE_VERB}
                </button>
                <button
                  type="button"
                  className="ghost"
                  disabled={busy || !isIpcRuntimeAvailable()}
                  onClick={updateSelectedArrangement}
                >
                  {WORKSPACE_UPDATE_VERB}
                </button>
              </div>
            </section>
          ) : null}

          <details className="arrangement-save-details">
            <summary>Save current windows as Arrangement</summary>
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
              {WORKSPACE_SAVE_VERB}
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
