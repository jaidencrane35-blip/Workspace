import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import {
  rankMomentsByRelevance,
  scoreMomentRelevance,
  type CognitiveMomentInput,
} from "../lib/cognitive";
import { invokeIpc } from "../lib/ipc";
import type { PilotPrimaryView } from "../lib/pilotChrome";
import type { SavedContext, Workspace } from "../types/domain";
import { useCognitiveEngine } from "./CognitiveEngine";
import { MomentCard, type MomentObjectState } from "./MomentCard";
import { useWorkspaceComposition } from "./WorkspaceComposition";

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
    reflectionDepth,
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
      list.map((c) => ({
        id: c.id,
        createdAt: c.created_at,
        windowCount: c.windows.length,
        handoffLength: c.handoff_note.trim().length,
        resumeAffinity: resumeAffinity[c.id] ?? 0,
        reflectionAffinity: reflectionDepth * 0.55,
      })),
    [resumeAffinity, reflectionDepth],
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
        const sorted = [...list].sort((a, b) =>
          b.created_at.localeCompare(a.created_at),
        );
        setContexts(sorted);
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
      return [] as Array<{ context: SavedContext; score: number }>;
    }
    const others = contexts.filter((c) => c.id !== primary.id);
    const ranked = rankMomentsByRelevance(toSignals(others));
    return ranked
      .map((entry) => {
        const context = others.find((c) => c.id === entry.id);
        return context ? { context, score: entry.score } : null;
      })
      .filter((entry): entry is { context: SavedContext; score: number } =>
        Boolean(entry),
      )
      .slice(0, 4);
  }, [contexts, primary, toSignals]);

  const neighbours = useMemo(
    () => neighbourRank.map((entry) => entry.context),
    [neighbourRank],
  );

  useEffect(() => {
    const map: Record<string, number> = {};
    for (const context of contexts) {
      map[context.id] = scoreMomentRelevance({
        id: context.id,
        createdAt: context.created_at,
        windowCount: context.windows.length,
        handoffLength: context.handoff_note.trim().length,
        resumeAffinity: resumeAffinity[context.id] ?? 0,
        reflectionAffinity: reflectionDepth * 0.55,
      });
    }
    setRelevanceMap(map);
  }, [contexts, resumeAffinity, reflectionDepth, setRelevanceMap]);

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

  const value = useMemo(
    () => ({
      primary,
      neighbours,
      neighbourScores,
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
 * Persistent Moment anchor — one object identity across destinations.
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
    presence,
    expanding,
    selectMoment,
  } = useActiveMoment();
  const registerHost = useContext(ExpandHostRegisterContext);

  if (!primary) {
    return null;
  }

  const showNeighbours =
    density !== "focus" &&
    neighbours.length > 0 &&
    presence === "presence";

  const showExpand =
    expanding ||
    presence === "writing" ||
    presence === "restoring" ||
    presence === "reflecting" ||
    presence === "guided";

  return (
    <div
      className="ws-object-stage"
      data-presence={presence}
      data-cognitive="on"
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
          className={`moment-object--anchor is-presence-${presence}`}
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
      {showNeighbours ? (
        <div
          className="home-field home-field--context home-field--waiting home-field--cognitive dash-grid ws-object-stage__neighbours"
          aria-label="Neighbouring moments"
        >
          {neighbours.slice(0, 3).map((context, index) => {
            const score = neighbourScores[context.id] ?? 0.3;
            return (
              <MomentCard
                key={context.id}
                variant="ambient"
                state="collapsed"
                attentionWeight={0.22 + score * 0.28}
                layoutId={`moment-neighbour-${context.id}`}
                className={`home-satellite home-satellite--${index % 3} moment-card--waiting`}
                context={context}
                busy={busy}
                onSelect={() => {
                  selectMoment(context.id);
                  onContinue?.(context.id);
                }}
                onContinue={() => onContinue?.(context.id)}
              />
            );
          })}
        </div>
      ) : null}
    </div>
  );
}
