use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    WorkspaceId, WorkspaceProfile, WorkspaceProfileId, WorkspaceProfileMember,
    WorkspaceProfileMemberType, WorkspaceProfileRelationship, WorkspaceProfileStatus,
};

/// Persistence for durable Workspace Environment Profiles (user-owned; never authority).
pub struct WorkspaceProfileRepository<'a> {
    db: &'a Database,
}

impl<'a> WorkspaceProfileRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert_profile(&self, profile: &WorkspaceProfile) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO workspace_profiles (
                id, workspace_id, name, description, status, created_at, updated_at, deleted
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0)
             ON CONFLICT(id) DO UPDATE SET
                workspace_id = excluded.workspace_id,
                name = excluded.name,
                description = excluded.description,
                status = excluded.status,
                updated_at = excluded.updated_at,
                deleted = 0",
            (
                profile.id.as_str(),
                profile.workspace_id.as_str(),
                &profile.name,
                &profile.description,
                profile.status.as_str(),
                &profile.created_at,
                &profile.updated_at,
            ),
        )?;
        Ok(())
    }

    pub fn replace_members(
        &self,
        profile_id: &WorkspaceProfileId,
        members: &[WorkspaceProfileMember],
        created_at: &str,
    ) -> Result<()> {
        self.db.connection().execute(
            "DELETE FROM workspace_profile_members WHERE profile_id = ?1",
            [profile_id.as_str()],
        )?;
        for member in members {
            self.db.connection().execute(
                "INSERT INTO workspace_profile_members (
                    id, profile_id, member_type, reference_id, relationship, evidence, label, created_at
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                (
                    &member.id,
                    profile_id.as_str(),
                    member.member_type.as_str(),
                    &member.reference_id,
                    member.relationship.as_str(),
                    &member.evidence,
                    &member.label,
                    created_at,
                ),
            )?;
        }
        Ok(())
    }

    pub fn get(&self, id: &WorkspaceProfileId) -> Result<Option<WorkspaceProfile>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, name, description, status, created_at, updated_at
             FROM workspace_profiles
             WHERE id = ?1 AND deleted = 0",
        )?;
        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            let mut profile = map_profile_row(row)?;
            profile.members = self.list_members(id)?;
            return Ok(Some(profile));
        }
        Ok(None)
    }

    pub fn list_by_workspace(
        &self,
        workspace_id: &WorkspaceId,
        limit: usize,
    ) -> Result<Vec<WorkspaceProfile>> {
        let limit = limit.clamp(1, 200) as i64;
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, name, description, status, created_at, updated_at
             FROM workspace_profiles
             WHERE workspace_id = ?1 AND deleted = 0
             ORDER BY name ASC, updated_at DESC
             LIMIT ?2",
        )?;
        let rows = stmt.query_map((workspace_id.as_str(), limit), map_profile_row)?;
        let mut profiles = rows
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(crate::error::DatabaseError::from)?;
        for profile in &mut profiles {
            profile.members = self.list_members(&profile.id)?;
        }
        Ok(profiles)
    }

    pub fn soft_delete(&self, id: &WorkspaceProfileId, updated_at: &str) -> Result<bool> {
        let changed = self.db.connection().execute(
            "UPDATE workspace_profiles
             SET deleted = 1, updated_at = ?2
             WHERE id = ?1 AND deleted = 0",
            (id.as_str(), updated_at),
        )?;
        if changed > 0 {
            self.db.connection().execute(
                "DELETE FROM workspace_profile_members WHERE profile_id = ?1",
                [id.as_str()],
            )?;
        }
        Ok(changed > 0)
    }

    fn list_members(&self, profile_id: &WorkspaceProfileId) -> Result<Vec<WorkspaceProfileMember>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, profile_id, member_type, reference_id, relationship, evidence, label
             FROM workspace_profile_members
             WHERE profile_id = ?1
             ORDER BY member_type ASC, reference_id ASC",
        )?;
        let rows = stmt.query_map([profile_id.as_str()], map_member_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

fn map_profile_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<WorkspaceProfile> {
    let status = WorkspaceProfileStatus::parse(row.get::<_, String>(4)?.as_str()).map_err(
        |error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)),
    )?;
    Ok(WorkspaceProfile {
        id: WorkspaceProfileId::new(row.get::<_, String>(0)?)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        workspace_id: WorkspaceId::new(row.get::<_, String>(1)?)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
        name: row.get(2)?,
        description: row.get(3)?,
        status,
        members: Vec::new(),
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
        authority_effect: WorkspaceProfile::AUTHORITY_EFFECT_NONE.into(),
    })
}

fn map_member_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<WorkspaceProfileMember> {
    let member_type = WorkspaceProfileMemberType::parse(row.get::<_, String>(2)?.as_str())
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    let relationship = WorkspaceProfileRelationship::parse(row.get::<_, String>(4)?.as_str())
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    Ok(WorkspaceProfileMember {
        id: row.get(0)?,
        profile_id: row.get(1)?,
        member_type,
        reference_id: row.get(3)?,
        relationship,
        evidence: row.get(5)?,
        label: row.get(6)?,
        authority_effect: WorkspaceProfileMember::AUTHORITY_EFFECT_NONE.into(),
    })
}
