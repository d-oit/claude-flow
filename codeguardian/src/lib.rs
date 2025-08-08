//! # CodeGuardian
//!
//! A comprehensive code quality and security analysis tool that performs:
//! - Corrupted file detection via cryptographic hashing
//! - Lint configuration drift analysis
//! - Non-production code identification
//!
//! ## Features
//!
//! - **Memory Safe**: Zero-copy operations where possible, RAII resource management
//! - **Security Hardened**: Sandboxed file access, no sensitive data in logs
//! - **Robust Error Handling**: Structured errors with context-aware recovery
//! - **Excellent UX**: Progress bars, colorized output, multiple output formats

pub mod analyzers;
pub mod cache;
pub mod cli;
pub mod config;
pub mod core;
pub mod error;
pub mod github;
pub mod github_api;
pub mod ml;
pub mod report;
pub mod streaming;
pub mod types;
pub mod utils;

// Re-export commonly used types
pub use crate::{
    config::Config,
    core::GuardianEngine,
    types::*,
};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");