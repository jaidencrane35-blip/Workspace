use crate::connection::Database;
use crate::error::Result;
use serde_json::{json, Value};
use workspace_domain::{
    PilotBaseline, PilotConsent, PilotInterviewPhase, PilotInterviewRecord, PilotLeaveResumeRecord,
};

const CONSENT_ROW_ID: &str = "local";

/// Persistence for consented pilot evaluation records (PP-P01E).
pub struct PilotMeasurementRepository<'a> {
    db: &'a Database,
}

impl<'a> PilotMeasurementRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn get_consent(&self) -> Result<Option<PilotConsent>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT scope_id, consented_at, withdrawn_at FROM pilot_consent WHERE id = ?1",
        )?;
        let mut rows = stmt.query([CONSENT_ROW_ID])?;
        let Some(row) = rows.next()? else {
            return Ok(None);
        };
        Ok(Some(PilotConsent {
            scope_id: row.get(0)?,
            consented_at: row.get(1)?,
            withdrawn_at: row.get(2)?,
        }))
    }

    pub fn upsert_consent(&self, consent: &PilotConsent) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO pilot_consent (id, scope_id, consented_at, withdrawn_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(id) DO UPDATE SET
                scope_id = excluded.scope_id,
                consented_at = excluded.consented_at,
                withdrawn_at = excluded.withdrawn_at",
            (
                CONSENT_ROW_ID,
                consent.scope_id.as_str(),
                consent.consented_at.as_str(),
                consent.withdrawn_at.as_deref(),
            ),
        )?;
        Ok(())
    }

    pub fn clear_all_records(&self) -> Result<()> {
        self.db
            .connection()
            .execute("DELETE FROM pilot_records", [])?;
        Ok(())
    }

    pub fn clear_consent(&self) -> Result<()> {
        self.db
            .connection()
            .execute("DELETE FROM pilot_consent WHERE id = ?1", [CONSENT_ROW_ID])?;
        Ok(())
    }

    pub fn upsert_baseline(&self, baseline: &PilotBaseline) -> Result<()> {
        self.replace_kind(
            "baseline",
            &baseline.recorded_at,
            "",
            &json!({
                "return_minutes": baseline.return_minutes,
                "notes": baseline.notes,
            }),
        )
    }

    pub fn get_baseline(&self) -> Result<Option<PilotBaseline>> {
        let Some((_, recorded_at, _, payload)) = self.latest_of_kind("baseline")? else {
            return Ok(None);
        };
        Ok(Some(PilotBaseline {
            return_minutes: payload["return_minutes"].as_u64().unwrap_or(0) as u32,
            recorded_at,
            notes: payload["notes"].as_str().unwrap_or("").to_string(),
        }))
    }

    pub fn insert_leave_resume(&self, record: &PilotLeaveResumeRecord) -> Result<()> {
        self.db.connection().execute(
            "INSERT INTO pilot_records (id, kind, recorded_at, local_day, payload_json)
             VALUES (?1, 'leave_resume', ?2, ?3, ?4)",
            (
                record.id.as_str(),
                record.recorded_at.as_str(),
                record.local_day.as_str(),
                json!({
                    "return_minutes": record.return_minutes,
                    "correction_needed": record.correction_needed,
                    "correction_note": record.correction_note,
                })
                .to_string(),
            ),
        )?;
        Ok(())
    }

    pub fn list_leave_resume(&self) -> Result<Vec<PilotLeaveResumeRecord>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, recorded_at, local_day, payload_json
             FROM pilot_records WHERE kind = 'leave_resume'
             ORDER BY recorded_at ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let recorded_at: String = row.get(1)?;
            let local_day: String = row.get(2)?;
            let payload: String = row.get(3)?;
            Ok((id, recorded_at, local_day, payload))
        })?;
        let mut out = Vec::new();
        for row in rows {
            let (id, recorded_at, local_day, payload) = row?;
            let value: Value = serde_json::from_str(&payload).unwrap_or(json!({}));
            out.push(PilotLeaveResumeRecord {
                id,
                recorded_at,
                local_day,
                return_minutes: value["return_minutes"].as_u64().unwrap_or(0) as u32,
                correction_needed: value["correction_needed"].as_bool().unwrap_or(false),
                correction_note: value["correction_note"].as_str().unwrap_or("").to_string(),
            });
        }
        Ok(out)
    }

    pub fn upsert_interview(&self, record: &PilotInterviewRecord) -> Result<()> {
        let kind = match record.phase {
            PilotInterviewPhase::Baseline => "interview_baseline",
            PilotInterviewPhase::WeekFour => "interview_week_four",
        };
        self.replace_kind(
            kind,
            &record.recorded_at,
            "",
            &json!({ "responses": record.responses }),
        )
    }

    pub fn get_interview(
        &self,
        phase: PilotInterviewPhase,
    ) -> Result<Option<PilotInterviewRecord>> {
        let kind = match phase {
            PilotInterviewPhase::Baseline => "interview_baseline",
            PilotInterviewPhase::WeekFour => "interview_week_four",
        };
        let Some((_, recorded_at, _, payload)) = self.latest_of_kind(kind)? else {
            return Ok(None);
        };
        Ok(Some(PilotInterviewRecord {
            phase,
            recorded_at,
            responses: payload["responses"].as_str().unwrap_or("").to_string(),
        }))
    }

    fn replace_kind(
        &self,
        kind: &str,
        recorded_at: &str,
        local_day: &str,
        payload: &Value,
    ) -> Result<()> {
        self.db.connection().execute(
            "DELETE FROM pilot_records WHERE kind = ?1",
            [kind],
        )?;
        let id = format!("{kind}-current");
        self.db.connection().execute(
            "INSERT INTO pilot_records (id, kind, recorded_at, local_day, payload_json)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                id.as_str(),
                kind,
                recorded_at,
                local_day,
                payload.to_string(),
            ),
        )?;
        Ok(())
    }

    fn latest_of_kind(
        &self,
        kind: &str,
    ) -> Result<Option<(String, String, String, Value)>> {
        let mut stmt = self.db.connection().prepare(
            "SELECT id, recorded_at, local_day, payload_json
             FROM pilot_records WHERE kind = ?1
             ORDER BY recorded_at DESC LIMIT 1",
        )?;
        let mut rows = stmt.query([kind])?;
        let Some(row) = rows.next()? else {
            return Ok(None);
        };
        let id: String = row.get(0)?;
        let recorded_at: String = row.get(1)?;
        let local_day: String = row.get(2)?;
        let payload: String = row.get(3)?;
        let value: Value = serde_json::from_str(&payload).unwrap_or(json!({}));
        Ok(Some((id, recorded_at, local_day, value)))
    }
}
