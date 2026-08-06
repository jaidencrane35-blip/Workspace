import {
  type FormEvent,
  type PointerEvent as ReactPointerEvent,
  type ReactNode,
  useCallback,
  useEffect,
  useId,
  useRef,
  useState,
} from "react";
import { resolveIntent, streamText } from "../../lib/intentBridge";
import { matchMomentByName } from "../../lib/momentMatch";
import {
  SHELL_MODE_EVENT,
  isTauriRuntime,
  loadShellMode,
  loadSpecializedTarget,
  saveShellMode,
  saveSpecializedTarget,
  type ShellMode,
} from "../../lib/shellRuntime";
import { canTransition } from "../../lib/shellStateMachine";
import {
  applyShellMode,
  exitWorkspace,
  installMainCloseCollapse,
  startOperatorDrag,
} from "../../lib/shellWindows";
import type { PilotPrimaryView } from "../../lib/pilotChrome";
import { DesktopOperator } from "./DesktopOperator";
import { OperatorSettingsPanel } from "./OperatorSettingsPanel";
import { RepositoryHealthPanel } from "./RepositoryHealthPanel";

export interface ChatMessage {
  id: string;
  role: "user" | "workspace";
  text: string;
  streaming?: boolean;
}

const DEV_KEY = "workspace.operator.developer";

interface OperatorRootProps {
  /** Mode 3 specialized tool surface (no Product Proof dock). */
  specializedSurface: ReactNode;
  onNavigateProduct: (
    view: PilotPrimaryView,
    opts?: { focusContextId?: string | null },
  ) => void;
  listMoments: () => Promise<Array<{ id: string; name: string }>>;
  activeSpecialized: PilotPrimaryView | "health" | null;
}

export function OperatorRoot({
  specializedSurface,
  onNavigateProduct,
  listMoments,
  activeSpecialized,
}: OperatorRootProps) {
  const listId = useId();
  const inputRef = useRef<HTMLTextAreaElement>(null);
  const scrollerRef = useRef<HTMLDivElement>(null);
  const abortRef = useRef<AbortController | null>(null);
  const brandClicks = useRef({ n: 0, t: 0 });
  const chromeDrag = useRef<{ sx: number; sy: number } | null>(null);

  const [mode, setModeState] = useState<ShellMode>(() => loadShellMode(1));
  const modeRef = useRef(mode);
  modeRef.current = mode;
  const [draft, setDraft] = useState("");
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [busy, setBusy] = useState(false);
  const [showHealth, setShowHealth] = useState(false);
  const [showSettings, setShowSettings] = useState(
    () => loadSpecializedTarget() === "settings",
  );
  const [developer, setDeveloper] = useState(
    () => localStorage.getItem(DEV_KEY) === "1",
  );

  const setMode = useCallback(async (next: ShellMode) => {
    if (!canTransition(modeRef.current, next) && modeRef.current !== next) {
      return;
    }
    if (next !== 3) {
      setShowSettings(false);
      setShowHealth(false);
      saveSpecializedTarget("none");
    }
    modeRef.current = next;
    setModeState(next);
    saveShellMode(next);
    await applyShellMode(next);
  }, []);

  useEffect(() => {
    document.documentElement.dataset.shellMode = String(mode);
    document.documentElement.dataset.operatorDeveloper = developer
      ? "on"
      : "off";
    localStorage.setItem(DEV_KEY, developer ? "1" : "0");
  }, [mode, developer]);

  useEffect(() => {
    void installMainCloseCollapse();
    void applyShellMode(mode);
  }, []); // eslint-disable-line react-hooks/exhaustive-deps

  useEffect(() => {
    const syncFromStorage = () => {
      const stored = loadShellMode(1);
      if (stored !== modeRef.current) {
        modeRef.current = stored;
        setModeState(stored);
      }
      const specialized = loadSpecializedTarget();
      setShowSettings(specialized === "settings");
      if (specialized === "health") {
        setShowHealth(true);
      }
    };
    window.addEventListener("storage", syncFromStorage);
    window.addEventListener(SHELL_MODE_EVENT, syncFromStorage);
    window.addEventListener("focus", syncFromStorage);
    document.addEventListener("visibilitychange", syncFromStorage);
    return () => {
      window.removeEventListener("storage", syncFromStorage);
      window.removeEventListener(SHELL_MODE_EVENT, syncFromStorage);
      window.removeEventListener("focus", syncFromStorage);
      document.removeEventListener("visibilitychange", syncFromStorage);
    };
  }, []);

  useEffect(() => {
    const el = scrollerRef.current;
    if (el) {
      el.scrollTop = el.scrollHeight;
    }
  }, [messages, mode]);

  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (event.ctrlKey && event.shiftKey && event.key.toLowerCase() === "d") {
        event.preventDefault();
        setDeveloper((prev) => !prev);
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);

  const pushWorkspace = useCallback(async (text: string) => {
    abortRef.current?.abort();
    const controller = new AbortController();
    abortRef.current = controller;
    const id = `w-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    setMessages((prev) => [
      ...prev,
      { id, role: "workspace", text: "", streaming: true },
    ]);
    setBusy(true);
    try {
      await streamText(
        text,
        (partial) => {
          setMessages((prev) =>
            prev.map((m) =>
              m.id === id ? { ...m, text: partial, streaming: true } : m,
            ),
          );
        },
        controller.signal,
      );
      setMessages((prev) =>
        prev.map((m) => (m.id === id ? { ...m, streaming: false } : m)),
      );
    } finally {
      setBusy(false);
    }
  }, []);

  const openSpecialized = useCallback(
    async (view: PilotPrimaryView) => {
      setShowHealth(false);
      setShowSettings(false);
      saveSpecializedTarget("none");
      onNavigateProduct(view);
      await setMode(3);
    },
    [onNavigateProduct, setMode],
  );

  const openSettingsSurface = useCallback(async () => {
    setShowHealth(false);
    setShowSettings(true);
    saveSpecializedTarget("settings");
    await setMode(3);
  }, [setMode]);

  const handleIntent = useCallback(
    async (raw: string) => {
      const action = resolveIntent(raw);
      switch (action.kind) {
        case "navigate":
          await openSpecialized(action.view);
          await pushWorkspace(action.reply);
          break;
        case "saveAs":
          await openSpecialized("save");
          await pushWorkspace(action.reply);
          break;
        case "navigateNamed": {
          const moments = await listMoments();
          const hit = matchMomentByName(moments, action.nameQuery);
          if (hit) {
            setShowHealth(false);
            onNavigateProduct("resume", { focusContextId: hit.id });
            await setMode(3);
            await pushWorkspace(
              `Opening restore review for “${hit.name}”. Approve the plan before anything moves.`,
            );
          } else {
            setShowHealth(false);
            onNavigateProduct("resume");
            await setMode(3);
            await pushWorkspace(
              `No unique Moment matched “${action.nameQuery}”. Opening restore review so you can choose — I won’t invent a restore.`,
            );
          }
          break;
        }
        case "expand":
          setShowHealth(false);
          setShowSettings(false);
          await setMode(2);
          await pushWorkspace(action.reply);
          break;
        case "collapse":
          setShowHealth(false);
          setShowSettings(false);
          // Leave the desktop — do not keep a reply in a hidden window.
          await setMode(0);
          break;
        case "settings":
          await openSettingsSurface();
          await pushWorkspace(action.reply);
          break;
        case "health":
          setDeveloper(true);
          setShowSettings(false);
          saveSpecializedTarget("health");
          setShowHealth(true);
          await setMode(3);
          await pushWorkspace(action.reply);
          break;
        case "developer":
          setDeveloper(action.enabled);
          if (!action.enabled) {
            setShowHealth(false);
          }
          await pushWorkspace(action.reply);
          break;
        case "proposal":
          await pushWorkspace(action.reply);
          break;
        case "unknown":
          await pushWorkspace(
            action.suggestion
              ? `${action.reply}\n${action.suggestion}`
              : action.reply,
          );
          break;
        default:
          break;
      }
    },
    [
      listMoments,
      onNavigateProduct,
      openSettingsSurface,
      openSpecialized,
      pushWorkspace,
      setMode,
    ],
  );

  const onSubmit = (event: FormEvent) => {
    event.preventDefault();
    const text = draft.trim();
    if (!text || busy) {
      return;
    }
    setDraft("");
    setMessages((prev) => [
      ...prev,
      { id: `u-${Date.now()}`, role: "user", text },
    ]);
    void handleIntent(text);
  };

  const onBrandActivate = () => {
    const now = Date.now();
    if (now - brandClicks.current.t > 900) {
      brandClicks.current = { n: 1, t: now };
      return;
    }
    brandClicks.current = { n: brandClicks.current.n + 1, t: now };
    if (brandClicks.current.n >= 3) {
      brandClicks.current = { n: 0, t: 0 };
      setDeveloper((prev) => !prev);
    }
  };

  const onChromePointerDown = (event: ReactPointerEvent<HTMLElement>) => {
    const target = event.target as HTMLElement;
    if (target.closest("button")) {
      return;
    }
    chromeDrag.current = { sx: event.clientX, sy: event.clientY };
    void startOperatorDrag();
  };

  const showSecondary = mode >= 2;
  const secondary =
    mode === 3 && showSettings ? (
      <OperatorSettingsPanel
        onClose={() => {
          setShowSettings(false);
          saveSpecializedTarget("none");
          void setMode(1);
        }}
        onOpenGuide={() => void openSpecialized("help")}
      />
    ) : mode === 3 && showHealth && developer ? (
      <RepositoryHealthPanel
        onClose={() => {
          setShowHealth(false);
          saveSpecializedTarget("none");
          void setMode(1);
        }}
      />
    ) : mode === 3 ? (
      specializedSurface
    ) : (
      <aside className="op-secondary-quiet" aria-label="Workspace context">
        <p className="op-secondary-quiet__copy">
          Ask Workspace to save, restore, or operate the desktop. Supporting
          tools appear here when needed — not as a dashboard.
        </p>
      </aside>
    );

  // Mode 0 in Tauri: main window is hidden — operator window owns UI.
  // Browser/demo fallback: in-window icon only (cannot leave the host page).
  if (mode === 0) {
    if (isTauriRuntime()) {
      return (
        <div className="op-root" data-mode={0} aria-hidden="true" />
      );
    }
    return (
      <div className="op-root" data-mode={0} data-shell-fallback="in-window">
        <DesktopOperator
          onOpenCompact={() => void setMode(1)}
          onExpand={() => void setMode(2)}
          onOpenSettings={() => void openSettingsSurface()}
        />
      </div>
    );
  }

  return (
    <div
      className="op-root"
      data-mode={mode}
      data-developer={developer ? "on" : "off"}
      data-specialized={activeSpecialized ?? undefined}
    >
      <div className={`op-stage op-stage--mode${mode}`}>
        <div className="op-stage__chat">
          <section
            className="op-shell"
            aria-label="Workspace conversation"
            data-operator-mode={mode}
          >
            <header
              className="op-shell__chrome"
              data-drag-frame="true"
              onPointerDown={onChromePointerDown}
            >
              <button
                type="button"
                className="op-shell__brand"
                onClick={onBrandActivate}
                title={developer ? "Developer mode on" : "Workspace"}
              >
                Workspace
                {developer && (
                  <span className="op-shell__dev-dot" aria-hidden="true" />
                )}
              </button>
                <div className="op-shell__actions">
                <button
                  type="button"
                  className="op-shell__btn"
                  onClick={() => void setMode(0)}
                  title="Collapse to desktop operator"
                >
                  Collapse
                </button>
                {mode === 1 && (
                  <button
                    type="button"
                    className="op-shell__btn op-shell__btn--primary"
                    onClick={() => {
                      setShowHealth(false);
                      setShowSettings(false);
                      void setMode(2);
                    }}
                    title="Expand around conversation"
                  >
                    Expand
                  </button>
                )}
                {mode >= 2 && (
                  <button
                    type="button"
                    className="op-shell__btn"
                    onClick={() => {
                      setShowHealth(false);
                      setShowSettings(false);
                      void setMode(1);
                    }}
                    title="Compact conversation"
                  >
                    Compact
                  </button>
                )}
                <button
                  type="button"
                  className="op-shell__btn"
                  onClick={() => void openSettingsSurface()}
                  title="Settings"
                >
                  Settings
                </button>
                {developer && mode >= 2 && (
                  <button
                    type="button"
                    className="op-shell__btn"
                    onClick={() => {
                      setShowSettings(false);
                      setShowHealth((v) => !v);
                      saveSpecializedTarget("health");
                      void setMode(3);
                    }}
                    title="Repository health"
                  >
                    Health
                  </button>
                )}
                <button
                  type="button"
                  className="op-shell__btn"
                  onClick={() => void exitWorkspace()}
                  title="Exit Workspace"
                >
                  Exit
                </button>
              </div>
            </header>

            <div
              className="op-shell__transcript"
              ref={scrollerRef}
              id={listId}
              role="log"
              aria-live="polite"
              aria-relevant="additions"
            >
              {messages.length === 0 ? (
                <div className="op-shell__waiting" aria-hidden="true" />
              ) : (
                messages.map((message) => (
                  <div
                    key={message.id}
                    className={`op-msg op-msg--${message.role}`}
                    data-streaming={message.streaming ? "true" : undefined}
                  >
                    <p className="op-msg__text">{message.text}</p>
                  </div>
                ))
              )}
            </div>

            <form className="op-shell__composer" onSubmit={onSubmit}>
              <textarea
                ref={inputRef}
                className="op-shell__input"
                value={draft}
                onChange={(e) => setDraft(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter" && !e.shiftKey) {
                    e.preventDefault();
                    e.currentTarget.form?.requestSubmit();
                  }
                }}
                rows={2}
                placeholder=""
                aria-label="Message Workspace"
                disabled={busy}
              />
              <button
                type="submit"
                className="op-shell__send"
                disabled={busy || !draft.trim()}
              >
                Send
              </button>
            </form>
          </section>
        </div>

        {showSecondary && (
          <div className="op-stage__workspace" data-specialized-shell="true">
            {secondary}
          </div>
        )}
      </div>
    </div>
  );
}
