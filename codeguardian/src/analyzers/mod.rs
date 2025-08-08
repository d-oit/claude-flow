pub mod integrity;
pub mod lint_drift;
pub mod non_production;

use crate::types::{AnalysisResults, Finding};
use anyhow::Result;
use std::path::Path;

pub trait Analyzer {
    fn name(&self) -> &str;
    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>>;
    fn supports_file(&self, file_path: &Path) -> bool;
}

pub struct AnalyzerRegistry {
    analyzers: Vec<Box<dyn Analyzer + Send + Sync>>,
}

impl AnalyzerRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            analyzers: Vec::new(),
        };
        
        // Register default analyzers
        registry.register(Box::new(integrity::IntegrityAnalyzer::new()));
        registry.register(Box::new(lint_drift::LintDriftAnalyzer::new()));
        registry.register(Box::new(non_production::NonProductionAnalyzer::new()));
        
        registry
    }
    
    pub fn register(&mut self, analyzer: Box<dyn Analyzer + Send + Sync>) {
        self.analyzers.push(analyzer);
    }
    
    pub fn analyze_file(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let mut all_findings = Vec::new();
        
        for analyzer in &self.analyzers {
            if analyzer.supports_file(file_path) {
                let findings = analyzer.analyze(file_path, content)?;
                all_findings.extend(findings);
            }
        }
        
        Ok(all_findings)
    }
}