import { getDemoRestoreHistory } from "./demoIpc";
import { isExperienceDemoActive } from "./demoMode";

function formatWhen(iso: string): string {
  const at = new Date(iso);
  if (Number.isNaN(at.getTime())) {
    return iso;
  }
  return at.toLocaleString(undefined, {
    month: "short",
    day: "numeric",
    hour: "numeric",
    minute: "2-digit",
  });
}

/**
 * Demo-only restore history strip. Removable with the rest of `app/src/demo/`.
 */
export function DemoRestoreHistory({ contextId }: { contextId: string }) {
  if (!isExperienceDemoActive()) {
    return null;
  }
  const history = getDemoRestoreHistory(contextId);
  if (history.length === 0) {
    return null;
  }

  return (
    <details className="exp-inspect demo-restore-history">
      <summary>Restore history</summary>
      <ul className="list compact">
        {history.map((entry) => (
          <li key={`${entry.restored_at}-${entry.outcome}`}>
            <div>
              <strong>{entry.outcome}</strong>
              <span className="muted"> · {formatWhen(entry.restored_at)}</span>
            </div>
            <div className="muted">{entry.summary}</div>
          </li>
        ))}
      </ul>
    </details>
  );
}
