use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use std::path::Path;

pub struct LintDriftAnalyzer {
    // Configuration for lint drift detection
}

impl LintDriftAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
    
    fn check_config_drift(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        let content_str = String::from_utf8_lossy(content);
        
        // Check for missing common configuration files
        if self.is_config_file(file_path) {
            findings.extend(self.analyze_config_content(file_path, &content_str)?);
        }
        
        // Check for inconsistent formatting in config files
        if self.is_json_config(file_path) {
            findings.extend(self.check_json_formatting(file_path, &content_str)?);
        }
        
        if self.is_yaml_config(file_path) {
            findings.extend(self.check_yaml_formatting(file_path, &content_str)?);
        }
        
        Ok(findings)
    }
    
    fn is_config_file(&self, file_path: &Path) -> bool {
        if let Some(name) = file_path.file_name().and_then(|n| n.to_str()) {
            matches!(name, 
                ".eslintrc.json" | ".eslintrc.js" | ".eslintrc.yml" |
                ".prettierrc" | ".prettierrc.json" | ".prettierrc.yml" |
                "tsconfig.json" | "jsconfig.json" |
                "Cargo.toml" | "package.json" |
                ".gitignore" | ".dockerignore"
            )
        } else {
            false
        }
    }
    
    fn is_json_config(&self, file_path: &Path) -> bool {
        file_path.extension().and_then(|e| e.to_str()) == Some("json") ||
        file_path.file_name().and_then(|n| n.to_str()).map_or(false, |n| n.contains(".json"))
    }
    
    fn is_yaml_config(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(ext, "yml" | "yaml")
        } else {
            false
        }
    }
    
    fn analyze_config_content(&self, file_path: &Path, content: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        
        // Check for TODO/FIXME in config files
        for (line_num, line) in content.lines().enumerate() {
            if line.to_lowercase().contains("todo") || line.to_lowercase().contains("fixme") {
                findings.push(
                    Finding::new(
                        "lint_drift",
                        "config_todo",
                        Severity::Low,
                        file_path.to_path_buf(),
                        (line_num + 1) as u32,
                        "Configuration file contains TODO/FIXME comment".to_string(),
                    )
                    .with_description("Configuration files should be production-ready".to_string())
                    .with_suggestion("Resolve the TODO/FIXME or move to documentation".to_string())
                );
            }
        }
        
        Ok(findings)
    }
    
    fn check_json_formatting(&self, file_path: &Path, content: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        
        // Try to parse and reformat JSON to check for formatting issues
        match serde_json::from_str::<serde_json::Value>(content) {
            Ok(value) => {
                let formatted = serde_json::to_string_pretty(&value)?;
                if content.trim() != formatted.trim() {
                    findings.push(
                        Finding::new(
                            "lint_drift",
                            "json_formatting",
                            Severity::Low,
                            file_path.to_path_buf(),
                            1,
                            "JSON file is not consistently formatted".to_string(),
                        )
                        .with_description("JSON formatting is inconsistent with standard pretty-printing".to_string())
                        .with_suggestion("Run a JSON formatter to standardize formatting".to_string())
                    );
                }
            }
            Err(_) => {
                findings.push(
                    Finding::new(
                        "lint_drift",
                        "invalid_json",
                        Severity::High,
                        file_path.to_path_buf(),
                        1,
                        "Invalid JSON syntax".to_string(),
                    )
                    .with_description("File appears to be JSON but contains syntax errors".to_string())
                    .with_suggestion("Fix JSON syntax errors".to_string())
                );
            }
        }
        
        Ok(findings)
    }
    
    fn check_yaml_formatting(&self, file_path: &Path, content: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        
        // Basic YAML validation
        match serde_yaml::from_str::<serde_yaml::Value>(content) {
            Ok(_) => {
                // Check for tabs (YAML should use spaces)
                if content.contains('\t') {
                    findings.push(
                        Finding::new(
                            "lint_drift",
                            "yaml_tabs",
                            Severity::Medium,
                            file_path.to_path_buf(),
                            1,
                            "YAML file contains tabs instead of spaces".to_string(),
                        )
                        .with_description("YAML specification requires spaces for indentation".to_string())
                        .with_suggestion("Replace tabs with spaces (typically 2 spaces per indent level)".to_string())
                    );
                }
            }
            Err(_) => {
                findings.push(
                    Finding::new(
                        "lint_drift",
                        "invalid_yaml",
                        Severity::High,
                        file_path.to_path_buf(),
                        1,
                        "Invalid YAML syntax".to_string(),
                    )
                    .with_description("File appears to be YAML but contains syntax errors".to_string())
                    .with_suggestion("Fix YAML syntax errors".to_string())
                );
            }
        }
        
        Ok(findings)
    }
}

impl Analyzer for LintDriftAnalyzer {
    fn name(&self) -> &str {
        "lint_drift"
    }
    
    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        self.check_config_drift(file_path, content)
    }
    
    fn supports_file(&self, file_path: &Path) -> bool {
        self.is_config_file(file_path)
    }
}