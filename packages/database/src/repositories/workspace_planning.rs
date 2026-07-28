use crate::connection::Database;
use crate::error::Result;
use workspace_domain::{
    PlanningAlternative, PlanningAssumption, PlanningConstraintReference, PlanningDependency,
    PlanningEvidenceReference, PlanningExplanation, PlanningGap, PlanningPlan, PlanningPlanStatus,
    PlanningProposal, PlanningRisk, PlanningSection, PlanningStep,
};

/// Persistence guards for Programme II planning artefacts (never execute).
pub struct WorkspacePlanningRepository<'a> {
    db: &'a Database,
}

impl<'a> WorkspacePlanningRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn upsert_plan(
        &self,
        plan: &PlanningPlan,
        explanation: &PlanningExplanation,
        sections: &[PlanningSection],
        alternatives: &[PlanningAlternative],
        constraint_refs: &[PlanningConstraintReference],
        evidence_refs: &[PlanningEvidenceReference],
    ) -> Result<()> {
        plan.validate().map_err(|e| {
            crate::error::DatabaseError::Migration(format!("planning plan invalid: {e}"))
        })?;
        let sections_json = serde_json::to_string(sections).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("sections serialize: {e}"))
        })?;
        let alternatives_json = serde_json::to_string(alternatives).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("alternatives serialize: {e}"))
        })?;
        let constraint_refs_json = serde_json::to_string(constraint_refs).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("constraint_refs serialize: {e}"))
        })?;
        let evidence_refs_json = serde_json::to_string(evidence_refs).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("evidence_refs serialize: {e}"))
        })?;
        self.db.connection().execute(
            "INSERT INTO planning_plans (
                id, workspace_id, title, summary, status, confidence, uncertainty,
                generated_at, superseded_at, explanation_summary, explanation_why,
                explanation_next, sections_json, alternatives_json, constraint_refs_json,
                evidence_refs_json, authority_effect
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17)
             ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                summary = excluded.summary,
                status = excluded.status,
                confidence = excluded.confidence,
                uncertainty = excluded.uncertainty,
                superseded_at = excluded.superseded_at,
                explanation_summary = excluded.explanation_summary,
                explanation_why = excluded.explanation_why,
                explanation_next = excluded.explanation_next,
                sections_json = excluded.sections_json,
                alternatives_json = excluded.alternatives_json,
                constraint_refs_json = excluded.constraint_refs_json,
                evidence_refs_json = excluded.evidence_refs_json,
                authority_effect = excluded.authority_effect
             WHERE planning_plans.workspace_id = excluded.workspace_id",
            rusqlite::params![
                &plan.id,
                &plan.workspace_id,
                &plan.title,
                &plan.summary,
                plan.status.as_str(),
                plan.confidence as i64,
                plan.uncertainty as i64,
                &plan.generated_at,
                &plan.superseded_at,
                &explanation.summary,
                &explanation.why,
                &explanation.next,
                sections_json,
                alternatives_json,
                constraint_refs_json,
                evidence_refs_json,
                &plan.authority_effect,
            ],
        )?;
        Ok(())
    }

    pub fn supersede_active(&self, workspace_id: &str, superseded_at: &str) -> Result<usize> {
        let changed = self.db.connection().execute(
            "UPDATE planning_plans
             SET status = 'superseded', superseded_at = ?2
             WHERE workspace_id = ?1 AND status = 'active'",
            (workspace_id, superseded_at),
        )?;
        Ok(changed)
    }

    pub fn insert_step(&self, step: &PlanningStep) -> Result<()> {
        let evidence_refs_json = serde_json::to_string(&step.evidence_refs).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("step evidence serialize: {e}"))
        })?;
        self.db.connection().execute(
            "INSERT INTO planning_steps (
                id, plan_id, workspace_id, ordinal, title, rationale,
                evidence_refs_json, confidence, uncertainty, authority_effect
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
            (
                &step.id,
                &step.plan_id,
                &step.workspace_id,
                step.ordinal as i64,
                &step.title,
                &step.rationale,
                evidence_refs_json,
                step.confidence as i64,
                step.uncertainty as i64,
                &step.authority_effect,
            ),
        )?;
        Ok(())
    }

    pub fn insert_assumption(&self, item: &PlanningAssumption) -> Result<()> {
        let evidence_refs_json = serde_json::to_string(&item.evidence_refs).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("assumption evidence serialize: {e}"))
        })?;
        self.db.connection().execute(
            "INSERT INTO planning_assumptions (
                id, plan_id, workspace_id, statement, confidence,
                evidence_refs_json, authority_effect
             ) VALUES (?1,?2,?3,?4,?5,?6,?7)",
            (
                &item.id,
                &item.plan_id,
                &item.workspace_id,
                &item.statement,
                item.confidence as i64,
                evidence_refs_json,
                &item.authority_effect,
            ),
        )?;
        Ok(())
    }

    pub fn insert_risk(&self, item: &PlanningRisk) -> Result<()> {
        let evidence_refs_json = serde_json::to_string(&item.evidence_refs).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("risk evidence serialize: {e}"))
        })?;
        self.db.connection().execute(
            "INSERT INTO planning_risks (
                id, plan_id, workspace_id, statement, uncertainty,
                evidence_refs_json, authority_effect
             ) VALUES (?1,?2,?3,?4,?5,?6,?7)",
            (
                &item.id,
                &item.plan_id,
                &item.workspace_id,
                &item.statement,
                item.uncertainty as i64,
                evidence_refs_json,
                &item.authority_effect,
            ),
        )?;
        Ok(())
    }

    pub fn insert_gap(&self, item: &PlanningGap) -> Result<()> {
        let evidence_refs_json = serde_json::to_string(&item.evidence_refs).map_err(|e| {
            crate::error::DatabaseError::Migration(format!("gap evidence serialize: {e}"))
        })?;
        self.db.connection().execute(
            "INSERT INTO planning_gaps (
                id, plan_id, workspace_id, statement, evidence_refs_json, authority_effect
             ) VALUES (?1,?2,?3,?4,?5,?6)",
            (
                &item.id,
                &item.plan_id,
                &item.workspace_id,
                &item.statement,
                evidence_refs_json,
                &item.authority_effect,
            ),
        )?;
        Ok(())
    }

    pub fn insert_dependency(&self, item: &PlanningDependency) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO planning_dependencies (
                id, plan_id, workspace_id, from_step_id, to_step_id, kind, authority_effect
             ) VALUES (?1,?2,?3,?4,?5,?6,?7)",
            (
                &item.id,
                &item.plan_id,
                &item.workspace_id,
                &item.from_step_id,
                &item.to_step_id,
                &item.kind,
                &item.authority_effect,
            ),
        )?;
        Ok(())
    }

    pub fn persist_proposal(&self, proposal: &PlanningProposal) -> Result<()> {
        self.upsert_plan(
            &proposal.plan,
            &proposal.explanation,
            &proposal.sections,
            &proposal.alternatives,
            &proposal.constraint_refs,
            &proposal.evidence_refs,
        )?;
        for step in &proposal.steps {
            self.insert_step(step)?;
        }
        for item in &proposal.assumptions {
            self.insert_assumption(item)?;
        }
        for item in &proposal.risks {
            self.insert_risk(item)?;
        }
        for item in &proposal.gaps {
            self.insert_gap(item)?;
        }
        for item in &proposal.dependencies {
            self.insert_dependency(item)?;
        }
        Ok(())
    }

    pub fn list_plans(&self, workspace_id: &str) -> Result<Vec<PlanningPlan>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, workspace_id, title, summary, status, confidence, uncertainty,
                    generated_at, superseded_at, authority_effect
             FROM planning_plans
             WHERE workspace_id = ?1
             ORDER BY generated_at DESC, id ASC",
        )?;
        let rows = stmt.query_map([workspace_id], map_plan)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn load_proposal(&self, plan: &PlanningPlan) -> Result<PlanningProposal> {
        let meta = self.load_plan_meta(&plan.id)?;
        let steps = self.list_steps(&plan.id)?;
        let assumptions = self.list_assumptions(&plan.id)?;
        let risks = self.list_risks(&plan.id)?;
        let gaps = self.list_gaps(&plan.id)?;
        let dependencies = self.list_dependencies(&plan.id)?;
        Ok(PlanningProposal {
            plan: plan.clone(),
            steps,
            sections: meta.sections,
            assumptions,
            risks,
            gaps,
            dependencies,
            alternatives: meta.alternatives,
            constraint_refs: meta.constraint_refs,
            evidence_refs: meta.evidence_refs,
            explanation: meta.explanation,
            authority_effect: PlanningProposal::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    fn load_plan_meta(&self, plan_id: &str) -> Result<PlanMeta> {
        let mut stmt = self.db.connection().prepare(
            "SELECT explanation_summary, explanation_why, explanation_next,
                    sections_json, alternatives_json, constraint_refs_json, evidence_refs_json
             FROM planning_plans WHERE id = ?1 LIMIT 1",
        )?;
        let mut rows = stmt.query([plan_id])?;
        let row = rows.next()?.ok_or_else(|| {
            crate::error::DatabaseError::Migration(format!("planning plan meta missing: {plan_id}"))
        })?;
        let explanation = PlanningExplanation {
            summary: row.get(0)?,
            why: row.get(1)?,
            next: row.get(2)?,
        };
        let sections: Vec<PlanningSection> = serde_json::from_str(&row.get::<_, String>(3)?)
            .unwrap_or_default();
        let alternatives: Vec<PlanningAlternative> =
            serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default();
        let constraint_refs: Vec<PlanningConstraintReference> =
            serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default();
        let evidence_refs: Vec<PlanningEvidenceReference> =
            serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default();
        Ok(PlanMeta {
            explanation,
            sections,
            alternatives,
            constraint_refs,
            evidence_refs,
        })
    }

    fn list_steps(&self, plan_id: &str) -> Result<Vec<PlanningStep>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, plan_id, workspace_id, ordinal, title, rationale,
                    evidence_refs_json, confidence, uncertainty, authority_effect
             FROM planning_steps WHERE plan_id = ?1 ORDER BY ordinal ASC, id ASC",
        )?;
        let rows = stmt.query_map([plan_id], map_step)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn list_assumptions(&self, plan_id: &str) -> Result<Vec<PlanningAssumption>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, plan_id, workspace_id, statement, confidence,
                    evidence_refs_json, authority_effect
             FROM planning_assumptions WHERE plan_id = ?1 ORDER BY id ASC",
        )?;
        let rows = stmt.query_map([plan_id], map_assumption)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn list_risks(&self, plan_id: &str) -> Result<Vec<PlanningRisk>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, plan_id, workspace_id, statement, uncertainty,
                    evidence_refs_json, authority_effect
             FROM planning_risks WHERE plan_id = ?1 ORDER BY id ASC",
        )?;
        let rows = stmt.query_map([plan_id], map_risk)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn list_gaps(&self, plan_id: &str) -> Result<Vec<PlanningGap>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, plan_id, workspace_id, statement, evidence_refs_json, authority_effect
             FROM planning_gaps WHERE plan_id = ?1 ORDER BY id ASC",
        )?;
        let rows = stmt.query_map([plan_id], map_gap)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn list_dependencies(&self, plan_id: &str) -> Result<Vec<PlanningDependency>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, plan_id, workspace_id, from_step_id, to_step_id, kind, authority_effect
             FROM planning_dependencies WHERE plan_id = ?1 ORDER BY id ASC",
        )?;
        let rows = stmt.query_map([plan_id], map_dependency)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

struct PlanMeta {
    explanation: PlanningExplanation,
    sections: Vec<PlanningSection>,
    alternatives: Vec<PlanningAlternative>,
    constraint_refs: Vec<PlanningConstraintReference>,
    evidence_refs: Vec<PlanningEvidenceReference>,
}

fn map_plan(row: &rusqlite::Row<'_>) -> rusqlite::Result<PlanningPlan> {
    let status = PlanningPlanStatus::parse(&row.get::<_, String>(4)?).map_err(|_| {
        rusqlite::Error::InvalidColumnType(4, "status".into(), rusqlite::types::Type::Text)
    })?;
    Ok(PlanningPlan {
        id: row.get(0)?,
        workspace_id: row.get(1)?,
        title: row.get(2)?,
        summary: row.get(3)?,
        status,
        confidence: row.get::<_, i64>(5)? as u8,
        uncertainty: row.get::<_, i64>(6)? as u8,
        generated_at: row.get(7)?,
        superseded_at: row.get(8)?,
        authority_effect: row.get(9)?,
    })
}

fn parse_refs(raw: String) -> Vec<String> {
    serde_json::from_str(&raw).unwrap_or_default()
}

fn map_step(row: &rusqlite::Row<'_>) -> rusqlite::Result<PlanningStep> {
    Ok(PlanningStep {
        id: row.get(0)?,
        plan_id: row.get(1)?,
        workspace_id: row.get(2)?,
        ordinal: row.get::<_, i64>(3)? as u32,
        title: row.get(4)?,
        rationale: row.get(5)?,
        evidence_refs: parse_refs(row.get(6)?),
        confidence: row.get::<_, i64>(7)? as u8,
        uncertainty: row.get::<_, i64>(8)? as u8,
        authority_effect: row.get(9)?,
    })
}

fn map_assumption(row: &rusqlite::Row<'_>) -> rusqlite::Result<PlanningAssumption> {
    Ok(PlanningAssumption {
        id: row.get(0)?,
        plan_id: row.get(1)?,
        workspace_id: row.get(2)?,
        statement: row.get(3)?,
        confidence: row.get::<_, i64>(4)? as u8,
        evidence_refs: parse_refs(row.get(5)?),
        authority_effect: row.get(6)?,
    })
}

fn map_risk(row: &rusqlite::Row<'_>) -> rusqlite::Result<PlanningRisk> {
    Ok(PlanningRisk {
        id: row.get(0)?,
        plan_id: row.get(1)?,
        workspace_id: row.get(2)?,
        statement: row.get(3)?,
        uncertainty: row.get::<_, i64>(4)? as u8,
        evidence_refs: parse_refs(row.get(5)?),
        authority_effect: row.get(6)?,
    })
}

fn map_gap(row: &rusqlite::Row<'_>) -> rusqlite::Result<PlanningGap> {
    Ok(PlanningGap {
        id: row.get(0)?,
        plan_id: row.get(1)?,
        workspace_id: row.get(2)?,
        statement: row.get(3)?,
        evidence_refs: parse_refs(row.get(4)?),
        authority_effect: row.get(5)?,
    })
}

fn map_dependency(row: &rusqlite::Row<'_>) -> rusqlite::Result<PlanningDependency> {
    Ok(PlanningDependency {
        id: row.get(0)?,
        plan_id: row.get(1)?,
        workspace_id: row.get(2)?,
        from_step_id: row.get(3)?,
        to_step_id: row.get(4)?,
        kind: row.get(5)?,
        authority_effect: row.get(6)?,
    })
}
