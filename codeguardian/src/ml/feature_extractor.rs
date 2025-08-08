use crate::types::{Finding, Severity};
use anyhow::Result;
use std::collections::HashMap;

/// Extract numerical features from findings for ML classification
pub struct FeatureExtractor {
    file_type_scores: HashMap<String, f32>,
    analyzer_confidence: HashMap<String, f32>,
}

impl FeatureExtractor {
    pub fn new() -> Self {
        let mut file_type_scores = HashMap::new();
        
        // Higher scores = more likely to have real issues
        file_type_scores.insert("rs".to_string(), 0.9);   // Rust - high confidence
        file_type_scores.insert("js".to_string(), 0.7);   // JavaScript - medium-high
        file_type_scores.insert("ts".to_string(), 0.8);   // TypeScript - high
        file_type_scores.insert("py".to_string(), 0.8);   // Python - high
        file_type_scores.insert("java".to_string(), 0.8); // Java - high
        file_type_scores.insert("cpp".to_string(), 0.9);  // C++ - very high
        file_type_scores.insert("c".to_string(), 0.9);    // C - very high
        file_type_scores.insert("go".to_string(), 0.8);   // Go - high
        file_type_scores.insert("json".to_string(), 0.6); // Config files - medium
        file_type_scores.insert("yaml".to_string(), 0.6); // Config files - medium
        file_type_scores.insert("toml".to_string(), 0.6); // Config files - medium
        file_type_scores.insert("md".to_string(), 0.3);   // Markdown - low
        file_type_scores.insert("txt".to_string(), 0.2);  // Text - very low

        let mut analyzer_confidence = HashMap::new();
        analyzer_confidence.insert("integrity".to_string(), 0.95);      // Very reliable
        analyzer_confidence.insert("lint_drift".to_string(), 0.85);     // Reliable
        analyzer_confidence.insert("non_production".to_string(), 0.75); // Good but some FPs

        Self {
            file_type_scores,
            analyzer_confidence,
        }
    }

    /// Extract feature vector from a finding
    pub fn extract_features(&self, finding: &Finding) -> Result<Vec<f32>> {
        let mut features = Vec::with_capacity(8);

        // Feature 1: Severity score (0.0-1.0)
        features.push(self.severity_to_score(&finding.severity));

        // Feature 2: File type relevance (0.0-1.0)
        features.push(self.file_type_score(&finding.file));

        // Feature 3: Analyzer confidence (0.0-1.0)
        features.push(self.analyzer_confidence_score(&finding.analyzer));

        // Feature 4: Message length (normalized)
        features.push(self.normalize_message_length(&finding.message));

        // Feature 5: Line number (normalized, early lines often more important)
        features.push(self.normalize_line_number(finding.line));

        // Feature 6: Has description (0.0 or 1.0)
        features.push(if finding.description.is_some() { 1.0 } else { 0.0 });

        // Feature 7: Has suggestion (0.0 or 1.0)
        features.push(if finding.suggestion.is_some() { 1.0 } else { 0.0 });

        // Feature 8: Rule specificity (based on rule name length/complexity)
        features.push(self.rule_specificity_score(&finding.rule));

        Ok(features)
    }

    fn severity_to_score(&self, severity: &Severity) -> f32 {
        match severity {
            Severity::Critical => 1.0,
            Severity::High => 0.8,
            Severity::Medium => 0.6,
            Severity::Low => 0.4,
            Severity::Info => 0.2,
        }
    }

    fn file_type_score(&self, file_path: &std::path::Path) -> f32 {
        if let Some(extension) = file_path.extension().and_then(|e| e.to_str()) {
            self.file_type_scores.get(extension).copied().unwrap_or(0.5)
        } else {
            0.3 // Files without extensions are less likely to have issues
        }
    }

    fn analyzer_confidence_score(&self, analyzer: &str) -> f32 {
        self.analyzer_confidence.get(analyzer).copied().unwrap_or(0.5)
    }

    fn normalize_message_length(&self, message: &str) -> f32 {
        let length = message.len() as f32;
        // Normalize to 0.0-1.0, with longer messages generally being more detailed/reliable
        (length / 200.0).min(1.0)
    }

    fn normalize_line_number(&self, line: u32) -> f32 {
        // Early lines in files are often more important (imports, declarations, etc.)
        // Use inverse relationship: line 1 = 1.0, line 1000+ = 0.1
        let normalized = 1.0 - ((line as f32 - 1.0) / 1000.0);
        normalized.max(0.1).min(1.0)
    }

    fn rule_specificity_score(&self, rule: &str) -> f32 {
        // More specific rules (longer names, underscores) are often more reliable
        let length_score = (rule.len() as f32 / 50.0).min(1.0);
        let specificity_score = if rule.contains('_') || rule.contains('-') {
            0.8 // Specific rule names
        } else {
            0.5 // Generic rule names
        };
        
        (length_score + specificity_score) / 2.0
    }

    /// Update file type scores based on feedback
    pub fn update_file_type_score(&mut self, extension: &str, is_reliable: bool) {
        let current_score = self.file_type_scores.get(extension).copied().unwrap_or(0.5);
        let adjustment = if is_reliable { 0.05 } else { -0.05 };
        let new_score = (current_score + adjustment).max(0.1).min(0.9);
        self.file_type_scores.insert(extension.to_string(), new_score);
    }

    /// Update analyzer confidence based on feedback
    pub fn update_analyzer_confidence(&mut self, analyzer: &str, is_reliable: bool) {
        let current_score = self.analyzer_confidence.get(analyzer).copied().unwrap_or(0.5);
        let adjustment = if is_reliable { 0.02 } else { -0.02 };
        let new_score = (current_score + adjustment).max(0.1).min(0.95);
        self.analyzer_confidence.insert(analyzer.to_string(), new_score);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Finding;
    use std::path::PathBuf;

    #[test]
    fn test_feature_extraction() {
        let extractor = FeatureExtractor::new();
        
        let finding = Finding::new(
            "integrity",
            "corrupted_binary",
            Severity::High,
            PathBuf::from("src/main.rs"),
            42,
            "Potential binary file corruption detected".to_string(),
        )
        .with_description("File appears to contain corruption indicators".to_string())
        .with_suggestion("Verify file integrity".to_string());

        let features = extractor.extract_features(&finding).unwrap();
        
        assert_eq!(features.len(), 8);
        assert!(features[0] > 0.7); // High severity
        assert!(features[1] > 0.8); // Rust file
        assert!(features[2] > 0.9); // Integrity analyzer
        assert!(features[5] > 0.0); // Has description
        assert!(features[6] > 0.0); // Has suggestion
    }

    #[test]
    fn test_severity_scoring() {
        let extractor = FeatureExtractor::new();
        
        assert_eq!(extractor.severity_to_score(&Severity::Critical), 1.0);
        assert_eq!(extractor.severity_to_score(&Severity::High), 0.8);
        assert_eq!(extractor.severity_to_score(&Severity::Medium), 0.6);
        assert_eq!(extractor.severity_to_score(&Severity::Low), 0.4);
        assert_eq!(extractor.severity_to_score(&Severity::Info), 0.2);
    }
}