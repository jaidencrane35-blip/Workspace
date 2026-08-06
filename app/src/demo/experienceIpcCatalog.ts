/**
 * Pilot experience IPC catalog — contract between frozen UI and runtime.
 *
 * This list is the constitutional **Product** IPC tier authority
 * (`architecture/ARCHITECTURAL_CONSTITUTION_V2.md` §4.8).
 * Machine-readable tiers: `app/src/generated/ipcTiers.ts`
 * (`pnpm sync:ipc-tiers` / `pnpm verify:ipc-tiers`).
 *
 * Demo mode is an adapter that implements this catalog in-memory.
 * Production Tauri commands with the same names are the authority when
 * `shouldUseExperienceDemo()` is false.
 *
 * Do not change frozen experience components to work around missing APIs;
 * extend this catalog + kernel/Tauri handlers instead.
 */

export type ExperienceIpcCommand =
  | "get_workspace_status"
  | "get_workspace_health"
  | "get_settings"
  | "update_settings"
  | "get_workspace"
  | "create_workspace"
  | "list_saved_contexts"
  | "get_saved_context"
  | "get_saved_context_capture_scope"
  | "save_workspace_context"
  | "resolve_resume_plan"
  | "execute_resume_plan"
  | "delete_saved_context"
  | "get_pilot_measurement_scope"
  | "get_pilot_measurement_snapshot"
  | "grant_pilot_consent"
  | "withdraw_pilot_consent"
  | "record_pilot_baseline"
  | "record_pilot_leave_resume"
  | "record_pilot_interview";

/** Commands used by the frozen Home / Save / Continue / Check-in / Guide shell. */
export const EXPERIENCE_IPC_COMMANDS: readonly ExperienceIpcCommand[] = [
  "get_workspace_status",
  "get_workspace_health",
  "get_settings",
  "update_settings",
  "get_workspace",
  "create_workspace",
  "list_saved_contexts",
  "get_saved_context",
  "get_saved_context_capture_scope",
  "save_workspace_context",
  "resolve_resume_plan",
  "execute_resume_plan",
  "delete_saved_context",
  "get_pilot_measurement_scope",
  "get_pilot_measurement_snapshot",
  "grant_pilot_consent",
  "withdraw_pilot_consent",
  "record_pilot_baseline",
  "record_pilot_leave_resume",
  "record_pilot_interview",
] as const;

/** Screen → commands (Guide has no IPC). */
export const EXPERIENCE_SCREEN_COMMANDS = {
  bootstrap: [
    "get_workspace_status",
    "get_workspace_health",
    "get_settings",
    "get_workspace",
    "create_workspace",
    "update_settings",
  ],
  home: ["list_saved_contexts"],
  save: ["get_saved_context_capture_scope", "save_workspace_context"],
  continue: [
    "list_saved_contexts",
    "get_saved_context",
    "resolve_resume_plan",
    "execute_resume_plan",
    "delete_saved_context",
  ],
  checkin: [
    "get_pilot_measurement_scope",
    "get_pilot_measurement_snapshot",
    "grant_pilot_consent",
    "withdraw_pilot_consent",
    "record_pilot_baseline",
    "record_pilot_leave_resume",
    "record_pilot_interview",
  ],
  guide: [] as ExperienceIpcCommand[],
} as const;

export function isExperienceIpcCommand(
  command: string,
): command is ExperienceIpcCommand {
  return (EXPERIENCE_IPC_COMMANDS as readonly string[]).includes(command);
}
