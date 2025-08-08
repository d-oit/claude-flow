#!/usr/bin/env rust-script
//! # CodeGuardian ML Training Example
//! 
//! This example demonstrates how to train the RUV-FANN classifier
//! with historical data and user feedback.
//!
//! ```cargo
//! [dependencies]
//! codeguardian = { path = ".." }
//! anyhow = "1.0"
//! tokio = { version = "1.0", features = ["full"] }
//! ```

use codeguardian::ml::{
    MLClassifier,
    training_data::{TrainingDataset, TrainingDataCollector, FeedbackSource},
    fann_classifier::{FannClassifier, NetworkConfig},
};
use codeguardian::types::{Finding, Severity};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🧠 CodeGuardian ML Training Example");
    println!("===================================\n");

    // Step 1: Create training dataset
    println!("📊 Step 1: Creating training dataset...");
    let mut dataset = TrainingDataset::new();
    
    // Generate synthetic training data for cold start
    dataset.generate_synthetic_data()?;
    println!("   Generated {} synthetic examples", dataset.examples.len());

    // Step 2: Add historical findings with known classifications
    println!("\n📚 Step 2: Adding historical data...");
    let historical_findings = create_sample_findings();
    let mut collector = TrainingDataCollector::new();
    
    // Apply heuristic classification to historical findings
    collector.apply_heuristics(&historical_findings)?;
    let mut dataset = collector.get_dataset();
    
    println!("   Added {} historical examples", dataset.examples.len());
    println!("   Dataset stats:\n{}", dataset.get_stats());

    // Step 3: Train the neural network
    println!("\n🧠 Step 3: Training RUV-FANN classifier...");
    
    // Create network configuration optimized for CodeGuardian
    let config = NetworkConfig {
        input_size: 8,           // 8 features per finding
        hidden_layers: vec![12, 8], // Two hidden layers
        output_size: 1,          // Binary classification
        learning_rate: 0.1,      // Fast convergence
        activation_function: "sigmoid".to_string(),
    };
    
    let mut classifier = FannClassifier::new(config)?;
    println!("   Created network: {}", classifier.get_stats());
    
    // Get balanced training data
    let training_pairs = dataset.get_balanced_training_pairs();
    println!("   Training with {} balanced examples", training_pairs.len());
    
    // Train the network
    let epochs = 1000;
    let final_error = classifier.train_batch(&training_pairs, epochs)?;
    println!("   Training completed! Final error: {:.6}", final_error);
    
    // Step 4: Test the trained classifier
    println!("\n🧪 Step 4: Testing classifier...");
    test_classifier(&classifier).await?;
    
    // Step 5: Save the trained model
    println!("\n💾 Step 5: Saving trained model...");
    classifier.save("codeguardian-model.fann")?;
    println!("   Model saved to: codeguardian-model.fann");
    
    // Step 6: Demonstrate online learning
    println!("\n📈 Step 6: Demonstrating online learning...");
    demonstrate_online_learning(&mut classifier).await?;
    
    println!("\n✅ Training complete! The model is ready for production use.");
    println!("\nNext steps:");
    println!("1. Copy codeguardian-model.fann to your project directory");
    println!("2. Run: codeguardian check . (ML filtering will be automatic)");
    println!("3. Provide feedback to improve the model over time");
    
    Ok(())
}

fn create_sample_findings() -> Vec<Finding> {
    vec![
        // High-confidence true positives
        Finding::new(
            "integrity",
            "corrupted_binary",
            Severity::Critical,
            PathBuf::from("src/core.rs"),
            42,
            "Critical file corruption detected in core module".to_string(),
        )
        .with_description("Binary file shows signs of corruption with invalid headers".to_string())
        .with_suggestion("Restore from backup and verify file integrity".to_string()),
        
        Finding::new(
            "non_production",
            "hardcoded_secret",
            Severity::High,
            PathBuf::from("src/auth.rs"),
            15,
            "Hardcoded API key detected in authentication module".to_string(),
        )
        .with_description("API key appears to be hardcoded in source code".to_string())
        .with_suggestion("Move API key to environment variables".to_string()),
        
        // High-confidence false positives
        Finding::new(
            "non_production",
            "todo_comment",
            Severity::Low,
            PathBuf::from("tests/integration_test.rs"),
            128,
            "TODO comment found in test file".to_string(),
        ),
        
        Finding::new(
            "non_production",
            "debug_statement",
            Severity::Medium,
            PathBuf::from("examples/demo.rs"),
            67,
            "Debug print statement found".to_string(),
        ),
        
        // Medium confidence examples
        Finding::new(
            "lint_drift",
            "json_formatting",
            Severity::Medium,
            PathBuf::from("package.json"),
            1,
            "JSON file formatting is inconsistent".to_string(),
        )
        .with_description("JSON formatting differs from standard pretty-printing".to_string())
        .with_suggestion("Run a JSON formatter to standardize formatting".to_string()),
    ]
}

async fn test_classifier(classifier: &FannClassifier) -> anyhow::Result<()> {
    use codeguardian::ml::feature_extractor::FeatureExtractor;
    
    let extractor = FeatureExtractor::new();
    let test_findings = create_sample_findings();
    
    println!("   Testing classifier on sample findings:");
    
    for (i, finding) in test_findings.iter().enumerate() {
        let features = extractor.extract_features(finding)?;
        let relevance = classifier.predict(&features)?;
        
        let classification = if relevance >= 0.5 { "TRUE POSITIVE" } else { "FALSE POSITIVE" };
        let confidence = if relevance >= 0.5 { relevance } else { 1.0 - relevance };
        
        println!("   Finding {}: {} ({:.1}% confidence)", 
                 i + 1, classification, confidence * 100.0);
        println!("     File: {}", finding.file.display());
        println!("     Message: {}", finding.message);
        println!("     Features: {:?}", features);
        println!("     Raw score: {:.3}\n", relevance);
    }
    
    Ok(())
}

async fn demonstrate_online_learning(classifier: &mut FannClassifier) -> anyhow::Result<()> {
    use codeguardian::ml::feature_extractor::FeatureExtractor;
    
    let extractor = FeatureExtractor::new();
    
    println!("   Simulating user feedback for online learning...");
    
    // Create a finding that might be misclassified initially
    let finding = Finding::new(
        "non_production",
        "todo_comment",
        Severity::Low,
        PathBuf::from("src/important.rs"), // Important file, not test
        5,
        "TODO: Implement critical security feature".to_string(),
    )
    .with_description("This TODO indicates missing security implementation".to_string());
    
    let features = extractor.extract_features(&finding)?;
    
    // Initial prediction
    let initial_prediction = classifier.predict(&features)?;
    println!("   Initial prediction: {:.3} ({})", 
             initial_prediction,
             if initial_prediction >= 0.5 { "TRUE POSITIVE" } else { "FALSE POSITIVE" });
    
    // Simulate user feedback: this is actually a true positive
    // (TODO in important security code should be flagged)
    println!("   User feedback: This is a TRUE POSITIVE (important security TODO)");
    
    // Apply online learning
    for _ in 0..10 { // Multiple training iterations
        classifier.train_incremental(&features, 1.0)?; // 1.0 = true positive
    }
    
    // Check updated prediction
    let updated_prediction = classifier.predict(&features)?;
    println!("   Updated prediction: {:.3} ({})", 
             updated_prediction,
             if updated_prediction >= 0.5 { "TRUE POSITIVE" } else { "FALSE POSITIVE" });
    
    let improvement = updated_prediction - initial_prediction;
    println!("   Improvement: {:+.3} (model learned from feedback!)", improvement);
    
    Ok(())
}