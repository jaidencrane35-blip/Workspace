import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import type { WorkspaceDensity } from "../lib/density";
import type { PilotPrimaryView } from "../lib/pilotChrome";
import { AttentionEngineProvider, useAttentionEngine } from "./AttentionEngine";
import type { AmbientFocus } from "./AmbientLighting";
import type { AttentionScene } from "../lib/attention";

interface WorkspaceCompositionValue {
  density: WorkspaceDensity;
  destination: PilotPrimaryView;
  writingMode: boolean;
  ambient: AmbientFocus;
  focusedObjectId: string | null;
  selectedObjectId: string | null;
  attentionScene: AttentionScene;
  primaryObjectId: string | null;
  setDestination: (view: PilotPrimaryView) => void;
  setWritingMode: (on: boolean) => void;
  setAmbient: (focus: AmbientFocus) => void;
  setFocusedObjectId: (id: string | null) => void;
  setSelectedObjectId: (id: string | null) => void;
  setPrimaryObject: (id: string | null) => void;
  setSecondaryObjects: (ids: string[]) => void;
  setAttentionScene: (scene: AttentionScene) => void;
}

const WorkspaceCompositionContext =
  createContext<WorkspaceCompositionValue | null>(null);

function CompositionInner({
  density,
  children,
}: {
  density: WorkspaceDensity;
  children: ReactNode;
}) {
  const {
    scene,
    primaryObjectId,
    setScene,
    setPrimary,
    setSecondary,
  } = useAttentionEngine();
  const [destination, setDestination] = useState<PilotPrimaryView>("home");
  const [writingMode, setWritingMode] = useState(false);
  const [ambient, setAmbient] = useState<AmbientFocus>("workspace");
  const [focusedObjectId, setFocusedObjectId] = useState<string | null>(null);
  const [selectedObjectId, setSelectedObjectId] = useState<string | null>(null);

  const setWriting = useCallback(
    (on: boolean) => {
      setWritingMode(on);
      if (on) {
        setAmbient("input");
        setScene("writing");
        setPrimary("write-surface");
      } else {
        setScene("default");
      }
    },
    [setScene, setPrimary],
  );

  const setPrimaryObject = useCallback(
    (id: string | null) => {
      setPrimary(id);
      setSelectedObjectId(id);
      if (id) {
        setFocusedObjectId(id);
      }
    },
    [setPrimary],
  );

  const value = useMemo(
    () => ({
      density,
      destination,
      writingMode,
      ambient,
      focusedObjectId,
      selectedObjectId,
      attentionScene: scene,
      primaryObjectId,
      setDestination,
      setWritingMode: setWriting,
      setAmbient,
      setFocusedObjectId,
      setSelectedObjectId,
      setPrimaryObject,
      setSecondaryObjects: setSecondary,
      setAttentionScene: setScene,
    }),
    [
      density,
      destination,
      writingMode,
      ambient,
      focusedObjectId,
      selectedObjectId,
      scene,
      primaryObjectId,
      setSecondary,
      setScene,
      setWriting,
      setPrimaryObject,
    ],
  );

  return (
    <WorkspaceCompositionContext.Provider value={value}>
      {children}
    </WorkspaceCompositionContext.Provider>
  );
}

export function WorkspaceCompositionProvider({
  density,
  children,
}: {
  density: WorkspaceDensity;
  children: ReactNode;
}) {
  return (
    <AttentionEngineProvider>
      <CompositionInner density={density}>{children}</CompositionInner>
    </AttentionEngineProvider>
  );
}

export function useWorkspaceComposition(): WorkspaceCompositionValue {
  const value = useContext(WorkspaceCompositionContext);
  if (!value) {
    return {
      density: "balanced",
      destination: "home",
      writingMode: false,
      ambient: "workspace",
      focusedObjectId: null,
      selectedObjectId: null,
      attentionScene: "default",
      primaryObjectId: null,
      setDestination: () => undefined,
      setWritingMode: () => undefined,
      setAmbient: () => undefined,
      setFocusedObjectId: () => undefined,
      setSelectedObjectId: () => undefined,
      setPrimaryObject: () => undefined,
      setSecondaryObjects: () => undefined,
      setAttentionScene: () => undefined,
    };
  }
  return value;
}
