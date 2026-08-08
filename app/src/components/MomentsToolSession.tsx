import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
  type Dispatch,
  type ReactNode,
  type SetStateAction,
} from "react";
import type {
  ActionOperationResult,
  ResumePlanPreview,
  SavedContext,
} from "../types/domain";

/** Resume tool flow — lives above view-swapped panels (P18.S2). */
export type ResumeToolStep =
  | "browse"
  | "inspect"
  | "confirm_delete"
  | "preview"
  | "done";

export type SaveToolStep = "naming" | "saved";

export interface ResumeToolSession {
  step: ResumeToolStep;
  inspected: SavedContext | null;
  preview: ResumePlanPreview | null;
  result: ActionOperationResult | null;
  deletedAck: string | null;
}

export interface SaveToolSession {
  step: SaveToolStep;
  name: string;
  handoffNote: string;
  saved: SavedContext | null;
  toolsOpen: boolean;
}

interface MomentsToolSessionValue {
  resume: ResumeToolSession;
  setResume: Dispatch<SetStateAction<ResumeToolSession>>;
  patchResume: (patch: Partial<ResumeToolSession>) => void;
  resetResume: () => void;
  save: SaveToolSession;
  setSave: Dispatch<SetStateAction<SaveToolSession>>;
  patchSave: (patch: Partial<SaveToolSession>) => void;
  resetSave: () => void;
}

const INITIAL_RESUME: ResumeToolSession = {
  step: "browse",
  inspected: null,
  preview: null,
  result: null,
  deletedAck: null,
};

const INITIAL_SAVE: SaveToolSession = {
  step: "naming",
  name: "",
  handoffNote: "",
  saved: null,
  toolsOpen: false,
};

const MomentsToolSessionContext =
  createContext<MomentsToolSessionValue | null>(null);

/**
 * P18.S2 — continuity holder for Moments tool sessions.
 * Not a second ActiveMoment; only preserves panel flow across remounts.
 */
export function MomentsToolSessionProvider({
  children,
}: {
  children: ReactNode;
}) {
  const [resume, setResume] = useState<ResumeToolSession>(INITIAL_RESUME);
  const [save, setSave] = useState<SaveToolSession>(INITIAL_SAVE);

  const patchResume = useCallback((patch: Partial<ResumeToolSession>) => {
    setResume((prev) => ({ ...prev, ...patch }));
  }, []);

  const resetResume = useCallback(() => {
    setResume(INITIAL_RESUME);
  }, []);

  const patchSave = useCallback((patch: Partial<SaveToolSession>) => {
    setSave((prev) => ({ ...prev, ...patch }));
  }, []);

  const resetSave = useCallback(() => {
    setSave(INITIAL_SAVE);
  }, []);

  const value = useMemo(
    () => ({
      resume,
      setResume,
      patchResume,
      resetResume,
      save,
      setSave,
      patchSave,
      resetSave,
    }),
    [resume, save, patchResume, resetResume, patchSave, resetSave],
  );

  return (
    <MomentsToolSessionContext.Provider value={value}>
      {children}
    </MomentsToolSessionContext.Provider>
  );
}

export function useMomentsToolSession(): MomentsToolSessionValue {
  const value = useContext(MomentsToolSessionContext);
  if (!value) {
    throw new Error(
      "useMomentsToolSession requires MomentsToolSessionProvider",
    );
  }
  return value;
}
