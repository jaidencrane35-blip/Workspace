//! Consented local pilot measurement service (PP-P01E).

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use chrono::{Local, Utc};
use uuid::Uuid;
use workspace_database::{Database, PilotMeasurementRepository};
use workspace_domain::{
    median_return_minutes, GrantPilotConsentRequest, PilotBaseline, PilotConsent,
    PilotInterviewPhase, PilotInterviewRecord, PilotLeaveResumeRecord, PilotMeasurementError,
    PilotMeasurementScope, PilotMeasurementSnapshot, RecordPilotBaselineRequest,
    RecordPilotInterviewRequest, RecordPilotLeaveResumeRequest, PILOT_MEASUREMENT_SCOPE_ID,
};

use crate::error::{KernelError, Result};

pub(crate) struct PilotMeasurementService;

impl PilotMeasurementService {
    pub(crate) fn scope() -> PilotMeasurementScope {
        PilotMeasurementScope::current()
    }

    pub(crate) fn snapshot(db: &Arc<Mutex<Database>>) -> Result<PilotMeasurementSnapshot> {
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        let repo = PilotMeasurementRepository::new(&guard);
        let consent = repo.get_consent()?;
        let baseline = repo.get_baseline()?;
        let leave_resume = repo.list_leave_resume()?;
        let interview_baseline = repo.get_interview(PilotInterviewPhase::Baseline)?;
        let interview_week_four = repo.get_interview(PilotInterviewPhase::WeekFour)?;
        let minutes: Vec<u32> = leave_resume.iter().map(|r| r.return_minutes).collect();
        let days: BTreeSet<_> = leave_resume.iter().map(|r| r.local_day.clone()).collect();
        Ok(PilotMeasurementSnapshot {
            scope: Self::scope(),
            consent,
            baseline,
            leave_resume,
            interview_baseline,
            interview_week_four,
            distinct_resume_days: days.len() as u32,
            median_return_minutes: median_return_minutes(&minutes),
        })
    }

    pub(crate) fn grant_consent(
        db: &Arc<Mutex<Database>>,
        request: &GrantPilotConsentRequest,
    ) -> Result<PilotConsent> {
        request
            .validate()
            .map_err(|e| KernelError::PilotMeasurementValidation {
                message: e.to_string(),
            })?;
        let consent = PilotConsent {
            scope_id: PILOT_MEASUREMENT_SCOPE_ID.to_string(),
            consented_at: Utc::now().to_rfc3339(),
            withdrawn_at: None,
        };
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        PilotMeasurementRepository::new(&guard).upsert_consent(&consent)?;
        Ok(consent)
    }

    pub(crate) fn withdraw_consent(
        db: &Arc<Mutex<Database>>,
        clear_records: bool,
    ) -> Result<PilotConsent> {
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        let repo = PilotMeasurementRepository::new(&guard);
        let mut consent = repo.get_consent()?.ok_or(KernelError::PilotMeasurementValidation {
            message: PilotMeasurementError::ConsentRequired.to_string(),
        })?;
        consent.withdrawn_at = Some(Utc::now().to_rfc3339());
        repo.upsert_consent(&consent)?;
        if clear_records {
            repo.clear_all_records()?;
        }
        Ok(consent)
    }

    pub(crate) fn record_baseline(
        db: &Arc<Mutex<Database>>,
        request: &RecordPilotBaselineRequest,
    ) -> Result<PilotBaseline> {
        request
            .validate()
            .map_err(|e| KernelError::PilotMeasurementValidation {
                message: e.to_string(),
            })?;
        Self::require_active_consent(db)?;
        let baseline = PilotBaseline {
            return_minutes: request.return_minutes,
            recorded_at: Utc::now().to_rfc3339(),
            notes: request.notes.trim().to_string(),
        };
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        PilotMeasurementRepository::new(&guard).upsert_baseline(&baseline)?;
        Ok(baseline)
    }

    pub(crate) fn record_leave_resume(
        db: &Arc<Mutex<Database>>,
        request: &RecordPilotLeaveResumeRequest,
    ) -> Result<PilotLeaveResumeRecord> {
        request
            .validate()
            .map_err(|e| KernelError::PilotMeasurementValidation {
                message: e.to_string(),
            })?;
        Self::require_active_consent(db)?;
        let local_day = if request.local_day.trim().is_empty() {
            Local::now().date_naive().format("%Y-%m-%d").to_string()
        } else {
            request.local_day.trim().to_string()
        };
        let record = PilotLeaveResumeRecord {
            id: Uuid::new_v4().to_string(),
            recorded_at: Utc::now().to_rfc3339(),
            local_day,
            return_minutes: request.return_minutes,
            correction_needed: request.correction_needed,
            correction_note: request.correction_note.trim().to_string(),
        };
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        PilotMeasurementRepository::new(&guard).insert_leave_resume(&record)?;
        Ok(record)
    }

    pub(crate) fn record_interview(
        db: &Arc<Mutex<Database>>,
        request: &RecordPilotInterviewRequest,
    ) -> Result<PilotInterviewRecord> {
        request
            .validate()
            .map_err(|e| KernelError::PilotMeasurementValidation {
                message: e.to_string(),
            })?;
        Self::require_active_consent(db)?;
        let record = PilotInterviewRecord {
            phase: request.phase,
            recorded_at: Utc::now().to_rfc3339(),
            responses: request.responses.trim().to_string(),
        };
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        PilotMeasurementRepository::new(&guard).upsert_interview(&record)?;
        Ok(record)
    }

    fn require_active_consent(db: &Arc<Mutex<Database>>) -> Result<()> {
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        let consent = PilotMeasurementRepository::new(&guard).get_consent()?;
        match consent {
            Some(c) if c.is_active() => Ok(()),
            Some(_) => Err(KernelError::PilotMeasurementValidation {
                message: PilotMeasurementError::ConsentWithdrawn.to_string(),
            }),
            None => Err(KernelError::PilotMeasurementValidation {
                message: PilotMeasurementError::ConsentRequired.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WorkspaceKernel;

    #[test]
    fn refuses_records_without_consent() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        let err = PilotMeasurementService::record_baseline(
            &db,
            &RecordPilotBaselineRequest {
                return_minutes: 10,
                notes: String::new(),
            },
        )
        .unwrap_err();
        assert!(matches!(err, KernelError::PilotMeasurementValidation { .. }));
    }

    #[test]
    fn consent_then_baseline_and_leave_resume() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        PilotMeasurementService::grant_consent(
            &db,
            &GrantPilotConsentRequest {
                approved_scope: PILOT_MEASUREMENT_SCOPE_ID.into(),
            },
        )
        .unwrap();
        PilotMeasurementService::record_baseline(
            &db,
            &RecordPilotBaselineRequest {
                return_minutes: 12,
                notes: "before Workspace".into(),
            },
        )
        .unwrap();
        PilotMeasurementService::record_leave_resume(
            &db,
            &RecordPilotLeaveResumeRequest {
                return_minutes: 4,
                correction_needed: true,
                correction_note: "moved one window".into(),
                local_day: "2026-08-02".into(),
            },
        )
        .unwrap();
        let snap = PilotMeasurementService::snapshot(&db).unwrap();
        assert_eq!(snap.baseline.as_ref().unwrap().return_minutes, 12);
        assert_eq!(snap.leave_resume.len(), 1);
        assert_eq!(snap.distinct_resume_days, 1);
        assert_eq!(snap.median_return_minutes, Some(4));
    }

    #[test]
    fn withdraw_with_clear_removes_records() {
        let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
        let db = kernel.shared_database();
        PilotMeasurementService::grant_consent(
            &db,
            &GrantPilotConsentRequest {
                approved_scope: PILOT_MEASUREMENT_SCOPE_ID.into(),
            },
        )
        .unwrap();
        PilotMeasurementService::record_baseline(
            &db,
            &RecordPilotBaselineRequest {
                return_minutes: 8,
                notes: String::new(),
            },
        )
        .unwrap();
        PilotMeasurementService::withdraw_consent(&db, true).unwrap();
        let snap = PilotMeasurementService::snapshot(&db).unwrap();
        assert!(snap.consent.as_ref().unwrap().withdrawn_at.is_some());
        assert!(snap.baseline.is_none());
        let refused = PilotMeasurementService::record_baseline(
            &db,
            &RecordPilotBaselineRequest {
                return_minutes: 8,
                notes: String::new(),
            },
        );
        assert!(refused.is_err());
    }
}
