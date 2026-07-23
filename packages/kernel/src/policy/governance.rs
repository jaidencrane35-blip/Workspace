use workspace_domain::ActorType;

/// Whether a read requires governance (DEC-017).
///
/// A read is governed when it is either declared `Governed` (a sensitive
/// resource kind, e.g. the audit log) or performed by a non-human actor.
/// Local-human reads of non-sensitive resources are `Ungoverned`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernanceClass {
    Ungoverned,
    Governed,
}

/// Read governance decision (DEC-017): actor-driven with a sensitivity override.
pub fn read_is_governed(actor_type: ActorType, class: GovernanceClass) -> bool {
    matches!(class, GovernanceClass::Governed) || actor_type != ActorType::LocalUser
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_non_sensitive_reads_are_ungoverned() {
        assert!(!read_is_governed(
            ActorType::LocalUser,
            GovernanceClass::Ungoverned
        ));
    }

    #[test]
    fn human_sensitive_reads_are_governed() {
        assert!(read_is_governed(
            ActorType::LocalUser,
            GovernanceClass::Governed
        ));
    }

    #[test]
    fn non_human_reads_are_always_governed() {
        assert!(read_is_governed(
            ActorType::AIAssistant,
            GovernanceClass::Ungoverned
        ));
        assert!(read_is_governed(
            ActorType::Plugin,
            GovernanceClass::Ungoverned
        ));
        assert!(read_is_governed(
            ActorType::Automation,
            GovernanceClass::Ungoverned
        ));
    }
}
