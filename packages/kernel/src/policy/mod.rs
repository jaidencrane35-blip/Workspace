//! Policy evaluation contracts for permission decisions.

mod always_allow;
mod context;
mod evaluator;
mod governance;
mod policy;

pub use always_allow::AlwaysAllowPolicy;
pub use context::{PolicyContext, PolicyDecision, PolicyResult};
pub use evaluator::{DefaultPolicyEvaluator, PolicyEvaluator};
pub use governance::{read_is_governed, GovernanceClass};
pub use policy::PermissionPolicy;
