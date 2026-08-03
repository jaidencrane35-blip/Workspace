import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
  type ReactNode,
} from "react";
import {
  computeCognitiveWeights,
  decayAffinity,
  decideGuideHint,
  type GuideDecision,
  type GuideHintId,
  visualFromCognitiveWeight,
} from "../lib/cognitive";
import {
  compositionForIntent,
  inferIntent,
  viewFromIntent,
  type IntentCommand,
  type IntentCompositionProfile,
  type WorkspaceIntent,
} from "../lib/intent";
import {
  resolveAttentionVisual,
  type AttentionScene,
  type AttentionVisual,
} from "../lib/attention";
import type { PilotPrimaryView } from "../lib/pilotChrome";

const ATTENTION_FALLBACK = 0.55;

interface CognitiveEngineValue {
  // Intent slice
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
  // Attention slice
  scene: AttentionScene;
  primaryObjectId: string | null;
  setScene: (scene: AttentionScene) => void;
  setPrimary: (id: string | null) => void;
  setSecondary: (ids: string[]) => void;
  register: (id: string) => void;
  unregister: (id: string) => void;
  getVisual: (id: string) => AttentionVisual;
  isPrimary: (id: string) => boolean;
  // Cognitive memory
  relevanceById: Record<string, number>;
  setRelevanceMap: (map: Record<string, number>) => void;
  noteResume: (momentId: string) => void;
  noteReflection: (depth: number) => void;
  /** Completing a write strengthens permanence and quietly ripples neighbours. */
  noteCapture: (momentId: string, neighbourIds?: string[]) => void;
  resumeAffinity: Record<string, number>;
  permanenceById: Record<string, number>;
  reflectionDepth: number;
  intentMemory: number;
  temporalCertainty: number;
  guideDecision: GuideDecision;
  noteGuideVisit: () => void;
  preferHint: GuideHintId | null;
}

const CognitiveContext = createContext<CognitiveEngineValue | null>(null);

/**
 * Single cognitive layer — consolidates IntentEngine + AttentionEngine.
 */
export function CognitiveEngineProvider({
  view,
  children,
}: {
  view: PilotPrimaryView;
  children: ReactNode;
}) {
  const [empty, setEmpty] = useState(false);
  const [writing, setWritingState] = useState(false);
  const [restoring, setRestoring] = useState(false);
  const [reflecting, setReflecting] = useState(false);
  const [influenceObjectId, setInfluenceObjectId] = useState<string | null>(
    null,
  );

  const [scene, setScene] = useState<AttentionScene>("default");
  const [primaryObjectId, setPrimaryObjectId] = useState<string | null>(null);
  const [secondaryIds, setSecondaryIds] = useState<string[]>([]);
  const [registry, setRegistry] = useState<string[]>([]);
  const registryRef = useRef(new Set<string>());

  const [relevanceById, setRelevanceById] = useState<Record<string, number>>(
    {},
  );
  const [resumeAffinity, setResumeAffinity] = useState<Record<string, number>>(
    {},
  );
  const [permanenceById, setPermanenceById] = useState<Record<string, number>>(
    {},
  );
  const [reflectionDepth, setReflectionDepth] = useState(0);
  const [intentMemory, setIntentMemory] = useState(0);
  const [guideVisits, setGuideVisits] = useState(0);

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
    setScene(profile.attentionScene);
  }, [profile.attentionScene]);

  // Calm decay of temporal memory — continuity without animation noise.
  useEffect(() => {
    const hasResume = Object.keys(resumeAffinity).length > 0;
    const hasPermanence = Object.keys(permanenceById).length > 0;
    if (!hasResume && !hasPermanence) {
      return;
    }
    const timer = window.setInterval(() => {
      if (hasResume) {
        setResumeAffinity((prev) => {
          const next: Record<string, number> = {};
          let changed = false;
          for (const [id, value] of Object.entries(prev)) {
            const decayed = decayAffinity(value, 0.96);
            if (decayed > 0) {
              next[id] = decayed;
            }
            if (decayed !== value) {
              changed = true;
            }
          }
          return changed ? next : prev;
        });
      }
      if (hasPermanence) {
        setPermanenceById((prev) => {
          const next: Record<string, number> = {};
          let changed = false;
          for (const [id, value] of Object.entries(prev)) {
            const decayed = decayAffinity(value, 0.985);
            if (decayed > 0) {
              next[id] = decayed;
            }
            if (decayed !== value) {
              changed = true;
            }
          }
          return changed ? next : prev;
        });
      }
      setIntentMemory((prev) => (prev < 0.04 ? 0 : prev * 0.994));
    }, 12000);
    return () => window.clearInterval(timer);
  }, [resumeAffinity, permanenceById]);

  const register = useCallback((id: string) => {
    if (registryRef.current.has(id)) {
      return;
    }
    registryRef.current.add(id);
    setRegistry(Array.from(registryRef.current));
  }, []);

  const unregister = useCallback((id: string) => {
    if (!registryRef.current.has(id)) {
      return;
    }
    registryRef.current.delete(id);
    setRegistry(Array.from(registryRef.current));
  }, []);

  const setPrimary = useCallback((id: string | null) => {
    setPrimaryObjectId(id);
  }, []);

  const setSecondary = useCallback((ids: string[]) => {
    setSecondaryIds(ids);
  }, []);

  const weights = useMemo(
    () =>
      computeCognitiveWeights({
        objectIds: registry,
        primaryId: primaryObjectId,
        secondaryIds: new Set(secondaryIds),
        relevanceById,
        scene,
      }),
    [registry, primaryObjectId, secondaryIds, relevanceById, scene],
  );

  const getVisual = useCallback(
    (id: string): AttentionVisual => {
      const weight = weights[id];
      const adjusted =
        weight ?? (id === primaryObjectId ? 1 : ATTENTION_FALLBACK);
      return visualFromCognitiveWeight(adjusted);
    },
    [weights, primaryObjectId],
  );

  const isPrimary = useCallback(
    (id: string) => primaryObjectId === id,
    [primaryObjectId],
  );

  const setWriting = useCallback((next: boolean) => {
    setWritingState(next);
  }, []);

  const adoptView = useCallback((_view: PilotPrimaryView) => {
    setWritingState(false);
    setRestoring(false);
    setReflecting(_view === "pilot");
  }, []);

  const requestIntent = useCallback((next: WorkspaceIntent) => {
    return viewFromIntent(next);
  }, []);

  const noteResume = useCallback((momentId: string) => {
    setResumeAffinity((prev) => ({
      ...prev,
      [momentId]: Math.min(1, (prev[momentId] ?? 0) * 0.4 + 0.85),
    }));
  }, []);

  const noteReflection = useCallback((depth: number) => {
    const next = Math.min(1, depth);
    setReflectionDepth((prev) => Math.min(1, Math.max(prev * 0.7, next)));
    // Repeated intent slowly reshapes future organisation.
    setIntentMemory((prev) => Math.min(1, prev * 0.82 + next * 0.28));
  }, []);

  const noteCapture = useCallback(
    (momentId: string, neighbourIds: string[] = []) => {
      setPermanenceById((prev) => ({
        ...prev,
        [momentId]: Math.min(1, (prev[momentId] ?? 0) * 0.45 + 0.74),
      }));
      setResumeAffinity((prev) => {
        const next = {
          ...prev,
          [momentId]: Math.min(1, (prev[momentId] ?? 0) * 0.5 + 0.4),
        };
        for (const id of neighbourIds) {
          if (id === momentId) {
            continue;
          }
          // Neighbours respond as if the workspace learned something.
          next[id] = Math.min(1, (next[id] ?? 0) * 0.88 + 0.16);
        }
        return next;
      });
    },
    [],
  );

  const noteGuideVisit = useCallback(() => {
    setGuideVisits((n) => n + 1);
  }, []);

  const setRelevanceMap = useCallback((map: Record<string, number>) => {
    setRelevanceById(map);
  }, []);

  const resumeAffinityMax = useMemo(
    () => Math.max(0, ...Object.values(resumeAffinity), 0),
    [resumeAffinity],
  );

  const permanenceMax = useMemo(
    () => Math.max(0, ...Object.values(permanenceById), 0),
    [permanenceById],
  );

  const temporalCertainty = useMemo(
    () =>
      Math.min(
        1,
        resumeAffinityMax * 0.4 +
          reflectionDepth * 0.28 +
          intentMemory * 0.18 +
          permanenceMax * 0.08 +
          Math.min(1, guideVisits / 5) * 0.06,
      ),
    [
      resumeAffinityMax,
      reflectionDepth,
      intentMemory,
      permanenceMax,
      guideVisits,
    ],
  );

  const guideDecision = useMemo(
    () =>
      decideGuideHint({
        hasPrimary: Boolean(primaryObjectId),
        neighbourCount: secondaryIds.length,
        resumeAffinityMax,
        reflectionDepth,
        idleVisits: guideVisits,
        temporalCertainty,
      }),
    [
      primaryObjectId,
      secondaryIds.length,
      resumeAffinityMax,
      reflectionDepth,
      guideVisits,
      temporalCertainty,
    ],
  );

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
      scene,
      primaryObjectId,
      setScene,
      setPrimary,
      setSecondary,
      register,
      unregister,
      getVisual,
      isPrimary,
      relevanceById,
      setRelevanceMap,
      noteResume,
      noteReflection,
      noteCapture,
      resumeAffinity,
      permanenceById,
      reflectionDepth,
      intentMemory,
      temporalCertainty,
      guideDecision,
      noteGuideVisit,
      preferHint: guideDecision.show ? guideDecision.hintId : null,
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
      scene,
      primaryObjectId,
      setPrimary,
      setSecondary,
      register,
      unregister,
      getVisual,
      isPrimary,
      relevanceById,
      setRelevanceMap,
      noteResume,
      noteReflection,
      noteCapture,
      resumeAffinity,
      permanenceById,
      reflectionDepth,
      intentMemory,
      temporalCertainty,
      guideDecision,
      noteGuideVisit,
    ],
  );

  return (
    <CognitiveContext.Provider value={value}>
      {children}
    </CognitiveContext.Provider>
  );
}

export function useCognitiveEngine(): CognitiveEngineValue {
  const value = useContext(CognitiveContext);
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
      scene: "default",
      primaryObjectId: null,
      setScene: () => undefined,
      setPrimary: () => undefined,
      setSecondary: () => undefined,
      register: () => undefined,
      unregister: () => undefined,
      getVisual: () => resolveAttentionVisual(ATTENTION_FALLBACK),
      isPrimary: () => false,
      relevanceById: {},
      setRelevanceMap: () => undefined,
      noteResume: () => undefined,
      noteReflection: () => undefined,
      noteCapture: () => undefined,
      resumeAffinity: {},
      permanenceById: {},
      reflectionDepth: 0,
      intentMemory: 0,
      temporalCertainty: 0,
      guideDecision: { show: false, hintId: "continue", confidence: 0 },
      noteGuideVisit: () => undefined,
      preferHint: null,
    };
  }
  return value;
}

/** Intent-compatible facade over the cognitive layer. */
export function useIntentEngine() {
  const c = useCognitiveEngine();
  return {
    intent: c.intent,
    profile: c.profile,
    empty: c.empty,
    writing: c.writing,
    restoring: c.restoring,
    setEmpty: c.setEmpty,
    setWriting: c.setWriting,
    setRestoring: c.setRestoring,
    setReflecting: c.setReflecting,
    adoptView: c.adoptView,
    requestIntent: c.requestIntent,
    influenceObjectId: c.influenceObjectId,
    setInfluenceObjectId: c.setInfluenceObjectId,
    commands: c.commands,
  };
}

/** Attention-compatible facade over the cognitive layer. */
export function useAttentionEngine() {
  const c = useCognitiveEngine();
  return {
    scene: c.scene,
    primaryObjectId: c.primaryObjectId,
    setScene: c.setScene,
    setPrimary: c.setPrimary,
    setSecondary: c.setSecondary,
    register: c.register,
    unregister: c.unregister,
    getVisual: c.getVisual,
    isPrimary: c.isPrimary,
  };
}

/** Register an object and read its continuous cognitive visual. */
export function useAttentionRegistration(objectId: string): AttentionVisual {
  const { register, unregister, getVisual, primaryObjectId } =
    useAttentionEngine();
  const wasPrimary = useRef(false);

  useEffect(() => {
    register(objectId);
    return () => unregister(objectId);
  }, [objectId, register, unregister]);

  useEffect(() => {
    const nowPrimary = primaryObjectId === objectId;
    if (nowPrimary && !wasPrimary.current) {
      const node = document.querySelector<HTMLElement>(
        `[data-object-id="${objectId}"]`,
      );
      const focusable = node?.querySelector<HTMLElement>(
        "button, input, textarea, [tabindex]:not([tabindex='-1'])",
      );
      focusable?.focus({ preventScroll: true });
    }
    wasPrimary.current = nowPrimary;
  }, [objectId, primaryObjectId]);

  return getVisual(objectId);
}
