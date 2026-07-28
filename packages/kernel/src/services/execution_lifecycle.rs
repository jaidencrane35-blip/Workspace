use std::sync::{Arc, Mutex};

use chrono::Utc;
use workspace_database::{Database, ExecutionLifecycleRepository};
use workspace_domain::{
    ExecutionLifecycleRecord, ExecutionOutcome, ExecutionOutcomeStatus, ExecutionReconciliation,
    ExecutionState,
};

use crate::error::{KernelError, Result};

/// Durable lifecycle authority for governed intent execution identities.
pub struct ExecutionLifecycleService;

impl ExecutionLifecycleService {
    pub fn claim(
        db: &Arc<Mutex<Database>>,
        execution_request_id: &str,
        suggestion_id: &str,
        intent_id: Option<&str>,
    ) -> Result<ExecutionLifecycleRecord> {
        let now = Utc::now().to_rfc3339();
        let record = ExecutionLifecycleRecord {
            execution_request_id: execution_request_id.trim().into(),
            suggestion_id: suggestion_id.trim().into(),
            intent_id: intent_id.map(str::to_string),
            state: ExecutionState::InProgress,
            claimed_at: now.clone(),
            completed_at: None,
            updated_at: now,
        };
        record.validate().map_err(|error| {
            KernelError::ExecutionReconciliationValidation {
                message: error.to_string(),
            }
        })?;

        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repository = ExecutionLifecycleRepository::new(&guard);
        let inserted = repository.insert_claim(&record).map_err(|source| {
            KernelError::ExecutionLifecyclePersistence {
                stage: "claim",
                source,
            }
        })?;
        if inserted {
            return Ok(record);
        }

        match repository.get(&record.execution_request_id).map_err(|source| {
            KernelError::ExecutionLifecyclePersistence {
                stage: "claim.read_existing",
                source,
            }
        })? {
            Some(existing) if existing.state == ExecutionState::Completed => {
                Err(KernelError::DuplicateExecution {
                    execution_request_id: record.execution_request_id,
                })
            }
            Some(_) => Err(KernelError::ExecutionInProgress {
                execution_request_id: record.execution_request_id,
            }),
            None => Err(KernelError::IntegrityViolation {
                message: "execution claim conflicted but no lifecycle row exists".into(),
            }),
        }
    }

    pub fn complete(
        db: &Arc<Mutex<Database>>,
        execution_request_id: &str,
        intent_id: Option<&str>,
    ) -> Result<ExecutionLifecycleRecord> {
        let completed_at = Utc::now().to_rfc3339();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repository = ExecutionLifecycleRepository::new(&guard);
        let updated = repository
            .mark_completed(execution_request_id, intent_id, &completed_at)
            .map_err(|source| KernelError::ExecutionLifecyclePersistence {
                stage: "completion",
                source,
            })?;
        if !updated {
            return Err(KernelError::IntegrityViolation {
                message: format!(
                    "execution lifecycle completion requires an in-progress claim: {execution_request_id}"
                ),
            });
        }
        repository
            .get(execution_request_id)
            .map_err(|source| KernelError::ExecutionLifecyclePersistence {
                stage: "completion.read",
                source,
            })?
            .ok_or_else(|| KernelError::IntegrityViolation {
                message: format!(
                    "completed execution lifecycle row disappeared: {execution_request_id}"
                ),
            })
    }

    pub fn release_failed_claim(
        db: &Arc<Mutex<Database>>,
        execution_request_id: &str,
    ) -> Result<()> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let released = ExecutionLifecycleRepository::new(&guard)
            .release_in_progress(execution_request_id)
            .map_err(|source| KernelError::ExecutionLifecyclePersistence {
                stage: "failure_release",
                source,
            })?;
        if released {
            Ok(())
        } else {
            Err(KernelError::IntegrityViolation {
                message: format!(
                    "failed execution did not have a releasable claim: {execution_request_id}"
                ),
            })
        }
    }

    pub fn get(
        db: &Arc<Mutex<Database>>,
        execution_request_id: &str,
    ) -> Result<Option<ExecutionLifecycleRecord>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        ExecutionLifecycleRepository::new(&guard)
            .get(execution_request_id)
            .map_err(|source| KernelError::ExecutionLifecyclePersistence {
                stage: "read",
                source,
            })
    }

    pub fn list_recent(
        db: &Arc<Mutex<Database>>,
        limit: usize,
    ) -> Result<Vec<ExecutionLifecycleRecord>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        ExecutionLifecycleRepository::new(&guard)
            .list_recent(limit)
            .map_err(|source| KernelError::ExecutionLifecyclePersistence {
                stage: "list",
                source,
            })
    }

    pub fn completed_outcome(record: &ExecutionLifecycleRecord) -> Result<Option<ExecutionOutcome>> {
        if record.state != ExecutionState::Completed {
            return Ok(None);
        }
        record.validate().map_err(|error| {
            KernelError::ExecutionReconciliationValidation {
                message: error.to_string(),
            }
        })?;
        Ok(Some(ExecutionOutcome {
            execution_request_id: record.execution_request_id.clone(),
            status: ExecutionOutcomeStatus::Completed,
            command_name: "ExecuteIntentRequest".into(),
            completed_at: record.completed_at.clone().ok_or_else(|| {
                KernelError::IntegrityViolation {
                    message: format!(
                        "completed execution lifecycle missing timestamp: {}",
                        record.execution_request_id
                    ),
                }
            })?,
            success: true,
            failure_reason: None,
            suggestion_id: Some(record.suggestion_id.clone()),
            intent_id: record.intent_id.clone(),
        }))
    }

    pub fn reconciliation(record: &ExecutionLifecycleRecord) -> ExecutionReconciliation {
        ExecutionReconciliation {
            execution_request_id: record.execution_request_id.clone(),
            current_state: record.state,
            dispatch_allowed: false,
            cancellation_allowed: false,
        }
    }
}
