use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

/// Schema version for results format
pub const RESULTS_SCHEMA_VERSION: &str = "1.0.0";

/// Stable finding ID generation using SHA-256
pub fn generate_finding_id(analyzer: &str, rule: &str, file: &str, line: u32, key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(analyzer.as_bytes());
    hasher.update(rule.as_bytes());
    hasher.update(file.as_bytes());
    hasher.update(line.to_le_bytes());
    hasher.update(key.as_bytes());
    format!("{:x}", hasher.finalize())[..16].to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResults {
    pub schema_version: String,
    pub tool_metadata: ToolMetadata,
    pub findings: Vec<Finding>,
    pub summary: ResultsSummary,
    pub config_hash: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub name: String,
    pub version: String,
    pub config_hash: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub analyzer: String,
    pub rule: String,
    pub severity: Severity,
    pub file: PathBuf,
    pub line: u32,
    pub column: Option<u32>,
    pub message: String,
    pub description: Option<String>,
    pub suggestion: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultsSummary {
    pub total_files_scanned: usize,
    pub total_findings: usize,
    pub findings_by_severity: HashMap<Severity, usize>,
    pub findings_by_analyzer: HashMap<String, usize>,
    pub scan_duration_ms: u64,
}

impl AnalysisResults {
    pub fn new(config_hash: String) -> Self {
        Self {
            schema_version: RESULTS_SCHEMA_VERSION.to_string(),
            tool_metadata: ToolMetadata {
                name: "codeguardian".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                config_hash: config_hash.clone(),
                timestamp: Utc::now(),
            },
            findings: Vec::new(),
            summary: ResultsSummary {
                total_files_scanned: 0,
                total_findings: 0,
                findings_by_severity: HashMap::new(),
                findings_by_analyzer: HashMap::new(),
                scan_duration_ms: 0,
            },
            config_hash,
            timestamp: Utc::now(),
        }
    }

    pub fn add_finding(&mut self, finding: Finding) {
        *self.summary.findings_by_severity.entry(finding.severity.clone()).or_insert(0) += 1;
        *self.summary.findings_by_analyzer.entry(finding.analyzer.clone()).or_insert(0) += 1;
        self.summary.total_findings += 1;
        self.findings.push(finding);
    }

    pub fn sort_findings(&mut self) {
        // Deterministic ordering: severity → file → line
        self.findings.sort_by(|a, b| {
            a.severity.cmp(&b.severity)
                .then_with(|| a.file.cmp(&b.file))
                .then_with(|| a.line.cmp(&b.line))
        });
    }

    pub fn has_issues(&self) -> bool {
        !self.findings.is_empty()
    }

    pub fn has_high_severity_issues(&self) -> bool {
        self.findings.iter().any(|f| matches!(f.severity, Severity::Critical | Severity::High))
    }
}

impl Finding {
    pub fn new(
        analyzer: &str,
        rule: &str,
        severity: Severity,
        file: PathBuf,
        line: u32,
        message: String,
    ) -> Self {
        let id = generate_finding_id(analyzer, rule, &file.to_string_lossy(), line, &message);
        
        Self {
            id,
            analyzer: analyzer.to_string(),
            rule: rule.to_string(),
            severity,
            file,
            line,
            column: None,
            message,
            description: None,
            suggestion: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_column(mut self, column: u32) -> Self {
        self.column = Some(column);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Critical => write!(f, "critical"),
            Severity::High => write!(f, "high"),
            Severity::Medium => write!(f, "medium"),
            Severity::Low => write!(f, "low"),
            Severity::Info => write!(f, "info"),
        }
    }
}