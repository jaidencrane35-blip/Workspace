import { useState } from "react";
import { invokeIpc } from "../lib/ipc";
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
}: AssistantIntelligencePanelProps) {
  const [humanAsk, setHumanAsk] = useState(
    "What do we already know about this workspace?",
  );
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

  async function run(
    okMessage: string,
    action: () => Promise<void>,
  ): Promise<void> {
    onBusy(true);
    onError(null);
    try {
      await action();
      onMessage(okMessage);
    } catch (err: unknown) {
      onError(err instanceof Error ? err.message : String(err));
    } finally {
      onBusy(false);
    }
  }

  async function composePackages(): Promise<void> {
    if (!workspace) return;
    const ask = humanAsk.trim();
    if (!ask) {
      onError("Enter a human ask before composing packages.");
      return;
    }
    await run("Assistant intelligence packages composed (Batches 11–16).", async () => {
      const nextSurface =
        await invokeIpc<WorkspaceAssistantSurfaceProjection>(
          "compose_workspace_assistant_turn",
          { workspaceId: workspace.id, humanAsk: ask },
        );
      setSurface(nextSurface);

      const nextContext =
        await invokeIpc<WorkspaceAssistantContextProjection>(
          "package_workspace_assistant_context",
          { workspaceId: workspace.id },
        );
      setContext(nextContext);

      const nextRetrieval =
        await invokeIpc<WorkspaceAssistantRetrievalProjection>(
          "package_workspace_assistant_retrieval",
          { workspaceId: workspace.id, humanAsk: ask },
        );
      setRetrieval(nextRetrieval);

      const nextExplanation =
        await invokeIpc<WorkspaceAssistantExplanationProjection>(
          "package_workspace_assistant_explanation",
          { workspaceId: workspace.id, humanAsk: ask },
        );
      setExplanation(nextExplanation);

      const nextInteraction =
        await invokeIpc<WorkspaceAssistantInteractionProjection>(
          "package_workspace_assistant_interaction",
          { workspaceId: workspace.id, humanAsk: ask },
        );
      setInteraction(nextInteraction);

      const nextPersonalisation =
        await invokeIpc<WorkspaceAssistantPersonalisationProjection>(
          "package_workspace_assistant_personalisation",
          { workspaceId: workspace.id, humanAsk: ask },
        );
      setPersonalisation(nextPersonalisation);
    });
  }

  async function refreshProjections(): Promise<void> {
    if (!workspace) return;
    await run("Assistant intelligence projections refreshed.", async () => {
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
          { workspaceId: workspace.id },
        ),
        invokeIpc<WorkspaceAssistantContextProjection>(
          "get_workspace_assistant_context",
          { workspaceId: workspace.id },
        ),
        invokeIpc<WorkspaceAssistantRetrievalProjection>(
          "get_workspace_assistant_retrieval",
          { workspaceId: workspace.id },
        ),
        invokeIpc<WorkspaceAssistantExplanationProjection>(
          "get_workspace_assistant_explanation",
          { workspaceId: workspace.id },
        ),
        invokeIpc<WorkspaceAssistantInteractionProjection>(
          "get_workspace_assistant_interaction",
          { workspaceId: workspace.id },
        ),
        invokeIpc<WorkspaceAssistantPersonalisationProjection>(
          "get_workspace_assistant_personalisation",
          { workspaceId: workspace.id },
        ),
      ]);
      setSurface(nextSurface);
      setContext(nextContext);
      setRetrieval(nextRetrieval);
      setExplanation(nextExplanation);
      setInteraction(nextInteraction);
      setPersonalisation(nextPersonalisation);
    });
  }

  if (!workspace) {
    return (
      <div className="assistant-intel-panel">
        <header className="assistant-intel-hero">
          <p className="assistant-kicker">Programme IV · Batches 11–16</p>
          <h2>Assistant Intelligence</h2>
          <p className="lede">
            Presentation-only stack over recorded workspace evidence.
          </p>
        </header>
        <div className="assistant-intel-empty-block">
          <p className="assistant-intel-empty">No workspace selected.</p>
          <p className="assistant-intel-empty-hint">
            Open the Canvas tab, create or select a workspace, then return here
            to compose the six-layer intelligence stack.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="assistant-intel-panel">
      <header className="assistant-intel-hero">
        <p className="assistant-kicker">Programme IV · Batches 11–16</p>
        <h2>Assistant Intelligence</h2>
        <p className="lede">
          Surface → context → retrieval → explanation → interaction →
          personalisation. Composes existing engines via IPC — never approves,
          executes, or decides.
        </p>
      </header>

      <div className="assistant-intel-ask">
        <label htmlFor="assistant-intel-ask">Human ask</label>
        <textarea
          id="assistant-intel-ask"
          rows={3}
          value={humanAsk}
          disabled={busy}
          placeholder="What do we already know about this workspace?"
          onChange={(event) => setHumanAsk(event.target.value)}
        />
        <div className="assistant-intel-actions">
          <button
            type="button"
            disabled={busy || !humanAsk.trim()}
            onClick={() => void composePackages()}
          >
            Compose intelligence packages
          </button>
          <button
            type="button"
            className="secondary"
            disabled={busy}
            onClick={() => void refreshProjections()}
          >
            Refresh projections
          </button>
        </div>
      </div>

      {!hasAnyPackage ? (
        <p className="assistant-intel-empty">
          No packages yet. Compose intelligence packages to project Batches
          11–16, or refresh if packages already exist for this workspace.
        </p>
      ) : (
        <div className="assistant-intel-layers">
          <LayerProjection
            title="Assistant Surface"
            batchLabel="Batch 11"
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
            title="Assistant Context"
            batchLabel="Batch 12"
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
            title="Assistant Retrieval"
            batchLabel="Batch 13"
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
            title="Assistant Explanation"
            batchLabel="Batch 14"
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
            title="Assistant Interaction"
            batchLabel="Batch 15"
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
            title="Assistant Personalisation"
            batchLabel="Batch 16"
            projection={personalisation}
            nonCommandable={
              personalisation == null ||
              isAssistantPersonalisationProjectionNonCommandable(
                personalisation,
              )
            }
            historyAuthoritative={
              personalisation == null ||
              assistantPersonalisationHistoryCountIsAuthoritative(
                personalisation as unknown as WorkspaceAssistantPersonalisationSummary,
              )
            }
          />
        </div>
      )}
    </div>
  );
}
