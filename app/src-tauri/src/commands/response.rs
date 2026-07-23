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
}
