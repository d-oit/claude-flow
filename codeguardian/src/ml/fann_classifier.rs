use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Lightweight neural network classifier using FANN
pub struct FannClassifier {
    network: fann::FannNetwork,
    input_size: usize,
    learning_rate: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub input_size: usize,
    pub hidden_layers: Vec<usize>,
    pub output_size: usize,
    pub learning_rate: f32,
    pub activation_function: String,
}

impl FannClassifier {
    /// Create a new classifier with the given configuration
    pub fn new(config: NetworkConfig) -> Result<Self> {
        let mut layers = vec![config.input_size];
        layers.extend(config.hidden_layers);
        layers.push(config.output_size);

        let network = fann::FannNetwork::new(&layers)
            .map_err(|e| anyhow!("Failed to create FANN network: {:?}", e))?;

        // Configure network
        network.set_learning_rate(config.learning_rate);
        network.set_activation_function_hidden(fann::ActivationFunction::Sigmoid);
        network.set_activation_function_output(fann::ActivationFunction::Sigmoid);

        Ok(Self {
            network,
            input_size: config.input_size,
            learning_rate: config.learning_rate,
        })
    }

    /// Load a pre-trained model from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let network = fann::FannNetwork::load(path.as_ref())
            .map_err(|e| anyhow!("Failed to load FANN network: {:?}", e))?;

        // Extract configuration from loaded network
        let input_size = network.get_num_input() as usize;
        let learning_rate = network.get_learning_rate();

        Ok(Self {
            network,
            input_size,
            learning_rate,
        })
    }

    /// Save the trained model to file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.network.save(path.as_ref())
            .map_err(|e| anyhow!("Failed to save FANN network: {:?}", e))
    }

    /// Predict output for given input features
    pub fn predict(&self, features: &[f32]) -> Result<f32> {
        if features.len() != self.input_size {
            return Err(anyhow!(
                "Input size mismatch: expected {}, got {}",
                self.input_size,
                features.len()
            ));
        }

        let output = self.network.run(features)
            .map_err(|e| anyhow!("FANN prediction failed: {:?}", e))?;

        Ok(output[0]) // Single output for binary classification
    }

    /// Train the network with a single example (online learning)
    pub fn train_incremental(&mut self, features: &[f32], target: f32) -> Result<()> {
        if features.len() != self.input_size {
            return Err(anyhow!(
                "Input size mismatch: expected {}, got {}",
                self.input_size,
                features.len()
            ));
        }

        let target_output = vec![target];
        
        self.network.train(features, &target_output)
            .map_err(|e| anyhow!("FANN training failed: {:?}", e))?;

        Ok(())
    }

    /// Batch training with multiple examples
    pub fn train_batch(&mut self, training_data: &[(Vec<f32>, f32)], epochs: u32) -> Result<f32> {
        let mut total_error = 0.0;

        for _ in 0..epochs {
            let mut epoch_error = 0.0;
            
            for (features, target) in training_data {
                self.train_incremental(features, *target)?;
                
                // Calculate error for this example
                let prediction = self.predict(features)?;
                epoch_error += (prediction - target).powi(2);
            }
            
            total_error = epoch_error / training_data.len() as f32;
        }

        Ok(total_error)
    }

    /// Get network statistics
    pub fn get_stats(&self) -> NetworkStats {
        NetworkStats {
            input_size: self.input_size,
            hidden_layers: self.network.get_num_layers() as usize - 2, // Exclude input/output
            total_neurons: self.network.get_total_neurons() as usize,
            total_connections: self.network.get_total_connections() as usize,
            learning_rate: self.learning_rate,
        }
    }
}

#[derive(Debug)]
pub struct NetworkStats {
    pub input_size: usize,
    pub hidden_layers: usize,
    pub total_neurons: usize,
    pub total_connections: usize,
    pub learning_rate: f32,
}

impl std::fmt::Display for NetworkStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FANN Network: {} inputs, {} hidden layers, {} neurons, {} connections, LR: {:.4}",
            self.input_size,
            self.hidden_layers,
            self.total_neurons,
            self.total_connections,
            self.learning_rate
        )
    }
}

/// Default configuration for CodeGuardian's use case
impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            input_size: 8,  // Feature vector size
            hidden_layers: vec![12, 8],  // Two hidden layers
            output_size: 1,  // Single relevance score
            learning_rate: 0.1,
            activation_function: "sigmoid".to_string(),
        }
    }
}