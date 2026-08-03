import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type CSSProperties,
  type ReactNode,
} from "react";
import {
  composeSemanticField,
  rankMomentsByRelevance,
  resolveTemporalPhase,
  scoreMomentRelevance,
  type CognitiveMomentInput,
  type SemanticPlacement,
  type TemporalPhase,
} from "../lib/cognitive";
import { invokeIpc } from "../lib/ipc";
import type { PilotPrimaryView } from "../lib/pilotChrome";
import type { SavedContext, Workspace } from "../types/domain";
import { useCognitiveEngine } from "./CognitiveEngine";
import { MomentCard, type MomentObjectState } from "./MomentCard";
import { useWorkspaceComposition } from "./WorkspaceComposition";

const GUIDE_LINES = {
  save: "Focus the note — writing expands from this Moment.",
  continue: "Remember this place — windows grow from the Moment itself.",
  checkin: "Reflect here — answers settle back into the object.",
} as const;

/** How the persistent Moment is currently being used. */
export type MomentPresence =
  | "presence"
  | "writing"
  | "restoring"
  | "reflecting"
  | "guided";

interface ActiveMomentValue {
  primary: SavedContext | null;
  neighbours: SavedContext[];
  /** Relevance scores for calm neighbour rebalancing (id → 0–1). */
  neighbourScores: Record<string, number>;
  /** Temporal phases for spatial / optical expression (id → phase). */
  neighbourPhases: Record<string, TemporalPhase>;
  primaryPhase: TemporalPhase | null;
  presence: MomentPresence;
  /** Host node inside the persistent Moment for destination attachments. */
  expandHost: HTMLDivElement | null;
  expanding: boolean;
  setPresence: (presence: MomentPresence) => void;
  setExpanding: (open: boolean) => void;
  selectMoment: (id: string) => void;
  reloadMoments: () => void;
}

const ActiveMomentContext = createContext<ActiveMomentValue | null>(null);

const ANCHOR_LAYOUT_ID = "workspace-anchor-moment";

export function ActiveMomentProvider({
  workspace,
  view,
  focusContextId,
  children,
}: {
  workspace: Workspace | null;
  view: PilotPrimaryView;
  focusContextId?: string | null;
  children: ReactNode;
}) {
  const {
    setPrimaryObject,
    setSecondaryObjects,
    setAmbient,
    setAttentionScene,
  } = useWorkspaceComposition();
  const {
    resumeAffinity,
    permanenceById,
    reflectionDepth,
    intentMemory,
    setRelevanceMap,
    noteResume,
  } = useCognitiveEngine();
  const [contexts, setContexts] = useState<SavedContext[]>([]);
  const [primaryId, setPrimaryId] = useState<string | null>(null);
  const [pinned, setPinned] = useState(false);
  const [presence, setPresence] = useState<MomentPresence>("presence");
  const [expanding, setExpanding] = useState(false);
  const [expandHost, setExpandHostState] = useState<HTMLDivElement | null>(
    null,
  );

  const toSignals = useCallback(
    (list: SavedContext[]): CognitiveMomentInput[] =>
      list.map((c) => {
        const resume = resumeAffinity[c.id] ?? 0;
        const permanence = permanenceById[c.id] ?? 0;
        // Reflection + intent memory reshape distance — never badges.
        const reflectionAffinity =
          reflectionDepth *
          (0.22 + resume * 0.45 + (c.windows.length > 0 ? 0.2 : 0));
        const memory =
          intentMemory *
          (0.2 + resume * 0.4 + permanence * 0.25 + (c.windows.length > 0 ? 0.15 : 0));
        return {
          id: c.id,
          createdAt: c.created_at,
          capturedAt: c.captured_at,
          windowCount: c.windows.length,
          handoffLength: c.handoff_note.trim().length,
          resumeAffinity: resume,
          reflectionAffinity,
          permanence,
          intentMemory: memory,
        };
      }),
    [resumeAffinity, permanenceById, reflectionDepth, intentMemory],
  );

  const setExpandHost = useCallback((node: HTMLDivElement | null) => {
    setExpandHostState(node);
  }, []);

  const reloadMoments = useCallback(() => {
    if (!workspace) {
      setContexts([]);
      return;
    }
    void invokeIpc<SavedContext[]>("list_saved_contexts", {
      workspaceId: workspace.id,
    })
      .then((list) => {
        // Stable load order — temporal ranking lives in toSignals / primary selection.
        setContexts(list);
      })
      .catch(() => {
        setContexts([]);
      });
  }, [workspace]);

  useEffect(() => {
    reloadMoments();
  }, [reloadMoments]);

  useEffect(() => {
    if (focusContextId && contexts.some((c) => c.id === focusContextId)) {
      setPrimaryId(focusContextId);
      setPinned(true);
      noteResume(focusContextId);
      return;
    }
    if (contexts.length === 0) {
      return;
    }
    if (pinned && primaryId && contexts.some((c) => c.id === primaryId)) {
      return;
    }
    // Self-organisation: recently resumed / relevant Moments return to centre.
    const ranked = rankMomentsByRelevance(toSignals(contexts));
    const centre = ranked[0]?.id ?? contexts[0]?.id ?? null;
    if (centre) {
      setPrimaryId(centre);
    }
  }, [
    contexts,
    focusContextId,
    primaryId,
    pinned,
    toSignals,
    noteResume,
  ]);

  useEffect(() => {
    setExpanding(false);
    setPinned(false);
    // Destinations refine presence; Save starts as place until writing begins.
    if (view === "save") {
      setPresence("presence");
    } else if (view === "resume") {
      setPresence((current) =>
        current === "restoring" ? current : "presence",
      );
    } else if (view === "pilot") {
      setPresence("reflecting");
    } else if (view === "help") {
      setPresence("guided");
    } else {
      setPresence("presence");
    }
  }, [view]);

  const primary = useMemo(
    () => contexts.find((c) => c.id === primaryId) ?? contexts[0] ?? null,
    [contexts, primaryId],
  );

  const neighbourRank = useMemo(() => {
    if (!primary) {
      return [] as Array<{
        context: SavedContext;
        score: number;
        phase: TemporalPhase;
      }>;
    }
    const others = contexts.filter((c) => c.id !== primary.id);
    const ranked = rankMomentsByRelevance(toSignals(others));
    return ranked
      .map((entry) => {
        const context = others.find((c) => c.id === entry.id);
        return context
          ? { context, score: entry.score, phase: entry.phase }
          : null;
      })
      .filter(
        (
          entry,
        ): entry is {
          context: SavedContext;
          score: number;
          phase: TemporalPhase;
        } => Boolean(entry),
      )
      .slice(0, 4);
  }, [contexts, primary, toSignals]);

  const neighbours = useMemo(
    () => neighbourRank.map((entry) => entry.context),
    [neighbourRank],
  );

  const primaryPhase = useMemo(() => {
    if (!primary) {
      return null;
    }
    const signal = toSignals([primary])[0];
    return signal ? resolveTemporalPhase(signal) : null;
  }, [primary, toSignals]);

  useEffect(() => {
    const map: Record<string, number> = {};
    for (const signal of toSignals(contexts)) {
      map[signal.id] = scoreMomentRelevance(signal);
    }
    setRelevanceMap(map);
  }, [contexts, toSignals, setRelevanceMap]);

  useEffect(() => {
    if (!primary) {
      setPrimaryObject(null);
      setSecondaryObjects([]);
      return;
    }
    setPrimaryObject(primary.id);
    setSecondaryObjects(neighbours.map((n) => n.id));
    setAmbient(presence === "writing" ? "input" : "moment");
    if (presence === "writing") {
      setAttentionScene("writing");
    } else if (presence === "restoring") {
      setAttentionScene("restore");
    } else if (presence === "reflecting") {
      setAttentionScene("checkin");
    } else if (presence === "guided") {
      setAttentionScene("guide");
    } else {
      setAttentionScene("default");
    }
  }, [
    primary,
    neighbours,
    presence,
    setPrimaryObject,
    setSecondaryObjects,
    setAmbient,
    setAttentionScene,
  ]);

  const selectMoment = useCallback((id: string) => {
    setPinned(true);
    setPrimaryId(id);
  }, []);

  const neighbourScores = useMemo(() => {
    const scores: Record<string, number> = {};
    for (const entry of neighbourRank) {
      scores[entry.context.id] = entry.score;
    }
    return scores;
  }, [neighbourRank]);

  const neighbourPhases = useMemo(() => {
    const phases: Record<string, TemporalPhase> = {};
    for (const entry of neighbourRank) {
      phases[entry.context.id] = entry.phase;
    }
    return phases;
  }, [neighbourRank]);

  const value = useMemo(
    () => ({
      primary,
      neighbours,
      neighbourScores,
      neighbourPhases,
      primaryPhase,
      presence,
      expandHost,
      expanding,
      setPresence,
      setExpanding,
      selectMoment,
      reloadMoments,
    }),
    [
      primary,
      neighbours,
      neighbourScores,
      neighbourPhases,
      primaryPhase,
      presence,
      expandHost,
      expanding,
      selectMoment,
      reloadMoments,
    ],
  );

  return (
    <ActiveMomentContext.Provider value={value}>
      <ExpandHostRegistrar register={setExpandHost}>
        {children}
      </ExpandHostRegistrar>
    </ActiveMomentContext.Provider>
  );
}

/** Lets PersistentMomentStage publish the expand host without circular hooks. */
const ExpandHostRegisterContext = createContext<
  ((node: HTMLDivElement | null) => void) | null
>(null);

function ExpandHostRegistrar({
  register,
  children,
}: {
  register: (node: HTMLDivElement | null) => void;
  children?: ReactNode;
}) {
  return (
    <ExpandHostRegisterContext.Provider value={register}>
      {children}
    </ExpandHostRegisterContext.Provider>
  );
}

export function useActiveMoment(): ActiveMomentValue {
  const value = useContext(ActiveMomentContext);
  if (!value) {
    return {
      primary: null,
      neighbours: [],
      neighbourScores: {},
      neighbourPhases: {},
      primaryPhase: null,
      presence: "presence",
      expandHost: null,
      expanding: false,
      setPresence: () => undefined,
      setExpanding: () => undefined,
      selectMoment: () => undefined,
      reloadMoments: () => undefined,
    };
  }
  return value;
}

function momentState(presence: MomentPresence): MomentObjectState {
  switch (presence) {
    case "writing":
      return "expanded";
    case "restoring":
      return "preview";
    case "reflecting":
      return "selected";
    case "guided":
      return "expanded";
    default:
      return "expanded";
  }
}

/**
 * Persistent Moment anchor — semantic field of related Moments.
 */
export function PersistentMomentStage({
  busy = false,
  onContinue,
}: {
  busy?: boolean;
  onContinue?: (id: string) => void;
}) {
  const { density } = useWorkspaceComposition();
  const {
    primary,
    neighbours,
    neighbourScores,
    neighbourPhases,
    primaryPhase,
    presence,
    expanding,
    selectMoment,
  } = useActiveMoment();
  const { guideDecision, preferHint } = useCognitiveEngine();
  const registerHost = useContext(ExpandHostRegisterContext);

  const placements = useMemo(() => {
    const ranked = neighbours.map((context) => ({
      id: context.id,
      score: neighbourScores[context.id] ?? 0.3,
      phase: neighbourPhases[context.id] ?? ("waiting" as TemporalPhase),
    }));
    ranked.sort((a, b) => b.score - a.score);
    return composeSemanticField(ranked, presence, 3);
  }, [neighbours, neighbourScores, neighbourPhases, presence]);

  const byId = useMemo(() => {
    const map = new Map<string, SavedContext>();
    for (const context of neighbours) {
      map.set(context.id, context);
    }
    return map;
  }, [neighbours]);

  if (!primary) {
    return null;
  }

  const showField = density !== "focus" && placements.length > 0;

  const showExpand =
    expanding ||
    presence === "writing" ||
    presence === "restoring" ||
    presence === "reflecting";

  const nearCluster = placements.filter((p) => p.band === "near");
  const guideLine =
    GUIDE_LINES[preferHint ?? guideDecision.hintId] ?? GUIDE_LINES.continue;

  return (
    <div
      className="ws-object-stage"
      data-presence={presence}
      data-cognitive="on"
      data-semantic="on"
      data-temporal={primaryPhase ?? "waiting"}
      data-material="place"
      data-testid="persistent-moment-stage"
    >
      <div className="ws-object-stage__anchor home-hero-band home-hero-band--living">
        <MomentCard
          variant="hero"
          state={momentState(presence)}
          context={primary}
          busy={busy}
          sparseMeta
          layoutId={ANCHOR_LAYOUT_ID}
          className={`moment-object--anchor is-presence-${presence} is-temporal-${primaryPhase ?? "waiting"}`}
          expandContent={
            showExpand ? (
              <div
                className="moment-attach-host"
                ref={registerHost}
                data-testid="moment-attach-host"
              />
            ) : undefined
          }
          quietActions={presence === "presence"}
          onSelect={() => selectMoment(primary.id)}
          onContinue={
            presence === "presence" && onContinue
              ? () => onContinue(primary.id)
              : undefined
          }
        />
      </div>
      {showField ? (
        <div
          className="semantic-field ws-object-stage__neighbours dash-grid"
          data-presence={presence}
          data-testid="semantic-field"
          aria-label="Related moments"
        >
          {presence === "guided" && guideDecision.show ? (
            <aside
              className="semantic-cluster__annotation"
              data-band={nearCluster[0]?.band ?? "near"}
              data-confidence={guideDecision.confidence.toFixed(2)}
              aria-label="Guide hint"
              style={{
                ...clusterAnnotationStyle(nearCluster[0] ?? placements[0]),
                opacity: Math.max(0.28, guideDecision.confidence),
              }}
            >
              <p className="moment-guide-hint__line">{guideLine}</p>
            </aside>
          ) : null}
          {placements.map((placement) => {
            const context = byId.get(placement.id);
            if (!context) {
              return null;
            }
            return (
              <div
                key={context.id}
                className="semantic-field__node"
                data-band={placement.band}
                data-temporal={placement.phase}
                data-proximity={placement.proximity.toFixed(2)}
                style={
                  {
                    "--sem-x": `${placement.x}px`,
                    "--sem-y": `${placement.y}px`,
                    "--sem-scale": String(placement.scale),
                    "--sem-opacity": String(placement.opacity),
                  } as CSSProperties
                }
              >
                <MomentCard
                  variant="ambient"
                  state="collapsed"
                  attentionWeight={0.2 + placement.proximity * 0.35}
                  layoutId={`moment-neighbour-${context.id}`}
                  className={`moment-card--waiting moment-card--semantic is-temporal-${placement.phase}`}
                  context={context}
                  busy={busy}
                  onSelect={() => {
                    selectMoment(context.id);
                    onContinue?.(context.id);
                  }}
                  onContinue={() => onContinue?.(context.id)}
                />
              </div>
            );
          })}
        </div>
      ) : null}
    </div>
  );
}

function clusterAnnotationStyle(
  placement: SemanticPlacement | undefined,
): CSSProperties {
  if (!placement) {
    return { left: "50%", top: "0.5rem" };
  }
  return {
    left: `calc(50% + ${Math.round(placement.x * 0.35)}px)`,
    top: `${Math.max(0, placement.y - 28)}px`,
  };
}
