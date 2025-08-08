pub mod fann_classifier;
pub mod feature_extractor;
pub mod training_data;

use crate::types::Finding;
use anyhow::Result;

/// Lightweight ML classifier using RUV-FANN for false positive reduction
pub struct MLClassifier {
    classifier: Option<fann_classifier::FannClassifier>,
    feature_extractor: feature_extractor::FeatureExtractor,
    enabled: bool,
}

impl MLClassifier {
    pub fn new(model_path: Option<&str>) -> Self {
        let classifier = model_path
            .and_then(|path| fann_classifier::FannClassifier::load(path).ok());
        
        Self {
            classifier,
            feature_extractor: feature_extractor::FeatureExtractor::new(),
            enabled: classifier.is_some(),
        }
    }

    /// Predict relevance score for a finding (0.0 = likely false positive, 1.0 = likely true positive)
    pub fn predict_relevance(&self, finding: &Finding) -> Result<f32> {
        if !self.enabled {
            return Ok(0.5); // Neutral score if ML disabled
        }

        let features = self.feature_extractor.extract_features(finding)?;
        
        if let Some(classifier) = &self.classifier {
            classifier.predict(&features)
        } else {
            Ok(0.5)
        }
    }

    /// Update model with user feedback (online learning)
    pub fn record_feedback(&mut self, finding: &Finding, is_true_positive: bool) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let features = self.feature_extractor.extract_features(finding)?;
        let target = if is_true_positive { 1.0 } else { 0.0 };

        if let Some(classifier) = &mut self.classifier {
            classifier.train_incremental(&features, target)?;
        }

        Ok(())
    }

    /// Filter findings based on confidence threshold
    pub fn filter_findings(&self, findings: Vec<Finding>, threshold: f32) -> Result<Vec<Finding>> {
        if !self.enabled {
            return Ok(findings);
        }

        let mut filtered = Vec::new();
        
        for finding in findings {
            let relevance = self.predict_relevance(&finding)?;
            if relevance >= threshold {
                filtered.push(finding);
            }
        }

        Ok(filtered)
    }
}