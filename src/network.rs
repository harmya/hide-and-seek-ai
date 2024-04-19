use ndarray::Array2;
use rand::Rng;
use rand_distr::{Distribution, Normal};

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub struct NeuralNetwork {
    input_weights: Array2<f64>,
    output_weights: Array2<f64>,
}

impl NeuralNetwork {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let input_weights = Array2::from_shape_fn((4, 8), |_| rng.gen::<f64>() * 2.0 - 1.0);
        let output_weights = Array2::from_shape_fn((8, 4), |_| rng.gen::<f64>() * 2.0 - 1.0);

        NeuralNetwork {
            input_weights,
            output_weights,
        }
    }

    pub fn forward(&self, inputs: Array2<f64>) -> Array2<f64> {
        let hidden_input = inputs.dot(&self.input_weights);
        let hidden_output = hidden_input.mapv(sigmoid);
        let final_input = hidden_output.dot(&self.output_weights);
        final_input.mapv(sigmoid)
    }

    pub fn perturbate(&mut self, mutation_strength: f64) {
        let normal_dist = Normal::new(0.0, mutation_strength).unwrap();
        let mut rng = rand::thread_rng();

        self.input_weights.map_inplace(|w| *w += normal_dist.sample(&mut rng));
        self.output_weights.map_inplace(|w| *w += normal_dist.sample(&mut rng));
    }

    pub fn show(&self) {
        println!("Input weights: {:?}", self.input_weights);
        println!("Output weights: {:?}", self.output_weights);
    }
}
