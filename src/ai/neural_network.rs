use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuralNetwork {
    pub weights_input_hidden: Vec<Vec<f32>>,
    pub weights_hidden_output: Vec<Vec<f32>>,
    pub bias_hidden: Vec<f32>,
    pub bias_output: Vec<f32>,
    pub input_size: usize,
    pub hidden_size: usize,
    pub output_size: usize,
}

impl NeuralNetwork {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        
        // Initialize weights with random values between -1 and 1
        let weights_input_hidden = (0..hidden_size)
            .map(|_| {
                (0..input_size)
                    .map(|_| rng.gen_range(-1.0..1.0))
                    .collect()
            })
            .collect();
            
        let weights_hidden_output = (0..output_size)
            .map(|_| {
                (0..hidden_size)
                    .map(|_| rng.gen_range(-1.0..1.0))
                    .collect()
            })
            .collect();
            
        let bias_hidden = (0..hidden_size)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect();
            
        let bias_output = (0..output_size)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect();
        
        Self {
            weights_input_hidden,
            weights_hidden_output,
            bias_hidden,
            bias_output,
            input_size,
            hidden_size,
            output_size,
        }
    }
    
    pub fn forward(&self, inputs: &[f32]) -> Vec<f32> {
        // Calculate hidden layer
        let mut hidden = vec![0.0; self.hidden_size];
        for i in 0..self.hidden_size {
            let mut sum = self.bias_hidden[i];
            for j in 0..self.input_size {
                sum += inputs[j] * self.weights_input_hidden[i][j];
            }
            hidden[i] = Self::sigmoid(sum);
        }
        
        // Calculate output layer
        let mut output = vec![0.0; self.output_size];
        for i in 0..self.output_size {
            let mut sum = self.bias_output[i];
            for j in 0..self.hidden_size {
                sum += hidden[j] * self.weights_hidden_output[i][j];
            }
            output[i] = Self::sigmoid(sum);
        }
        
        output
    }
    
    fn sigmoid(x: f32) -> f32 {
        1.0 / (1.0 + (-x).exp())
    }
    
    pub fn mutate(&mut self, mutation_rate: f32, mutation_strength: f32) {
        let mut rng = rand::thread_rng();
        
        // Mutate input to hidden weights
        for i in 0..self.hidden_size {
            for j in 0..self.input_size {
                if rng.gen::<f32>() < mutation_rate {
                    self.weights_input_hidden[i][j] += rng.gen_range(-mutation_strength..mutation_strength);
                    self.weights_input_hidden[i][j] = self.weights_input_hidden[i][j].clamp(-2.0, 2.0);
                }
            }
        }
        
        // Mutate hidden to output weights
        for i in 0..self.output_size {
            for j in 0..self.hidden_size {
                if rng.gen::<f32>() < mutation_rate {
                    self.weights_hidden_output[i][j] += rng.gen_range(-mutation_strength..mutation_strength);
                    self.weights_hidden_output[i][j] = self.weights_hidden_output[i][j].clamp(-2.0, 2.0);
                }
            }
        }
        
        // Mutate hidden biases
        for i in 0..self.hidden_size {
            if rng.gen::<f32>() < mutation_rate {
                self.bias_hidden[i] += rng.gen_range(-mutation_strength..mutation_strength);
                self.bias_hidden[i] = self.bias_hidden[i].clamp(-2.0, 2.0);
            }
        }
        
        // Mutate output biases
        for i in 0..self.output_size {
            if rng.gen::<f32>() < mutation_rate {
                self.bias_output[i] += rng.gen_range(-mutation_strength..mutation_strength);
                self.bias_output[i] = self.bias_output[i].clamp(-2.0, 2.0);
            }
        }
    }
    
    pub fn crossover(&self, other: &NeuralNetwork) -> NeuralNetwork {
        let mut rng = rand::thread_rng();
        let mut child = self.clone();
        
        // Crossover input to hidden weights
        for i in 0..self.hidden_size {
            for j in 0..self.input_size {
                if rng.gen::<f32>() < 0.5 {
                    child.weights_input_hidden[i][j] = other.weights_input_hidden[i][j];
                }
            }
        }
        
        // Crossover hidden to output weights
        for i in 0..self.output_size {
            for j in 0..self.hidden_size {
                if rng.gen::<f32>() < 0.5 {
                    child.weights_hidden_output[i][j] = other.weights_hidden_output[i][j];
                }
            }
        }
        
        // Crossover biases
        for i in 0..self.hidden_size {
            if rng.gen::<f32>() < 0.5 {
                child.bias_hidden[i] = other.bias_hidden[i];
            }
        }
        
        for i in 0..self.output_size {
            if rng.gen::<f32>() < 0.5 {
                child.bias_output[i] = other.bias_output[i];
            }
        }
        
        child
    }
}