use crate::types::{Finding, Severity};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Training data management for the ML classifier
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingDataset {
    pub examples: Vec<TrainingExample>,
    pub version: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingExample {
    pub finding_id: String,
    pub features: Vec<f32>,
    pub is_true_positive: bool,
    pub feedback_source: FeedbackSource,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackSource {
    UserFeedback,      // Manual user classification
    AutomaticHeuristic, // Rule-based classification
    ExpertReview,      // Security expert review
}

impl TrainingDataset {
    pub fn new() -> Self {
        Self {
            examples: Vec::new(),
            version: "1.0.0".to_string(),
            created_at: chrono::Utc::now(),
        }
    }

    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let dataset: Self = serde_json::from_str(&content)?;
        Ok(dataset)
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn add_example(&mut self, example: TrainingExample) {
        self.examples.push(example);
    }

    pub fn add_feedback(
        &mut self,
        finding: &Finding,
        features: Vec<f32>,
        is_true_positive: bool,
        source: FeedbackSource,
    ) {
        let example = TrainingExample {
            finding_id: finding.id.clone(),
            features,
            is_true_positive,
            feedback_source: source,
            timestamp: chrono::Utc::now(),
        };
        
        self.add_example(example);
    }

    /// Get training data in format suitable for FANN
    pub fn get_training_pairs(&self) -> Vec<(Vec<f32>, f32)> {
        self.examples
            .iter()
            .map(|example| {
                let target = if example.is_true_positive { 1.0 } else { 0.0 };
                (example.features.clone(), target)
            })
            .collect()
    }

    /// Get balanced training set (equal true/false positives)
    pub fn get_balanced_training_pairs(&self) -> Vec<(Vec<f32>, f32)> {
        let mut true_positives = Vec::new();
        let mut false_positives = Vec::new();

        for example in &self.examples {
            if example.is_true_positive {
                true_positives.push((example.features.clone(), 1.0));
            } else {
                false_positives.push((example.features.clone(), 0.0));
            }
        }

        // Balance the dataset
        let min_count = true_positives.len().min(false_positives.len());
        let mut balanced = Vec::new();
        
        balanced.extend(true_positives.into_iter().take(min_count));
        balanced.extend(false_positives.into_iter().take(min_count));
        
        balanced
    }

    /// Generate synthetic training data for cold start
    pub fn generate_synthetic_data(&mut self) -> Result<()> {
        // Generate examples based on common patterns
        
        // High-confidence true positives
        self.add_synthetic_example(
            "integrity_critical_corruption",
            vec![1.0, 0.9, 0.95, 0.8, 0.9, 1.0, 1.0, 0.8], // Strong features
            true,
        );

        self.add_synthetic_example(
            "secrets_hardcoded_token",
            vec![1.0, 0.8, 0.75, 0.9, 0.7, 1.0, 1.0, 0.9], // Critical + good context
            true,
        );

        // High-confidence false positives
        self.add_synthetic_example(
            "todo_in_test_file",
            vec![0.2, 0.3, 0.75, 0.4, 0.5, 0.0, 0.0, 0.3], // Low severity, test file
            false,
        );

        self.add_synthetic_example(
            "debug_in_development",
            vec![0.4, 0.7, 0.75, 0.3, 0.8, 0.0, 0.0, 0.4], // Medium severity but likely dev code
            false,
        );

        // Medium confidence examples
        self.add_synthetic_example(
            "config_drift_minor",
            vec![0.6, 0.6, 0.85, 0.5, 0.6, 1.0, 0.0, 0.6], // Medium across the board
            true,
        );

        Ok(())
    }

    fn add_synthetic_example(&mut self, id: &str, features: Vec<f32>, is_true_positive: bool) {
        let example = TrainingExample {
            finding_id: format!("synthetic_{}", id),
            features,
            is_true_positive,
            feedback_source: FeedbackSource::AutomaticHeuristic,
            timestamp: chrono::Utc::now(),
        };
        
        self.add_example(example);
    }

    /// Get statistics about the training dataset
    pub fn get_stats(&self) -> DatasetStats {
        let total = self.examples.len();
        let true_positives = self.examples.iter().filter(|e| e.is_true_positive).count();
        let false_positives = total - true_positives;

        let mut source_counts = std::collections::HashMap::new();
        for example in &self.examples {
            *source_counts.entry(format!("{:?}", example.feedback_source)).or_insert(0) += 1;
        }

        DatasetStats {
            total_examples: total,
            true_positives,
            false_positives,
            balance_ratio: if false_positives > 0 {
                true_positives as f32 / false_positives as f32
            } else {
                f32::INFINITY
            },
            source_distribution: source_counts,
        }
    }
}

#[derive(Debug)]
pub struct DatasetStats {
    pub total_examples: usize,
    pub true_positives: usize,
    pub false_positives: usize,
    pub balance_ratio: f32,
    pub source_distribution: std::collections::HashMap<String, usize>,
}

impl std::fmt::Display for DatasetStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Training Dataset Statistics:")?;
        writeln!(f, "  Total examples: {}", self.total_examples)?;
        writeln!(f, "  True positives: {}", self.true_positives)?;
        writeln!(f, "  False positives: {}", self.false_positives)?;
        writeln!(f, "  Balance ratio: {:.2}", self.balance_ratio)?;
        writeln!(f, "  Source distribution:")?;
        for (source, count) in &self.source_distribution {
            writeln!(f, "    {}: {}", source, count)?;
        }
        Ok(())
    }
}

/// Helper to create training data from historical analysis results
pub struct TrainingDataCollector {
    dataset: TrainingDataset,
    feature_extractor: crate::ml::feature_extractor::FeatureExtractor,
}

impl TrainingDataCollector {
    pub fn new() -> Self {
        Self {
            dataset: TrainingDataset::new(),
            feature_extractor: crate::ml::feature_extractor::FeatureExtractor::new(),
        }
    }

    /// Collect training data from a set of findings with known classifications
    pub fn collect_from_findings(
        &mut self,
        findings: &[(Finding, bool)], // (finding, is_true_positive)
    ) -> Result<()> {
        for (finding, is_true_positive) in findings {
            let features = self.feature_extractor.extract_features(finding)?;
            
            self.dataset.add_feedback(
                finding,
                features,
                *is_true_positive,
                FeedbackSource::ExpertReview,
            );
        }
        
        Ok(())
    }

    /// Apply heuristics to automatically classify findings
    pub fn apply_heuristics(&mut self, findings: &[Finding]) -> Result<()> {
        for finding in findings {
            let features = self.feature_extractor.extract_features(finding)?;
            let is_true_positive = self.heuristic_classification(finding);
            
            self.dataset.add_feedback(
                finding,
                features,
                is_true_positive,
                FeedbackSource::AutomaticHeuristic,
            );
        }
        
        Ok(())
    }

    /// Simple heuristic classification based on patterns
    fn heuristic_classification(&self, finding: &Finding) -> bool {
        // High confidence true positives
        if matches!(finding.severity, Severity::Critical | Severity::High) {
            if finding.analyzer == "integrity" || 
               (finding.analyzer == "non_production" && finding.message.contains("secret")) {
                return true;
            }
        }

        // High confidence false positives
        if finding.message.to_lowercase().contains("todo") && 
           finding.file.to_string_lossy().contains("test") {
            return false;
        }

        if finding.message.to_lowercase().contains("debug") &&
           finding.file.to_string_lossy().contains("dev") {
            return false;
        }

        // Default to true positive for medium+ severity
        !matches!(finding.severity, Severity::Low | Severity::Info)
    }

    pub fn get_dataset(self) -> TrainingDataset {
        self.dataset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_training_dataset() {
        let mut dataset = TrainingDataset::new();
        
        // Add some examples
        dataset.add_synthetic_example("test1", vec![1.0, 0.8, 0.9], true);
        dataset.add_synthetic_example("test2", vec![0.2, 0.3, 0.1], false);
        
        let pairs = dataset.get_training_pairs();
        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs[0].1, 1.0); // True positive
        assert_eq!(pairs[1].1, 0.0); // False positive
    }

    #[test]
    fn test_balanced_dataset() {
        let mut dataset = TrainingDataset::new();
        
        // Add unbalanced data
        for i in 0..10 {
            dataset.add_synthetic_example(&format!("tp_{}", i), vec![0.8; 3], true);
        }
        for i in 0..3 {
            dataset.add_synthetic_example(&format!("fp_{}", i), vec![0.2; 3], false);
        }
        
        let balanced = dataset.get_balanced_training_pairs();
        assert_eq!(balanced.len(), 6); // 3 true + 3 false
        
        let true_count = balanced.iter().filter(|(_, label)| *label > 0.5).count();
        let false_count = balanced.len() - true_count;
        assert_eq!(true_count, false_count);
    }
}