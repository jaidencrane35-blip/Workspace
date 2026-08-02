//! Consented local pilot measurement commands (PP-P01E).

use crate::commands::context::CommandContext;
use crate::commands::r#trait::{MutationCommand, QueryCommand};
use crate::error::{KernelError, Result};
use crate::lifecycle::LifecycleState;
use crate::policy::GovernanceClass;
use crate::security::PermissionSubject;
use crate::services::PilotMeasurementService;
use workspace_domain::{
    Capability, GrantPilotConsentRequest, PilotBaseline, PilotConsent, PilotInterviewRecord,
    PilotLeaveResumeRecord, PilotMeasurementScope, PilotMeasurementSnapshot,
    RecordPilotBaselineRequest, RecordPilotInterviewRequest, RecordPilotLeaveResumeRequest,
};

/// Describes what pilot measurement includes. Observes nothing.
pub struct GetPilotMeasurementScope;

impl crate::commands::Command for GetPilotMeasurementScope {
    fn name(&self) -> &'static str {
        "GetPilotMeasurementScope"
    }
}

impl QueryCommand for GetPilotMeasurementScope {
    type Output = PilotMeasurementScope;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::settings_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Ungoverned
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<PilotMeasurementScope> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        Ok(PilotMeasurementService::scope())
    }
}

/// Loads the local pilot measurement snapshot (evaluation data only).
pub struct GetPilotMeasurementSnapshot;

impl crate::commands::Command for GetPilotMeasurementSnapshot {
    fn name(&self) -> &'static str {
        "GetPilotMeasurementSnapshot"
    }
}

impl QueryCommand for GetPilotMeasurementSnapshot {
    type Output = PilotMeasurementSnapshot;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::settings_read()
    }

    fn governance_class(&self) -> GovernanceClass {
        GovernanceClass::Ungoverned
    }

    fn execute(self, ctx: &CommandContext<'_>) -> Result<PilotMeasurementSnapshot> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        PilotMeasurementService::snapshot(&ctx.database)
    }
}

pub struct GrantPilotConsent {
    pub request: GrantPilotConsentRequest,
}

impl crate::commands::Command for GrantPilotConsent {
    fn name(&self) -> &'static str {
        "GrantPilotConsent"
    }
}

impl MutationCommand for GrantPilotConsent {
    type Output = PilotConsent;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::settings_write()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            serde_json::json!({
                "scope_id": output.scope_id,
                "consented": true,
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<PilotConsent> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        PilotMeasurementService::grant_consent(&ctx.database, &self.request)
    }
}

pub struct WithdrawPilotConsent {
    pub clear_records: bool,
}

impl crate::commands::Command for WithdrawPilotConsent {
    fn name(&self) -> &'static str {
        "WithdrawPilotConsent"
    }
}

impl MutationCommand for WithdrawPilotConsent {
    type Output = PilotConsent;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::settings_write()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            serde_json::json!({
                "scope_id": output.scope_id,
                "withdrawn": true,
                "cleared_records": self.clear_records,
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<PilotConsent> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        PilotMeasurementService::withdraw_consent(&ctx.database, self.clear_records)
    }
}

pub struct RecordPilotBaseline {
    pub request: RecordPilotBaselineRequest,
}

impl crate::commands::Command for RecordPilotBaseline {
    fn name(&self) -> &'static str {
        "RecordPilotBaseline"
    }
}

impl MutationCommand for RecordPilotBaseline {
    type Output = PilotBaseline;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::settings_write()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            serde_json::json!({
                "return_minutes": output.return_minutes,
                "kind": "baseline",
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<PilotBaseline> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        PilotMeasurementService::record_baseline(&ctx.database, &self.request)
    }
}

pub struct RecordPilotLeaveResume {
    pub request: RecordPilotLeaveResumeRequest,
}

impl crate::commands::Command for RecordPilotLeaveResume {
    fn name(&self) -> &'static str {
        "RecordPilotLeaveResume"
    }
}

impl MutationCommand for RecordPilotLeaveResume {
    type Output = PilotLeaveResumeRecord;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::settings_write()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            serde_json::json!({
                "kind": "leave_resume",
                "local_day": output.local_day,
                "correction_needed": output.correction_needed,
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<PilotLeaveResumeRecord> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        PilotMeasurementService::record_leave_resume(&ctx.database, &self.request)
    }
}

pub struct RecordPilotInterview {
    pub request: RecordPilotInterviewRequest,
}

impl crate::commands::Command for RecordPilotInterview {
    fn name(&self) -> &'static str {
        "RecordPilotInterview"
    }
}

impl MutationCommand for RecordPilotInterview {
    type Output = PilotInterviewRecord;

    fn permission_subject(&self) -> PermissionSubject {
        PermissionSubject::System
    }

    fn required_capability(&self) -> Capability {
        Capability::settings_write()
    }

    fn audit_metadata(&self, output: &Self::Output) -> Option<String> {
        Some(
            serde_json::json!({
                "kind": "interview",
                "phase": output.phase.as_str(),
            })
            .to_string(),
        )
    }

    fn execute(&self, ctx: &CommandContext<'_>) -> Result<PilotInterviewRecord> {
        if ctx.state.lifecycle != LifecycleState::Ready {
            return Err(KernelError::NotReady);
        }
        PilotMeasurementService::record_interview(&ctx.database, &self.request)
    }
}
