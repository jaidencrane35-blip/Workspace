use super::gate::{PermissionDecision, PermissionGate, PermissionRequest};
use crate::error::Result;

/// Default permission gate — allows all mutations until approval storage exists.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AllowAllPermissionGate;

impl PermissionGate for AllowAllPermissionGate {
    fn authorize(&self, _request: &PermissionRequest) -> Result<PermissionDecision> {
        Ok(PermissionDecision::Allowed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::PermissionSubject;

    #[test]
    fn allows_all_mutations() {
        let gate = AllowAllPermissionGate;
        let request = PermissionRequest {
            command: "CreateWorkspace",
            subject: PermissionSubject::Workspace,
        };

        assert_eq!(
            gate.authorize(&request).unwrap(),
            PermissionDecision::Allowed
        );
        assert!(gate.require(&request).is_ok());
    }
}
