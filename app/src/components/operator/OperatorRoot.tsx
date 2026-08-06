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
import type { PilotPrimaryView } from "../../lib/pilotChrome";
import { RepositoryHealthPanel } from "./RepositoryHealthPanel";

export type OperatorMode = 1 | 2 | 3;

export interface ChatMessage {
  id: string;
  role: "user" | "workspace";
  text: string;
  streaming?: boolean;
}

const FLOAT_KEY = "workspace.operator.float";
const PANEL_KEY = "workspace.operator.panel";
const MODE_KEY = "workspace.operator.mode";
const DEV_KEY = "workspace.operator.developer";

interface OperatorRootProps {
  productSurface: ReactNode;
  onNavigateProduct: (view: PilotPrimaryView) => void;
}

function loadPoint(key: string, fallback: { x: number; y: number }) {
  try {
    const raw = localStorage.getItem(key);
    if (!raw) {
      return fallback;
    }
    const parsed = JSON.parse(raw) as { x?: number; y?: number };
    return {
      x: typeof parsed.x === "number" ? parsed.x : fallback.x,
      y: typeof parsed.y === "number" ? parsed.y : fallback.y,
    };
  } catch {
    return fallback;
  }
}

export function OperatorRoot({
  productSurface,
  onNavigateProduct,
}: OperatorRootProps) {
  const listId = useId();
  const inputRef = useRef<HTMLTextAreaElement>(null);
  const scrollerRef = useRef<HTMLDivElement>(null);
  const dragRef = useRef<{
    kind: "float" | "panel";
    ox: number;
    oy: number;
    sx: number;
    sy: number;
  } | null>(null);
  const abortRef = useRef<AbortController | null>(null);
  const brandClicks = useRef({ n: 0, t: 0 });

  const [mode, setMode] = useState<OperatorMode>(() => {
    const stored = localStorage.getItem(MODE_KEY);
    if (stored === "1" || stored === "2" || stored === "3") {
      return Number(stored) as OperatorMode;
    }
    return 2;
  });
  const [floatPos, setFloatPos] = useState(() =>
    loadPoint(FLOAT_KEY, { x: 24, y: 24 }),
  );
  const [panelPos, setPanelPos] = useState(() =>
    loadPoint(PANEL_KEY, { x: 0, y: 0 }),
  );
  const [draft, setDraft] = useState("");
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [busy, setBusy] = useState(false);
  const [showHealth, setShowHealth] = useState(false);
  const [developer, setDeveloper] = useState(
    () => localStorage.getItem(DEV_KEY) === "1",
  );

  useEffect(() => {
    localStorage.setItem(MODE_KEY, String(mode));
  }, [mode]);

  useEffect(() => {
    localStorage.setItem(FLOAT_KEY, JSON.stringify(floatPos));
  }, [floatPos]);

  useEffect(() => {
    localStorage.setItem(PANEL_KEY, JSON.stringify(panelPos));
  }, [panelPos]);

  useEffect(() => {
    localStorage.setItem(DEV_KEY, developer ? "1" : "0");
    document.documentElement.dataset.operatorDeveloper = developer
      ? "on"
      : "off";
    return () => {
      delete document.documentElement.dataset.operatorDeveloper;
    };
  }, [developer]);

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

  const handleIntent = useCallback(
    async (raw: string) => {
      const action = resolveIntent(raw);
      switch (action.kind) {
        case "navigate":
          setMode(3);
          setShowHealth(false);
          onNavigateProduct(action.view);
          await pushWorkspace(action.reply);
          break;
        case "expand":
          setMode(3);
          setShowHealth(false);
          await pushWorkspace(action.reply);
          break;
        case "collapse":
          setMode(1);
          setShowHealth(false);
          await pushWorkspace(action.reply);
          break;
        case "health":
          setDeveloper(true);
          setMode(3);
          setShowHealth(true);
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
    [onNavigateProduct, pushWorkspace],
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
      {
        id: `u-${Date.now()}`,
        role: "user",
        text,
      },
    ]);
    void handleIntent(text);
  };

  const onFloatPointerDown = (event: ReactPointerEvent<HTMLButtonElement>) => {
    event.currentTarget.setPointerCapture(event.pointerId);
    dragRef.current = {
      kind: "float",
      ox: floatPos.x,
      oy: floatPos.y,
      sx: event.clientX,
      sy: event.clientY,
    };
  };

  const onChromePointerDown = (event: ReactPointerEvent<HTMLElement>) => {
    if (mode !== 2) {
      return;
    }
    const target = event.target as HTMLElement;
    if (target.closest("button")) {
      return;
    }
    event.currentTarget.setPointerCapture(event.pointerId);
    dragRef.current = {
      kind: "panel",
      ox: panelPos.x,
      oy: panelPos.y,
      sx: event.clientX,
      sy: event.clientY,
    };
  };

  const onSharedPointerMove = (event: ReactPointerEvent<HTMLElement>) => {
    if (!dragRef.current) {
      return;
    }
    const dx = event.clientX - dragRef.current.sx;
    const dy = event.clientY - dragRef.current.sy;
    if (dragRef.current.kind === "float") {
      setFloatPos({
        x: Math.max(8, dragRef.current.ox + dx),
        y: Math.max(8, dragRef.current.oy + dy),
      });
      return;
    }
    setPanelPos({
      x: dragRef.current.ox + dx,
      y: dragRef.current.oy + dy,
    });
  };

  const onFloatPointerUp = (event: ReactPointerEvent<HTMLButtonElement>) => {
    const started = dragRef.current;
    dragRef.current = null;
    if (!started || started.kind !== "float") {
      return;
    }
    const moved =
      Math.hypot(event.clientX - started.sx, event.clientY - started.sy) > 6;
    if (!moved) {
      setMode(2);
      requestAnimationFrame(() => inputRef.current?.focus());
    }
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

  const conversation = (
    <section
      className="op-shell"
      aria-label="Workspace conversation"
      data-operator-mode={mode}
    >
      <header
        className="op-shell__chrome"
        data-drag-frame={mode === 2 ? "true" : undefined}
        onPointerDown={onChromePointerDown}
        onPointerMove={onSharedPointerMove}
        onPointerUp={() => {
          dragRef.current = null;
        }}
        onPointerCancel={() => {
          dragRef.current = null;
        }}
      >
        <button
          type="button"
          className="op-shell__brand"
          onClick={onBrandActivate}
          title={developer ? "Developer mode on" : "Workspace"}
        >
          Workspace
          {developer && <span className="op-shell__dev-dot" aria-hidden="true" />}
        </button>
        <div className="op-shell__actions">
          {mode !== 1 && (
            <button
              type="button"
              className="op-shell__btn"
              onClick={() => setMode(1)}
              title="Collapse to floating operator"
            >
              Collapse
            </button>
          )}
          {mode !== 3 && (
            <button
              type="button"
              className="op-shell__btn op-shell__btn--primary"
              onClick={() => {
                setShowHealth(false);
                setMode(3);
              }}
              title="Expand Workspace around conversation"
            >
              Expand
            </button>
          )}
          {mode === 3 && (
            <button
              type="button"
              className="op-shell__btn"
              onClick={() => {
                setShowHealth(false);
                setMode(2);
              }}
              title="Conversation only"
            >
              Compact
            </button>
          )}
          {developer && mode === 3 && (
            <button
              type="button"
              className="op-shell__btn"
              onClick={() => setShowHealth((v) => !v)}
              title="Repository health"
            >
              Health
            </button>
          )}
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
  );

  return (
    <div
      className="op-root"
      data-mode={mode}
      data-developer={developer ? "on" : "off"}
    >
      {mode === 1 && (
        <button
          type="button"
          className="op-float"
          style={{ left: floatPos.x, top: floatPos.y }}
          aria-label="Open Workspace conversation"
          onPointerDown={onFloatPointerDown}
          onPointerMove={onSharedPointerMove}
          onPointerUp={onFloatPointerUp}
          onPointerCancel={() => {
            dragRef.current = null;
          }}
        >
          <span className="op-float__mark" aria-hidden="true">
            W
          </span>
        </button>
      )}

      {mode >= 2 && (
        <div
          className={`op-stage op-stage--mode${mode}`}
          style={
            mode === 2
              ? {
                  ["--op-panel-x" as string]: `${panelPos.x}px`,
                  ["--op-panel-y" as string]: `${panelPos.y}px`,
                }
              : undefined
          }
        >
          <div className="op-stage__chat">{conversation}</div>
          {mode === 3 && (
            <div className="op-stage__workspace" data-nested-shell="true">
              {showHealth && developer ? (
                <RepositoryHealthPanel onClose={() => setShowHealth(false)} />
              ) : (
                productSurface
              )}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
