use crate::analyzers::Analyzer;
use crate::types::{Finding, Severity};
use anyhow::Result;
use blake3::Hasher;
use std::path::Path;

pub struct IntegrityAnalyzer {
    // Configuration for integrity checking
}

impl IntegrityAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
    
    fn compute_hash(&self, content: &[u8]) -> String {
        let mut hasher = Hasher::new();
        hasher.update(content);
        hasher.finalize().to_hex().to_string()
    }
    
    fn check_file_integrity(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        
        // Check for binary files that might be corrupted
        if self.is_binary_file(content) && self.has_corruption_indicators(content) {
            findings.push(
                Finding::new(
                    "integrity",
                    "corrupted_binary",
                    Severity::High,
                    file_path.to_path_buf(),
                    1,
                    "Potential binary file corruption detected".to_string(),
                )
                .with_description("File appears to contain corruption indicators or unexpected patterns".to_string())
                .with_suggestion("Verify file integrity and re-download if necessary".to_string())
            );
        }
        
        // Check for suspicious file size (empty files that shouldn't be empty)
        if content.is_empty() && self.should_not_be_empty(file_path) {
            findings.push(
                Finding::new(
                    "integrity",
                    "unexpected_empty_file",
                    Severity::Medium,
                    file_path.to_path_buf(),
                    1,
                    "File is unexpectedly empty".to_string(),
                )
                .with_description("This file type typically should not be empty".to_string())
            );
        }
        
        Ok(findings)
    }
    
    fn is_binary_file(&self, content: &[u8]) -> bool {
        // Simple heuristic: if file contains null bytes, consider it binary
        content.contains(&0)
    }
    
    fn has_corruption_indicators(&self, content: &[u8]) -> bool {
        // Look for patterns that might indicate corruption
        let content_str = String::from_utf8_lossy(content);
        
        // Check for repeated null bytes (potential corruption)
        if content.windows(4).any(|w| w == [0, 0, 0, 0]) {
            return true;
        }
        
        // Check for other corruption patterns
        content_str.contains("�") || // Unicode replacement character
        content_str.contains("\x00\x00\x00") // Multiple null characters
    }
    
    fn should_not_be_empty(&self, file_path: &Path) -> bool {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            matches!(ext, "rs" | "js" | "ts" | "py" | "java" | "cpp" | "c" | "h")
        } else {
            false
        }
    }
}

impl Analyzer for IntegrityAnalyzer {
    fn name(&self) -> &str {
        "integrity"
    }
    
    fn analyze(&self, file_path: &Path, content: &[u8]) -> Result<Vec<Finding>> {
        self.check_file_integrity(file_path, content)
    }
    
    fn supports_file(&self, _file_path: &Path) -> bool {
        true // Integrity checking applies to all files
    }
}