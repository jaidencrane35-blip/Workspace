import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { WorkspaceStatus } from "./types/workspace";

export default function App() {
  const [status, setStatus] = useState<WorkspaceStatus | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<WorkspaceStatus>("get_workspace_status")
      .then(setStatus)
      .catch((err: unknown) => setError(String(err)));
  }, []);

  return (
    <main className="container">
      <h1>Workspace</h1>
      {error && <p className="error">Error: {error}</p>}
      {status ? (
        <dl>
          <dt>Status</dt>
          <dd>{status.status}</dd>
          <dt>Version</dt>
          <dd>{status.version}</dd>
        </dl>
      ) : (
        !error && <p>Loading status from Rust core…</p>
      )}
    </main>
  );
}
