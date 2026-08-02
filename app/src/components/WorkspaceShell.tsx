import {
  BookmarkPlus,
  Compass,
  Home,
  MessageCircle,
  Play,
} from "lucide-react";
import {
  AnimatePresence,
  motion,
  useMotionValue,
  useReducedMotion,
  useSpring,
} from "motion/react";
import {
  type KeyboardEvent,
  type MouseEvent,
  type ReactNode,
  useCallback,
  useEffect,
  useId,
  useRef,
  useState,
} from "react";
import { interaction, spring } from "../design-system";
import { useWorkspaceDensity } from "../hooks/useWorkspaceDensity";
import {
  PILOT_PRIMARY_VIEWS,
  PILOT_VIEW_LABELS,
  type PilotPrimaryView,
} from "../lib/pilotChrome";
import { AmbientLighting, type AmbientFocus } from "./AmbientLighting";
import { WorkspaceCompositionProvider } from "./WorkspaceComposition";

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

function DockItem({
  id,
  active,
  contentId,
  reduceMotion,
  onNavigate,
  onFocusAmbient,
}: {
  id: PilotPrimaryView;
  active: boolean;
  contentId: string;
  reduceMotion: boolean | null;
  onNavigate: (view: PilotPrimaryView) => void;
  onFocusAmbient: (focus: AmbientFocus) => void;
}) {
  const Icon = DOCK_ICONS[id];
  const x = useMotionValue(0);
  const y = useMotionValue(0);
  const springX = useSpring(x, spring.dock);
  const springY = useSpring(y, spring.dock);
  const ref = useRef<HTMLButtonElement>(null);

  const onMove = useCallback(
    (event: MouseEvent<HTMLButtonElement>) => {
      if (reduceMotion || !ref.current) {
        return;
      }
      const rect = ref.current.getBoundingClientRect();
      const dx = event.clientX - (rect.left + rect.width / 2);
      const dy = event.clientY - (rect.top + rect.height / 2);
      x.set(
        Math.max(
          -interaction.magneticMax,
          Math.min(interaction.magneticMax, dx * interaction.magneticFactor),
        ),
      );
      y.set(
        Math.max(
          -interaction.magneticMax,
          Math.min(interaction.magneticMax, dy * interaction.magneticFactor),
        ),
      );
    },
    [reduceMotion, x, y],
  );

  return (
    <motion.button
      ref={ref}
      type="button"
      role="tab"
      className={
        active ? "tab active ws-dock__item is-active" : "tab ws-dock__item"
      }
      aria-current={active ? "page" : undefined}
      aria-selected={active}
      aria-controls={contentId}
      aria-label={PILOT_VIEW_LABELS[id]}
      onClick={() => onNavigate(id)}
      onMouseMove={onMove}
      onMouseLeave={() => {
        x.set(0);
        y.set(0);
        onFocusAmbient("workspace");
      }}
      onMouseEnter={() => onFocusAmbient("dock")}
      onFocus={() => onFocusAmbient("dock")}
      style={{ x: springX, y: springY }}
      whileHover={reduceMotion ? undefined : { scale: 1.06 }}
      whileTap={reduceMotion ? undefined : { scale: interaction.pressScale }}
      transition={spring.snappy}
    >
      {active && (
        <motion.span
          className="ws-dock__pill"
          layoutId="ws-dock-pill"
          transition={reduceMotion ? { duration: 0.01 } : spring.layout}
          aria-hidden="true"
        />
      )}
      <span className="ws-dock__icon">
        <Icon size={20} strokeWidth={1.75} aria-hidden="true" />
      </span>
      <span className="ws-dock__label">{PILOT_VIEW_LABELS[id]}</span>
    </motion.button>
  );
}

export function WorkspaceShell({
  view,
  onNavigate,
  status,
  children,
}: WorkspaceShellProps) {
  const reduceMotion = useReducedMotion();
  const density = useWorkspaceDensity();
  const contentRef = useRef<HTMLDivElement>(null);
  const contentId = useId();
  const liveRef = useRef<HTMLDivElement>(null);
  const [ambient, setAmbient] = useState<AmbientFocus>("workspace");

  useEffect(() => {
    contentRef.current?.focus({ preventScroll: true });
    setAmbient(view === "save" ? "input" : "workspace");
    if (liveRef.current) {
      liveRef.current.textContent = `${PILOT_VIEW_LABELS[view]} selected`;
    }
  }, [view]);

  const onDockKeyDown = (event: KeyboardEvent<HTMLElement>) => {
    const index = PILOT_PRIMARY_VIEWS.indexOf(view);
    if (index < 0) {
      return;
    }
    if (event.key === "ArrowRight" || event.key === "ArrowDown") {
      event.preventDefault();
      onNavigate(PILOT_PRIMARY_VIEWS[(index + 1) % PILOT_PRIMARY_VIEWS.length]);
    } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
      event.preventDefault();
      onNavigate(
        PILOT_PRIMARY_VIEWS[
          (index - 1 + PILOT_PRIMARY_VIEWS.length) % PILOT_PRIMARY_VIEWS.length
        ],
      );
    } else if (event.key === "Home") {
      event.preventDefault();
      onNavigate(PILOT_PRIMARY_VIEWS[0]);
    } else if (event.key === "End") {
      event.preventDefault();
      onNavigate(PILOT_PRIMARY_VIEWS[PILOT_PRIMARY_VIEWS.length - 1]);
    }
  };

  return (
    <WorkspaceCompositionProvider density={density}>
      <div
        className="app-shell exp-shell ws-env ws-shell"
        data-density={density}
        data-ambient={ambient}
      >
        <div className="ws-layer ws-layer--bg" aria-hidden="true">
          <div className="ws-atmosphere">
            <div className="ws-atmosphere__glow ws-atmosphere__glow--a" />
            <div className="ws-atmosphere__glow ws-atmosphere__glow--b" />
            <div className="ws-atmosphere__glow ws-atmosphere__glow--c" />
            <div className="ws-atmosphere__glow ws-atmosphere__glow--d" />
            <div className="ws-atmosphere__glow ws-atmosphere__glow--e" />
            <div className="ws-atmosphere__grain" />
            <div className="ws-atmosphere__vignette" />
          </div>
          <AmbientLighting focus={ambient} />
        </div>

        <div className="sr-only" aria-live="polite" ref={liveRef} />

        <header className="app-chrome exp-chrome ws-menubar ws-layer ws-layer--chrome">
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

        <div className="ws-stage ws-layer ws-layer--plane" role="presentation">
          <motion.div className="ws-spatial" layout={!reduceMotion}>
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
                    : { opacity: 0, y: 16, scale: 0.986, filter: "blur(3px)" }
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
                    ? { duration: 0.01 }
                    : { ...spring.soft, opacity: { duration: 0.26 } }
                }
                onFocusCapture={(event) => {
                  const target = event.target as HTMLElement;
                  if (
                    target.tagName === "INPUT" ||
                    target.tagName === "TEXTAREA"
                  ) {
                    setAmbient("input");
                  }
                }}
              >
                {children}
              </motion.div>
            </AnimatePresence>
          </motion.div>
        </div>

        <nav
          className="tabs exp-nav ws-dock ws-layer ws-layer--dock"
          aria-label="Workspace"
          role="tablist"
          onKeyDown={onDockKeyDown}
        >
          <span className="ws-dock__breath" aria-hidden="true" />
          {PILOT_PRIMARY_VIEWS.map((id) => (
            <DockItem
              key={id}
              id={id}
              active={view === id}
              contentId={contentId}
              reduceMotion={reduceMotion}
              onNavigate={onNavigate}
              onFocusAmbient={setAmbient}
            />
          ))}
        </nav>
      </div>
    </WorkspaceCompositionProvider>
  );
}
