import { useEffect, useState } from "react";
import { invokeIpc } from "./lib/ipc";
import type {
  WorkspaceHealth,
  WorkspaceSettings,
  WorkspaceStatus,
} from "./types/workspace";

function formatError(err: unknown): string {
  if (err instanceof Error) {
    return err.message;
  }
  return String(err);
}

export default function App() {
  const [status, setStatus] = useState<WorkspaceStatus | null>(null);
  const [health, setHealth] = useState<WorkspaceHealth | null>(null);
  const [settings, setSettings] = useState<WorkspaceSettings | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    Promise.all([
      invokeIpc<WorkspaceStatus>("get_workspace_status"),
      invokeIpc<WorkspaceHealth>("get_workspace_health"),
      invokeIpc<WorkspaceSettings>("get_settings"),
    ])
      .then(([nextStatus, nextHealth, nextSettings]) => {
        setStatus(nextStatus);
        setHealth(nextHealth);
        setSettings(nextSettings);
      })
      .catch((err: unknown) => setError(formatError(err)));
  }, []);

  return (
    <main className="container">
      <h1>Workspace</h1>
      {error && <p className="error">Error: {error}</p>}
      {status ? (
        <section>
          <h2>Runtime</h2>
          <dl>
            <dt>Status</dt>
            <dd>{status.status}</dd>
            <dt>Version</dt>
            <dd>{status.version}</dd>
            <dt>Initialized</dt>
            <dd>{status.initialized ? "yes" : "no"}</dd>
          </dl>
        </section>
      ) : (
        !error && <p>Loading runtime status from kernel…</p>
      )}
      {health ? (
        <section>
          <h2>Health</h2>
          <dl>
            <dt>Status</dt>
            <dd>{health.status}</dd>
            <dt>Services</dt>
            <dd>{health.services.join(", ")}</dd>
          </dl>
        </section>
      ) : (
        !error && status && <p>Loading health from kernel…</p>
      )}
      {settings ? (
        <section>
          <h2>Settings</h2>
          <dl>
            <dt>Theme</dt>
            <dd>{settings.theme}</dd>
            <dt>First run</dt>
            <dd>{settings.first_run ? "yes" : "no"}</dd>
            <dt>Settings version</dt>
            <dd>{settings.settings_version}</dd>
          </dl>
        </section>
      ) : (
        !error && health && <p>Loading settings from kernel…</p>
      )}
    </main>
  );
}
