use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use workspace_database::{Database, ExecutionLifecycleRepository};
use workspace_domain::{
    reconcile_execution_lifecycle, ExecutionLifecycleRecord, ExecutionOutcome,
    ExecutionOutcomeStatus, ExecutionReconciliation, ExecutionState,
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
            retry_allowed: false,
            failure_reason: None,
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
            Some(existing) if existing.state == ExecutionState::Failed => {
                Err(KernelError::ExecutionReconciliationRequired {
                    execution_request_id: record.execution_request_id,
                })
            }
            Some(existing) => {
                if Self::is_stale(&existing)? {
                    repository
                        .mark_failed(
                            &existing.execution_request_id,
                            false,
                            "stale execution claim; dispatch outcome unknown",
                            &Utc::now().to_rfc3339(),
                        )
                        .map_err(|source| KernelError::ExecutionLifecyclePersistence {
                            stage: "stale_reconciliation",
                            source,
                        })?;
                    return Err(KernelError::ExecutionReconciliationRequired {
                        execution_request_id: record.execution_request_id,
                    });
                }
                Err(KernelError::ExecutionInProgress {
                    execution_request_id: record.execution_request_id,
                })
            }
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

    pub fn mark_failed(
        db: &Arc<Mutex<Database>>,
        execution_request_id: &str,
        retry_allowed: bool,
        reason: &str,
    ) -> Result<ExecutionLifecycleRecord> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repository = ExecutionLifecycleRepository::new(&guard);
        let failed_at = Utc::now().to_rfc3339();
        let updated = repository
            .mark_failed(execution_request_id, retry_allowed, reason, &failed_at)
            .map_err(|source| KernelError::ExecutionLifecyclePersistence {
                stage: "failure",
                source,
            })?;
        if !updated {
            return Err(KernelError::IntegrityViolation {
                message: format!(
                    "failed execution did not have an in-progress claim: {execution_request_id}"
                ),
            });
        }
        repository
            .get(execution_request_id)
            .map_err(|source| KernelError::ExecutionLifecyclePersistence {
                stage: "failure.read",
                source,
            })?
            .ok_or_else(|| KernelError::IntegrityViolation {
                message: format!("failed execution lifecycle row disappeared: {execution_request_id}"),
            })
    }

    pub fn record_cancelled(
        db: &Arc<Mutex<Database>>,
        execution_request_id: &str,
    ) -> Result<ExecutionLifecycleRecord> {
        let suggestion_id = execution_request_id
            .trim()
            .strip_prefix("execution:")
            .filter(|value| !value.is_empty())
            .ok_or_else(|| KernelError::ExecutionReconciliationValidation {
                message: format!(
                    "invalid canonical execution request id: {execution_request_id}"
                ),
            })?;
        let cancelled_at = Utc::now().to_rfc3339();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repository = ExecutionLifecycleRepository::new(&guard);
        let inserted = repository
            .record_cancelled(execution_request_id, suggestion_id, &cancelled_at)
            .map_err(|source| KernelError::ExecutionLifecyclePersistence {
                stage: "cancellation",
                source,
            })?;
        if !inserted {
            return match repository.get(execution_request_id).map_err(|source| {
                KernelError::ExecutionLifecyclePersistence {
                    stage: "cancellation.read_existing",
                    source,
                }
            })? {
                Some(existing) if existing.state == ExecutionState::Completed => {
                    Err(KernelError::CannotCancelCompletedExecution {
                        execution_request_id: execution_request_id.into(),
                    })
                }
                Some(existing) if existing.state == ExecutionState::InProgress => {
                    Err(KernelError::ExecutionInProgress {
                        execution_request_id: execution_request_id.into(),
                    })
                }
                Some(_) => Err(KernelError::ExecutionCancellationValidation {
                    message: format!(
                        "execution request '{execution_request_id}' is already cancelled"
                    ),
                }),
                None => Err(KernelError::IntegrityViolation {
                    message: "execution cancellation conflicted but no lifecycle row exists".into(),
                }),
            };
        }
        repository
            .get(execution_request_id)
            .map_err(|source| KernelError::ExecutionLifecyclePersistence {
                stage: "cancellation.read",
                source,
            })?
            .ok_or_else(|| KernelError::IntegrityViolation {
                message: format!(
                    "cancelled execution lifecycle row disappeared: {execution_request_id}"
                ),
            })
    }

    pub fn get(
        db: &Arc<Mutex<Database>>,
        execution_request_id: &str,
    ) -> Result<Option<ExecutionLifecycleRecord>> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repository = ExecutionLifecycleRepository::new(&guard);
        let record = repository
            .get(execution_request_id)
            .map_err(|source| KernelError::ExecutionLifecyclePersistence {
                stage: "read",
                source,
            })?;
        if let Some(record) = record.as_ref() {
            if Self::is_stale(record)? {
                repository
                    .mark_failed(
                        execution_request_id,
                        false,
                        "stale execution claim; dispatch outcome unknown",
                        &Utc::now().to_rfc3339(),
                    )
                    .map_err(|source| KernelError::ExecutionLifecyclePersistence {
                        stage: "stale_reconciliation",
                        source,
                    })?;
                return repository.get(execution_request_id).map_err(|source| {
                    KernelError::ExecutionLifecyclePersistence {
                        stage: "stale_reconciliation.read",
                        source,
                    }
                });
            }
        }
        Ok(record)
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

    pub fn lifecycle_outcome(record: &ExecutionLifecycleRecord) -> Result<Option<ExecutionOutcome>> {
        if record.state == ExecutionState::InProgress {
            return Ok(None);
        }
        record.validate().map_err(|error| {
            KernelError::ExecutionReconciliationValidation {
                message: error.to_string(),
            }
        })?;
        let status = match record.state {
            ExecutionState::Completed => ExecutionOutcomeStatus::Completed,
            ExecutionState::Failed => ExecutionOutcomeStatus::Failed,
            ExecutionState::Cancelled => ExecutionOutcomeStatus::Cancelled,
            _ => return Ok(None),
        };
        Ok(Some(ExecutionOutcome {
            execution_request_id: record.execution_request_id.clone(),
            status,
            command_name: "ExecuteIntentRequest".into(),
            completed_at: record.completed_at.clone().unwrap_or_else(|| record.updated_at.clone()),
            success: status == ExecutionOutcomeStatus::Completed,
            failure_reason: record.failure_reason.clone(),
            suggestion_id: Some(record.suggestion_id.clone()),
            intent_id: record.intent_id.clone(),
        }))
    }

    pub fn reconciliation(record: &ExecutionLifecycleRecord) -> ExecutionReconciliation {
        reconcile_execution_lifecycle(record)
    }

    fn is_stale(record: &ExecutionLifecycleRecord) -> Result<bool> {
        if record.state != ExecutionState::InProgress {
            return Ok(false);
        }
        let claimed_at = DateTime::parse_from_rfc3339(&record.claimed_at).map_err(|_| {
            KernelError::IntegrityViolation {
                message: format!(
                    "invalid execution claim timestamp: {}",
                    record.execution_request_id
                ),
            }
        })?;
        Ok(Utc::now()
            .signed_duration_since(claimed_at.with_timezone(&Utc))
            .num_seconds()
            >= 300)
    }
}
