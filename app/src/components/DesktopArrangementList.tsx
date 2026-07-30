import type { DesktopArrangement } from "../types/desktopArrangement";
import type { WorkspaceStateWindow } from "../types/domain";
import { arrangementListMetaLine } from "../lib/desktopArrangementUi";
import { deriveArrangementProductMeta } from "../lib/arrangementProductUi";

interface DesktopArrangementListProps {
  arrangements: DesktopArrangement[];
  selectedId: string | null;
  busy: boolean;
  onSelect: (id: string) => void;
  /** Live desktop windows for derived overlap/readiness (WorkspaceState). */
  observedWindows?: readonly WorkspaceStateWindow[];
}

export function DesktopArrangementList({
  arrangements,
  selectedId,
  busy,
  onSelect,
  observedWindows = [],
}: DesktopArrangementListProps) {
  if (arrangements.length === 0) {
    return null;
  }

  return (
    <ul className="arrangement-list" aria-label="Saved Arrangements">
      {arrangements.map((arrangement) => {
        const selected = arrangement.id === selectedId;
        const meta = deriveArrangementProductMeta(
          arrangement,
          observedWindows,
        );
        return (
          <li key={arrangement.id}>
            <button
              type="button"
              className={
                selected
                  ? "arrangement-list-item selected"
                  : "arrangement-list-item"
              }
              disabled={busy}
              aria-pressed={selected}
              title={meta.readinessLine}
              onClick={() => onSelect(arrangement.id)}
            >
              <span className="arrangement-list-name">{arrangement.name}</span>
              <span className="arrangement-list-meta muted">
                {arrangementListMetaLine(arrangement, meta.summaryLine)}
              </span>
            </button>
          </li>
        );
      })}
    </ul>
  );
}
