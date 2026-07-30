/**
 * Purpose: Quiet orientation only — Stage remains the product.
 * Owner: Frontend product shell (Product Contract V3)
 * Inputs: bootstrapped flag, navigate
 * Outputs: Navigate to Stage
 * Non-goals: Dashboard cards, profile identity hero, dual CTAs, roadmap lists
 */

export type ProductPrimaryView =
  | "home"
  | "workspaces"
  | "applications"
  | "layouts";

interface WorkspaceHomeProps {
  bootstrapped: boolean;
  onNavigate: (view: ProductPrimaryView) => void;
}

export function WorkspaceHome({
  bootstrapped,
  onNavigate,
}: WorkspaceHomeProps) {
  if (!bootstrapped) {
    return (
      <section className="product-panel product-home" aria-label="Home">
        <p className="muted">Loading…</p>
      </section>
    );
  }

  return (
    <section className="product-panel product-home" aria-label="Home">
      <header className="product-panel-hero">
        <p className="arrangement-eyebrow">Home</p>
        <h2>Desktop</h2>
      </header>
      <button type="button" onClick={() => onNavigate("layouts")}>
        Open Stage
      </button>
    </section>
  );
}
