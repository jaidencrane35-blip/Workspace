//! Temporary IPC actor provider — attaches LocalUser to frontend-originated requests.

use workspace_domain::ActorContext;

/// Returns the execution identity for IPC-originated commands.
pub fn ipc_actor_context() -> ActorContext {
    ActorContext::local_user()
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_domain::{ActorType, LOCAL_USER_ACTOR_ID};

    #[test]
    fn ipc_actor_is_local_user() {
        let actor = ipc_actor_context();
        assert_eq!(actor.actor.actor_type, ActorType::LocalUser);
        assert_eq!(actor.actor.id.as_str(), LOCAL_USER_ACTOR_ID);
    }
}
