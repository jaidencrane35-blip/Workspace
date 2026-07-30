/**
 * Purpose: Shared Focus-mode supporting application chip row.
 * Owner: Frontend product shell
 * Inputs: app list with id/name, optional busy, select callback
 * Outputs: Promote/emphasise intent via onSelect
 * Dependencies: productShellUi monogram
 * Non-responsibilities: IPC, launch, OS windows, mode switching
 */

import { monogramFromName } from "../lib/productShellUi";

export interface FocusChipApp {
  id: string;
  name: string;
}

interface FocusSupportingAppChipsProps {
  apps: readonly FocusChipApp[];
  busy?: boolean;
  onSelect: (id: string) => void;
  /** Accessible/title hint for each chip action */
  selectTitle?: string;
}

export function FocusSupportingAppChips({
  apps,
  busy = false,
  onSelect,
  selectTitle = "Emphasise in Focus",
}: FocusSupportingAppChipsProps) {
  if (apps.length === 0) {
    return null;
  }

  return (
    <div className="focus-supporting">
      <p className="home-current-label">Supporting (available)</p>
      <ul className="home-app-chip-row">
        {apps.map((app) => (
          <li key={app.id}>
            <button
              type="button"
              className="home-app-chip"
              disabled={busy}
              title={selectTitle}
              onClick={() => onSelect(app.id)}
            >
              <span
                className="application-monogram compact"
                aria-hidden="true"
              >
                {monogramFromName(app.name)}
              </span>
              <span>{app.name}</span>
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}
