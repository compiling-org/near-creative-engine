//! Iron Learn integration for enhanced GPU-accelerated machine learning
//!
//! This module provides integration with the iron_learn library for:
//! - GPU-accelerated tensor operations
//! - Linear and logistic regression with CUDA support
//! - Complex number arithmetic for signal processing
//! - Type-safe machine learning operations

// Iron Learn crate is not available in current workspace
// #[cfg(feature = "ai-ml")]
// use iron_learn::{
//     Tensor, Complex, linear_regression, logistic_regression,
//     predict_linear, predict_logistic, run_logistics_cuda
// };
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for Iron Learn ML operations
#[derive(Serialize, Deserialize, Clone)]
pub struct IronLearnConfig {
    pub learning_rate: f64,
    pub epochs: usize,
    pub use_gpu: bool,
    pub regularization: f64,
    pub batch_size: usize,
}

impl Default for IronLearnConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.01,
            epochs: 1000,
            use_gpu: true,
            regularization: 0.001,
            batch_size: 32,
        }
    }
}

/// Iron Learn enhanced ML processor
pub struct IronLearnProcessor {
    config: IronLearnConfig,
    models: HashMap<String, IronLearnModel>,
    training_history: Vec<TrainingMetrics>,
}

/// Iron Learn model wrapper
#[derive(Serialize, Deserialize)]
pub struct IronLearnModel {
    pub model_type: String, // "linear", "logistic", "neural"
    pub weights: Vec<f64>,
    pub input_shape: Vec<usize>,
    pub output_shape: Vec<usize>,
    pub feature_names: Vec<String>,
    pub training_metrics: TrainingMetrics,
}

/// Training metrics and performance data
#[derive(Serialize, Deserialize, Clone)]
pub struct TrainingMetrics {
    pub epochs_completed: usize,
    pub final_loss: f64,
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub training_time_ms: u64,
}

/// Complex signal data for biometric processing
#[derive(Serialize, Deserialize)]
pub struct ComplexSignal {
    pub real: Vec<f64>,
    pub imaginary: Vec<f64>,
    pub sampling_rate: f64,
    pub signal_type: String, // "eeg", "emg", "ecg"
}

impl IronLearnProcessor {
    /// Create a new Iron Learn processor
    pub fn new(config: IronLearnConfig) -> Self {
        Self {
            config,
            models: HashMap::new(),
            training_history: Vec::new(),
        }
    }

    /// Train a linear regression model on biometric data
    #[cfg(feature = "ai-ml")]
    pub fn train_linear_model(
        &mut self,
        model_name: &str,
        features: &[Vec<f64>],
        targets: &[f64],
        feature_names: Vec<String>,
    ) -> Result<IronLearnModel, Box<dyn std::error::Error>> {
        // Validate input dimensions
        if features.is_empty() || targets.is_empty() {
            return Err("Empty training data".into());
        }
        
        let n_samples = features.len();
        let n_features = features[0].len();
        
        if targets.len() != n_samples {
            return Err("Mismatched samples and targets".into());
        }

        // Flatten features for tensor creation
        let mut flattened_features = Vec::new();
        for sample in features {
            if sample.len() != n_features {
                return Err("Inconsistent feature dimensions".into());
            }
            flattened_features.extend_from_slice(sample);
        }

        // Create tensors
        let x_tensor = Tensor::new(vec![n_samples, n_features], flattened_features)?;
        let y_tensor = Tensor::new(vec![n_samples, 1], targets.to_vec())?;
        
        // Initialize weights with zeros
        let mut weights = Tensor::new(vec![n_features + 1, 1], vec![0.0; n_features + 1])?;

        let start_time = std::time::Instant::now();
        
        // Train using linear regression
        for epoch in 0..self.config.epochs {
            weights = linear_regression(&x_tensor, &y_tensor, &weights, self.config.learning_rate)?;
            
            if epoch % 100 == 0 {
                // Calculate current loss for monitoring
                let predictions = predict_linear(&x_tensor, &weights)?;
                let loss = self.calculate_mse_loss(&predictions, targets);
                web_sys::console::log_1(&format!("Epoch {}: Loss = {:.6}", epoch, loss).into());
            }
        }

        let training_time = start_time.elapsed().as_millis() as u64;

        // Extract trained weights
        let trained_weights = weights.get_data();
        
        // Calculate final metrics
        let final_predictions = predict_linear(&x_tensor, &weights)?;
        let final_loss = self.calculate_mse_loss(&final_predictions, targets);
        let accuracy = self.calculate_r2_score(&final_predictions, targets);

        let metrics = TrainingMetrics {
            epochs_completed: self.config.epochs,
            final_loss,
            accuracy,
            precision: 0.0, // Not applicable for regression
            recall: 0.0,
            f1_score: 0.0,
            training_time_ms: training_time,
        };

        let model = IronLearnModel {
            model_type: "linear".to_string(),
            weights: trained_weights,
            input_shape: vec![n_features],
            output_shape: vec![1],
            feature_names,
            training_metrics: metrics.clone(),
        };

        self.models.insert(model_name.to_string(), model.clone());
        self.training_history.push(metrics);

        Ok(model)
    }

    /// Train a logistic regression model for classification
    #[cfg(feature = "ai-ml")]
    pub fn train_logistic_model(
        &mut self,
        model_name: &str,
        features: &[Vec<f64>],
        labels: &[f64],
        feature_names: Vec<String>,
    ) -> Result<IronLearnModel, Box<dyn std::error::Error>> {
        // Validate input dimensions
        if features.is_empty() || labels.is_empty() {
            return Err("Empty training data".into());
        }
        
        let n_samples = features.len();
        let n_features = features[0].len();
        
        if labels.len() != n_samples {
            return Err("Mismatched samples and labels".into());
        }

        // Validate binary labels
        for &label in labels {
            if label != 0.0 && label != 1.0 {
                return Err("Logistic regression requires binary labels (0 or 1)".into());
            }
        }

        // Flatten features for tensor creation
        let mut flattened_features = Vec::new();
        for sample in features {
            if sample.len() != n_features {
                return Err("Inconsistent feature dimensions".into());
            }
            flattened_features.extend_from_slice(sample);
        }

        // Create tensors
        let x_tensor = Tensor::new(vec![n_samples, n_features], flattened_features)?;
        let y_tensor = Tensor::new(vec![n_samples, 1], labels.to_vec())?;
        
        // Initialize weights with small random values
        let mut weights = Tensor::new(vec![n_features + 1, 1], self.initialize_random_weights(n_features + 1))?;

        let start_time = std::time::Instant::now();
        
        // Train using logistic regression
        for epoch in 0..self.config.epochs {
            weights = logistic_regression(&x_tensor, &y_tensor, &weights, self.config.learning_rate)?;
            
            if epoch % 100 == 0 {
                // Calculate current metrics for monitoring
                let predictions = predict_logistic(&x_tensor, &weights)?;
                let accuracy = self.calculate_binary_accuracy(&predictions, labels);
                web_sys::console::log_1(&format!("Epoch {}: Accuracy = {:.4}", epoch, accuracy).into());
            }
        }

        let training_time = start_time.elapsed().as_millis() as u64;

        // Extract trained weights
        let trained_weights = weights.get_data();
        
        // Calculate final metrics
        let final_predictions = predict_logistic(&x_tensor, &weights)?;
        let final_loss = self.calculate_log_loss(&final_predictions, labels);
        let accuracy = self.calculate_binary_accuracy(&final_predictions, labels);
        let (precision, recall, f1) = self.calculate_classification_metrics(&final_predictions, labels);

        let metrics = TrainingMetrics {
            epochs_completed: self.config.epochs,
            final_loss,
            accuracy,
            precision,
            recall,
            f1_score: f1,
            training_time_ms: training_time,
        };

        let model = IronLearnModel {
            model_type: "logistic".to_string(),
            weights: trained_weights,
            input_shape: vec![n_features],
            output_shape: vec![1],
            feature_names,
            training_metrics: metrics.clone(),
        };

        self.models.insert(model_name.to_string(), model.clone());
        self.training_history.push(metrics);

        Ok(model)
    }

    /// Process complex biometric signals using complex number arithmetic
    #[cfg(feature = "ai-ml")]
    pub fn process_complex_signal(
        &self,
        signal: ComplexSignal,
    ) -> Result<ComplexAnalysis, Box<dyn std::error::Error>> {
        // Convert to complex numbers
        let complex_data: Vec<Complex> = signal.real
            .iter()
            .zip(signal.imaginary.iter())
            .map(|(&r, &i)| Complex::new(r, i))
            .collect();

        // Perform FFT-like analysis (simplified)
        let frequencies = self.extract_frequencies(&complex_data, signal.sampling_rate)?;
        let power_spectrum = self.calculate_power_spectrum(&frequencies)?;
        
        // Find dominant frequency
        let (dominant_freq, max_power) = power_spectrum
            .iter()
            .enumerate()
            .max_by(|(_, &power1), (_, &power2)| power1.partial_cmp(&power2).unwrap())
            .map(|(idx, &power)| (idx as f64 * signal.sampling_rate / complex_data.len() as f64, power))
            .unwrap_or((0.0, 0.0));

        Ok(ComplexAnalysis {
            dominant_frequency: dominant_freq,
            max_power,
            average_power: power_spectrum.iter().sum::<f64>() / power_spectrum.len() as f64,
            signal_type: signal.signal_type,
            frequency_resolution: signal.sampling_rate / complex_data.len() as f64,
        })
    }

    /// Predict using a trained model
    #[cfg(feature = "ai-ml")]
    pub fn predict(&self, model_name: &str, features: &[f64]) -> Result<f64, Box<dyn std::error::Error>> {
        let model = self.models.get(model_name).ok_or("Model not found")?;
        
        if features.len() != model.input_shape[0] {
            return Err(format!("Expected {} features, got {}", model.input_shape[0], features.len()).into());
        }

        // Create feature tensor
        let x_tensor = Tensor::new(vec![1, features.len()], features.to_vec())?;
        
        // Create weight tensor from stored weights
        let weights_tensor = Tensor::new(vec![model.weights.len(), 1], model.weights.clone())?;
        
        match model.model_type.as_str() {
            "linear" => {
                let prediction = predict_linear(&x_tensor, &weights_tensor)?;
                Ok(prediction.get_data()[0])
            }
            "logistic" => {
                let prediction = predict_logistic(&x_tensor, &weights_tensor)?;
                Ok(prediction.get_data()[0])
            }
            _ => Err("Unsupported model type".into()),
        }
    }

    /// Get model performance metrics
    pub fn get_model_metrics(&self, model_name: &str) -> Option<&TrainingMetrics> {
        self.models.get(model_name).map(|model| &model.training_metrics)
    }

    /// Get all trained models
    pub fn get_models(&self) -> &HashMap<String, IronLearnModel> {
        &self.models
    }

    /// Clear all models and history
    pub fn clear_models(&mut self) {
        self.models.clear();
        self.training_history.clear();
    }

    // Helper methods
    fn calculate_mse_loss(&self, predictions: &Tensor, targets: &[f64]) -> f64 {
        let pred_data = predictions.get_data();
        let mut sum_squared_error = 0.0;
        
        for (i, &target) in targets.iter().enumerate() {
            if i < pred_data.len() {
                let error = pred_data[i] - target;
                sum_squared_error += error * error;
            }
        }
        
        sum_squared_error / targets.len() as f64
    }

    fn calculate_log_loss(&self, predictions: &Tensor, targets: &[f64]) -> f64 {
        let pred_data = predictions.get_data();
        let mut log_loss = 0.0;
        
        for (i, &target) in targets.iter().enumerate() {
            if i < pred_data.len() {
                let pred = pred_data[i].max(1e-15).min(1.0 - 1e-15); // Clip to avoid log(0)
                log_loss -= target * pred.ln() + (1.0 - target) * (1.0 - pred).ln();
            }
        }
        
        log_loss / targets.len() as f64
    }

    fn calculate_r2_score(&self, predictions: &Tensor, targets: &[f64]) -> f64 {
        let pred_data = predictions.get_data();
        let mean_target = targets.iter().sum::<f64>() / targets.len() as f64;
        
        let mut total_sum_squares = 0.0;
        let mut residual_sum_squares = 0.0;
        
        for (i, &target) in targets.iter().enumerate() {
            if i < pred_data.len() {
                let target_deviation = target - mean_target;
                total_sum_squares += target_deviation * target_deviation;
                
                let residual = target - pred_data[i];
                residual_sum_squares += residual * residual;
            }
        }
        
        1.0 - (residual_sum_squares / total_sum_squares)
    }

    fn calculate_binary_accuracy(&self, predictions: &Tensor, targets: &[f64]) -> f64 {
        let pred_data = predictions.get_data();
        let mut correct = 0;
        
        for (i, &target) in targets.iter().enumerate() {
            if i < pred_data.len() {
                let pred = if pred_data[i] >= 0.5 { 1.0 } else { 0.0 };
                if (pred - target).abs() < 0.001 {
                    correct += 1;
                }
            }
        }
        
        correct as f64 / targets.len() as f64
    }

    fn calculate_classification_metrics(&self, predictions: &Tensor, targets: &[f64]) -> (f64, f64, f64) {
        let pred_data = predictions.get_data();
        let mut true_positives = 0;
        let mut false_positives = 0;
        let mut false_negatives = 0;
        
        for (i, &target) in targets.iter().enumerate() {
            if i < pred_data.len() {
                let pred = if pred_data[i] >= 0.5 { 1.0 } else { 0.0 };
                
                if pred == 1.0 && target == 1.0 {
                    true_positives += 1;
                } else if pred == 1.0 && target == 0.0 {
                    false_positives += 1;
                } else if pred == 0.0 && target == 1.0 {
                    false_negatives += 1;
                }
            }
        }
        
        let precision = if true_positives + false_positives > 0 {
            true_positives as f64 / (true_positives + false_positives) as f64
        } else {
            0.0
        };
        
        let recall = if true_positives + false_negatives > 0 {
            true_positives as f64 / (true_positives + false_negatives) as f64
        } else {
            0.0
        };
        
        let f1 = if precision + recall > 0.0 {
            2.0 * precision * recall / (precision + recall)
        } else {
            0.0
        };
        
        (precision, recall, f1)
    }

    fn initialize_random_weights(&self, size: usize) -> Vec<f64> {
        let mut weights = Vec::with_capacity(size);
        for _ in 0..size {
            weights.push((js_sys::Math::random() as f64 - 0.5) * 0.1);
        }
        weights
    }

    fn extract_frequencies(&self, complex_data: &[Complex], sampling_rate: f64) -> Result<Vec<Complex>, Box<dyn std::error::Error>> {
        // Simplified frequency extraction - would use proper FFT in production
        let mut frequencies = Vec::new();
        let n = complex_data.len();
        
        for k in 0..n {
            let mut sum = Complex::new(0.0, 0.0);
            for (t, &sample) in complex_data.iter().enumerate() {
                let angle = -2.0 * std::f64::consts::PI * k as f64 * t as f64 / n as f64;
                let twiddle = Complex::new(angle.cos(), angle.sin());
                sum = sum + sample * twiddle;
            }
            frequencies.push(sum);
        }
        
        Ok(frequencies)
    }

    fn calculate_power_spectrum(&self, frequencies: &[Complex]) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        Ok(frequencies.iter().map(|&freq| {
            let real = freq.re;
            let imag = freq.im;
            real * real + imag * imag
        }).collect())
    }
}

/// Complex signal analysis results
#[derive(Serialize, Deserialize)]
pub struct ComplexAnalysis {
    pub dominant_frequency: f64,
    pub max_power: f64,
    pub average_power: f64,
    pub signal_type: String,
    pub frequency_resolution: f64,
}

/// Integration with enhanced WebGPU engine
impl IronLearnProcessor {
    /// Create processor with biometric-specific configuration
    pub fn new_biometric() -> Self {
        let config = IronLearnConfig {
            learning_rate: 0.001,
            epochs: 500,
            use_gpu: true,
            regularization: 0.0001,
            batch_size: 16,
        };
        
        Self::new(config)
    }
    
    /// Train emotion classification model from biometric data
    pub fn train_emotion_classifier(
        &mut self,
        model_name: &str,
        eeg_features: &[Vec<f64>],
        emotion_labels: &[String],
    ) -> Result<IronLearnModel, Box<dyn std::error::Error>> {
        // Convert emotion labels to binary format
        let mut binary_labels = Vec::new();
        for emotion in emotion_labels {
            // Simple binary classification: relaxed vs stressed
            let label = match emotion.as_str() {
                "relaxed" | "calm" | "peaceful" => 0.0,
                "stressed" | "anxious" | "tense" => 1.0,
                _ => 0.5, // neutral
            };
            binary_labels.push(label);
        }
        
        let feature_names = vec![
            "alpha_power".to_string(),
            "beta_power".to_string(),
            "theta_power".to_string(),
            "gamma_power".to_string(),
            "dominant_frequency".to_string(),
            "spectral_entropy".to_string(),
        ];
        
        self.train_logistic_model(model_name, eeg_features, &binary_labels, feature_names)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_iron_learn_config() {
        let config = IronLearnConfig::default();
        assert_eq!(config.learning_rate, 0.01);
        assert_eq!(config.epochs, 1000);
        assert!(config.use_gpu);
    }

    #[wasm_bindgen_test]
    fn test_iron_learn_processor_creation() {
        let processor = IronLearnProcessor::new(IronLearnConfig::default());
        assert!(processor.models.is_empty());
        assert!(processor.training_history.is_empty());
    }

    #[wasm_bindgen_test]
    fn test_complex_signal_creation() {
        let signal = ComplexSignal {
            real: vec![1.0, 2.0, 3.0],
            imaginary: vec![0.1, 0.2, 0.3],
            sampling_rate: 256.0,
            signal_type: "eeg".to_string(),
        };
        
        assert_eq!(signal.real.len(), 3);
        assert_eq!(signal.sampling_rate, 256.0);
        assert_eq!(signal.signal_type, "eeg");
    }
}