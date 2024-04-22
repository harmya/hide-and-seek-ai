use ndarray::Array2;
use rand::Rng;
use rand_distr::{Distribution, Normal};


pub struct NeuralNetwork {
    input_weights: Array2<f64>,
    hidden_weights: Array2<f64>,
    output_weights: Array2<f64>,
}

impl Clone for NeuralNetwork {
    fn clone(&self) -> NeuralNetwork {
        NeuralNetwork {
            input_weights: self.input_weights.clone(),
            hidden_weights: self.hidden_weights.clone(),
            output_weights: self.output_weights.clone(),
        }
    }
}

impl NeuralNetwork {
    pub fn new (input_size: usize, hidden_size: usize, output_size: usize) -> NeuralNetwork {
        let mut rng = rand::thread_rng();
        let input_weights = Array2::from_shape_fn((input_size, hidden_size), |_| rng.gen_range(-1.0..1.0)); 
        let hidden_weights = Array2::from_shape_fn((hidden_size, output_size), |_| rng.gen_range(-1.0..1.0));
        let output_weights = Array2::from_shape_fn((output_size, 4), |_| rng.gen_range(-1.0..1.0));
        NeuralNetwork {
            input_weights,
            hidden_weights,
            output_weights,
        }
    }

    pub fn forward(&self, input: Array2<f64>) -> Array2<f64> {
        let hidden_layer = input.dot(&self.input_weights);
        let hidden_layer = hidden_layer.map(|x| 1.0 / (1.0 + (-x).exp()));
        let output_layer = hidden_layer.dot(&self.hidden_weights);
        let output_layer = output_layer.map(|x| 1.0 / (1.0 + (-x).exp()));
        output_layer.dot(&self.output_weights)
    }

    pub fn get_direction(&self, input: Array2<f64>) -> usize {
        let output = self.forward(input);
        let mut max = output[[0, 0]];
        let mut max_index = 0;

        for i in 0..output.shape()[1] {
            if output[[0, i]] > max {
                println!("Output: {:?}", output[[0, i]]);
                max = output[[0, i]];
                max_index = i;
            }
        }
        println!("Max: {}", max_index);
        println!("Output: {:?}", output);
        max_index
    }

    pub fn mutate(&mut self, mutation_rate: f64) {
        let mut rng = rand::thread_rng();
        for i in 0..self.input_weights.shape()[0] {
            for j in 0..self.input_weights.shape()[1] {
                if rng.gen::<f64>() < mutation_rate {
                    self.input_weights[[i, j]] += rng.gen_range(-1.0..1.0);
                }
            }
        }
        for i in 0..self.hidden_weights.shape()[0] {
            for j in 0..self.hidden_weights.shape()[1] {
                if rng.gen::<f64>() < mutation_rate {
                    self.hidden_weights[[i, j]] += rng.gen_range(-1.0..1.0);
                }
            }
        }
        for i in 0..self.output_weights.shape()[0] {
            for j in 0..self.output_weights.shape()[1] {
                if rng.gen::<f64>() < mutation_rate {
                    self.output_weights[[i, j]] += rng.gen_range(-1.0..1.0);
                }
            }
        }
    }
}
