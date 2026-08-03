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
  type CSSProperties,
  type KeyboardEvent,
  type MouseEvent,
  type ReactNode,
  useCallback,
  useEffect,
  useId,
  useRef,
} from "react";
import { interaction, spring } from "../design-system";
import { ICON } from "../lib/icons";
import { INTENT_LABELS } from "../lib/intent";
import { contentTransition, motionPrimitive } from "../lib/motion";
import { useWorkspaceDensity } from "../hooks/useWorkspaceDensity";
import {
  PILOT_PRIMARY_VIEWS,
  PILOT_VIEW_LABELS,
  type PilotPrimaryView,
} from "../lib/pilotChrome";
import type { Workspace } from "../types/domain";
import {
  ActiveMomentProvider,
  PersistentMomentStage,
} from "./ActiveMoment";
import { AmbientLighting } from "./AmbientLighting";
import { CommandSurface } from "./CommandSurface";
import { CognitiveEngineProvider, useIntentEngine } from "./CognitiveEngine";
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
  onCreateWorkspace?: () => void;
  onContinueMoment?: (contextId: string) => void;
  workspace?: Workspace | null;
  focusContextId?: string | null;
  busy?: boolean;
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
  onFocusAmbient: (focus: "dock" | "workspace") => void;
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
      aria-current={active ? "true" : undefined}
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
      whileHover={reduceMotion ? undefined : { scale: 1.03 }}
      whileTap={reduceMotion ? undefined : { scale: interaction.pressScale }}
      transition={spring.soft}
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
        <Icon size={ICON.lg} strokeWidth={ICON.stroke} aria-hidden="true" />
      </span>
      <span className="ws-dock__label">{PILOT_VIEW_LABELS[id]}</span>
    </motion.button>
  );
}

function ShellBody({
  view,
  onNavigate,
  onCreateWorkspace,
  onContinueMoment,
  workspace = null,
  focusContextId = null,
  busy = false,
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
    primaryObjectId,
    setAmbient,
    setWritingMode,
  } = useWorkspaceComposition();
  const { intent, profile, adoptView, setWriting } = useIntentEngine();
  // Focus shift inside one place — not a page enter/exit.
  const focusShift = motionPrimitive("focus", Boolean(reduceMotion));

  useEffect(() => {
    contentRef.current?.focus({ preventScroll: true });
    // Object identity persists across destinations — do not clear primary.
    if (view !== "save") {
      setWritingMode(false);
      setWriting(false);
    }
    adoptView(view);
    setAmbient(view === "save" ? "input" : "moment");
  }, [view, setAmbient, setWritingMode, setWriting, adoptView]);

  useEffect(() => {
    if (liveRef.current) {
      liveRef.current.textContent = `${INTENT_LABELS[intent]} · ${PILOT_VIEW_LABELS[view]}`;
    }
  }, [intent, view]);

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
      data-writing={writingMode || profile.attentionScene === "writing" ? "on" : "off"}
      data-destination={view}
      data-place="continuous"
      data-living={primaryObjectId && !writingMode ? "on" : "off"}
      data-intent={intent}
      data-scene={profile.attentionScene}
      data-light={profile.lightingBias}
      style={
        {
          "--intent-space": String(profile.spacingScale),
          "--intent-depth": String(profile.atmosphereDepth),
        } as CSSProperties
      }
    >
      <div className="ws-layer ws-layer--bg" aria-hidden="true">
        <div className="ws-atmosphere" data-material="environmental">
          <div className="ws-atmosphere__glow ws-atmosphere__glow--a" />
          <div className="ws-atmosphere__glow ws-atmosphere__glow--b" />
          <div className="ws-atmosphere__grain" />
          <div className="ws-atmosphere__vignette" />
        </div>
        <AmbientLighting
          focus={
            intent === "capture"
              ? "input"
              : intent === "restore"
                ? "moment"
                : ambient === "none"
                  ? "workspace"
                  : ambient
          }
          scene={profile.attentionScene}
          living={Boolean(primaryObjectId) && !writingMode}
          active
        />
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
        <motion.div className="ws-spatial" layout={false}>
          <WorkspaceCanvas destination={view}>
            <ActiveMomentProvider
              workspace={workspace}
              view={view}
              focusContextId={focusContextId}
            >
              <div className="ws-place-world">
                <PersistentMomentStage
                  busy={busy}
                  onContinue={onContinueMoment}
                />
                <AnimatePresence mode="sync" initial={false}>
                  <motion.div
                    key={view}
                    ref={contentRef}
                    id={contentId}
                    className="ws-content ws-region ws-region--attach"
                    role="region"
                    aria-label={INTENT_LABELS[intent]}
                    tabIndex={-1}
                    initial={focusShift.initial}
                    animate={focusShift.animate}
                    exit={{ ...focusShift.exit, pointerEvents: "none" }}
                    transition={contentTransition(reduceMotion)}
                    onFocusCapture={(event) => {
                      const target = event.target as HTMLElement;
                      if (
                        target.tagName === "INPUT" ||
                        target.tagName === "TEXTAREA"
                      ) {
                        setAmbient("input");
                        setWriting(true);
                        setWritingMode(true);
                      }
                    }}
                    onBlurCapture={(event) => {
                      const next = event.relatedTarget as HTMLElement | null;
                      if (
                        next &&
                        (next.tagName === "INPUT" || next.tagName === "TEXTAREA")
                      ) {
                        return;
                      }
                      if (view !== "save") {
                        setWriting(false);
                        setWritingMode(false);
                      }
                    }}
                  >
                    {children}
                    <CommandSurface
                      onNavigate={onNavigate}
                      onCreateWorkspace={onCreateWorkspace}
                      busy={busy}
                    />
                  </motion.div>
                </AnimatePresence>
              </div>
            </ActiveMomentProvider>
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
                opacity: profile.dockEmphasis,
                y: profile.dockEmphasis < 0.5 ? 12 : profile.dockEmphasis < 0.8 ? 4 : 0,
                scale: profile.dockEmphasis < 0.5 ? 0.95 : 1,
              }
        }
        transition={spring.soft}
      >
        {PILOT_PRIMARY_VIEWS.map((id) => (
          <DockItem
            key={id}
            id={id}
            active={view === id}
            contentId={contentId}
            reduceMotion={reduceMotion}
            onNavigate={onNavigate}
            onFocusAmbient={(focus) =>
              setAmbient(focus === "dock" ? "dock" : "workspace")
            }
          />
        ))}
      </motion.nav>
    </div>
  );
}

export function WorkspaceShell(props: WorkspaceShellProps) {
  const density = useWorkspaceDensity();
  return (
    <CognitiveEngineProvider view={props.view}>
      <WorkspaceCompositionProvider density={density}>
        <ShellBody {...props} />
      </WorkspaceCompositionProvider>
    </CognitiveEngineProvider>
  );
}
