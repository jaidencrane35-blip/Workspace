import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { DesktopOperator } from "./components/operator/DesktopOperator";
import {
  disposeExperienceInstrumentation,
  initExperienceInstrumentation,
} from "./dev";
import { isTauriRuntime, loadShellMode } from "./lib/shellRuntime";
import {
  bootstrapShellOnLaunch,
  currentWindowLabel,
} from "./lib/shellWindows";
import "./design-system/tokens.css";
import "./App.css";

initExperienceInstrumentation();

async function resolveShellEntry(): Promise<React.ReactElement> {
  if (isTauriRuntime()) {
    await bootstrapShellOnLaunch();
    const label = await currentWindowLabel();
    if (label === "operator") {
      return <DesktopOperator />;
    }
    // Main window: if durable mode is floating, keep App mounted (hidden) for recovery.
    const mode = loadShellMode(0);
    if (mode === 0) {
      // Still mount App so close/sync handlers live; window stays hidden.
      return <App />;
    }
  }
  return <App />;
}

void resolveShellEntry().then((element) => {
  ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>{element}</React.StrictMode>,
  );
});

// Evidence dashboard is opt-in only — never auto-mount into Product Owner review.
if (import.meta.env.DEV && localStorage.getItem("workspace.operator.developer") === "1") {
  void import("./dev/mountExperienceEvidence").then((mod) => {
    mod.mountExperienceEvidenceDashboard();
  });
}

if (import.meta.hot) {
  import.meta.hot.dispose(() => {
    disposeExperienceInstrumentation();
    void import("./dev/mountExperienceEvidence").then((mod) => {
      mod.unmountExperienceEvidenceDashboard();
    });
  });
}
