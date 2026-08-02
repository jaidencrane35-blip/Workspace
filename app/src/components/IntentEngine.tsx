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
  compositionForIntent,
  inferIntent,
  viewFromIntent,
  type IntentCommand,
  type IntentCompositionProfile,
  type WorkspaceIntent,
} from "../lib/intent";
import type { PilotPrimaryView } from "../lib/pilotChrome";
import { useAttentionEngine } from "./AttentionEngine";

interface IntentEngineValue {
  intent: WorkspaceIntent;
  profile: IntentCompositionProfile;
  empty: boolean;
  writing: boolean;
  restoring: boolean;
  setEmpty: (empty: boolean) => void;
  setWriting: (writing: boolean) => void;
  setRestoring: (restoring: boolean) => void;
  setReflecting: (reflecting: boolean) => void;
  adoptView: (view: PilotPrimaryView) => void;
  requestIntent: (intent: WorkspaceIntent) => PilotPrimaryView;
  influenceObjectId: string | null;
  setInfluenceObjectId: (id: string | null) => void;
  commands: IntentCommand[];
}

const IntentContext = createContext<IntentEngineValue | null>(null);

export function IntentEngineProvider({
  view,
  children,
}: {
  view: PilotPrimaryView;
  children: ReactNode;
}) {
  const attention = useAttentionEngine();
  const [empty, setEmpty] = useState(false);
  const [writing, setWritingState] = useState(false);
  const [restoring, setRestoring] = useState(false);
  const [reflecting, setReflecting] = useState(false);
  const [influenceObjectId, setInfluenceObjectId] = useState<string | null>(
    null,
  );

  const intent = useMemo(
    () =>
      inferIntent({
        view,
        writing,
        restoring,
        empty,
        reflecting,
      }),
    [view, writing, restoring, empty, reflecting],
  );

  const profile = useMemo(
    () => compositionForIntent(intent, empty && intent === "landing"),
    [intent, empty],
  );

  useEffect(() => {
    attention.setScene(profile.attentionScene);
  }, [profile.attentionScene, attention]);

  const setWriting = useCallback((next: boolean) => {
    setWritingState(next);
  }, []);

  const adoptView = useCallback(
    (_view: PilotPrimaryView) => {
      setWritingState(false);
      setRestoring(false);
      setReflecting(_view === "pilot");
    },
    [],
  );

  const requestIntent = useCallback((next: WorkspaceIntent) => {
    return viewFromIntent(next);
  }, []);

  const value = useMemo(
    () => ({
      intent,
      profile,
      empty,
      writing,
      restoring,
      setEmpty,
      setWriting,
      setRestoring,
      setReflecting,
      adoptView,
      requestIntent,
      influenceObjectId,
      setInfluenceObjectId,
      commands: profile.commands,
    }),
    [
      intent,
      profile,
      empty,
      writing,
      restoring,
      setWriting,
      adoptView,
      requestIntent,
      influenceObjectId,
    ],
  );

  return (
    <IntentContext.Provider value={value}>{children}</IntentContext.Provider>
  );
}

export function useIntentEngine(): IntentEngineValue {
  const value = useContext(IntentContext);
  if (!value) {
    const profile = compositionForIntent("landing");
    return {
      intent: "landing",
      profile,
      empty: false,
      writing: false,
      restoring: false,
      setEmpty: () => undefined,
      setWriting: () => undefined,
      setRestoring: () => undefined,
      setReflecting: () => undefined,
      adoptView: () => undefined,
      requestIntent: viewFromIntent,
      influenceObjectId: null,
      setInfluenceObjectId: () => undefined,
      commands: profile.commands,
    };
  }
  return value;
}
