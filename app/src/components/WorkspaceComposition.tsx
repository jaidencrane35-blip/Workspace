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
import type { AmbientFocus } from "./AmbientLighting";

interface WorkspaceCompositionValue {
  density: WorkspaceDensity;
  destination: PilotPrimaryView;
  writingMode: boolean;
  ambient: AmbientFocus;
  focusedObjectId: string | null;
  selectedObjectId: string | null;
  setDestination: (view: PilotPrimaryView) => void;
  setWritingMode: (on: boolean) => void;
  setAmbient: (focus: AmbientFocus) => void;
  setFocusedObjectId: (id: string | null) => void;
  setSelectedObjectId: (id: string | null) => void;
}

const WorkspaceCompositionContext =
  createContext<WorkspaceCompositionValue | null>(null);

export function WorkspaceCompositionProvider({
  density,
  children,
}: {
  density: WorkspaceDensity;
  children: ReactNode;
}) {
  const [destination, setDestination] = useState<PilotPrimaryView>("home");
  const [writingMode, setWritingMode] = useState(false);
  const [ambient, setAmbient] = useState<AmbientFocus>("workspace");
  const [focusedObjectId, setFocusedObjectId] = useState<string | null>(null);
  const [selectedObjectId, setSelectedObjectId] = useState<string | null>(null);

  const setWriting = useCallback((on: boolean) => {
    setWritingMode(on);
    if (on) {
      setAmbient("input");
    }
  }, []);

  const value = useMemo(
    () => ({
      density,
      destination,
      writingMode,
      ambient,
      focusedObjectId,
      selectedObjectId,
      setDestination,
      setWritingMode: setWriting,
      setAmbient,
      setFocusedObjectId,
      setSelectedObjectId,
    }),
    [
      density,
      destination,
      writingMode,
      ambient,
      focusedObjectId,
      selectedObjectId,
      setWriting,
    ],
  );

  return (
    <WorkspaceCompositionContext.Provider value={value}>
      {children}
    </WorkspaceCompositionContext.Provider>
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
      setDestination: () => undefined,
      setWritingMode: () => undefined,
      setAmbient: () => undefined,
      setFocusedObjectId: () => undefined,
      setSelectedObjectId: () => undefined,
    };
  }
  return value;
}
