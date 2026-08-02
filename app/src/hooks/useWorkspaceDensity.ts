import { useEffect, useState } from "react";
import { densityFromViewport, type WorkspaceDensity } from "../lib/density";

export function useWorkspaceDensity(
  override: WorkspaceDensity | null = null,
): WorkspaceDensity {
  const [auto, setAuto] = useState<WorkspaceDensity>(() =>
    typeof window === "undefined"
      ? "balanced"
      : densityFromViewport(window.innerWidth),
  );

  useEffect(() => {
    const onResize = () => {
      setAuto(densityFromViewport(window.innerWidth));
    };
    onResize();
    window.addEventListener("resize", onResize);
    return () => window.removeEventListener("resize", onResize);
  }, []);

  return override ?? auto;
}
