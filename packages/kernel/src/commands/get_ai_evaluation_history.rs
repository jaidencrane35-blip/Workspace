use crate::commands::context::CommandContext;
use crate::commands::r#trait::QueryCommand;
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::AiEvaluationService;
use workspace_domain::{AiProposalEvaluation, Capability};

const DEFAULT_LIMIT: usize = 50;
const MAX_LIMIT: usize = 200;

/// Returns derived AI proposal evaluation history from operational audits.
pub struct GetAiEvaluationHistory {
    pub limit: usize,
}

impl GetAiEvaluationHistory {
    pub fn new(limit: Option<usize>) -> Self {
        Self {
            limit: limit.unwrap_or(DEFAULT_LIMIT),
        }
    }
}

impl crate::commands::Command for GetAiEvaluationHistory {
    fn name(&self) -> &'static str {
        "GetAiEvaluationHistory"
    }
}

impl QueryCommand for GetAiEvaluationHistory {
    type Output = Vec<AiProposalEvaluation>;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::audit_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Governed
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<Vec<AiProposalEvaluation>> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }

        let limit = self.limit.clamp(1, MAX_LIMIT);
        AiEvaluationService::list_recent_evaluations(&ctx.database, limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{read_is_governed, GovernanceClass};
    use workspace_domain::ActorType;

    #[test]
    fn evaluation_history_read_is_governed() {
        let command = GetAiEvaluationHistory::new(None);
        assert_eq!(command.governance_class(), GovernanceClass::Governed);
        assert!(read_is_governed(
            ActorType::LocalUser,
            command.governance_class(),
        ));
        assert_eq!(command.required_capability(), Capability::audit_read());
    }
}
