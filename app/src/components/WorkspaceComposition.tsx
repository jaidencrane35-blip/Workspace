import {
  createContext,
  useContext,
  type ReactNode,
} from "react";
import type { WorkspaceDensity } from "../lib/density";

interface WorkspaceCompositionValue {
  density: WorkspaceDensity;
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
  return (
    <WorkspaceCompositionContext.Provider value={{ density }}>
      {children}
    </WorkspaceCompositionContext.Provider>
  );
}

export function useWorkspaceComposition(): WorkspaceCompositionValue {
  const value = useContext(WorkspaceCompositionContext);
  if (!value) {
    return { density: "balanced" };
  }
  return value;
}
