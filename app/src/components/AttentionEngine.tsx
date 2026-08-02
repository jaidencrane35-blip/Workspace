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
  computeAttentionWeights,
  resolveAttentionVisual,
  type AttentionScene,
  type AttentionVisual,
} from "../lib/attention";

const ATTENTION_FALLBACK = 0.55;

interface AttentionEngineValue {
  scene: AttentionScene;
  primaryObjectId: string | null;
  setScene: (scene: AttentionScene) => void;
  setPrimary: (id: string | null) => void;
  setSecondary: (ids: string[]) => void;
  register: (id: string) => void;
  unregister: (id: string) => void;
  getVisual: (id: string) => AttentionVisual;
  isPrimary: (id: string) => boolean;
}

const AttentionContext = createContext<AttentionEngineValue | null>(null);

export function AttentionEngineProvider({ children }: { children: ReactNode }) {
  const [scene, setScene] = useState<AttentionScene>("default");
  const [primaryObjectId, setPrimaryObjectId] = useState<string | null>(null);
  const [secondaryIds, setSecondaryIds] = useState<string[]>([]);
  const [registry, setRegistry] = useState<string[]>([]);
  const registryRef = useRef(new Set<string>());

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
      computeAttentionWeights(
        registry,
        primaryObjectId,
        new Set(secondaryIds),
      ),
    [registry, primaryObjectId, secondaryIds],
  );

  const getVisual = useCallback(
    (id: string): AttentionVisual => {
      const weight = weights[id];
      let adjusted =
        weight ?? (id === primaryObjectId ? 1 : ATTENTION_FALLBACK);
      if (scene === "writing" && id !== primaryObjectId) {
        adjusted *= 0.55;
      }
      if (scene === "restore" && id !== primaryObjectId) {
        adjusted *= 0.7;
      }
      if (scene === "empty" && id !== primaryObjectId) {
        adjusted = Math.min(adjusted, 0.4);
      }
      return resolveAttentionVisual(adjusted);
    },
    [weights, scene, primaryObjectId],
  );

  const isPrimary = useCallback(
    (id: string) => primaryObjectId === id,
    [primaryObjectId],
  );

  const value = useMemo(
    () => ({
      scene,
      primaryObjectId,
      setScene,
      setPrimary,
      setSecondary,
      register,
      unregister,
      getVisual,
      isPrimary,
    }),
    [
      scene,
      primaryObjectId,
      setPrimary,
      setSecondary,
      register,
      unregister,
      getVisual,
      isPrimary,
    ],
  );

  return (
    <AttentionContext.Provider value={value}>
      {children}
    </AttentionContext.Provider>
  );
}

export function useAttentionEngine(): AttentionEngineValue {
  const value = useContext(AttentionContext);
  if (!value) {
    return {
      scene: "default",
      primaryObjectId: null,
      setScene: () => undefined,
      setPrimary: () => undefined,
      setSecondary: () => undefined,
      register: () => undefined,
      unregister: () => undefined,
      getVisual: () => resolveAttentionVisual(ATTENTION_FALLBACK),
      isPrimary: () => false,
    };
  }
  return value;
}

/** Register an object and read its continuous attention visual. */
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
