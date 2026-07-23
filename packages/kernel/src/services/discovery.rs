//! Derived capability discovery — evaluates policy without granting authority (Sprint 16).

use chrono::Utc;
use workspace_domain::{
    ActionIntentCategory, ActionIntentRegistry, ActorContext, AvailableIntentSummary,
    Capability, CapabilityDiscovery, CapabilitySet, IntentContext,
};

use crate::error::{KernelError, Result};
use crate::policy::{DefaultPolicyEvaluator, PolicyContext, PolicyEvaluator};
use crate::security::{PermissionDecision, PermissionGate, PermissionRequest, PermissionSubject};

/// Derives actor capabilities and available intents from existing governance rules.
pub struct CapabilityResolver;

impl CapabilityResolver {
    pub fn discover(
        actor_context: &ActorContext,
        intent_context: &IntentContext,
        nominal_capabilities: &CapabilitySet,
        policy: &dyn crate::policy::PermissionPolicy,
        gate: &dyn PermissionGate,
    ) -> Result<CapabilityDiscovery> {
        let mut granted_capabilities = Vec::new();
        let mut available_intents = Vec::new();

        for definition in ActionIntentRegistry::all() {
            let nominally_granted =
                nominal_capabilities.contains(&definition.capability_required);
            let authorized = nominally_granted
                && Self::is_authorized(
                    actor_context,
                    intent_context,
                    definition.command_name,
                    &definition.capability_required,
                    subject_for_category(definition.category),
                    policy,
                    gate,
                )?;

            available_intents.push(AvailableIntentSummary {
                intent_id: definition.id.clone(),
                name: definition.name.clone(),
                description: definition.description.clone(),
                category: definition.category,
                command_name: definition.command_name.to_string(),
                capability_required: definition.capability_required.clone(),
                available: authorized,
            });

            if authorized
                && !granted_capabilities.iter().any(|capability: &Capability| {
                    capability.id == definition.capability_required.id
                })
            {
                granted_capabilities.push(definition.capability_required.clone());
            }
        }

        granted_capabilities.sort_by(|left, right| left.id.cmp(&right.id));

        let discovery = CapabilityDiscovery {
            actor_id: actor_context.actor.id.clone(),
            actor_type: actor_context.actor.actor_type,
            capabilities: granted_capabilities,
            available_intents,
            generated_at: Utc::now().to_rfc3339(),
        };

        discovery
            .validate()
            .map_err(|error| KernelError::CapabilityDiscoveryValidation {
                message: error.to_string(),
            })?;

        Ok(discovery)
    }

    fn is_authorized(
        actor_context: &ActorContext,
        intent_context: &IntentContext,
        command_name: &'static str,
        capability: &Capability,
        subject: PermissionSubject,
        policy: &dyn crate::policy::PermissionPolicy,
        gate: &dyn PermissionGate,
    ) -> Result<bool> {
        let policy_context = PolicyContext::new(
            actor_context.actor.id.to_string(),
            intent_context.intent.clone(),
            capability.clone(),
            command_name,
            subject.clone(),
        );

        let policy_result = DefaultPolicyEvaluator.evaluate(policy, &policy_context)?;
        if !policy_result.is_allowed() {
            return Ok(false);
        }

        let request = PermissionRequest {
            actor: actor_context.actor.clone(),
            intent: intent_context.intent.clone(),
            capability: capability.clone(),
            command: command_name,
            subject,
        };

        Ok(matches!(gate.authorize(&request)?, PermissionDecision::Allowed))
    }
}

fn subject_for_category(category: ActionIntentCategory) -> PermissionSubject {
    use workspace_domain::ResourceKind;

    match category {
        ActionIntentCategory::Workspace => {
            PermissionSubject::Resource(ResourceKind::Workspace)
        }
        ActionIntentCategory::Zone => PermissionSubject::Resource(ResourceKind::Zone),
        ActionIntentCategory::Application => {
            PermissionSubject::Resource(ResourceKind::Application)
        }
        ActionIntentCategory::Widget => PermissionSubject::Resource(ResourceKind::Widget),
        ActionIntentCategory::Layout => PermissionSubject::Resource(ResourceKind::Workspace),
        ActionIntentCategory::Settings
        | ActionIntentCategory::Audit
        | ActionIntentCategory::System => PermissionSubject::System,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::AlwaysAllowPolicy;
    use crate::security::{AllowAllPermissionGate, PermissionDecision, PermissionGate};
    use workspace_domain::{ActionIntentId, ActorType, IntentContext};

    struct DenyAllGate;

    impl PermissionGate for DenyAllGate {
        fn authorize(
            &self,
            _request: &PermissionRequest,
        ) -> Result<PermissionDecision> {
            Ok(PermissionDecision::Denied {
                reason: "denied".into(),
            })
        }
    }

    #[test]
    fn local_user_discovers_standard_capabilities() {
        let discovery = CapabilityResolver::discover(
            &ActorContext::local_user(),
            &IntentContext::user_request(),
            &CapabilitySet::local_user_standard(),
            &AlwaysAllowPolicy,
            &AllowAllPermissionGate,
        )
        .unwrap();

        assert_eq!(discovery.actor_type, ActorType::LocalUser);
        assert!(!discovery.capabilities.is_empty());
        assert!(discovery.available_intent_ids().len() > 10);
        assert!(discovery.validate().is_ok());
    }

    #[test]
    fn system_actor_has_limited_capabilities() {
        let discovery = CapabilityResolver::discover(
            &ActorContext::system(),
            &IntentContext::system_startup(),
            &CapabilitySet::system_standard(),
            &AlwaysAllowPolicy,
            &AllowAllPermissionGate,
        )
        .unwrap();

        assert_eq!(discovery.actor_type, ActorType::System);
        assert!(discovery.available_intent_ids().is_empty());
    }

    #[test]
    fn denied_gate_marks_intents_unavailable() {
        let discovery = CapabilityResolver::discover(
            &ActorContext::local_user(),
            &IntentContext::user_request(),
            &CapabilitySet::local_user_standard(),
            &AlwaysAllowPolicy,
            &DenyAllGate,
        )
        .unwrap();

        assert!(discovery.capabilities.is_empty());
        assert!(discovery
            .available_intents
            .iter()
            .all(|intent| !intent.available));
    }

    #[test]
    fn unavailable_intents_remain_listed() {
        let discovery = CapabilityResolver::discover(
            &ActorContext::local_user(),
            &IntentContext::user_request(),
            &CapabilitySet::new(),
            &AlwaysAllowPolicy,
            &AllowAllPermissionGate,
        )
        .unwrap();

        assert!(discovery.available_intent_ids().is_empty());
        assert_eq!(
            discovery.available_intents.len(),
            ActionIntentRegistry::all().len()
        );
        assert!(discovery.available_intents.iter().any(|intent| {
            intent.intent_id == ActionIntentId::new("create-workspace").unwrap()
        }));
    }
}
