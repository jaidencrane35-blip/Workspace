//! Temporary IPC execution context — attaches LocalUser + UserRequest to frontend requests.

use workspace_domain::{ActorContext, IntentContext};

/// Returns the execution identity for IPC-originated commands.
pub fn ipc_actor_context() -> ActorContext {
    ActorContext::local_user()
}

/// Returns the execution intent for IPC-originated commands.
pub fn ipc_intent_context() -> IntentContext {
    IntentContext::user_request()
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::{ActorType, IntentType, LOCAL_USER_ACTOR_ID};

    #[test]
    fn ipc_actor_is_local_user() {
        let actor = ipc_actor_context();
        assert_eq!(actor.actor.actor_type, ActorType::LocalUser);
        assert_eq!(actor.actor.id.as_str(), LOCAL_USER_ACTOR_ID);
    }

    #[test]
    fn ipc_intent_is_user_request() {
        let intent = ipc_intent_context();
        assert_eq!(intent.intent.intent_type, IntentType::UserRequest);
    }
}
