use ndarray::Array2;
use rand::Rng;

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

struct Network {
    input_weights: Array2<f32>,
    hidden_weights: Array2<f32>,
    output_weights: Array2<f32>,
}

impl Network {
    fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let input_weights = Array2::from_shape_fn((hidden_size, input_size), |_| rng.gen_range(-1.0..1.0));
        let hidden_weights = Array2::from_shape_fn((output_size, hidden_size), |_| rng.gen_range(-1.0..1.0));
        let output_weights = Array2::from_shape_fn((output_size, 1), |_| rng.gen_range(-1.0..1.0));
        Self {
            input_weights,
            hidden_weights,
            output_weights,
        }
    }

    fn forward(&self, input: &Array2<f64>) -> Array2<f64> {
        let input = self.input_weights.dot(input);
        let hidden_output = input.mapv(sigmoid);

        let hidden_output = self.hidden_weights.dot(&hidden_output);
        let hidden_output = hidden_output.mapv(sigmoid);
        
        let final_input = self.output_weights.dot(&hidden_output);
        final_input.mapv(sigmoid)
    }

    fn perturb(&mut self, mutation_rate: f32) {
        let mut rng = rand::thread_rng();
        for i in 0..self.input_weights.len() {
            if rng.gen_range(0.0..1.0) < mutation_rate {
                self.input_weights[i] += rng.gen_range(-1.0..1.0);
            }
        }
        for i in 0..self.hidden_weights.len() {
            if rng.gen_range(0.0..1.0) < mutation_rate {
                self.hidden_weights[i] += rng.gen_range(-1.0..1.0);
            }
        }
        for i in 0..self.output_weights.len() {
            if rng.gen_range(0.0..1.0) < mutation_rate {
                self.output_weights[i] += rng.gen_range(-1.0..1.0);
            }
        }
    }
}