/**
 * Purpose: Observed running desktop applications as interactive objects.
 * Owner: Frontend product shell (Product Contract V5)
 * Inputs: WorkspaceActiveApplication + windows from get_workspace_state
 * Outputs: Object grid; click focuses a matching observed window
 * Dependencies: applicationsUi + productShellUi + focus_desktop_window IPC
 * Non-responsibilities: Observation capture, registry mutation, Assistant
 */

import {
  activeApplicationName,
  activeApplicationWindowLine,
} from "../lib/applicationsUi";
import { monogramFromName } from "../lib/productShellUi";
import type {
  WorkspaceActiveApplication,
  WorkspaceStateWindow,
} from "../types/domain";

interface ActiveApplicationsViewProps {
  applications: WorkspaceActiveApplication[];
  windows: WorkspaceStateWindow[];
  loading: boolean;
  busy: boolean;
  onFocusApplication: (app: WorkspaceActiveApplication) => void;
}

export function resolveActiveApplicationHwnd(
  app: WorkspaceActiveApplication,
  windows: WorkspaceStateWindow[],
): string | null {
  const matches = windows.filter(
    (window) => window.process_id === app.process_id,
  );
  if (matches.length === 0) {
    return null;
  }
  const focused = matches.find((window) => window.focused);
  if (focused) {
    return focused.hwnd;
  }
  const visible = matches
    .filter((window) => window.visible && !window.minimized)
    .sort((a, b) => (a.z_order ?? Number.MAX_SAFE_INTEGER) - (b.z_order ?? Number.MAX_SAFE_INTEGER));
  if (visible[0]) {
    return visible[0].hwnd;
  }
  const ordered = [...matches].sort(
    (a, b) =>
      (a.z_order ?? Number.MAX_SAFE_INTEGER) -
      (b.z_order ?? Number.MAX_SAFE_INTEGER),
  );
  return ordered[0]?.hwnd ?? null;
}

export function ActiveApplicationsView({
  applications,
  windows,
  loading,
  busy,
  onFocusApplication,
}: ActiveApplicationsViewProps) {
  if (loading && applications.length === 0) {
    return <p className="muted">Loading…</p>;
  }
  if (applications.length === 0) {
    return <p className="muted">Nothing observed yet.</p>;
  }
  return (
    <ul className="app-object-grid" aria-label="Running applications">
      {applications.map((app) => {
        const name = activeApplicationName(app);
        const focusable = resolveActiveApplicationHwnd(app, windows) != null;
        return (
          <li key={`${app.process_id}-${app.process_name ?? "unknown"}`}>
            <button
              type="button"
              className="app-object-card"
              disabled={busy || !focusable}
              title={focusable ? `Focus ${name}` : `${name} — no window handle`}
              onClick={() => {
                onFocusApplication(app);
              }}
            >
              <span className="application-monogram large" aria-hidden="true">
                {monogramFromName(name)}
              </span>
              <span className="app-object-name">{name}</span>
              <span className="muted app-object-meta">
                {activeApplicationWindowLine(app)}
              </span>
            </button>
          </li>
        );
      })}
    </ul>
  );
}
