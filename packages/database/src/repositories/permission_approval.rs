use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    ActorType, CapabilityGrant, CapabilityGrantId, CapabilityGrantStatus, GrantKind, IntentType,
    PermissionApprovalRequest, PermissionApprovalRequestId, PermissionApprovalStatus,
};

/// Persistence for permission approval requests and capability grants.
pub struct PermissionApprovalRepository<'a> {
    db: &'a Database,
}

impl<'a> PermissionApprovalRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn create_request(&self, request: &PermissionApprovalRequest) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO permission_approval_requests (
                id, created_at, status, requesting_actor_type, requesting_actor_id,
                command_name, capability, subject, intent_type, reason,
                decided_at, decided_by_actor_id
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            (
                request.id.as_str(),
                &request.created_at,
                request.status.as_str(),
                actor_type_to_str(request.requesting_actor_type),
                &request.requesting_actor_id,
                &request.command_name,
                &request.capability,
                &request.subject,
                intent_type_to_str(request.intent_type),
                &request.reason,
                &request.decided_at,
                &request.decided_by_actor_id,
            ),
        )?;
        Ok(())
    }

    pub fn find_pending(
        &self,
        actor_id: &str,
        command_name: &str,
        capability: &str,
    ) -> Result<Option<PermissionApprovalRequest>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, created_at, status, requesting_actor_type, requesting_actor_id,
                    command_name, capability, subject, intent_type, reason,
                    decided_at, decided_by_actor_id
             FROM permission_approval_requests
             WHERE requesting_actor_id = ?1
               AND command_name = ?2
               AND capability = ?3
               AND status = 'pending'
             ORDER BY created_at DESC
             LIMIT 1",
        )?;
        let mut rows = stmt.query((actor_id, command_name, capability))?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_request_row(row)?));
        }
        Ok(None)
    }

    pub fn get_request(
        &self,
        id: &PermissionApprovalRequestId,
    ) -> Result<Option<PermissionApprovalRequest>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, created_at, status, requesting_actor_type, requesting_actor_id,
                    command_name, capability, subject, intent_type, reason,
                    decided_at, decided_by_actor_id
             FROM permission_approval_requests
             WHERE id = ?1",
        )?;
        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_request_row(row)?));
        }
        Ok(None)
    }

    pub fn list_recent(&self, limit: usize) -> Result<Vec<PermissionApprovalRequest>> {
        let limit = limit.clamp(1, 200) as i64;
        let mut stmt = self.db.connection().prepare(
            "SELECT id, created_at, status, requesting_actor_type, requesting_actor_id,
                    command_name, capability, subject, intent_type, reason,
                    decided_at, decided_by_actor_id
             FROM permission_approval_requests
             ORDER BY
                CASE status WHEN 'pending' THEN 0 ELSE 1 END,
                created_at DESC
             LIMIT ?1",
        )?;
        let rows = stmt.query_map([limit], map_request_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    /// Atomically transitions a pending request to a terminal status.
    /// Returns false if the row was not pending (prevents double-decide races).
    pub fn decide_if_pending(&self, request: &PermissionApprovalRequest) -> Result<bool> {
        let changed = self.db.connection().execute(
            "UPDATE permission_approval_requests
             SET status = ?1, decided_at = ?2, decided_by_actor_id = ?3
             WHERE id = ?4 AND status = 'pending'",
            (
                request.status.as_str(),
                &request.decided_at,
                &request.decided_by_actor_id,
                request.id.as_str(),
            ),
        )?;
        Ok(changed > 0)
    }

    pub fn create_grant(&self, grant: &CapabilityGrant) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO capability_grants (
                id, approval_request_id, grantee_actor_id, capability, command_name,
                grant_kind, status, created_at, consumed_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (
                grant.id.as_str(),
                grant.approval_request_id.as_str(),
                &grant.grantee_actor_id,
                &grant.capability,
                &grant.command_name,
                grant.grant_kind.as_str(),
                grant.status.as_str(),
                &grant.created_at,
                &grant.consumed_at,
            ),
        )?;
        Ok(())
    }

    pub fn list_active_grants_for_actor(&self, actor_id: &str) -> Result<Vec<CapabilityGrant>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, approval_request_id, grantee_actor_id, capability, command_name,
                    grant_kind, status, created_at, consumed_at
             FROM capability_grants
             WHERE grantee_actor_id = ?1 AND status = 'active'
             ORDER BY created_at ASC",
        )?;
        let rows = stmt.query_map([actor_id], map_grant_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn consume_grant(&self, id: &CapabilityGrantId, consumed_at: &str) -> Result<bool> {
        let changed = self.db.connection().execute(
            "UPDATE capability_grants
             SET status = 'consumed', consumed_at = ?1
             WHERE id = ?2 AND status = 'active'",
            (consumed_at, id.as_str()),
        )?;
        Ok(changed > 0)
    }
}

fn map_request_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<PermissionApprovalRequest> {
    let status_raw: String = row.get(2)?;
    let status = PermissionApprovalStatus::parse(&status_raw).map_err(|_| {
        rusqlite::Error::InvalidColumnType(2, "status".into(), rusqlite::types::Type::Text)
    })?;
    let actor_type = parse_actor_type(&row.get::<_, String>(3)?)?;
    let intent_type = parse_intent_type(&row.get::<_, String>(8)?)?;

    Ok(PermissionApprovalRequest {
        id: PermissionApprovalRequestId::new(row.get::<_, String>(0)?).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "id".into(), rusqlite::types::Type::Text)
        })?,
        created_at: row.get(1)?,
        status,
        requesting_actor_type: actor_type,
        requesting_actor_id: row.get(4)?,
        command_name: row.get(5)?,
        capability: row.get(6)?,
        subject: row.get(7)?,
        intent_type,
        reason: row.get(9)?,
        decided_at: row.get(10)?,
        decided_by_actor_id: row.get(11)?,
    })
}

fn map_grant_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<CapabilityGrant> {
    let grant_kind = GrantKind::parse(&row.get::<_, String>(5)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(5, "grant_kind".into(), rusqlite::types::Type::Text)
    })?;
    let status = CapabilityGrantStatus::parse(&row.get::<_, String>(6)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(6, "status".into(), rusqlite::types::Type::Text)
    })?;

    Ok(CapabilityGrant {
        id: CapabilityGrantId::new(row.get::<_, String>(0)?).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "id".into(), rusqlite::types::Type::Text)
        })?,
        approval_request_id: PermissionApprovalRequestId::new(row.get::<_, String>(1)?).map_err(
            |_| {
                rusqlite::Error::InvalidColumnType(
                    1,
                    "approval_request_id".into(),
                    rusqlite::types::Type::Text,
                )
            },
        )?,
        grantee_actor_id: row.get(2)?,
        capability: row.get(3)?,
        command_name: row.get(4)?,
        grant_kind,
        status,
        created_at: row.get(7)?,
        consumed_at: row.get(8)?,
    })
}

fn actor_type_to_str(actor_type: ActorType) -> &'static str {
    match actor_type {
        ActorType::LocalUser => "local_user",
        ActorType::System => "system",
        ActorType::AIAssistant => "ai_assistant",
        ActorType::Automation => "automation",
        ActorType::Plugin => "plugin",
        ActorType::RemoteSession => "remote_session",
    }
}

fn intent_type_to_str(intent_type: IntentType) -> &'static str {
    match intent_type {
        IntentType::UserRequest => "user_request",
        IntentType::AISuggestion => "ai_suggestion",
        IntentType::Automation => "automation",
        IntentType::PluginRequest => "plugin_request",
        IntentType::SystemStartup => "system_startup",
        IntentType::SystemShutdown => "system_shutdown",
        IntentType::ScheduledTask => "scheduled_task",
        IntentType::ExternalIntegration => "external_integration",
    }
}

fn parse_actor_type(value: &str) -> rusqlite::Result<ActorType> {
    match value {
        "local_user" => Ok(ActorType::LocalUser),
        "system" => Ok(ActorType::System),
        "ai_assistant" => Ok(ActorType::AIAssistant),
        "automation" => Ok(ActorType::Automation),
        "plugin" => Ok(ActorType::Plugin),
        "remote_session" => Ok(ActorType::RemoteSession),
        other => Err(rusqlite::Error::InvalidColumnType(
            0,
            format!("actor_type:{other}"),
            rusqlite::types::Type::Text,
        )),
    }
}

fn parse_intent_type(value: &str) -> rusqlite::Result<IntentType> {
    match value {
        "user_request" => Ok(IntentType::UserRequest),
        "ai_suggestion" => Ok(IntentType::AISuggestion),
        "automation" => Ok(IntentType::Automation),
        "plugin_request" => Ok(IntentType::PluginRequest),
        "system_startup" => Ok(IntentType::SystemStartup),
        "system_shutdown" => Ok(IntentType::SystemShutdown),
        "scheduled_task" => Ok(IntentType::ScheduledTask),
        "external_integration" => Ok(IntentType::ExternalIntegration),
        other => Err(rusqlite::Error::InvalidColumnType(
            0,
            format!("intent_type:{other}"),
            rusqlite::types::Type::Text,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init::DatabaseService;
    use tempfile::tempdir;

    fn test_db() -> (tempfile::TempDir, Database) {
        let dir = tempdir().unwrap();
        let db = DatabaseService::initialize(dir.path().join("workspace.db"))
            .unwrap()
            .into_database();
        (dir, db)
    }

    #[test]
    fn creates_pending_request_and_lists_it() {
        let (_dir, db) = test_db();
        let repo = PermissionApprovalRepository::new(&db);
        let request = PermissionApprovalRequest {
            id: PermissionApprovalRequestId::generate(),
            created_at: "2026-07-24T00:00:00Z".into(),
            status: PermissionApprovalStatus::Pending,
            requesting_actor_type: ActorType::AIAssistant,
            requesting_actor_id: "ai-1".into(),
            command_name: "LaunchApplication".into(),
            capability: "application.launch".into(),
            subject: "Resource(Application)".into(),
            intent_type: IntentType::UserRequest,
            reason: "needs approval".into(),
            decided_at: None,
            decided_by_actor_id: None,
        };
        repo.create_request(&request).unwrap();
        let listed = repo.list_recent(10).unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, request.id);
    }
}
