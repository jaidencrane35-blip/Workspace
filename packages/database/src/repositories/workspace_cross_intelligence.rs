use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    CrossWorkspaceAssessment, CrossWorkspaceCompleteness, CrossWorkspaceConstraintPattern,
    CrossWorkspaceEvidenceRef, CrossWorkspaceGap, CrossWorkspaceIntelligenceHistoryEntry,
    CrossWorkspaceIntelligenceSnapshot, CrossWorkspaceIntelligenceStatus, CrossWorkspacePattern,
    CrossWorkspaceRiskSignal, CrossWorkspaceTheme,
};

/// Persistence for Programme III cross-workspace intelligence.
/// Derived aggregate artefacts only — never Memory SoT, decisions, or upstream mutation.
pub struct WorkspaceCrossIntelligenceRepository<'a> {
    db: &'a Database,
}

impl<'a> WorkspaceCrossIntelligenceRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn persist_view(&self, snap: &CrossWorkspaceIntelligenceSnapshot) -> Result<()> {
        snap.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!(
                "cross-workspace intelligence invalid: {e}"
            ))
        })?;
        let participating_workspaces_json =
            serde_json::to_string(&snap.participating_workspaces).map_err(ser_err)?;
        let source_revisions_json =
            serde_json::to_string(&snap.source_revisions).map_err(ser_err)?;
        let patterns_json = serde_json::to_string(&snap.patterns).map_err(ser_err)?;
        let themes_json = serde_json::to_string(&snap.themes).map_err(ser_err)?;
        let risk_signals_json = serde_json::to_string(&snap.risk_signals).map_err(ser_err)?;
        let constraint_patterns_json =
            serde_json::to_string(&snap.constraint_patterns).map_err(ser_err)?;
        let gaps_json = serde_json::to_string(&snap.gaps).map_err(ser_err)?;
        let assessment_json = serde_json::to_string(&snap.assessment).map_err(ser_err)?;
        let provenance_links_json =
            serde_json::to_string(&snap.provenance_links).map_err(ser_err)?;
        let limitations_json = serde_json::to_string(&snap.limitations).map_err(ser_err)?;

        self.db.connection().execute(
            "INSERT INTO workspace_cross_intelligence_snapshots (
                id, scope_id, status, created_at, superseded_at,
                completeness, pattern_count, theme_count, risk_signal_count,
                constraint_pattern_count, gap_count, workspace_count,
                summary, narrative,
                participating_workspaces_json, source_revisions_json,
                patterns_json, themes_json, risk_signals_json,
                constraint_patterns_json, gaps_json, assessment_json,
                provenance_links_json, limitations_json,
                authority_effect, terminal, actionable
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,'none',0,0)",
            rusqlite::params![
                &snap.intelligence_id,
                &snap.scope_id,
                snap.status.as_str(),
                &snap.generated_at,
                &snap.superseded_at,
                snap.completeness.as_str(),
                snap.patterns.len() as i64,
                snap.themes.len() as i64,
                snap.risk_signals.len() as i64,
                snap.constraint_patterns.len() as i64,
                snap.gaps.len() as i64,
                snap.participating_workspaces.len() as i64,
                &snap.summary,
                &snap.narrative,
                participating_workspaces_json,
                source_revisions_json,
                patterns_json,
                themes_json,
                risk_signals_json,
                constraint_patterns_json,
                gaps_json,
                assessment_json,
                provenance_links_json,
                limitations_json,
            ],
        )?;
        Ok(())
    }

    pub fn supersede_current(
        &self,
        scope_id: &str,
        superseded_at: &str,
    ) -> Result<Vec<CrossWorkspaceIntelligenceSnapshot>> {
        let currents =
            self.list_by_status(scope_id, CrossWorkspaceIntelligenceStatus::Current)?;
        let mut out = Vec::new();
        for mut snap in currents {
            snap.mark_superseded(superseded_at);
            self.db.connection().execute(
                "UPDATE workspace_cross_intelligence_snapshots
                 SET status = ?2, superseded_at = ?3, terminal = 1, actionable = 0
                 WHERE id = ?1 AND scope_id = ?4",
                (
                    &snap.intelligence_id,
                    snap.status.as_str(),
                    &snap.superseded_at,
                    scope_id,
                ),
            )?;
            if let Some(entry) = CrossWorkspaceIntelligenceHistoryEntry::from_snapshot(&snap) {
                self.append_history(scope_id, &entry, superseded_at)?;
            }
            out.push(snap);
        }
        Ok(out)
    }

    pub fn append_history(
        &self,
        scope_id: &str,
        entry: &CrossWorkspaceIntelligenceHistoryEntry,
        recorded_at: &str,
    ) -> Result<()> {
        if !entry.is_non_actionable() {
            return Err(crate::error::DatabaseError::Migration(
                "cross-workspace intelligence history must be non-actionable terminal evidence"
                    .into(),
            ));
        }
        let id = format!(
            "workspace_cross_intelligence_hist:{}",
            uuid::Uuid::new_v4()
        );
        self.db.connection().execute(
            "INSERT INTO workspace_cross_intelligence_history (
                id, scope_id, intelligence_id, status, created_at, superseded_at,
                completeness, pattern_count, theme_count, risk_signal_count,
                constraint_pattern_count, gap_count, workspace_count,
                terminal, actionable, authority_effect, recorded_at
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,1,0,'none',?14)",
            rusqlite::params![
                id,
                scope_id,
                &entry.intelligence_id,
                &entry.status,
                &entry.created_at,
                &entry.superseded_at,
                &entry.completeness,
                entry.pattern_count as i64,
                entry.theme_count as i64,
                entry.risk_signal_count as i64,
                entry.constraint_pattern_count as i64,
                entry.gap_count as i64,
                entry.workspace_count as i64,
                recorded_at,
            ],
        )?;
        Ok(())
    }

    pub fn list_by_status(
        &self,
        scope_id: &str,
        status: CrossWorkspaceIntelligenceStatus,
    ) -> Result<Vec<CrossWorkspaceIntelligenceSnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, scope_id, status, created_at, superseded_at,
                    completeness, summary, narrative,
                    participating_workspaces_json, source_revisions_json,
                    patterns_json, themes_json, risk_signals_json,
                    constraint_patterns_json, gaps_json, assessment_json,
                    provenance_links_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_cross_intelligence_snapshots
             WHERE scope_id = ?1 AND status = ?2
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(rusqlite::params![scope_id, status.as_str()], map_row)?;
        let mut out = Vec::new();
        for row in rows {
            out.push(map_snapshot(row?)?);
        }
        Ok(out)
    }

    pub fn list_views(&self, scope_id: &str) -> Result<Vec<CrossWorkspaceIntelligenceSnapshot>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, scope_id, status, created_at, superseded_at,
                    completeness, summary, narrative,
                    participating_workspaces_json, source_revisions_json,
                    patterns_json, themes_json, risk_signals_json,
                    constraint_patterns_json, gaps_json, assessment_json,
                    provenance_links_json, limitations_json,
                    authority_effect, terminal, actionable
             FROM workspace_cross_intelligence_snapshots
             WHERE scope_id = ?1
             ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([scope_id], map_row)?;
        let mut out = Vec::new();
        for row in rows {
            out.push(map_snapshot(row?)?);
        }
        Ok(out)
    }

    pub fn list_history(
        &self,
        scope_id: &str,
    ) -> Result<Vec<CrossWorkspaceIntelligenceHistoryEntry>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT intelligence_id, status, created_at, superseded_at,
                    completeness, pattern_count, theme_count, risk_signal_count,
                    constraint_pattern_count, gap_count, workspace_count,
                    terminal, actionable, authority_effect
             FROM workspace_cross_intelligence_history
             WHERE scope_id = ?1
             ORDER BY recorded_at DESC",
        )?;
        let rows = stmt.query_map([scope_id], |row| {
            Ok(CrossWorkspaceIntelligenceHistoryEntry {
                intelligence_id: row.get(0)?,
                status: row.get(1)?,
                created_at: row.get(2)?,
                superseded_at: row.get(3)?,
                completeness: row.get(4)?,
                pattern_count: row.get::<_, i64>(5)? as usize,
                theme_count: row.get::<_, i64>(6)? as usize,
                risk_signal_count: row.get::<_, i64>(7)? as usize,
                constraint_pattern_count: row.get::<_, i64>(8)? as usize,
                gap_count: row.get::<_, i64>(9)? as usize,
                workspace_count: row.get::<_, i64>(10)? as usize,
                terminal: row.get::<_, i64>(11)? != 0,
                actionable: row.get::<_, i64>(12)? != 0,
                authority_effect: row.get(13)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    pub fn history_count(&self, scope_id: &str) -> Result<usize> {
        let count: i64 = self.db.connection().query_row(
            "SELECT COUNT(*) FROM workspace_cross_intelligence_history WHERE scope_id = ?1",
            [scope_id],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}

type SnapshotRow = (
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
    String,
    String,
    String,
    String,
    i64,
    i64,
);

fn map_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SnapshotRow> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
        row.get(7)?,
        row.get(8)?,
        row.get(9)?,
        row.get(10)?,
        row.get(11)?,
        row.get(12)?,
        row.get(13)?,
        row.get(14)?,
        row.get(15)?,
        row.get(16)?,
        row.get(17)?,
        row.get(18)?,
        row.get(19)?,
        row.get(20)?,
    ))
}

fn map_snapshot(row: SnapshotRow) -> Result<CrossWorkspaceIntelligenceSnapshot> {
    let (
        intelligence_id,
        scope_id,
        status_raw,
        created_at,
        superseded_at,
        completeness_raw,
        summary,
        narrative,
        participating_workspaces_json,
        source_revisions_json,
        patterns_json,
        themes_json,
        risk_signals_json,
        constraint_patterns_json,
        gaps_json,
        assessment_json,
        provenance_links_json,
        limitations_json,
        authority_effect,
        terminal,
        actionable,
    ) = row;
    let status = CrossWorkspaceIntelligenceStatus::parse(&status_raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!(
            "bad cross-workspace intelligence status: {e}"
        ))
    })?;
    let completeness = CrossWorkspaceCompleteness::parse(&completeness_raw).map_err(|e| {
        crate::error::DatabaseError::Migration(format!("bad completeness: {e}"))
    })?;
    let participating_workspaces: Vec<String> =
        serde_json::from_str(&participating_workspaces_json).map_err(ser_err)?;
    let source_revisions: Vec<String> =
        serde_json::from_str(&source_revisions_json).map_err(ser_err)?;
    let patterns: Vec<CrossWorkspacePattern> =
        serde_json::from_str(&patterns_json).map_err(ser_err)?;
    let themes: Vec<CrossWorkspaceTheme> = serde_json::from_str(&themes_json).map_err(ser_err)?;
    let risk_signals: Vec<CrossWorkspaceRiskSignal> =
        serde_json::from_str(&risk_signals_json).map_err(ser_err)?;
    let constraint_patterns: Vec<CrossWorkspaceConstraintPattern> =
        serde_json::from_str(&constraint_patterns_json).map_err(ser_err)?;
    let gaps: Vec<CrossWorkspaceGap> = serde_json::from_str(&gaps_json).map_err(ser_err)?;
    let assessment: CrossWorkspaceAssessment =
        serde_json::from_str(&assessment_json).map_err(ser_err)?;
    let provenance_links: Vec<CrossWorkspaceEvidenceRef> =
        serde_json::from_str(&provenance_links_json).map_err(ser_err)?;
    let limitations: Vec<String> = serde_json::from_str(&limitations_json).map_err(ser_err)?;
    Ok(CrossWorkspaceIntelligenceSnapshot {
        intelligence_id,
        scope_id,
        generated_at: created_at,
        status,
        superseded_at,
        participating_workspaces,
        source_revisions,
        patterns,
        themes,
        risk_signals,
        constraint_patterns,
        gaps,
        assessment,
        completeness,
        provenance_links,
        summary,
        narrative,
        limitations,
        authority_effect,
        actionable: actionable != 0,
        terminal: terminal != 0,
    })
}

fn ser_err(err: serde_json::Error) -> crate::error::DatabaseError {
    crate::error::DatabaseError::Migration(format!("cross-workspace intelligence json: {err}"))
}
