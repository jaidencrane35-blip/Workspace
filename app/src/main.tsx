import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import {
  disposeExperienceInstrumentation,
  initExperienceInstrumentation,
} from "./dev";
import "./design-system/tokens.css";
import "./App.css";

initExperienceInstrumentation();

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);

// Evidence dashboard is DEV-only; Vite drops this branch from production bundles.
if (import.meta.env.DEV) {
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
