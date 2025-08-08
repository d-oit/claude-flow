use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use regex::Regex;
use std::path::Path;

pub struct NonProductionAnalyzer {
    // Patterns for detecting non-production code
    todo_pattern: Regex,
    debug_pattern: Regex,
    console_pattern: Regex,
}

impl NonProductionAnalyzer {
    pub fn new() -> Self {
        Self {
            todo_pattern: Regex::new(r"(?i)(todo|fixme|hack|xxx|bug)").unwrap(),
            debug_pattern: Regex::new(r"(?i)(debug|debugger|console\.log|print\(|println!)").unwrap(),
            console_pattern: Regex::new(r"console\.(log|debug|info|warn|error)").unwrap(),
        }
    }
    
    fn check_non_production_code(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        let content_str = String::from_utf8_lossy(content);
        
        for (line_num, line) in content_str.lines().enumerate() {
            let line_number = (line_num + 1) as u32;
            
            // Check for TODO/FIXME comments
            if let Some(captures) = self.todo_pattern.captures(line) {
                let keyword = captures.get(1).unwrap().as_str();
                let severity = match keyword.to_lowercase().as_str() {
                    "bug" | "xxx" => Severity::High,
                    "fixme" | "hack" => Severity::Medium,
                    "todo" => Severity::Low,
                    _ => Severity::Info,
                };
                
                findings.push(
                    Finding::new(
                        "non_production",
                        "todo_comment",
                        severity,
                        file_path.to_path_buf(),
                        line_number,
                        format!("{} comment found", keyword.to_uppercase()),
                    )
                    .with_description(format!("Line contains a {} comment that should be resolved before production", keyword))
                    .with_suggestion("Resolve the issue or create a proper issue tracker entry".to_string())
                    .with_metadata("comment_type".to_string(), serde_json::Value::String(keyword.to_string()))
                );
            }
            
            // Check for debug statements
            if self.debug_pattern.is_match(line) {
                let severity = if line.contains("debugger") {
                    Severity::High
                } else {
                    Severity::Medium
                };
                
                findings.push(
                    Finding::new(
                        "non_production",
                        "debug_statement",
                        severity,
                        file_path.to_path_buf(),
                        line_number,
                        "Debug statement found".to_string(),
                    )
                    .with_description("Debug statements should not be present in production code".to_string())
                    .with_suggestion("Remove debug statements or replace with proper logging".to_string())
                );
            }
            
            // Check for console statements in JavaScript/TypeScript
            if self.is_js_ts_file(file_path) && self.console_pattern.is_match(line) {
                findings.push(
                    Finding::new(
                        "non_production",
                        "console_statement",
                        Severity::Low,
                        file_path.to_path_buf(),
                        line_number,
                        "Console statement found".to_string(),
                    )
                    .with_description("Console statements should be replaced with proper logging in production".to_string())
                    .with_suggestion("Use a proper logging library instead of console statements".to_string())
                );
            }
            
            // Check for hardcoded credentials or secrets
            if self.contains_potential_secret(line) {
                findings.push(
                    Finding::new(
                        "non_production",
                        "potential_secret",
                        Severity::Critical,
                        file_path.to_path_buf(),
                        line_number,
                        "Potential hardcoded secret detected".to_string(),
                    )
                    .with_description("Line may contain hardcoded credentials or API keys".to_string())
                    .with_suggestion("Move secrets to environment variables or secure configuration".to_string())
                );
            }
        }
        
        Ok(findings)
    }
    
    fn is_js_ts_file(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(ext, "js" | "ts" | "jsx" | "tsx")
        } else {
            false
        }
    }
    
    fn contains_potential_secret(&self, line: &str) -> bool {
        let line_lower = line.to_lowercase();
        
        // Look for patterns that might indicate secrets
        let secret_indicators = [
            "password", "secret", "token", "api_key", "apikey",
            "private_key", "access_key", "auth_token"
        ];
        
        for indicator in &secret_indicators {
            if line_lower.contains(indicator) && line.contains("=") {
                // Check if it looks like an assignment with a non-placeholder value
                if let Some(value_part) = line.split('=').nth(1) {
                    let value = value_part.trim().trim_matches('"').trim_matches('\'');
                    // Skip obvious placeholders
                    if !value.is_empty() && 
                       !value.contains("your_") && 
                       !value.contains("placeholder") &&
                       !value.contains("example") &&
                       value.len() > 8 {
                        return true;
                    }
                }
            }
        }
        
        false
    }
}

impl Analyzer for NonProductionAnalyzer {
    fn name(&self) -> &str {
        "non_production"
    }
    
    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        self.check_non_production_code(file_path, content)
    }
    
    fn supports_file(&self, file_path: &Path) -> bool {
        // Support common source code file types
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(ext, 
                "rs" | "js" | "ts" | "jsx" | "tsx" | "py" | "java" | 
                "cpp" | "c" | "h" | "hpp" | "go" | "rb" | "php" |
                "cs" | "swift" | "kt" | "scala" | "clj" | "hs"
            )
        } else {
            false
        }
    }
}