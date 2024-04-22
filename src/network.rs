use ndarray::Array2;
use rand::Rng;
use rand_distr::{Distribution, Normal};


pub struct NeuralNetwork {
    input_weights: Array2<f64>,
    hidden_weights: Array2<f64>,
    output_weights: Array2<f64>,
}


impl NeuralNetwork {
    pub fn new (input_size: usize, hidden_size: usize, output_size: usize) -> NeuralNetwork {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 1.0).unwrap();
        let input_weights = Array2::from_shape_fn((input_size, hidden_size), |_| normal.sample(&mut rng));
        let hidden_weights = Array2::from_shape_fn((hidden_size, output_size), |_| normal.sample(&mut rng));
        let output_weights = Array2::from_shape_fn((output_size, 1), |_| normal.sample(&mut rng));
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
