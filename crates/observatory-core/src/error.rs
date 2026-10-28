//! Error taxonomy and process exit codes.
//!
//! Every fallible operation in the workspace returns [`ObservatoryError`]. Each
//! variant carries a stable machine-readable [`ErrorCode`] and maps to a stable
//! process [`ExitCode`] so scripts and CI can branch on failure kind.

use std::fmt;

/// Stable, machine-readable error codes emitted in JSON error reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    /// Invalid user input or arguments.
    InvalidInput,
    /// Filesystem or I/O failure.
    Io,
    /// A decode/parse failure on otherwise valid input.
    Decode,
    /// A well-formed input uses a feature the tool does not support.
    Unsupported,
    /// A malformed or invalid WASM module.
    Wasm,
    /// A malformed or invalid contract specification.
    Spec,
    /// An RPC transport or protocol failure.
    Rpc,
    /// Verification failed or was inconclusive.
    Verification,
    /// An internal invariant was violated.
    Internal,
}

impl ErrorCode {
    /// The stable string form used in JSON output.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorCode::InvalidInput => "invalid_input",
            ErrorCode::Io => "io",
            ErrorCode::Decode => "decode",
            ErrorCode::Unsupported => "unsupported",
            ErrorCode::Wasm => "wasm",
            ErrorCode::Spec => "spec",
            ErrorCode::Rpc => "rpc",
            ErrorCode::Verification => "verification",
            ErrorCode::Internal => "internal",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A stable classification of JSON-RPC failures.
///
/// The kind lets callers branch on the failure without parsing the message.
/// Codes outside the standard JSON-RPC ranges are preserved verbatim on the
/// accompanying [`RpcError`] and classified as [`RpcErrorKind::Unknown`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RpcErrorKind {
    /// The request never produced a JSON-RPC response (transport or I/O).
    Transport,
    /// The server could not parse the request (`-32700`).
    Parse,
    /// The request envelope was not valid JSON-RPC (`-32600`).
    InvalidRequest,
    /// The requested method does not exist (`-32601`).
    MethodNotFound,
    /// The method parameters were invalid (`-32602`).
    InvalidParams,
    /// The server hit an internal error (`-32603`).
    Internal,
    /// A server-defined error in the reserved `-32000..=-32099` range.
    Server,
    /// Any other provider-specific code.
    Unknown,
}

impl RpcErrorKind {
    /// Classify a raw JSON-RPC error code.
    #[must_use]
    pub fn from_code(code: i64) -> Self {
        match code {
            -32700 => RpcErrorKind::Parse,
            -32600 => RpcErrorKind::InvalidRequest,
            -32601 => RpcErrorKind::MethodNotFound,
            -32602 => RpcErrorKind::InvalidParams,
            -32603 => RpcErrorKind::Internal,
            -32099..=-32000 => RpcErrorKind::Server,
            _ => RpcErrorKind::Unknown,
        }
    }

    /// The stable string form used in JSON output.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            RpcErrorKind::Transport => "transport",
            RpcErrorKind::Parse => "parse",
            RpcErrorKind::InvalidRequest => "invalid_request",
            RpcErrorKind::MethodNotFound => "method_not_found",
            RpcErrorKind::InvalidParams => "invalid_params",
            RpcErrorKind::Internal => "internal",
            RpcErrorKind::Server => "server",
            RpcErrorKind::Unknown => "unknown",
        }
    }
}

impl fmt::Display for RpcErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A typed JSON-RPC error.
///
/// The raw code is kept exactly as the server sent it so callers can log or
/// branch on provider-specific values even when it is not a known standard
/// code.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct RpcError {
    kind: RpcErrorKind,
    code: Option<i64>,
    message: String,
}

impl RpcError {
    /// Build an error from an explicit kind, raw code, and message.
    #[must_use]
    pub fn new(kind: RpcErrorKind, code: Option<i64>, message: impl Into<String>) -> Self {
        RpcError {
            kind,
            code,
            message: message.into(),
        }
    }

    /// Build an error from a raw JSON-RPC code, classifying it automatically.
    #[must_use]
    pub fn from_code(code: i64, message: impl Into<String>) -> Self {
        RpcError::new(RpcErrorKind::from_code(code), Some(code), message)
    }

    /// Build an error from an optional raw JSON-RPC code.
    ///
    /// A missing code is preserved as `None` and classified as
    /// [`RpcErrorKind::Unknown`] rather than being invented.
    #[must_use]
    pub fn from_optional_code(code: Option<i64>, message: impl Into<String>) -> Self {
        match code {
            Some(code) => RpcError::from_code(code, message),
            None => RpcError::new(RpcErrorKind::Unknown, None, message),
        }
    }

    /// Build a transport error that has no JSON-RPC code.
    #[must_use]
    pub fn transport(message: impl Into<String>) -> Self {
        RpcError::new(RpcErrorKind::Transport, None, message)
    }

    /// The stable failure kind.
    #[must_use]
    pub fn kind(&self) -> RpcErrorKind {
        self.kind
    }

    /// The raw JSON-RPC code exactly as received, when one was present.
    #[must_use]
    pub fn code(&self) -> Option<i64> {
        self.code
    }

    /// The raw JSON-RPC code, preserved verbatim.
    ///
    /// Alias for [`RpcError::code`] for callers that want the intent explicit.
    #[must_use]
    pub fn raw_code(&self) -> Option<i64> {
        self.code
    }

    /// The human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            Some(code) => write!(f, "{} (code {code})", self.message),
            None => f.write_str(&self.message),
        }
    }
}

/// Stable process exit codes.
///
/// These are part of the public contract with CI users and must not change
/// without a documented deprecation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExitCode {
    /// Success.
    Success = 0,
    /// An unexpected internal failure.
    Internal = 1,
    /// Usage error (clap also uses this).
    Usage = 2,
    /// Input or filesystem error.
    Input = 3,
    /// A parse/decode error.
    Parse = 4,
    /// A requested feature is unsupported.
    Unsupported = 5,
    /// An interface change was classified as incompatible.
    Incompatible = 6,
    /// Verification found a mismatch.
    Mismatch = 7,
    /// Verification could not be completed with available data.
    InsufficientData = 8,
}

impl ExitCode {
    /// The numeric code.
    #[must_use]
    pub fn code(self) -> u8 {
        self as u8
    }
}

/// The workspace-wide error type.
#[derive(Debug, thiserror::Error)]
pub enum ObservatoryError {
    /// Invalid user input.
    #[error("invalid input: {0}")]
    InvalidInput(String),
    /// Filesystem or I/O failure.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// Decode/parse failure.
    #[error("decode error: {0}")]
    Decode(String),
    /// A well-formed input using an unsupported feature.
    #[error("unsupported: {0}")]
    Unsupported(String),
    /// Malformed or invalid WASM.
    #[error("wasm error: {0}")]
    Wasm(String),
    /// Malformed or invalid contract spec.
    #[error("spec error: {0}")]
    Spec(String),
    /// RPC failure with a typed kind and preserved raw code.
    #[error("rpc error: {0}")]
    Rpc(RpcError),
    /// Failed or inconclusive verification.
    #[error("verification error: {0}")]
    Verification(String),
    /// Internal invariant violation.
    #[error("internal error: {0}")]
    Internal(String),
}

impl ObservatoryError {
    /// Construct an invalid-input error.
    pub fn invalid(message: impl Into<String>) -> Self {
        ObservatoryError::InvalidInput(message.into())
    }

    /// Construct a decode error.
    pub fn decode(message: impl Into<String>) -> Self {
        ObservatoryError::Decode(message.into())
    }

    /// Construct an unsupported-feature error.
    pub fn unsupported(message: impl Into<String>) -> Self {
        ObservatoryError::Unsupported(message.into())
    }

    /// Construct a WASM error.
    pub fn wasm(message: impl Into<String>) -> Self {
        ObservatoryError::Wasm(message.into())
    }

    /// Construct a spec error.
    pub fn spec(message: impl Into<String>) -> Self {
        ObservatoryError::Spec(message.into())
    }

    /// Construct a transport-level RPC error with no JSON-RPC code.
    pub fn rpc(message: impl Into<String>) -> Self {
        ObservatoryError::Rpc(RpcError::transport(message))
    }

    /// Construct an RPC error from an already-typed [`RpcError`].
    pub fn from_rpc_error(error: RpcError) -> Self {
        ObservatoryError::Rpc(error)
    }

    /// Construct a verification error.
    pub fn verification(message: impl Into<String>) -> Self {
        ObservatoryError::Verification(message.into())
    }

    /// Construct an internal error.
    pub fn internal(message: impl Into<String>) -> Self {
        ObservatoryError::Internal(message.into())
    }

    /// The stable machine-readable error code.
    #[must_use]
    pub fn code(&self) -> ErrorCode {
        match self {
            ObservatoryError::InvalidInput(_) => ErrorCode::InvalidInput,
            ObservatoryError::Io(_) => ErrorCode::Io,
            ObservatoryError::Decode(_) => ErrorCode::Decode,
            ObservatoryError::Unsupported(_) => ErrorCode::Unsupported,
            ObservatoryError::Wasm(_) => ErrorCode::Wasm,
            ObservatoryError::Spec(_) => ErrorCode::Spec,
            ObservatoryError::Rpc(_) => ErrorCode::Rpc,
            ObservatoryError::Verification(_) => ErrorCode::Verification,
            ObservatoryError::Internal(_) => ErrorCode::Internal,
        }
    }

    /// The typed RPC error, when this is an RPC failure.
    #[must_use]
    pub fn rpc_error(&self) -> Option<&RpcError> {
        match self {
            ObservatoryError::Rpc(error) => Some(error),
            _ => None,
        }
    }

    /// The process exit code that best represents this failure.
    #[must_use]
    pub fn exit_code(&self) -> ExitCode {
        match self {
            ObservatoryError::InvalidInput(_) => ExitCode::Usage,
            ObservatoryError::Io(_) => ExitCode::Input,
            ObservatoryError::Decode(_) => ExitCode::Parse,
            ObservatoryError::Unsupported(_) => ExitCode::Unsupported,
            ObservatoryError::Wasm(_) | ObservatoryError::Spec(_) => ExitCode::Parse,
            ObservatoryError::Rpc(_) => ExitCode::Input,
            ObservatoryError::Verification(_) => ExitCode::InsufficientData,
            ObservatoryError::Internal(_) => ExitCode::Internal,
        }
    }
}

/// Convenience alias used throughout the workspace.
pub type Result<T> = std::result::Result<T, ObservatoryError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes_are_stable_strings() {
        assert_eq!(ObservatoryError::wasm("x").code().as_str(), "wasm");
        assert_eq!(
            ObservatoryError::invalid("x").code(),
            ErrorCode::InvalidInput
        );
    }

    #[test]
    fn exit_codes_map_sensibly() {
        assert_eq!(ObservatoryError::invalid("x").exit_code(), ExitCode::Usage);
        assert_eq!(
            ObservatoryError::verification("x").exit_code(),
            ExitCode::InsufficientData
        );
    }

    #[test]
    fn io_errors_convert() {
        let err: ObservatoryError =
            std::io::Error::new(std::io::ErrorKind::NotFound, "missing").into();
        assert_eq!(err.code(), ErrorCode::Io);
    }

    #[test]
    fn rpc_error_kinds_cover_standard_codes() {
        assert_eq!(RpcErrorKind::from_code(-32700), RpcErrorKind::Parse);
        assert_eq!(
            RpcErrorKind::from_code(-32600),
            RpcErrorKind::InvalidRequest
        );
        assert_eq!(
            RpcErrorKind::from_code(-32601),
            RpcErrorKind::MethodNotFound
        );
        assert_eq!(RpcErrorKind::from_code(-32602), RpcErrorKind::InvalidParams);
        assert_eq!(RpcErrorKind::from_code(-32603), RpcErrorKind::Internal);
        assert_eq!(RpcErrorKind::from_code(-32000), RpcErrorKind::Server);
        assert_eq!(RpcErrorKind::from_code(-32099), RpcErrorKind::Server);
        assert_eq!(RpcErrorKind::from_code(-32100), RpcErrorKind::Unknown);
    }

    #[test]
    fn rpc_error_preserves_unknown_raw_code() {
        let error = RpcError::from_code(-12345, "provider-specific failure");
        assert_eq!(error.kind(), RpcErrorKind::Unknown);
        assert_eq!(error.kind().as_str(), "unknown");
        assert_eq!(error.code(), Some(-12345));
        assert_eq!(error.raw_code(), Some(-12345));
        assert_eq!(error.message(), "provider-specific failure");
    }

    #[test]
    fn rpc_error_without_code_is_not_invented() {
        let error = RpcError::from_optional_code(None, "no code");
        assert_eq!(error.kind(), RpcErrorKind::Unknown);
        assert_eq!(error.code(), None);
        assert_eq!(error.to_string(), "no code");
    }

    #[test]
    fn rpc_error_display_includes_code() {
        let error = ObservatoryError::from_rpc_error(RpcError::from_code(
            -32601,
            "getNetwork failed: method not found",
        ));
        assert_eq!(error.code(), ErrorCode::Rpc);
        assert!(error.to_string().contains("method not found"));
        assert!(error.to_string().contains("-32601"));
        assert_eq!(
            error.rpc_error().map(RpcError::kind),
            Some(RpcErrorKind::MethodNotFound)
        );
    }
}
