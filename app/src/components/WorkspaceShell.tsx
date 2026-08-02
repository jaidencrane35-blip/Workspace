import {
  BookmarkPlus,
  Compass,
  Home,
  MessageCircle,
  Play,
} from "lucide-react";
import { AnimatePresence, motion, useReducedMotion } from "motion/react";
import {
  type ReactNode,
  useEffect,
  useId,
  useRef,
} from "react";
import {
  PILOT_PRIMARY_VIEWS,
  PILOT_VIEW_LABELS,
  type PilotPrimaryView,
} from "../lib/pilotChrome";

const DOCK_ICONS: Record<PilotPrimaryView, typeof Home> = {
  home: Home,
  save: BookmarkPlus,
  resume: Play,
  pilot: MessageCircle,
  help: Compass,
};

interface WorkspaceShellProps {
  view: PilotPrimaryView;
  onNavigate: (view: PilotPrimaryView) => void;
  status?: ReactNode;
  children: ReactNode;
}

/**
 * Persistent Workspace Shell — environment stays mounted.
 * Only the content slot transitions. Background, dock, logo never remount.
 */
export function WorkspaceShell({
  view,
  onNavigate,
  status,
  children,
}: WorkspaceShellProps) {
  const reduceMotion = useReducedMotion();
  const contentRef = useRef<HTMLDivElement>(null);
  const contentId = useId();

  useEffect(() => {
    const node = contentRef.current;
    if (!node) {
      return;
    }
    // Move focus into the content region without scrolling the shell.
    if (typeof node.focus === "function") {
      node.focus({ preventScroll: true });
    }
  }, [view]);

  const duration = reduceMotion ? 0.01 : 0.32;
  const transition = reduceMotion
    ? { duration: 0.01 }
    : { type: "spring" as const, stiffness: 320, damping: 34, mass: 0.85 };

  return (
    <div className="app-shell exp-shell ws-env ws-shell">
      <div className="ws-atmosphere" aria-hidden="true">
        <div className="ws-atmosphere__glow ws-atmosphere__glow--a" />
        <div className="ws-atmosphere__glow ws-atmosphere__glow--b" />
        <div className="ws-atmosphere__glow ws-atmosphere__glow--c" />
        <div className="ws-atmosphere__grain" />
        <div className="ws-atmosphere__vignette" />
      </div>

      <header className="app-chrome exp-chrome ws-menubar">
        <div className="exp-brand ws-brand">
          <span className="exp-brand-mark ws-mark" aria-hidden="true">
            <span className="ws-mark__plane" />
            <span className="ws-mark__plane ws-mark__plane--b" />
          </span>
          <h1>Workspace</h1>
        </div>
        <div className="ws-menubar__status" aria-live="polite">
          {status}
        </div>
      </header>

      <div className="ws-stage" role="presentation">
        <div className="ws-spatial">
          <AnimatePresence mode="wait" initial={false}>
            <motion.div
              key={view}
              ref={contentRef}
              id={contentId}
              className="ws-content"
              role="region"
              aria-label={PILOT_VIEW_LABELS[view]}
              tabIndex={-1}
              initial={
                reduceMotion
                  ? { opacity: 1 }
                  : { opacity: 0, y: 16, scale: 0.985, filter: "blur(3px)" }
              }
              animate={
                reduceMotion
                  ? { opacity: 1 }
                  : { opacity: 1, y: 0, scale: 1, filter: "blur(0px)" }
              }
              exit={
                reduceMotion
                  ? { opacity: 0 }
                  : { opacity: 0, y: -10, scale: 0.99, filter: "blur(2px)" }
              }
              transition={
                reduceMotion
                  ? { duration }
                  : { ...transition, opacity: { duration: 0.28 } }
              }
            >
              {children}
            </motion.div>
          </AnimatePresence>
        </div>
      </div>

      <nav
        className="tabs exp-nav ws-dock"
        aria-label="Workspace"
        role="tablist"
      >
        {PILOT_PRIMARY_VIEWS.map((id) => {
          const Icon = DOCK_ICONS[id];
          const active = view === id;
          return (
            <button
              key={id}
              type="button"
              role="tab"
              className={
                active
                  ? "tab active ws-dock__item is-active"
                  : "tab ws-dock__item"
              }
              aria-current={active ? "page" : undefined}
              aria-selected={active}
              aria-controls={contentId}
              onClick={() => onNavigate(id)}
            >
              {active && (
                <motion.span
                  className="ws-dock__pill"
                  layoutId="ws-dock-pill"
                  transition={
                    reduceMotion
                      ? { duration: 0.01 }
                      : { type: "spring", stiffness: 420, damping: 36 }
                  }
                  aria-hidden="true"
                />
              )}
              <span className="ws-dock__icon">
                <Icon size={20} strokeWidth={1.75} aria-hidden="true" />
              </span>
              <span className="ws-dock__label">{PILOT_VIEW_LABELS[id]}</span>
            </button>
          );
        })}
      </nav>
    </div>
  );
}
