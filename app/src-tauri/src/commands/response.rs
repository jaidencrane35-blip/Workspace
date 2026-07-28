use serde::Serialize;

use super::error::CommandError;

/// Standard IPC error payload.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct IpcErrorBody {
    pub code: String,
    pub message: String,
}

impl From<CommandError> for IpcErrorBody {
    fn from(error: CommandError) -> Self {
        Self {
            code: error.code,
            message: error.message,
        }
    }
}

/// Standard IPC response envelope (Sprint 03).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct IpcResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<IpcErrorBody>,
}

impl<T> IpcResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn failure(error: CommandError) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::error::CommandError;

    #[test]
    fn success_response_format() {
        let response = IpcResponse::success("ok");
        assert!(response.success);
        assert_eq!(response.data, Some("ok"));
        assert!(response.error.is_none());
    }

    #[test]
    fn failure_response_format() {
        let response = IpcResponse::<String>::failure(CommandError::new(
            "not_ready",
            "Workspace is still initializing.",
        ));

        assert!(!response.success);
        assert!(response.data.is_none());
        assert_eq!(
            response.error,
            Some(IpcErrorBody {
                code: "not_ready".into(),
                message: "Workspace is still initializing.".into(),
            })
        );
    }

    #[test]
    fn serialized_envelope_has_stable_success_and_failure_shapes() {
        let success = serde_json::to_value(IpcResponse::success("ok")).unwrap();
        assert_eq!(
            success,
            serde_json::json!({
                "success": true,
                "data": "ok"
            })
        );

        let failure = serde_json::to_value(IpcResponse::<String>::failure(CommandError::new(
            "not_ready",
            "Workspace is still initializing.",
        )))
        .unwrap();
        assert_eq!(
            failure,
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "not_ready",
                    "message": "Workspace is still initializing."
                }
            })
        );
    }

    #[test]
    fn unit_success_serializes_data_as_null() {
        let response = serde_json::to_value(IpcResponse::success(())).unwrap();
        assert_eq!(
            response,
            serde_json::json!({
                "success": true,
                "data": null
            })
        );
    }
}
