//! Governed personalization service — explicit preferences only (Sprints 64–65).
//!
//! Never calls Permission Gateway decision APIs, grant mutators, or launchers.
//! Personalization may influence planning ranking/explanations only.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, UserPreferenceRepository};
use workspace_domain::{
    ActorContext, AiPersonalizationAwareness, IntentContext, PreferenceCategory, PreferenceSource,
    UserPreference, UserPreferenceId, UserPreferenceProfile, WorkspaceId,
};

use crate::error::{KernelError, Result};
use crate::services::{ConfigurationService, AuditService};

pub(crate) struct AiPersonalizationService;

impl AiPersonalizationService {
    pub(crate) fn create(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        category: PreferenceCategory,
        key: impl Into<String>,
        value: impl Into<String>,
        source: PreferenceSource,
        workspace_id: Option<String>,
        label: Option<String>,
        attributes: Option<String>,
    ) -> Result<UserPreference> {
        let workspace_id = workspace_id
            .map(WorkspaceId::new)
            .transpose()
            .map_err(KernelError::Domain)?;
        let preference = UserPreference::new(
            category,
            key,
            value,
            source,
            workspace_id,
            label,
            attributes,
        )
        .map_err(KernelError::from)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            UserPreferenceRepository::new(&guard).upsert(&preference)?;
        }

        Self::audit(db, actor, "ai.personalization.created", &preference)?;
        Ok(preference)
    }

    pub(crate) fn update(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        id: impl Into<String>,
        value: Option<String>,
        label: Option<Option<String>>,
        attributes: Option<Option<String>>,
        confidence: Option<u8>,
    ) -> Result<UserPreference> {
        let id = UserPreferenceId::new(id.into()).map_err(KernelError::Domain)?;
        let preference = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            let repo = UserPreferenceRepository::new(&guard);
            let mut preference = repo.get(&id)?.ok_or_else(|| {
                KernelError::AiPersonalizationValidation {
                    message: format!("preference not found: {}", id.as_str()),
                }
            })?;
            preference
                .apply_update(value, label, attributes, confidence)
                .map_err(KernelError::from)?;
            repo.upsert(&preference)?;
            preference
        };
        Self::audit(db, actor, "ai.personalization.updated", &preference)?;
        Ok(preference)
    }

    pub(crate) fn list_active(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: Option<&str>,
        limit: usize,
    ) -> Result<Vec<UserPreference>> {
        let preferences = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            UserPreferenceRepository::new(&guard).list_active(workspace_id, limit)?
        };
        let enabled = Self::is_enabled(db)?;
        let metadata = json!({
            "workspace_id": workspace_id,
            "preference_count": preferences.len(),
            "personalization_enabled": enabled,
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "ai.personalization.used",
            true,
            metadata,
        )?;
        Ok(preferences)
    }

    pub(crate) fn get_profile(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: Option<&str>,
        limit: usize,
    ) -> Result<UserPreferenceProfile> {
        let preferences = Self::list_active(db, actor, workspace_id, limit)?;
        let enabled = Self::is_enabled(db)?;
        Ok(UserPreferenceProfile::from_preferences(preferences, enabled))
    }

    pub(crate) fn assemble_awareness(
        db: &Arc<Mutex<Database>>,
        workspace_id: Option<&str>,
        limit: usize,
        force_enabled: Option<bool>,
    ) -> Result<AiPersonalizationAwareness> {
        let enabled = match force_enabled {
            Some(value) => value,
            None => Self::is_enabled(db)?,
        };
        if !enabled {
            return Ok(AiPersonalizationAwareness::empty_disabled());
        }
        let preferences = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            UserPreferenceRepository::new(&guard).list_active(workspace_id, limit)?
        };
        Ok(AiPersonalizationAwareness::from_preferences(
            preferences,
            true,
        ))
    }

    pub(crate) fn delete(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        id: impl Into<String>,
    ) -> Result<UserPreference> {
        let id = UserPreferenceId::new(id.into()).map_err(KernelError::Domain)?;
        let updated_at = Utc::now().to_rfc3339();
        let preference = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            let repo = UserPreferenceRepository::new(&guard);
            let mut preference = repo.get(&id)?.ok_or_else(|| {
                KernelError::AiPersonalizationValidation {
                    message: format!("preference not found: {}", id.as_str()),
                }
            })?;
            if !repo.soft_delete(&id, &updated_at)? {
                return Err(KernelError::AiPersonalizationValidation {
                    message: format!("preference already deleted: {}", id.as_str()),
                });
            }
            preference.mark_deleted();
            preference.updated_at = updated_at;
            preference
        };
        Self::audit(db, actor, "ai.personalization.deleted", &preference)?;
        Ok(preference)
    }

    pub(crate) fn set_enabled(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        enabled: bool,
    ) -> Result<bool> {
        let update = crate::config::SettingsUpdate {
            personalization_enabled: Some(enabled),
            ..Default::default()
        };
        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
            ConfigurationService::update(&guard, update)?;
        }
        let metadata = json!({
            "personalization_enabled": enabled,
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "ai.personalization.updated",
            true,
            metadata,
        )?;
        Ok(enabled)
    }

    pub(crate) fn is_enabled(db: &Arc<Mutex<Database>>) -> Result<bool> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::Config("database lock poisoned".into()))?;
        Ok(ConfigurationService::load(&guard)?.personalization_enabled)
    }

    pub(crate) fn audit_used_in_planning(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        awareness: &AiPersonalizationAwareness,
    ) -> Result<()> {
        let metadata = json!({
            "enabled": awareness.enabled,
            "preference_count": awareness.preferences.len(),
            "preferred_application_ids": awareness.preferred_application_ids(),
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::ai_suggestion(),
            "ai.personalization.used",
            true,
            metadata,
        )
    }

    fn audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event_type: &str,
        preference: &UserPreference,
    ) -> Result<()> {
        let metadata = json!({
            "preference_id": preference.id.as_str(),
            "category": preference.category.as_str(),
            "key": preference.key,
            "source": preference.source.as_str(),
            "workspace_id": preference.scope_workspace_id.as_ref().map(|id| id.as_str()),
            "confidence": preference.confidence,
            "authority_effect": "none",
        })
        .to_string();
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            event_type,
            true,
            metadata,
        )
    }
}
