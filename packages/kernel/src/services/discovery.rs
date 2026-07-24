//! Derived capability discovery — evaluates policy without granting authority (Sprint 16).

use chrono::Utc;
use workspace_domain::{
    ActionIntentCategory, ActionIntentRegistry, ActorContext, AvailableIntentSummary,
    Capability, CapabilityDiscovery, CapabilitySet, IntentContext,
};

use crate::error::{KernelError, Result};
use crate::security::{
    GatewayDecision, PermissionGate, PermissionGateway, PermissionRequest, PermissionSubject,
};

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
                    nominal_capabilities,
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
        granted: &CapabilitySet,
        policy: &dyn crate::policy::PermissionPolicy,
        gate: &dyn PermissionGate,
    ) -> Result<bool> {
        let request = PermissionRequest {
            actor: actor_context.actor.clone(),
            intent: intent_context.intent.clone(),
            capability: capability.clone(),
            command: command_name,
            subject,
            target_resource_id: None,
        };

        // Discovery probes evaluate without writing permission audit records.
        let decision =
            PermissionGateway::evaluate(policy, gate, &request, granted)?;

        Ok(matches!(decision, GatewayDecision::Allow { .. }))
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
        | ActionIntentCategory::Suggestion
        | ActionIntentCategory::Permission
        | ActionIntentCategory::System => PermissionSubject::System,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{AlwaysAllowPolicy, CapabilityBoundPolicy};
    use crate::security::{
        AllowAllPermissionGate, PermissionDecision, StandardPermissionGate,
    };
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
            &CapabilityBoundPolicy,
            &StandardPermissionGate,
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
            &CapabilityBoundPolicy,
            &StandardPermissionGate,
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
            &CapabilityBoundPolicy,
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
