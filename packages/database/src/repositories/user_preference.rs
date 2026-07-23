use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    PreferenceCategory, PreferenceSource, UserPreference, UserPreferenceId, WorkspaceId,
};

/// Persistence for explicit user preferences (informational only).
pub struct UserPreferenceRepository<'a> {
    db: &'a Database,
}

impl<'a> UserPreferenceRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert(&self, preference: &UserPreference) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO user_preferences (
                id, category, key, value, label, source, confidence,
                scope_workspace_id, editable, attributes, created_at, updated_at, deleted
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
             ON CONFLICT(id) DO UPDATE SET
                category = excluded.category,
                key = excluded.key,
                value = excluded.value,
                label = excluded.label,
                source = excluded.source,
                confidence = excluded.confidence,
                scope_workspace_id = excluded.scope_workspace_id,
                editable = excluded.editable,
                attributes = excluded.attributes,
                updated_at = excluded.updated_at,
                deleted = excluded.deleted",
            (
                preference.id.as_str(),
                preference.category.as_str(),
                &preference.key,
                &preference.value,
                &preference.label,
                preference.source.as_str(),
                i64::from(preference.confidence),
                preference
                    .scope_workspace_id
                    .as_ref()
                    .map(|id| id.as_str().to_string()),
                if preference.editable { 1_i64 } else { 0 },
                &preference.attributes,
                &preference.created_at,
                &preference.updated_at,
                if preference.deleted { 1_i64 } else { 0 },
            ),
        )?;
        Ok(())
    }

    pub fn get(&self, id: &UserPreferenceId) -> Result<Option<UserPreference>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, category, key, value, label, source, confidence,
                    scope_workspace_id, editable, attributes, created_at, updated_at, deleted
             FROM user_preferences
             WHERE id = ?1",
        )?;
        let mut rows = stmt.query([id.as_str()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(map_row(row)?));
        }
        Ok(None)
    }

    pub fn list_active(
        &self,
        workspace_id: Option<&str>,
        limit: usize,
    ) -> Result<Vec<UserPreference>> {
        let limit = limit.clamp(1, 200) as i64;
        if let Some(workspace_id) = workspace_id {
            let mut stmt = self.db.connection().prepare(
                "SELECT id, category, key, value, label, source, confidence,
                        scope_workspace_id, editable, attributes, created_at, updated_at, deleted
                 FROM user_preferences
                 WHERE deleted = 0
                   AND (scope_workspace_id IS NULL OR scope_workspace_id = ?1)
                 ORDER BY updated_at DESC
                 LIMIT ?2",
            )?;
            let rows = stmt.query_map((workspace_id, limit), map_row)?;
            return rows
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(Into::into);
        }

        let mut stmt = self.db.connection().prepare(
            "SELECT id, category, key, value, label, source, confidence,
                    scope_workspace_id, editable, attributes, created_at, updated_at, deleted
             FROM user_preferences
             WHERE deleted = 0
             ORDER BY updated_at DESC
             LIMIT ?1",
        )?;
        let rows = stmt.query_map([limit], map_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn soft_delete(&self, id: &UserPreferenceId, updated_at: &str) -> Result<bool> {
        let changed = self.db.connection().execute(
            "UPDATE user_preferences
             SET deleted = 1, updated_at = ?2
             WHERE id = ?1 AND deleted = 0",
            (id.as_str(), updated_at),
        )?;
        Ok(changed > 0)
    }
}

fn map_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<UserPreference> {
    let category = PreferenceCategory::parse(row.get::<_, String>(1)?.as_str())
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    let source = PreferenceSource::parse(row.get::<_, String>(5)?.as_str())
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;
    let scope_workspace_id = row
        .get::<_, Option<String>>(7)?
        .map(WorkspaceId::new)
        .transpose()
        .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;

    Ok(UserPreference {
        id: UserPreferenceId::new(row.get::<_, String>(0)?)
            .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?,
        category,
        key: row.get(2)?,
        value: row.get(3)?,
        label: row.get(4)?,
        source,
        confidence: row.get::<_, i64>(6)? as u8,
        scope_workspace_id,
        editable: row.get::<_, i64>(8)? != 0,
        attributes: row.get(9)?,
        created_at: row.get(10)?,
        updated_at: row.get(11)?,
        deleted: row.get::<_, i64>(12)? != 0,
    })
}
