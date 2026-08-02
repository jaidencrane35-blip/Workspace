/**
 * Experience Demo Dataset — authentic fixture for development evaluation.
 * Isolated and removable. Never imported by production runtime paths except via demoIpc.
 */

import type {
  ActionOperationResult,
  PilotMeasurementScope,
  PilotMeasurementSnapshot,
  ResumePlanPreview,
  SavedContext,
  SavedContextCaptureScope,
  SavedContextMonitor,
  SavedContextWindow,
  Workspace,
} from "../types/domain";

/** Anchor “now” for relative ages (matches capture date). */
const ANCHOR = Date.parse("2026-08-02T10:00:00+10:00");

function at(offsetMs: number): string {
  return new Date(ANCHOR + offsetMs).toISOString();
}

const hour = 3_600_000;
const day = 24 * hour;

export const DEMO_WORKSPACE: Workspace = {
  id: "demo-workspace-atelier",
  name: "Atelier",
  created_at: at(-45 * day),
  updated_at: at(-2 * hour),
};

const MONITORS: SavedContextMonitor[] = [
  {
    id: "demo-mon-0",
    monitor_index: 0,
    name: "DELL U2723QE",
    x: 0,
    y: 0,
    width: 2560,
    height: 1440,
    is_primary: true,
  },
  {
    id: "demo-mon-1",
    monitor_index: 1,
    name: "LG 27UL850",
    x: 2560,
    y: 0,
    width: 2560,
    height: 1440,
    is_primary: false,
  },
];

function windowOf(
  partial: Omit<SavedContextWindow, "restore_identity"> & {
    restore_identity?: SavedContextWindow["restore_identity"];
  },
): SavedContextWindow {
  return {
    restore_identity: partial.restore_identity ?? {
      identity_schema_version: "1",
      desktop_session_id: "demo-session-aug2",
      captured_hwnd: `0x${(10000 + partial.process_id).toString(16)}`,
      captured_process_id: partial.process_id,
      title_fingerprint: partial.title.slice(0, 48),
      captured_at: at(-3 * hour),
    },
    restore_identity_unavailable_reason: null,
    ...partial,
  };
}

/** Moments newest-first. Tags appear after · in the name (display-only). */
export const DEMO_SAVED_CONTEXTS: SavedContext[] = [
  {
    id: "demo-ctx-northwind",
    workspace_id: DEMO_WORKSPACE.id,
    name: "Northwind deck · Client",
    created_at: at(-1.2 * hour),
    captured_at: at(-1.2 * hour),
    approved_scope: "pp-p01-capture-v1",
    handoff_note:
      "Finish the pricing slide, then send the PDF to Mara before standup.",
    observation_pass_id: "demo-pass-northwind",
    monitors: MONITORS,
    windows: [
      windowOf({
        id: "demo-win-nw-ppt",
        title: "Northwind Q3 — PowerPoint",
        process_id: 14822,
        x: 120,
        y: 80,
        width: 1680,
        height: 1050,
        monitor_index: 0,
        minimized: false,
        focused: true,
        z_order: 1,
      }),
      windowOf({
        id: "demo-win-nw-chrome",
        title: "Competitor pricing — Chrome",
        process_id: 9021,
        x: 2560 + 40,
        y: 60,
        width: 1280,
        height: 1100,
        monitor_index: 1,
        minimized: false,
        focused: false,
        z_order: 2,
      }),
      windowOf({
        id: "demo-win-nw-slack",
        title: "Mara · Slack",
        process_id: 12044,
        x: 2560 + 1340,
        y: 60,
        width: 720,
        height: 980,
        monitor_index: 1,
        minimized: false,
        focused: false,
        z_order: 3,
      }),
    ],
  },
  {
    id: "demo-ctx-sprint",
    workspace_id: DEMO_WORKSPACE.id,
    name: "Sprint board · Engineering",
    created_at: at(-5 * hour),
    captured_at: at(-5 * hour),
    approved_scope: "pp-p01-capture-v1",
    handoff_note:
      "Move AUTH-214 to review once the token refresh PR is green.",
    observation_pass_id: "demo-pass-sprint",
    monitors: [MONITORS[0]],
    windows: [
      windowOf({
        id: "demo-win-sp-linear",
        title: "Atelier · Linear",
        process_id: 7712,
        x: 0,
        y: 0,
        width: 1440,
        height: 1440,
        monitor_index: 0,
        minimized: false,
        focused: true,
        z_order: 1,
      }),
      windowOf({
        id: "demo-win-sp-code",
        title: "workspace — Cursor",
        process_id: 5102,
        x: 1440,
        y: 0,
        width: 1120,
        height: 1440,
        monitor_index: 0,
        minimized: false,
        focused: false,
        z_order: 2,
      }),
    ],
  },
  {
    id: "demo-ctx-writing",
    workspace_id: DEMO_WORKSPACE.id,
    name: "Pilot notes · Writing",
    created_at: at(-1 * day - 3 * hour),
    captured_at: at(-1 * day - 3 * hour),
    approved_scope: "pp-p01-capture-v1",
    handoff_note:
      "Tighten the restore-limits paragraph; keep the voice first-person.",
    observation_pass_id: "demo-pass-writing",
    monitors: [MONITORS[0]],
    windows: [
      windowOf({
        id: "demo-win-wr-docs",
        title: "Pilot week-two notes — Google Docs",
        process_id: 6601,
        x: 200,
        y: 100,
        width: 1100,
        height: 1200,
        monitor_index: 0,
        minimized: false,
        focused: true,
        z_order: 1,
      }),
      windowOf({
        id: "demo-win-wr-ref",
        title: "RESTORE_LIMITS_SUMMARY — Notepad",
        process_id: 3340,
        x: 1400,
        y: 180,
        width: 720,
        height: 640,
        monitor_index: 0,
        minimized: false,
        focused: false,
        z_order: 2,
      }),
    ],
  },
  {
    id: "demo-ctx-finance",
    workspace_id: DEMO_WORKSPACE.id,
    name: "July close · Finance",
    created_at: at(-2 * day - 6 * hour),
    captured_at: at(-2 * day - 6 * hour),
    approved_scope: "pp-p01-capture-v1",
    handoff_note:
      "Reconcile the Stripe payout row, then export the CSV for accounting.",
    observation_pass_id: "demo-pass-finance",
    monitors: MONITORS,
    windows: [
      windowOf({
        id: "demo-win-fi-excel",
        title: "July_close.xlsx — Excel",
        process_id: 8840,
        x: 80,
        y: 40,
        width: 1600,
        height: 1200,
        monitor_index: 0,
        minimized: false,
        focused: true,
        z_order: 1,
      }),
      windowOf({
        id: "demo-win-fi-stripe",
        title: "Payouts — Stripe Dashboard",
        process_id: 9021,
        x: 2560 + 100,
        y: 80,
        width: 1400,
        height: 1000,
        monitor_index: 1,
        minimized: false,
        focused: false,
        z_order: 2,
      }),
      windowOf({
        id: "demo-win-fi-mail",
        title: "Re: July close — Outlook",
        process_id: 4102,
        x: 2560 + 1600,
        y: 200,
        width: 880,
        height: 900,
        monitor_index: 1,
        minimized: true,
        focused: false,
        z_order: 4,
      }),
    ],
  },
  {
    id: "demo-ctx-design",
    workspace_id: DEMO_WORKSPACE.id,
    name: "Shell polish · Design",
    created_at: at(-4 * day),
    captured_at: at(-4 * day),
    approved_scope: "pp-p01-capture-v1",
    handoff_note:
      "Compare dock breath against the concept board; prefer quieter motion.",
    observation_pass_id: "demo-pass-design",
    monitors: [MONITORS[0]],
    windows: [
      windowOf({
        id: "demo-win-de-figma",
        title: "Workspace concept — Figma",
        process_id: 9901,
        x: 0,
        y: 0,
        width: 1920,
        height: 1440,
        monitor_index: 0,
        minimized: false,
        focused: true,
        z_order: 1,
      }),
      windowOf({
        id: "demo-win-de-app",
        title: "Workspace — preview",
        process_id: 5102,
        x: 1920,
        y: 200,
        width: 640,
        height: 1000,
        monitor_index: 0,
        minimized: false,
        focused: false,
        z_order: 2,
      }),
    ],
  },
  {
    id: "demo-ctx-ops",
    workspace_id: DEMO_WORKSPACE.id,
    name: "Vendor call · Ops",
    created_at: at(-6 * day - 2 * hour),
    captured_at: at(-6 * day - 2 * hour),
    approved_scope: "pp-p01-capture-v1",
    handoff_note:
      "Ask about the SLA credit; note the new support escalation path.",
    observation_pass_id: "demo-pass-ops",
    monitors: [MONITORS[0]],
    windows: [
      windowOf({
        id: "demo-win-op-zoom",
        title: "Zoom Meeting — Acme support",
        process_id: 2201,
        x: 320,
        y: 120,
        width: 1280,
        height: 800,
        monitor_index: 0,
        minimized: false,
        focused: true,
        z_order: 1,
      }),
      windowOf({
        id: "demo-win-op-notes",
        title: "Call notes — OneNote",
        process_id: 5500,
        x: 1640,
        y: 160,
        width: 800,
        height: 960,
        monitor_index: 0,
        minimized: false,
        focused: false,
        z_order: 2,
      }),
    ],
  },
];

export const DEMO_CAPTURE_SCOPE: SavedContextCaptureScope = {
  id: "pp-p01-capture-v1",
  purpose:
    "Capture open windows and monitors in this Windows session so you can continue later.",
  captured: [
    { key: "windows", summary: "Open top-level windows (title, bounds, focus)" },
    { key: "monitors", summary: "Connected monitors and arrangement" },
    { key: "handoff", summary: "The note you write yourself" },
  ],
  excluded: [
    { key: "files", summary: "File contents and document bodies" },
    { key: "credentials", summary: "Passwords, tokens, and cookies" },
    { key: "closed", summary: "Apps you already quit" },
    { key: "cloud", summary: "Anything sent off this computer" },
  ],
};

export const DEMO_PILOT_SCOPE: PilotMeasurementScope = {
  id: "pp-p01e-scope-v1",
  purpose:
    "Local pilot pulse: how long returning to work takes after an interruption.",
  measured: [
    { key: "baseline", summary: "Self-reported minutes to return before Workspace" },
    { key: "leave_resume", summary: "Minutes you enter after a Continue" },
    { key: "interview", summary: "Short written reflections you choose to save" },
  ],
  not_measured: [
    { key: "keystrokes", summary: "Keystrokes, screenshots, or ambient watching" },
    { key: "upload", summary: "Nothing leaves this computer for the pilot" },
    { key: "saved_context", summary: "Check-in answers are not saved contexts" },
  ],
};

export const DEMO_PILOT_SNAPSHOT: PilotMeasurementSnapshot = {
  scope: DEMO_PILOT_SCOPE,
  consent: {
    scope_id: DEMO_PILOT_SCOPE.id,
    consented_at: at(-28 * day),
    withdrawn_at: null,
  },
  baseline: {
    return_minutes: 22,
    recorded_at: at(-27 * day),
    notes: "Usually rebuild the desktop from memory and Slack threads.",
  },
  leave_resume: [
    {
      id: "demo-lr-1",
      recorded_at: at(-20 * day),
      local_day: "2026-07-13",
      return_minutes: 14,
      correction_needed: false,
      correction_note: "",
    },
    {
      id: "demo-lr-2",
      recorded_at: at(-15 * day),
      local_day: "2026-07-18",
      return_minutes: 11,
      correction_needed: true,
      correction_note: "Had to re-open Outlook; it had been closed overnight.",
    },
    {
      id: "demo-lr-3",
      recorded_at: at(-10 * day),
      local_day: "2026-07-23",
      return_minutes: 9,
      correction_needed: false,
      correction_note: "",
    },
    {
      id: "demo-lr-4",
      recorded_at: at(-5 * day),
      local_day: "2026-07-28",
      return_minutes: 8,
      correction_needed: false,
      correction_note: "",
    },
    {
      id: "demo-lr-5",
      recorded_at: at(-1 * day),
      local_day: "2026-08-01",
      return_minutes: 7,
      correction_needed: false,
      correction_note: "",
    },
  ],
  interview_baseline: {
    phase: "baseline",
    recorded_at: at(-27 * day),
    responses:
      "Usually 20–25 minutes. I use a messy desktop plus sticky notes. I switch contexts about six times a day.",
  },
  interview_week_four: {
    phase: "week_four",
    recorded_at: at(-2 * day),
    responses:
      "Used Resume on four days this week. The handoff note mattered more than perfect window placement. I’d keep Workspace after the pilot.",
  },
  distinct_resume_days: 5,
  median_return_minutes: 9,
};

export interface DemoRestoreHistoryEntry {
  restored_at: string;
  outcome: string;
  summary: string;
}

/** Prior restores for Continue preview — demo presentation only. */
export const DEMO_RESTORE_HISTORY: Record<string, DemoRestoreHistoryEntry[]> = {
  "demo-ctx-northwind": [
    {
      restored_at: at(-26 * hour),
      outcome: "partially completed",
      summary: "PowerPoint and Chrome placed; Slack stayed minimized.",
    },
    {
      restored_at: at(-3 * day),
      outcome: "completed",
      summary: "All three windows focused as saved.",
    },
  ],
  "demo-ctx-sprint": [
    {
      restored_at: at(-2 * day - 4 * hour),
      outcome: "completed",
      summary: "Linear and Cursor restored on the main display.",
    },
  ],
  "demo-ctx-finance": [
    {
      restored_at: at(-3 * day),
      outcome: "partially completed",
      summary: "Excel restored; Outlook had already been closed.",
    },
  ],
};

export function buildDemoResumePreview(context: SavedContext): ResumePlanPreview {
  const expires = at(12 * hour);
  const items = context.windows.map((window, index) => {
    const skipMinimized = window.minimized;
    return {
      item_id: `demo-plan-item-${context.id}-${index}`,
      action_type: "place_window",
      target_summary: window.title,
      proposed_effect: {
        x: window.x,
        y: window.y,
        width: window.width,
        height: window.height,
        monitor_index: window.monitor_index,
      },
      permission_scope: "desktop.window.place",
      projected_disposition: skipMinimized
        ? ("will_skip_unresolvable" as const)
        : ("will_attempt" as const),
      reason: skipMinimized
        ? "Was minimized when saved; may need a click to show."
        : window.focused
          ? "You were working here — will try to focus again."
          : "Still open in this Windows session.",
      error_code: null,
    };
  });

  const willAttempt = items.filter(
    (item) => item.projected_disposition === "will_attempt",
  ).length;
  const willSkipUnresolvable = items.filter(
    (item) => item.projected_disposition === "will_skip_unresolvable",
  ).length;
  const total = items.length;
  const ratio = total === 0 ? 0 : willAttempt / total;
  const confidence_band =
    total === 0
      ? "empty"
      : ratio >= 0.85
        ? "high"
        : ratio >= 0.5
          ? "steady"
          : "limited";

  return {
    saved_context_id: context.id,
    saved_context_name: context.name,
    handoff_note: context.handoff_note,
    plan: {
      plan_id: `demo-plan-${context.id}`,
      expires_at: expires,
      plan_digest: `digest-${context.id}-v1`,
      purpose: `Restore “${context.name}” within this Windows session.`,
      items,
    },
    compatibility: {
      total_items: total,
      will_attempt: willAttempt,
      will_skip_unsupported: 0,
      will_skip_unresolvable: willSkipUnresolvable,
      missing_window_count: 0,
      confidence_band,
      restore_eligible: willAttempt > 0,
    },
  };
}

export function buildDemoExecuteResult(
  preview: ResumePlanPreview,
): ActionOperationResult {
  const items = preview.plan.items.map((item) => {
    const skipped = item.projected_disposition !== "will_attempt";
    return {
      item_id: item.item_id,
      action_type: item.action_type,
      target_summary: item.target_summary,
      disposition: skipped
        ? ("skipped_unresolvable" as const)
        : ("completed" as const),
      what: skipped
        ? "Left as-is — window was not placeable."
        : "Moved and sized to the saved place.",
      why: item.reason ?? "Matches the approved restore plan.",
      reason: item.reason,
      error_code: null,
      user_action_available: skipped
        ? "Open the app if you still need it."
        : "None",
    };
  });
  const completed = items.filter((i) => i.disposition === "completed").length;
  return {
    operation_id: `demo-op-${preview.plan.plan_id}`,
    outcome:
      completed === items.length
        ? "completed"
        : completed > 0
          ? "partially_completed"
          : "failed",
    items,
  };
}
