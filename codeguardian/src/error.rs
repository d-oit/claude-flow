use thiserror::Error;
use std::path::PathBuf;

/// Result type alias for CodeGuardian operations
pub type Result<T> = std::result::Result<T, GuardianError>;

/// Comprehensive error types for CodeGuardian
#[derive(Error, Debug)]
pub enum GuardianError {
    /// I/O related errors
    #[error("I/O error: {message}")]
    Io {
        message: String,
        #[source]
        source: std::io::Error,
    },

    /// Configuration errors
    #[error("Configuration error: {message}")]
    Config {
        message: String,
        file: Option<PathBuf>,
    },

    /// Security-related errors
    #[error("Security violation: {message}")]
    Security {
        message: String,
        severity: SecuritySeverity,
    },

    /// Analysis errors
    #[error("Analysis failed: {message}")]
    Analysis {
        message: String,
        analyzer: String,
        file: Option<PathBuf>,
    },

    /// Cryptographic errors
    #[error("Cryptographic operation failed: {message}")]
    Crypto {
        message: String,
        algorithm: String,
    },

    /// Memory allocation errors
    #[error("Memory allocation failed: {message}")]
    Memory {
        message: String,
        requested_size: Option<usize>,
    },

    /// Serialization errors
    #[error("Serialization error: {message}")]
    Serialization {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Permission errors
    #[error("Permission denied: {message}")]
    Permission {
        message: String,
        path: PathBuf,
    },

    /// Validation errors
    #[error("Validation failed: {message}")]
    Validation {
        message: String,
        field: Option<String>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for SecuritySeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecuritySeverity::Low => write!(f, "LOW"),
            SecuritySeverity::Medium => write!(f, "MEDIUM"),
            SecuritySeverity::High => write!(f, "HIGH"),
            SecuritySeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl GuardianError {
    /// Create an I/O error with context
    pub fn io(message: impl Into<String>, source: std::io::Error) -> Self {
        Self::Io {
            message: message.into(),
            source,
        }
    }

    /// Create a configuration error
    pub fn config(message: impl Into<String>, file: Option<PathBuf>) -> Self {
        Self::Config {
            message: message.into(),
            file,
        }
    }

    /// Create a security error
    pub fn security(message: impl Into<String>, severity: SecuritySeverity) -> Self {
        Self::Security {
            message: message.into(),
            severity,
        }
    }

    /// Create an analysis error
    pub fn analysis(
        message: impl Into<String>,
        analyzer: impl Into<String>,
        file: Option<PathBuf>,
    ) -> Self {
        Self::Analysis {
            message: message.into(),
            analyzer: analyzer.into(),
            file,
        }
    }

    /// Get the exit code for this error
    pub fn exit_code(&self) -> i32 {
        match self {
            GuardianError::Io { .. } => 3,
            GuardianError::Config { .. } => 4,
            GuardianError::Security { severity, .. } => match severity {
                SecuritySeverity::Critical => 5,
                SecuritySeverity::High => 6,
                SecuritySeverity::Medium => 7,
                SecuritySeverity::Low => 8,
            },
            GuardianError::Analysis { .. } => 9,
            GuardianError::Crypto { .. } => 10,
            GuardianError::Memory { .. } => 11,
            GuardianError::Serialization { .. } => 12,
            GuardianError::Permission { .. } => 13,
            GuardianError::Validation { .. } => 14,
        }
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            GuardianError::Io { .. } => true,
            GuardianError::Config { .. } => false,
            GuardianError::Security { severity, .. } => {
                matches!(severity, SecuritySeverity::Low | SecuritySeverity::Medium)
            }
            GuardianError::Analysis { .. } => true,
            GuardianError::Crypto { .. } => false,
            GuardianError::Memory { .. } => false,
            GuardianError::Serialization { .. } => true,
            GuardianError::Permission { .. } => true,
            GuardianError::Validation { .. } => true,
        }
    }
}

/// Convert from common error types
impl From<std::io::Error> for GuardianError {
    fn from(err: std::io::Error) -> Self {
        Self::io("I/O operation failed", err)
    }
}

impl From<serde_json::Error> for GuardianError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization {
            message: "JSON serialization failed".to_string(),
            source: Box::new(err),
        }
    }
}

impl From<toml::de::Error> for GuardianError {
    fn from(err: toml::de::Error) -> Self {
        Self::Serialization {
            message: "TOML deserialization failed".to_string(),
            source: Box::new(err),
        }
    }
}

impl From<toml::ser::Error> for GuardianError {
    fn from(err: toml::ser::Error) -> Self {
        Self::Serialization {
            message: "TOML serialization failed".to_string(),
            source: Box::new(err),
        }
    }
}