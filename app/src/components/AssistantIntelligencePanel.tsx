/**
 * Purpose: ChatGPT-like Assistant companion — history, input, send, typing.
 * Owner: Frontend product shell (Product Contract V5)
 * Inputs: Active workspace (optional), busy/error/message, ensure-workspace
 * Outputs: compose_workspace_assistant_turn using observed desktop when available
 * Dependencies: Existing assistant surface IPC + get_workspace_state for context
 * Non-responsibilities: New AI engines, streaming IPC (none yet), OS control
 */

import { useEffect, useId, useMemo, useRef, useState } from "react";
import {
  appendCompanionRecentTurn,
  companionAnswerFromSurface,
  companionThreadTurns,
  enrichAskWithDesktopObservation,
  loadCompanionRecentTurns,
  type AssistantCompanionTurn,
} from "../lib/assistantCompanion";
import { invokeIpc, isIpcRuntimeAvailable } from "../lib/ipc";
import type {
  Workspace,
  WorkspaceAssistantContextProjection,
  WorkspaceAssistantContextSummary,
  WorkspaceAssistantExplanationProjection,
  WorkspaceAssistantExplanationSummary,
  WorkspaceAssistantInteractionProjection,
  WorkspaceAssistantInteractionSummary,
  WorkspaceAssistantPersonalisationProjection,
  WorkspaceAssistantPersonalisationSummary,
  WorkspaceAssistantRetrievalProjection,
  WorkspaceAssistantRetrievalSummary,
  WorkspaceAssistantSurfaceProjection,
  WorkspaceAssistantSurfaceSummary,
  WorkspaceState,
} from "../types/domain";
import {
  assistantContextHistoryCountIsAuthoritative,
  isAssistantContextProjectionNonCommandable,
} from "./assistantContextProjection";
import {
  assistantExplanationHistoryCountIsAuthoritative,
  isAssistantExplanationProjectionNonCommandable,
} from "./assistantExplanationProjection";
import {
  assistantInteractionHistoryCountIsAuthoritative,
  isAssistantInteractionProjectionNonCommandable,
} from "./assistantInteractionProjection";
import {
  assistantPersonalisationHistoryCountIsAuthoritative,
  isAssistantPersonalisationProjectionNonCommandable,
} from "./assistantPersonalisationProjection";
import {
  assistantRetrievalHistoryCountIsAuthoritative,
  isAssistantRetrievalProjectionNonCommandable,
} from "./assistantRetrievalProjection";
import {
  assistantSurfaceHistoryCountIsAuthoritative,
  isAssistantSurfaceProjectionNonCommandable,
} from "./assistantSurfaceProjection";

interface AssistantIntelligencePanelProps {
  workspace: Workspace | null;
  busy: boolean;
  onBusy: (busy: boolean) => void;
  onError: (message: string | null) => void;
  onMessage: (message: string | null) => void;
  /** Quietly create/select a profile when compose needs a workspace id. */
  onEnsureWorkspace?: () => Promise<Workspace>;
  /** Rail omits duplicate hero chrome; conversation stays primary. */
  presentation?: "standalone" | "rail";
}

type GapLike = {
  gap_kind: string;
  description: string;
};

type SnapshotLike = {
  narrative_summary: string;
  narrative: string;
  gaps: GapLike[];
  diagnostics: { notes: string[] };
  authority_effect: string;
  actionable: boolean;
};

type ProjectionLike = {
  current: SnapshotLike | null;
  history: unknown[];
  history_count: number;
  authority_effect: string;
  projected_at: string;
};

function LayerProjection({
  title,
  batchLabel,
  projection,
  nonCommandable,
  historyAuthoritative,
}: {
  title: string;
  batchLabel: string;
  projection: ProjectionLike | null;
  nonCommandable: boolean;
  historyAuthoritative: boolean;
}) {
  if (!projection) {
    return (
      <section className="assistant-intel-layer">
        <h3>
          {title}{" "}
          <span className="assistant-intel-batch">{batchLabel}</span>
        </h3>
        <p className="assistant-intel-empty">No package yet.</p>
      </section>
    );
  }

  const current = projection.current;

  return (
    <section className="assistant-intel-layer">
      <h3>
        {title} <span className="assistant-intel-batch">{batchLabel}</span>
      </h3>
      {!nonCommandable ? (
        <p className="assistant-intel-warn" role="alert">
          Warning: projection appears commandable — Programme IV packages must
          remain presentation-only (actionable=false, authority_effect=none).
        </p>
      ) : null}
      <dl className="assistant-intel-meta">
        <dt>authority_effect</dt>
        <dd>
          <code>{projection.authority_effect}</code>
        </dd>
        <dt>history_count</dt>
        <dd>
          {projection.history_count}
          {historyAuthoritative ? (
            <span className="assistant-intel-note"> (authoritative)</span>
          ) : (
            <span className="assistant-intel-warn-inline">
              {" "}
              (not authoritative vs history length)
            </span>
          )}
        </dd>
        <dt>projected_at</dt>
        <dd>{projection.projected_at}</dd>
      </dl>
      {!current ? (
        <p className="assistant-intel-empty">No current snapshot.</p>
      ) : (
        <div className="assistant-intel-snapshot">
          <div className="assistant-intel-badges">
            <span
              className={
                current.actionable
                  ? "badge assistant-intel-badge-warn"
                  : "badge"
              }
            >
              actionable={String(current.actionable)}
            </span>
            <span className="badge">
              authority_effect={current.authority_effect}
            </span>
          </div>
          <p className="assistant-intel-summary">{current.narrative_summary}</p>
          <p className="assistant-intel-narrative">{current.narrative}</p>
          {current.gaps.length > 0 ? (
            <div className="assistant-intel-gaps">
              <h4>Gaps</h4>
              <ul>
                {current.gaps.map((gap, index) => (
                  <li key={`${gap.gap_kind}-${index}`}>
                    <strong>{gap.gap_kind}</strong> — {gap.description}
                  </li>
                ))}
              </ul>
            </div>
          ) : (
            <p className="assistant-intel-empty">No gaps recorded.</p>
          )}
          {current.diagnostics.notes.length > 0 ? (
            <div className="assistant-intel-diagnostics">
              <h4>Diagnostics</h4>
              <ul>
                {current.diagnostics.notes.map((note, index) => (
                  <li key={`${note}-${index}`}>{note}</li>
                ))}
              </ul>
            </div>
          ) : null}
        </div>
      )}
    </section>
  );
}

export function AssistantIntelligencePanel({
  workspace,
  busy,
  onBusy,
  onError,
  onMessage,
  onEnsureWorkspace,
  presentation = "standalone",
}: AssistantIntelligencePanelProps) {
  const rail = presentation === "rail";
  const askId = useId();
  const threadRef = useRef<HTMLDivElement | null>(null);
  const [humanAsk, setHumanAsk] = useState("");
  const [sending, setSending] = useState(false);
  const [packagesOpen, setPackagesOpen] = useState(false);
  const [recent, setRecent] = useState<AssistantCompanionTurn[]>(() =>
    loadCompanionRecentTurns(),
  );
  const [pendingAsk, setPendingAsk] = useState<string | null>(null);
  const [surface, setSurface] =
    useState<WorkspaceAssistantSurfaceProjection | null>(null);
  const [context, setContext] =
    useState<WorkspaceAssistantContextProjection | null>(null);
  const [retrieval, setRetrieval] =
    useState<WorkspaceAssistantRetrievalProjection | null>(null);
  const [explanation, setExplanation] =
    useState<WorkspaceAssistantExplanationProjection | null>(null);
  const [interaction, setInteraction] =
    useState<WorkspaceAssistantInteractionProjection | null>(null);
  const [personalisation, setPersonalisation] =
    useState<WorkspaceAssistantPersonalisationProjection | null>(null);

  const hasAnyPackage =
    surface != null ||
    context != null ||
    retrieval != null ||
    explanation != null ||
    interaction != null ||
    personalisation != null;

  const thread = useMemo(() => companionThreadTurns(recent), [recent]);
  const inputBusy = busy || sending;

  useEffect(() => {
    setRecent(loadCompanionRecentTurns());
  }, []);

  useEffect(() => {
    const node = threadRef.current;
    if (!node) {
      return;
    }
    node.scrollTop = node.scrollHeight;
  }, [thread, pendingAsk, sending]);

  async function run(
    okMessage: string | null,
    action: () => Promise<void>,
  ): Promise<void> {
    onBusy(true);
    onError(null);
    try {
      await action();
      if (okMessage) {
        onMessage(okMessage);
      } else {
        onMessage(null);
      }
    } catch (err: unknown) {
      onError(err instanceof Error ? err.message : String(err));
    } finally {
      onBusy(false);
    }
  }

  async function resolveWorkspace(): Promise<Workspace | null> {
    if (workspace) {
      return workspace;
    }
    if (!onEnsureWorkspace) {
      return null;
    }
    return onEnsureWorkspace();
  }

  async function readDesktopObservation(): Promise<WorkspaceState | null> {
    if (!isIpcRuntimeAvailable()) {
      return null;
    }
    try {
      try {
        await invokeIpc("ensure_observation_freshness", {
          consumerId: "workspace_assistant",
        });
      } catch {
        // Freshness is best-effort before reading projected desktop state.
      }
      return await invokeIpc<WorkspaceState>("get_workspace_state");
    } catch {
      return null;
    }
  }

  async function sendAsk(): Promise<void> {
    const ask = humanAsk.trim();
    if (!ask) {
      onError("Type a question first.");
      return;
    }
    setSending(true);
    setPendingAsk(ask);
    setHumanAsk("");
    try {
      await run(null, async () => {
        const active = await resolveWorkspace();
        if (!active) {
          throw new Error(
            "Assistant needs a quiet Desktop profile once to store answers. Create one under Profiles, or open the desktop app.",
          );
        }
        const desktop = await readDesktopObservation();
        const composedAsk = enrichAskWithDesktopObservation(ask, desktop);
        const nextSurface =
          await invokeIpc<WorkspaceAssistantSurfaceProjection>(
            "compose_workspace_assistant_turn",
            { workspaceId: active.id, humanAsk: composedAsk },
          );
        setSurface(nextSurface);
        const answer = companionAnswerFromSurface(nextSurface);
        setRecent(
          appendCompanionRecentTurn({
            id:
              nextSurface.current?.utterance?.utterance_id ??
              nextSurface.current?.surface_id ??
              `${Date.now()}`,
            ask,
            answer,
            at: nextSurface.current?.generated_at ?? new Date().toISOString(),
          }),
        );
      });
    } finally {
      setPendingAsk(null);
      setSending(false);
    }
  }

  async function refreshEvidencePackages(): Promise<void> {
    const active = workspace;
    if (!active) {
      onError("Select a profile to refresh evidence packages.");
      return;
    }
    await run("Evidence packages refreshed.", async () => {
      const [
        nextSurface,
        nextContext,
        nextRetrieval,
        nextExplanation,
        nextInteraction,
        nextPersonalisation,
      ] = await Promise.all([
        invokeIpc<WorkspaceAssistantSurfaceProjection>(
          "get_workspace_assistant_surface",
          { workspaceId: active.id },
        ),
        invokeIpc<WorkspaceAssistantContextProjection>(
          "get_workspace_assistant_context",
          { workspaceId: active.id },
        ),
        invokeIpc<WorkspaceAssistantRetrievalProjection>(
          "get_workspace_assistant_retrieval",
          { workspaceId: active.id },
        ),
        invokeIpc<WorkspaceAssistantExplanationProjection>(
          "get_workspace_assistant_explanation",
          { workspaceId: active.id },
        ),
        invokeIpc<WorkspaceAssistantInteractionProjection>(
          "get_workspace_assistant_interaction",
          { workspaceId: active.id },
        ),
        invokeIpc<WorkspaceAssistantPersonalisationProjection>(
          "get_workspace_assistant_personalisation",
          { workspaceId: active.id },
        ),
      ]);
      setSurface(nextSurface);
      setContext(nextContext);
      setRetrieval(nextRetrieval);
      setExplanation(nextExplanation);
      setInteraction(nextInteraction);
      setPersonalisation(nextPersonalisation);
      if (nextSurface?.current) {
        const ask = nextSurface.current.human_ask?.trim() || null;
        const answer = companionAnswerFromSurface(nextSurface);
        if (ask && answer) {
          setRecent(
            appendCompanionRecentTurn({
              id:
                nextSurface.current.utterance?.utterance_id ??
                nextSurface.current.surface_id ??
                `${Date.now()}`,
              ask,
              answer,
              at: nextSurface.current.generated_at ?? new Date().toISOString(),
            }),
          );
        }
      }
      setPackagesOpen(true);
    });
  }

  const layers = (
    <div className="assistant-intel-layers">
      <LayerProjection
        title="Surface"
        batchLabel="Layer 1"
        projection={surface}
        nonCommandable={
          surface == null ||
          isAssistantSurfaceProjectionNonCommandable(surface)
        }
        historyAuthoritative={
          surface == null ||
          assistantSurfaceHistoryCountIsAuthoritative(
            surface as unknown as WorkspaceAssistantSurfaceSummary,
          )
        }
      />
      <LayerProjection
        title="Context"
        batchLabel="Layer 2"
        projection={context}
        nonCommandable={
          context == null ||
          isAssistantContextProjectionNonCommandable(context)
        }
        historyAuthoritative={
          context == null ||
          assistantContextHistoryCountIsAuthoritative(
            context as unknown as WorkspaceAssistantContextSummary,
          )
        }
      />
      <LayerProjection
        title="Retrieval"
        batchLabel="Layer 3"
        projection={retrieval}
        nonCommandable={
          retrieval == null ||
          isAssistantRetrievalProjectionNonCommandable(retrieval)
        }
        historyAuthoritative={
          retrieval == null ||
          assistantRetrievalHistoryCountIsAuthoritative(
            retrieval as unknown as WorkspaceAssistantRetrievalSummary,
          )
        }
      />
      <LayerProjection
        title="Explanation"
        batchLabel="Layer 4"
        projection={explanation}
        nonCommandable={
          explanation == null ||
          isAssistantExplanationProjectionNonCommandable(explanation)
        }
        historyAuthoritative={
          explanation == null ||
          assistantExplanationHistoryCountIsAuthoritative(
            explanation as unknown as WorkspaceAssistantExplanationSummary,
          )
        }
      />
      <LayerProjection
        title="Interaction"
        batchLabel="Layer 5"
        projection={interaction}
        nonCommandable={
          interaction == null ||
          isAssistantInteractionProjectionNonCommandable(interaction)
        }
        historyAuthoritative={
          interaction == null ||
          assistantInteractionHistoryCountIsAuthoritative(
            interaction as unknown as WorkspaceAssistantInteractionSummary,
          )
        }
      />
      <LayerProjection
        title="Personalisation"
        batchLabel="Layer 6"
        projection={personalisation}
        nonCommandable={
          personalisation == null ||
          isAssistantPersonalisationProjectionNonCommandable(personalisation)
        }
        historyAuthoritative={
          personalisation == null ||
          assistantPersonalisationHistoryCountIsAuthoritative(
            personalisation as unknown as WorkspaceAssistantPersonalisationSummary,
          )
        }
      />
    </div>
  );

  return (
    <div
      className={
        rail
          ? "assistant-intel-panel presentation-rail"
          : "assistant-intel-panel"
      }
    >
      {rail ? null : (
        <header className="assistant-intel-hero">
          <p className="assistant-kicker">Supporting companion</p>
          <h2>Ask about your desktop</h2>
          <p className="lede">
            Lightweight help from what Workspace already observes. Stage stays
            primary.
          </p>
        </header>
      )}

      <div className="assistant-companion-chat" aria-label="Assistant conversation">
        <div
          ref={threadRef}
          className="assistant-companion-thread"
          aria-live="polite"
        >
          {thread.length === 0 && !pendingAsk ? (
            <p className="assistant-companion-quiet muted">
              Ask about what is already on your desktop. Answers stay beside
              Stage — they never replace it.
            </p>
          ) : (
            thread.map((turn) => (
              <article key={turn.id} className="assistant-companion-turn">
                <p className="assistant-companion-ask">{turn.ask}</p>
                <p className="assistant-companion-answer">{turn.answer}</p>
              </article>
            ))
          )}
          {pendingAsk ? (
            <article
              className="assistant-companion-turn assistant-companion-pending"
              aria-busy="true"
            >
              <p className="assistant-companion-ask">{pendingAsk}</p>
              <p
                className="assistant-companion-typing muted"
                aria-label="Assistant is responding"
              >
                <span className="assistant-typing-dot" />
                <span className="assistant-typing-dot" />
                <span className="assistant-typing-dot" />
              </p>
            </article>
          ) : null}
        </div>

        <div className="assistant-intel-ask">
          <label htmlFor={askId}>Message</label>
          <textarea
            id={askId}
            rows={rail ? 2 : 3}
            value={humanAsk}
            disabled={inputBusy}
            placeholder="What is open on my desktop?"
            onChange={(event) => setHumanAsk(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === "Enter" && !event.shiftKey) {
                event.preventDefault();
                if (!inputBusy && humanAsk.trim()) {
                  void sendAsk();
                }
              }
            }}
          />
          <div className="assistant-intel-actions">
            <button
              type="button"
              disabled={inputBusy || !humanAsk.trim()}
              onClick={() => void sendAsk()}
            >
              Send
            </button>
          </div>
        </div>
      </div>

      {rail ? null : (
        <div className="assistant-intel-packages">
          <button
            type="button"
            className="ghost assistant-packages-toggle"
            aria-expanded={packagesOpen}
            onClick={() => setPackagesOpen((open) => !open)}
          >
            {packagesOpen ? "Hide evidence packages" : "Evidence packages"}
          </button>
          {packagesOpen ? (
            <>
              <div className="assistant-intel-actions">
                <button
                  type="button"
                  className="secondary"
                  disabled={busy || !workspace}
                  onClick={() => void refreshEvidencePackages()}
                >
                  Refresh packages
                </button>
              </div>
              {hasAnyPackage ? (
                layers
              ) : (
                <p className="assistant-intel-empty">
                  No packages loaded. Refresh after sending an ask if you need
                  diagnostic layers.
                </p>
              )}
            </>
          ) : null}
        </div>
      )}
    </div>
  );
}
