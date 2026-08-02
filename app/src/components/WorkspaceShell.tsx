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
} from "react";
import { interaction, spring } from "../design-system";
import { useWorkspaceDensity } from "../hooks/useWorkspaceDensity";
import {
  PILOT_PRIMARY_VIEWS,
  PILOT_VIEW_LABELS,
  type PilotPrimaryView,
} from "../lib/pilotChrome";
import { AmbientLighting, type AmbientFocus } from "./AmbientLighting";
import { WorkspaceCanvas } from "./WorkspaceCanvas";
import {
  useWorkspaceComposition,
  WorkspaceCompositionProvider,
} from "./WorkspaceComposition";

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
  const last = useRef({ t: 0, cx: 0, cy: 0 });

  const onMove = useCallback(
    (event: MouseEvent<HTMLButtonElement>) => {
      if (reduceMotion || !ref.current) {
        return;
      }
      const now = performance.now();
      const dt = Math.max(8, now - last.current.t);
      const vx = (event.clientX - last.current.cx) / dt;
      const vy = (event.clientY - last.current.cy) / dt;
      last.current = { t: now, cx: event.clientX, cy: event.clientY };

      const rect = ref.current.getBoundingClientRect();
      const dx = event.clientX - (rect.left + rect.width / 2);
      const dy = event.clientY - (rect.top + rect.height / 2);
      const velocityBoost = 1 + Math.min(0.45, Math.hypot(vx, vy) * 8);
      const factor = interaction.magneticFactor * velocityBoost;
      x.set(
        Math.max(
          -interaction.magneticMax,
          Math.min(interaction.magneticMax, dx * factor),
        ),
      );
      y.set(
        Math.max(
          -interaction.magneticMax,
          Math.min(interaction.magneticMax, dy * factor),
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

function ShellBody({
  view,
  onNavigate,
  status,
  children,
}: WorkspaceShellProps) {
  const reduceMotion = useReducedMotion();
  const contentRef = useRef<HTMLDivElement>(null);
  const contentId = useId();
  const liveRef = useRef<HTMLDivElement>(null);
  const {
    density,
    writingMode,
    ambient,
    attentionScene,
    setAmbient,
    setWritingMode,
    setSelectedObjectId,
    setFocusedObjectId,
    setAttentionScene,
    setPrimaryObject,
  } = useWorkspaceComposition();

  useEffect(() => {
    contentRef.current?.focus({ preventScroll: true });
    setWritingMode(false);
    setSelectedObjectId(null);
    setFocusedObjectId(null);
    setPrimaryObject(null);
    setAttentionScene(
      view === "save"
        ? "writing"
        : view === "help"
          ? "guide"
          : view === "pilot"
            ? "checkin"
            : "default",
    );
    setAmbient(view === "save" ? "input" : "workspace");
    if (liveRef.current) {
      liveRef.current.textContent = `${PILOT_VIEW_LABELS[view]} selected`;
    }
  }, [
    view,
    setAmbient,
    setWritingMode,
    setSelectedObjectId,
    setFocusedObjectId,
    setAttentionScene,
    setPrimaryObject,
  ]);

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
    <div
      className="app-shell exp-shell ws-env ws-shell"
      data-density={density}
      data-ambient={ambient}
      data-writing={writingMode ? "on" : "off"}
      data-destination={view}
      data-scene={attentionScene}
    >
      <div className="ws-layer ws-layer--bg" aria-hidden="true">
        <div className="ws-atmosphere">
          <div className="ws-atmosphere__glow ws-atmosphere__glow--a" />
          <div className="ws-atmosphere__glow ws-atmosphere__glow--b" />
          <div className="ws-atmosphere__glow ws-atmosphere__glow--c" />
          <div className="ws-atmosphere__glow ws-atmosphere__glow--d" />
          <div className="ws-atmosphere__glow ws-atmosphere__glow--e" />
          <div className="ws-atmosphere__glow ws-atmosphere__glow--f" />
          <div className="ws-atmosphere__grain" />
          <div className="ws-atmosphere__vignette" />
        </div>
        <AmbientLighting focus={ambient} active />
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
          <WorkspaceCanvas destination={view}>
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
                    : { opacity: 0, y: 14, scale: 0.988, filter: "blur(2px)" }
                }
                animate={
                  reduceMotion
                    ? { opacity: 1 }
                    : { opacity: 1, y: 0, scale: 1, filter: "blur(0px)" }
                }
                exit={
                  reduceMotion
                    ? { opacity: 0 }
                    : { opacity: 0, y: -8, scale: 0.992, filter: "blur(2px)" }
                }
                transition={
                  reduceMotion
                    ? { duration: 0.01 }
                    : { ...spring.soft, opacity: { duration: 0.24 } }
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
          </WorkspaceCanvas>
        </motion.div>
      </div>

      <motion.nav
        className="tabs exp-nav ws-dock ws-layer ws-layer--dock"
        aria-label="Workspace"
        role="tablist"
        onKeyDown={onDockKeyDown}
        animate={
          reduceMotion
            ? undefined
            : {
                opacity:
                  writingMode || attentionScene === "writing"
                    ? 0.32
                    : attentionScene === "restore"
                      ? 0.72
                      : attentionScene === "empty"
                        ? 0.85
                        : 1,
                y:
                  writingMode || attentionScene === "writing"
                    ? 12
                    : attentionScene === "restore"
                      ? 4
                      : 0,
                scale:
                  writingMode || attentionScene === "writing" ? 0.95 : 1,
              }
        }
        transition={spring.soft}
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
      </motion.nav>
    </div>
  );
}

export function WorkspaceShell(props: WorkspaceShellProps) {
  const density = useWorkspaceDensity();
  return (
    <WorkspaceCompositionProvider density={density}>
      <ShellBody {...props} />
    </WorkspaceCompositionProvider>
  );
}
