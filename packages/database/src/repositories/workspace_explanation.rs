use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    EvidenceReference, ExplanationCompleteness, ExplanationConfidence, ExplanationConflict,
    ExplanationGap, ExplanationPackage, ExplanationScope, ExplanationSection, ExplanationStatus,
    WorkspaceExplanationHistoryEntry,
};

/// Persistence guards for Programme III workspace explanation evidence.
/// Stores explanation packages / history — never executes, approves, or mutates reality.
pub struct WorkspaceExplanationRepository<'a> {
    db: &'a Database,
}

impl<'a> WorkspaceExplanationRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, package: &ExplanationPackage) -> Result<()> {
        package.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!(
                "explanation package invalid: {e}"
            ))
        })?;
        let scope_json = serde_json::to_string(&package.scope).map_err(ser_err)?;
        let sections_json = serde_json::to_string(&package.sections).map_err(ser_err)?;
        let gaps_json = serde_json::to_string(&package.gaps).map_err(ser_err)?;
        let conflicts_json = serde_json::to_string(&package.conflicts).map_err(ser_err)?;
        let confidence_json = serde_json::to_string(&package.confidence).map_err(ser_err)?;
        let provenance_links_json =
            serde_json::to_string(&package.provenance_links).map_err(ser_err)?;
        let limitations_json = serde_json::to_string(&package.limitations).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_explanation_snapshots (
                id, workspace_id, status, created_at, superseded_at,
                completeness, section_count, gap_count, conflict_count, narrative,
                scope_json, sections_json, gaps_json, conflicts_json,
                confidence_json, provenance_links_json, limitations_json,
                authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,'none',0,0)",
            rusqlite::params![
                &package.explanation_id,
                &package.workspace_id,
                package.status.as_str(),
                &package.generated_at,
                &package.superseded_at,
                package.completeness.as_str(),
                package.sections.len() as i64,
                package.gaps.len() as i64,
                package.conflicts.len() as i64,
                &package.narrative,
                scope_json,
                sections_json,
                gaps_json,
                conflicts_json,
                confidence_json,
                provenance_links_json,
                limitations_json,
            ],
        )?;
        Ok(())
    }

    pub fn supersede_current(
        &self,
        workspace_id: &str,
        superseded_at: &str,
    ) -> Result<Vec<ExplanationPackage>> {
        let currents = self.list_by_status(workspace_id, ExplanationStatus::Current)?;
        let mut out = Vec::new();
        for mut package in currents {
            package.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_explanation_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND workspace_id = ?4",
                (
                    &package.explanation_id,
                    package.status.as_str(),
                    &package.superseded_at,
                    workspace_id,
                ),
            )?;
            if let Some(entry) = WorkspaceExplanationHistoryEntry::from_package(&package) {
                self.append_history(workspace_id, &entry, superseded_at)?;
            }
            out.push(package);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        workspace_id: &str,
        entry: &WorkspaceExplanationHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "workspace explanation history must be non-actionable terminal evidence".into(),
            ));
        }
        let id = format!("workspace_explanation_hist:{}", uuid::Uuid::new_v4());
        self.db.connection().execute(
            "INSERT INTO workspace_explanation_history (
                id, workspace_id, explanation_id, status, created_at, superseded_at,
                completeness, section_count, gap_count, conflict_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,1,0,'none',?11)",
            rusqlite::params![
                id,
                workspace_id,
                &entry.explanation_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                &entry.completeness,
                entry.section_count as i64,
                entry.gap_count as i64,
                entry.conflict_count as i64,
                recorded_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_by_status(
        &self,
        workspace_id: &str,
        status: ExplanationStatus,
    ) -> Result<Vec<ExplanationPackage>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, narrative,
                    scope_json, sections_json, gaps_json, conflicts_json,
                    confidence_json, provenance_links_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_explanation_snapshots
             WHERE workspace_id = ?1 AND status = ?2
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(rusqlite::params![workspace_id, status.as_str()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, String>(8)?,
                row.get::<_, String>(9)?,
                row.get::<_, String>(10)?,
                row.get::<_, String>(11)?,
                row.get::<_, String>(12)?,
                row.get::<_, String>(13)?,
                row.get::<_, String>(14)?,
                row.get::<_, i64>(15)?,
                row.get::<_, i64>(16)?,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(map_package(row?)?);
        }
        Ok(out)
    }

    pub fn list_views(&self, workspace_id: &str) -> Result<Vec<ExplanationPackage>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, status, created_at, superseded_at,
                    completeness, narrative,
                    scope_json, sections_json, gaps_json, conflicts_json,
                    confidence_json, provenance_links_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_explanation_snapshots
             WHERE workspace_id = ?1
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
                row.get::<_, String>(8)?,
                row.get::<_, String>(9)?,
                row.get::<_, String>(10)?,
                row.get::<_, String>(11)?,
                row.get::<_, String>(12)?,
                row.get::<_, String>(13)?,
                row.get::<_, String>(14)?,
                row.get::<_, i64>(15)?,
                row.get::<_, i64>(16)?,
            ))
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(map_package(row?)?);
        }
        Ok(out)
    }

    pub fn list_history(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<WorkspaceExplanationHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT explanation_id, status, created_at, superseded_at,
                    completeness, section_count, gap_count, conflict_count,
                    terminal, actionable, authority_effect
             FROM workspace_explanation_history
             WHERE workspace_id = ?1
             ORDER BY recorded_at DESC",
        )?;
        let rows = stmt.query_map([workspace_id], |row| {
            Ok(WorkspaceExplanationHistoryEntry {
                explanation_id: row.get(0)?,
                status: row.get(1)?,
                created_at: row.get(2)?,
                superseded_at: row.get(3)?,
                completeness: row.get(4)?,
                section_count: row.get::<_, i64>(5)? as usize,
                gap_count: row.get::<_, i64>(6)? as usize,
                conflict_count: row.get::<_, i64>(7)? as usize,
                terminal: row.get::<_, i64>(8)? != 0,
                actionable: row.get::<_, i64>(9)? != 0,
                authority_effect: row.get(10)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn history_count(&self, workspace_id: &str) -> Result<usize> {
        let count: i64 = self.db.connection().query_row(
            "SELECT COUNT(*) FROM workspace_explanation_history WHERE workspace_id = ?1",
            [workspace_id],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

type PackageRow = (
    String,
    String,
    String,
    String,
    Option<String>,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    i64,
    i64,
);

fn map_package(row: PackageRow) -> Result<ExplanationPackage> {
    let (
        explanation_id,
        workspace_id,
        status_raw,
        created_at,
        superseded_at,
        completeness_raw,
        narrative,
        scope_json,
        sections_json,
        gaps_json,
        conflicts_json,
        confidence_json,
        provenance_links_json,
        limitations_json,
        authority_effect,
        terminal,
        actionable,
    ) = row;
    let status = ExplanationStatus::parse(&status_raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("bad explanation status: {e}"))
    })?;
    let completeness = ExplanationCompleteness::parse(&completeness_raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("bad completeness: {e}"))
    })?;
    let scope: ExplanationScope = serde_json::from_str(&scope_json).map_err(ser_err)?;
    let sections: Vec<ExplanationSection> =
        serde_json::from_str(&sections_json).map_err(ser_err)?;
    let gaps: Vec<ExplanationGap> = serde_json::from_str(&gaps_json).map_err(ser_err)?;
    let conflicts: Vec<ExplanationConflict> =
        serde_json::from_str(&conflicts_json).map_err(ser_err)?;
    let confidence: ExplanationConfidence =
        serde_json::from_str(&confidence_json).map_err(ser_err)?;
    let provenance_links: Vec<EvidenceReference> =
        serde_json::from_str(&provenance_links_json).map_err(ser_err)?;
    let limitations: Vec<String> = serde_json::from_str(&limitations_json).map_err(ser_err)?;
    Ok(ExplanationPackage {
        explanation_id,
        workspace_id,
        generated_at: created_at,
        status,
        superseded_at,
        scope,
        sections,
        gaps,
        conflicts,
        confidence,
        completeness,
        provenance_links,
        narrative,
        limitations,
        authority_effect,
        actionable: actionable != 0,
        terminal: terminal != 0,
    })
}

fn ser_err(err: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("json error: {err}"))
}
