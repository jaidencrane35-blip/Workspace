/**
 * DEV-only mount entry for the Experience Evidence dashboard.
 * Loaded via dynamic import gated by `import.meta.env.DEV` so production
 * bundles do not include this chunk.
 */

import { createRoot, type Root } from "react-dom/client";
import { ExperienceEvidenceDashboard } from "./ExperienceEvidenceDashboard";
import { isExperienceValidationEnabled } from "./experienceValidationGate";

const HOST_ID = "ws-dev-experience-evidence-host";

let root: Root | null = null;

export function mountExperienceEvidenceDashboard(): void {
  if (!import.meta.env.DEV || !isExperienceValidationEnabled()) {
    return;
  }
  if (typeof document === "undefined") {
    return;
  }
  let host = document.getElementById(HOST_ID);
  if (!host) {
    host = document.createElement("div");
    host.id = HOST_ID;
    document.body.appendChild(host);
  }
  if (!root) {
    root = createRoot(host);
  }
  root.render(<ExperienceEvidenceDashboard />);
}

export function unmountExperienceEvidenceDashboard(): void {
  root?.unmount();
  root = null;
  document.getElementById(HOST_ID)?.remove();
}
