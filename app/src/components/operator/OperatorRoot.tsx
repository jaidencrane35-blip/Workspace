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
import { IpcCommandError, invokeIpc } from "../../lib/ipc";
import { matchMomentByName } from "../../lib/momentMatch";
import {
  SHELL_MODE_EVENT,
  isTauriRuntime,
  loadShellMode,
  saveShellMode,
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
import { RepositoryHealthPanel } from "./RepositoryHealthPanel";

export interface ChatMessage {
  id: string;
  role: "user" | "workspace";
  text: string;
  streaming?: boolean;
}

const DEV_KEY = "workspace.operator.developer";

interface OperatorRootProps {
  /** Secondary tool surface launched from conversation (not a shell form). */
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

  const [mode, setModeState] = useState<ShellMode>(() => loadShellMode(1));
  const modeRef = useRef(mode);
  modeRef.current = mode;
  const [draft, setDraft] = useState("");
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [busy, setBusy] = useState(false);
  /** Tool dock inside Conversation — never a third shell form. */
  const [toolDock, setToolDock] = useState(false);
  const [showHealth, setShowHealth] = useState(false);
  const [developer, setDeveloper] = useState(
    () => localStorage.getItem(DEV_KEY) === "1",
  );

  const setMode = useCallback(async (next: ShellMode) => {
    if (!canTransition(modeRef.current, next) && modeRef.current !== next) {
      return;
    }
    if (next === 0) {
      setToolDock(false);
      setShowHealth(false);
    }
    modeRef.current = next;
    setModeState(next);
    saveShellMode(next);
    await applyShellMode(next);
  }, []);

  useEffect(() => {
    document.documentElement.dataset.shellMode = String(mode);
    document.documentElement.dataset.shellForm =
      mode === 0 ? "operator" : "conversation";
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
        if (stored === 0) {
          setToolDock(false);
          setShowHealth(false);
        }
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
    if (mode !== 1) {
      return;
    }
    const id = window.setTimeout(() => {
      inputRef.current?.focus();
    }, 40);
    return () => window.clearTimeout(id);
  }, [mode]);

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

  const ensureConversation = useCallback(async () => {
    if (modeRef.current !== 1) {
      await setMode(1);
    }
  }, [setMode]);

  const openToolSurface = useCallback(
    async (view: PilotPrimaryView) => {
      setShowHealth(false);
      onNavigateProduct(view);
      setToolDock(true);
      await ensureConversation();
    },
    [ensureConversation, onNavigateProduct],
  );

  const handleIntent = useCallback(
    async (raw: string) => {
      const action = resolveIntent(raw);
      switch (action.kind) {
        case "navigate":
          await openToolSurface(action.view);
          await pushWorkspace(action.reply);
          break;
        case "saveAs":
          await openToolSurface("save");
          await pushWorkspace(action.reply);
          break;
        case "navigateNamed": {
          const moments = await listMoments();
          const hit = matchMomentByName(moments, action.nameQuery);
          setShowHealth(false);
          if (hit) {
            onNavigateProduct("resume", { focusContextId: hit.id });
            setToolDock(true);
            await ensureConversation();
            await pushWorkspace(
              `Opening restore review for “${hit.name}”. Approve the plan before anything moves.`,
            );
          } else {
            onNavigateProduct("resume");
            setToolDock(true);
            await ensureConversation();
            await pushWorkspace(
              `No unique Moment matched “${action.nameQuery}”. Opening restore review so you can choose — I won’t invent a restore.`,
            );
          }
          break;
        }
        case "expand":
          // Expanded Workspace is a presentation dock, not a shell form.
          // Without an earned satellite, stay conversation-only.
          await ensureConversation();
          await pushWorkspace(action.reply);
          break;
        case "collapse":
          await setMode(0);
          break;
        case "settings":
          await pushWorkspace(action.reply);
          break;
        case "health":
          setDeveloper(true);
          setShowHealth(true);
          setToolDock(true);
          await ensureConversation();
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
        case "clipboardRead": {
          try {
            const result = await invokeIpc<{
              format: string;
              bytes: number;
              preview: string;
              text: string;
            }>("read_clipboard");
            if (!result.text) {
              await pushWorkspace(
                "Clipboard is empty (or has no text). Capability Runtime read completed.",
              );
            } else {
              await pushWorkspace(
                `Clipboard (${result.format}, ${result.bytes} bytes):\n${result.preview}`,
              );
            }
          } catch (error) {
            const message =
              error instanceof IpcCommandError
                ? error.message
                : "Clipboard read failed.";
            await pushWorkspace(message);
          }
          break;
        }
        case "clipboardWrite": {
          try {
            const result = await invokeIpc<{
              format: string;
              bytes: number;
              preview: string;
              message: string;
            }>("write_clipboard", { text: action.text });
            await pushWorkspace(
              `${result.message} (${result.bytes} bytes). Preview: ${result.preview}`,
            );
          } catch (error) {
            const message =
              error instanceof IpcCommandError
                ? error.message
                : "Clipboard write failed.";
            await pushWorkspace(message);
          }
          break;
        }
        case "winEnumerate":
        case "winActive":
        case "winMonitors":
        case "winBounds":
        case "winMaximize":
        case "winMinimize":
        case "winRestore":
        case "winSnap":
        case "winCenter":
        case "winMoveMonitor":
        case "winFocus":
        case "winResize": {
          try {
            type WinResult = {
              operation: string;
              ok: boolean;
              status?: string;
              target?: string;
              message?: string;
              text?: string;
              items?: Array<{ title: string }>;
              monitors?: Array<{ index: number; name: string; isPrimary: boolean }>;
            };
            const run = (args: Record<string, unknown>) =>
              invokeIpc<WinResult>("execute_window_operation", {
                query: null,
                path: null,
                hwnd: null,
                pid: null,
                x: null,
                y: null,
                width: null,
                height: null,
                monitor_index: null,
                snap: null,
                ...args,
              });

            if (action.kind === "winEnumerate") {
              const result = await run({ operation: "enumerate" });
              if (!result.ok) {
                await pushWorkspace(
                  result.message ?? "I couldn’t list windows.",
                );
                break;
              }
              const titles =
                result.items
                  ?.slice(0, 12)
                  .map((item) => `• ${item.title}`)
                  .join("\n") ?? "(none)";
              const count = result.items?.length ?? 0;
              await pushWorkspace(
                count === 0
                  ? "No open windows found."
                  : `Open windows (${count}):\n${titles}`,
              );
              break;
            }
            if (action.kind === "winMonitors") {
              const result = await run({ operation: "monitors" });
              if (!result.ok) {
                await pushWorkspace(
                  result.message ?? "I couldn’t list monitors.",
                );
                break;
              }
              const lines =
                result.monitors
                  ?.map(
                    (m) =>
                      `• Monitor ${m.index}: ${m.name}${m.isPrimary ? " (primary)" : ""}`,
                  )
                  .join("\n") ?? "(none)";
              await pushWorkspace(
                `${result.message ?? "Monitors attached."}\n${lines}`,
              );
              break;
            }
            if (action.kind === "winActive") {
              const result = await run({ operation: "active" });
              await pushWorkspace(
                result.ok
                  ? (result.message ?? "Active window found.")
                  : (result.message ?? "No active window."),
              );
              break;
            }
            if (action.kind === "winSnap") {
              const result = await run({
                operation: "snap",
                query: action.query,
                snap: action.snap,
              });
              await pushWorkspace(
                result.message ??
                  (result.ok ? "Moved the window." : "Couldn’t move that window."),
              );
              break;
            }
            if (action.kind === "winMoveMonitor") {
              const result = await run({
                operation: "move",
                query: action.query,
                monitor_index: action.monitorIndex,
              });
              await pushWorkspace(
                result.message ??
                  (result.ok ? "Moved the window." : "Couldn’t move that window."),
              );
              break;
            }
            if (action.kind === "winResize") {
              const result = await run({
                operation: "resize",
                query: action.query,
                width: action.width,
                height: action.height,
              });
              await pushWorkspace(
                result.message ??
                  (result.ok ? "Resized the window." : "Couldn’t resize that window."),
              );
              break;
            }
            const operation =
              action.kind === "winBounds"
                ? "bounds"
                : action.kind === "winMaximize"
                  ? "maximize"
                  : action.kind === "winMinimize"
                    ? "minimize"
                    : action.kind === "winRestore"
                      ? "restore"
                      : action.kind === "winCenter"
                        ? "center"
                        : "focus";
            const result = await run({
              operation,
              query: "query" in action ? action.query : undefined,
            });
            await pushWorkspace(
              result.message ??
                result.text ??
                (result.ok
                  ? "Done."
                  : "That window action didn’t succeed."),
            );
          } catch (error) {
            const message =
              error instanceof IpcCommandError
                ? error.message
                : "That window action failed.";
            await pushWorkspace(message);
          }
          break;
        }
        case "appEnumerate":
        case "appLaunch":
        case "appFocus":
        case "appClose":
        case "appMinimize":
        case "appRestore":
        case "appOpen": {
          try {
            type AppResult = {
              operation: string;
              ok: boolean;
              status?: string;
              target?: string;
              message?: string;
              preview?: string;
              items?: Array<{ title: string; processId: number; minimized: boolean }>;
            };
            const run = (operation: string, query?: string) =>
              invokeIpc<AppResult>("execute_application_operation", {
                operation,
                query: query ?? null,
                path: null,
                hwnd: null,
              });

            if (action.kind === "appEnumerate") {
              const result = await run("enumerate");
              const titles =
                result.items
                  ?.slice(0, 12)
                  .map((item) => `• ${item.title}`)
                  .join("\n") ?? "(none)";
              await pushWorkspace(
                `${result.message ?? "Applications listed."}\n${titles}`,
              );
              break;
            }

            if (action.kind === "appOpen") {
              const found = await run("find", action.query);
              if (found.ok && (found.items?.length ?? 0) > 0) {
                const focused = await run("focus", action.query);
                await pushWorkspace(
                  focused.message ??
                    `Focused “${action.query}” (already running).`,
                );
              } else {
                const launched = await run("launch", action.query);
                await pushWorkspace(
                  launched.message ?? `Launch requested for “${action.query}”.`,
                );
              }
              break;
            }

            const operation =
              action.kind === "appLaunch"
                ? "launch"
                : action.kind === "appFocus"
                  ? "focus"
                  : action.kind === "appClose"
                    ? "close"
                    : action.kind === "appMinimize"
                      ? "minimize"
                      : "restore";
            const result = await run(operation, action.query);
            await pushWorkspace(
              result.message ??
                `${operation} → ${result.status ?? (result.ok ? "ok" : "failed")}`,
            );
          } catch (error) {
            const message =
              error instanceof IpcCommandError
                ? error.message
                : "Application operation failed.";
            await pushWorkspace(message);
          }
          break;
        }
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
      ensureConversation,
      listMoments,
      onNavigateProduct,
      openToolSurface,
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
    brandClicks.current.n += 1;
    brandClicks.current.t = now;
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
    void startOperatorDrag();
  };

  const secondary =
    showHealth && developer ? (
      <RepositoryHealthPanel
        onClose={() => {
          setShowHealth(false);
          if (!activeSpecialized) {
            setToolDock(false);
          }
        }}
      />
    ) : activeSpecialized ? (
      specializedSurface
    ) : null;

  // Expanded Workspace presentation = Form B + earned satellite only.
  const presentationExpanded = Boolean(
    toolDock && (showHealth || activeSpecialized),
  );

  // Form A in Tauri: conversation window hidden — operator window owns UI.
  if (mode === 0) {
    if (isTauriRuntime()) {
      return (
        <div className="op-root" data-mode={0} aria-hidden="true" />
      );
    }
    return (
      <div className="op-root" data-mode={0} data-shell-fallback="in-window">
        <DesktopOperator onOpenConversation={() => void setMode(1)} />
      </div>
    );
  }

  return (
    <div
      className="op-root"
      data-mode={1}
      data-gravity="conversation"
      data-presentation={presentationExpanded ? "expanded" : "conversation"}
      data-developer={developer ? "on" : "off"}
      data-specialized={activeSpecialized ?? undefined}
    >
      <div
        className="op-stage op-stage--conversation"
        data-dock={presentationExpanded ? "on" : "off"}
      >
        <div className="op-stage__chat">
          <section
            className="op-shell"
            aria-label="Workspace conversation"
            data-operator-mode={1}
            data-surface="gravity"
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
                  title="Collapse to desktop companion"
                  aria-label="Collapse"
                >
                  Collapse
                </button>
                {developer && (
                  <button
                    type="button"
                    className="op-shell__btn"
                    onClick={() => {
                      setShowHealth((v) => !v);
                      setToolDock(true);
                    }}
                    title="Repository health"
                  >
                    Health
                  </button>
                )}
                <button
                  type="button"
                  className="op-shell__btn op-shell__btn--quiet"
                  onClick={() => void exitWorkspace()}
                  title="Exit Workspace"
                  aria-label="Exit Workspace"
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
                aria-label="Talk to Workspace"
                disabled={busy}
              />
              <button
                type="submit"
                className="op-shell__send"
                disabled={busy || !draft.trim()}
                aria-label="Send"
                title="Send"
              >
                ↵
              </button>
            </form>
          </section>
        </div>

        {presentationExpanded && secondary && (
          <div
            className="op-stage__workspace"
            data-specialized-shell="true"
            data-satellite="true"
          >
            {secondary}
          </div>
        )}
      </div>
    </div>
  );
}
