import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import { invokeIpc } from "../lib/ipc";
import type { PilotPrimaryView } from "../lib/pilotChrome";
import type { SavedContext, Workspace } from "../types/domain";
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
  const [contexts, setContexts] = useState<SavedContext[]>([]);
  const [primaryId, setPrimaryId] = useState<string | null>(null);
  const [presence, setPresence] = useState<MomentPresence>("presence");
  const [expanding, setExpanding] = useState(false);
  const [expandHost, setExpandHostState] = useState<HTMLDivElement | null>(
    null,
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
      return;
    }
    if (!primaryId && contexts[0]) {
      setPrimaryId(contexts[0].id);
      return;
    }
    if (primaryId && !contexts.some((c) => c.id === primaryId) && contexts[0]) {
      setPrimaryId(contexts[0].id);
    }
  }, [contexts, focusContextId, primaryId]);

  useEffect(() => {
    setExpanding(false);
    if (view === "save") {
      setPresence("writing");
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
  const neighbours = useMemo(
    () => contexts.filter((c) => c.id !== primary?.id).slice(0, 4),
    [contexts, primary],
  );

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
    setPrimaryId(id);
  }, []);

  const value = useMemo(
    () => ({
      primary,
      neighbours,
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
  const { primary, neighbours, presence, expanding, selectMoment } =
    useActiveMoment();
  const registerHost = useContext(ExpandHostRegisterContext);

  if (!primary) {
    return null;
  }

  const showNeighbours =
    density !== "focus" &&
    neighbours.length > 0 &&
    presence !== "writing" &&
    presence !== "restoring";

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
          className="home-field home-field--context home-field--waiting dash-grid ws-object-stage__neighbours"
          aria-label="Neighbouring moments"
        >
          {neighbours.slice(0, 3).map((context, index) => (
            <MomentCard
              key={context.id}
              variant="ambient"
              state="collapsed"
              attentionWeight={0.34 - index * 0.04}
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
          ))}
        </div>
      ) : null}
    </div>
  );
}
