import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { WorkspaceSettings, WorkspaceStatus } from "./types/workspace";

function formatError(err: unknown): string {
  if (typeof err === "string") {
    return err;
  }
  if (err && typeof err === "object" && "message" in err) {
    return String((err as { message: unknown }).message);
  }
  return String(err);
}

export default function App() {
  const [status, setStatus] = useState<WorkspaceStatus | null>(null);
  const [settings, setSettings] = useState<WorkspaceSettings | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    Promise.all([
      invoke<WorkspaceStatus>("get_workspace_status"),
      invoke<WorkspaceSettings>("get_settings"),
    ])
      .then(([nextStatus, nextSettings]) => {
        setStatus(nextStatus);
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
            <dt>Initialization</dt>
            <dd>{status.initialization}</dd>
          </dl>
        </section>
      ) : (
        !error && <p>Loading runtime status from kernel…</p>
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
        !error && status && <p>Loading settings from kernel…</p>
      )}
    </main>
  );
}
