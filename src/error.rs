//! Error types for the cadlib bindings.

use serde::Deserialize;

/// An error returned by the cadlib native library, or by the bindings layer.
#[derive(Debug, Clone)]
pub enum CadLibError {
    /// A known, anticipated CAD error: a machine-readable `error_code` plus
    /// structured `error_details`.
    Expected {
        /// Machine-readable error identifier.
        error_code: String,
        /// Structured context for the error (shape depends on `error_code`).
        error_details: serde_json::Value,
    },
    /// An unexpected internal error from the library, or a bindings-side failure
    /// (parameter marshalling, result decoding, or an interior NUL byte).
    Unexpected {
        /// Human-readable description.
        message: String,
    },
}

impl std::fmt::Display for CadLibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expected { error_code, .. } => write!(f, "expected cad lib error: {error_code}"),
            Self::Unexpected { message } => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for CadLibError {}

/// Mirrors the JSON error payload.
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum CadErrorPayload {
    Expected {
        error_code: String,
        #[serde(default)]
        error_details: serde_json::Value,
    },
    Unexpected {
        message: String,
    },
}

/// Decode the JSON error payload from the FFI into a [`CadLibError`]. Falls back
/// to [`CadLibError::Unexpected`] when the message is not the expected JSON shape.
pub(crate) fn parse_cad_error(raw: &str) -> CadLibError {
    match serde_json::from_str::<CadErrorPayload>(raw) {
        Ok(CadErrorPayload::Expected {
            error_code,
            error_details,
        }) => CadLibError::Expected {
            error_code,
            error_details,
        },
        Ok(CadErrorPayload::Unexpected { message }) => CadLibError::Unexpected { message },
        Err(_) => CadLibError::Unexpected {
            message: raw.to_string(),
        },
    }
}
