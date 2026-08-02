/**
 * Experience demo IPC adapter — in-memory implementation of
 * `experienceIpcCatalog`. Production Tauri handlers own the same command names.
 */

import type {
  ActionPlan,
  PilotMeasurementSnapshot,
  ResumePlanPreview,
  SavedContext,
  Workspace,
} from "../types/domain";
import type {
  WorkspaceHealth,
  WorkspaceSettings,
  WorkspaceStatus,
} from "../types/workspace";
import {
  buildDemoExecuteResult,
  buildDemoResumePreview,
  DEMO_CAPTURE_SCOPE,
  DEMO_PILOT_SCOPE,
  DEMO_PILOT_SNAPSHOT,
  DEMO_RESTORE_HISTORY,
  DEMO_SAVED_CONTEXTS,
  DEMO_WORKSPACE,
  type DemoRestoreHistoryEntry,
} from "./experienceDemoDataset";
import { isExperienceIpcCommand } from "./experienceIpcCatalog";

class DemoIpcError extends Error {
  readonly code: string;
  constructor(code: string, message: string) {
    super(message);
    this.code = code;
  }
}

interface DemoState {
  workspace: Workspace;
  contexts: SavedContext[];
  settings: WorkspaceSettings;
  pilot: PilotMeasurementSnapshot;
  restoreHistory: Record<string, DemoRestoreHistoryEntry[]>;
}

function clonePilot(snapshot: PilotMeasurementSnapshot): PilotMeasurementSnapshot {
  return structuredClone(snapshot);
}

function createInitialState(): DemoState {
  return {
    workspace: { ...DEMO_WORKSPACE },
    contexts: structuredClone(DEMO_SAVED_CONTEXTS),
    settings: {
      theme: "dark",
      first_run: false,
      settings_version: 1,
      active_workspace_id: DEMO_WORKSPACE.id,
      personalization_enabled: false,
    },
    pilot: clonePilot(DEMO_PILOT_SNAPSHOT),
    restoreHistory: structuredClone(DEMO_RESTORE_HISTORY),
  };
}

let state = createInitialState();

export function resetExperienceDemoState(): void {
  state = createInitialState();
}

function requireContext(id: string): SavedContext {
  const found = state.contexts.find((context) => context.id === id);
  if (!found) {
    throw new DemoIpcError("not_found", "That saved moment is not available.");
  }
  return found;
}

function recomputePilotAggregates(): void {
  const minutes = state.pilot.leave_resume.map((r) => r.return_minutes).sort(
    (a, b) => a - b,
  );
  state.pilot.distinct_resume_days = new Set(
    state.pilot.leave_resume.map((r) => r.local_day),
  ).size;
  if (minutes.length === 0) {
    state.pilot.median_return_minutes = null;
  } else {
    const mid = Math.floor(minutes.length / 2);
    state.pilot.median_return_minutes =
      minutes.length % 2 === 0
        ? Math.round((minutes[mid - 1] + minutes[mid]) / 2)
        : minutes[mid];
  }
}

/**
 * Adapter entry — same command names as Tauri `generate_handler` experience set.
 * Caller must gate with shouldUseExperienceDemo().
 */
export async function demoInvoke<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  // Tiny async tick so callers keep their await shape.
  await Promise.resolve();

  if (!isExperienceIpcCommand(command)) {
    throw new DemoIpcError(
      "demo_unsupported",
      `Demo adapter does not implement “${command}”.`,
    );
  }

  switch (command) {
    case "get_workspace_status": {
      const data: WorkspaceStatus = {
        status: "ok",
        version: "demo",
        initialized: true,
      };
      return data as T;
    }
    case "get_workspace_health": {
      const data: WorkspaceHealth = {
        status: "healthy",
        initialized: true,
        services: ["demo"],
        version: "demo",
      };
      return data as T;
    }
    case "get_settings":
      return { ...state.settings } as T;
    case "update_settings": {
      const update = (args?.update ?? {}) as Partial<WorkspaceSettings> & {
        active_workspace_id?: string;
      };
      if (typeof update.active_workspace_id === "string") {
        state.settings.active_workspace_id =
          update.active_workspace_id.trim() || null;
      }
      if (typeof update.theme === "string") {
        state.settings.theme = update.theme;
      }
      return { ...state.settings } as T;
    }
    case "get_workspace": {
      const id = String(args?.id ?? "");
      if (id && id !== state.workspace.id) {
        throw new DemoIpcError("not_found", "Workspace not found.");
      }
      return { ...state.workspace } as T;
    }
    case "create_workspace": {
      const name = String(args?.name ?? "My Workspace").trim() || "My Workspace";
      state.workspace = {
        ...state.workspace,
        name,
        updated_at: new Date().toISOString(),
      };
      state.settings.active_workspace_id = state.workspace.id;
      return { ...state.workspace } as T;
    }
    case "list_saved_contexts": {
      const workspaceId = String(args?.workspaceId ?? "");
      if (workspaceId && workspaceId !== state.workspace.id) {
        return [] as T;
      }
      return structuredClone(state.contexts) as T;
    }
    case "get_saved_context": {
      const id = String(args?.savedContextId ?? "");
      return structuredClone(requireContext(id)) as T;
    }
    case "get_saved_context_capture_scope":
      return structuredClone(DEMO_CAPTURE_SCOPE) as T;
    case "save_workspace_context": {
      const workspaceId = String(args?.workspaceId ?? "");
      if (workspaceId !== state.workspace.id) {
        throw new DemoIpcError("invalid", "Workspace mismatch.");
      }
      const name = String(args?.name ?? "").trim() || "Untitled moment";
      const handoffNote = String(args?.handoffNote ?? "").trim();
      const now = new Date().toISOString();
      const created: SavedContext = {
        id: `demo-ctx-${Date.now()}`,
        workspace_id: state.workspace.id,
        name,
        created_at: now,
        captured_at: now,
        approved_scope: String(args?.approvedScope ?? DEMO_CAPTURE_SCOPE.id),
        handoff_note: handoffNote,
        observation_pass_id: `demo-pass-${Date.now()}`,
        monitors: structuredClone(DEMO_SAVED_CONTEXTS[0]?.monitors ?? []),
        windows: structuredClone(DEMO_SAVED_CONTEXTS[0]?.windows ?? []).map(
          (window, index) => ({
            ...window,
            id: `demo-win-new-${index}`,
            focused: index === 0,
          }),
        ),
      };
      state.contexts = [created, ...state.contexts];
      return structuredClone(created) as T;
    }
    case "resolve_resume_plan": {
      const id = String(args?.savedContextId ?? "");
      const context = requireContext(id);
      return buildDemoResumePreview(context) as T;
    }
    case "execute_resume_plan": {
      const plan = args?.plan as ActionPlan | undefined;
      const digest = String(args?.approvedPlanDigest ?? "");
      if (!plan || plan.plan_digest !== digest) {
        throw new DemoIpcError(
          "plan_mismatch",
          "That restore plan is no longer valid. Preview again.",
        );
      }
      const preview: ResumePlanPreview = {
        saved_context_id: plan.plan_id.replace(/^demo-plan-/, "demo-ctx-"),
        saved_context_name: plan.purpose,
        handoff_note: "",
        plan,
      };
      // Recover context id from plan_id `demo-plan-${id}`
      const contextId = plan.plan_id.startsWith("demo-plan-")
        ? plan.plan_id.slice("demo-plan-".length)
        : "";
      const context = state.contexts.find((c) => c.id === contextId);
      if (context) {
        preview.saved_context_id = context.id;
        preview.saved_context_name = context.name;
        preview.handoff_note = context.handoff_note;
      }
      const outcome = buildDemoExecuteResult(preview);
      if (contextId) {
        const entry: DemoRestoreHistoryEntry = {
          restored_at: new Date().toISOString(),
          outcome: outcome.outcome.replace(/_/g, " "),
          summary:
            outcome.outcome === "completed"
              ? "All projected windows restored."
              : "Some windows needed a manual nudge.",
        };
        state.restoreHistory[contextId] = [
          entry,
          ...(state.restoreHistory[contextId] ?? []),
        ].slice(0, 6);
      }
      return outcome as T;
    }
    case "delete_saved_context": {
      const id = String(args?.savedContextId ?? "");
      state.contexts = state.contexts.filter((context) => context.id !== id);
      delete state.restoreHistory[id];
      return null as T;
    }
    case "get_pilot_measurement_scope":
      return structuredClone(DEMO_PILOT_SCOPE) as T;
    case "get_pilot_measurement_snapshot":
      return clonePilot(state.pilot) as T;
    case "grant_pilot_consent": {
      const scopeId = String(args?.approvedScope ?? DEMO_PILOT_SCOPE.id);
      state.pilot.consent = {
        scope_id: scopeId,
        consented_at: new Date().toISOString(),
        withdrawn_at: null,
      };
      return null as T;
    }
    case "withdraw_pilot_consent": {
      const clearRecords = Boolean(args?.clearRecords);
      if (state.pilot.consent) {
        state.pilot.consent = {
          ...state.pilot.consent,
          withdrawn_at: new Date().toISOString(),
        };
      }
      if (clearRecords) {
        state.pilot.baseline = null;
        state.pilot.leave_resume = [];
        state.pilot.interview_baseline = null;
        state.pilot.interview_week_four = null;
        recomputePilotAggregates();
      }
      return null as T;
    }
    case "record_pilot_baseline": {
      state.pilot.baseline = {
        return_minutes: Number(args?.returnMinutes ?? 0),
        recorded_at: new Date().toISOString(),
        notes: String(args?.notes ?? ""),
      };
      return null as T;
    }
    case "record_pilot_leave_resume": {
      const localDay =
        String(args?.localDay ?? "").trim() ||
        new Date().toISOString().slice(0, 10);
      state.pilot.leave_resume = [
        {
          id: `demo-lr-${Date.now()}`,
          recorded_at: new Date().toISOString(),
          local_day: localDay,
          return_minutes: Number(args?.returnMinutes ?? 0),
          correction_needed: Boolean(args?.correctionNeeded),
          correction_note: String(args?.correctionNote ?? ""),
        },
        ...state.pilot.leave_resume,
      ];
      recomputePilotAggregates();
      return null as T;
    }
    case "record_pilot_interview": {
      const phase = args?.phase === "week_four" ? "week_four" : "baseline";
      const record = {
        phase: phase as "baseline" | "week_four",
        recorded_at: new Date().toISOString(),
        responses: String(args?.responses ?? ""),
      };
      if (phase === "week_four") {
        state.pilot.interview_week_four = record;
      } else {
        state.pilot.interview_baseline = record;
      }
      return null as T;
    }
    default: {
      const _exhaustive: never = command;
      throw new DemoIpcError(
        "demo_unsupported",
        `Demo adapter missing case for “${String(_exhaustive)}”.`,
      );
    }
  }
}
