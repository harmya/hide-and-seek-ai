use ndarray::Array2;
use rand::Rng;
use rand_distr::{Distribution, Normal};

pub struct NeuralNetwork {
    input_weights: Array2<f64>,
    hidden_weights: Array2<f64>,
    output_weights: Array2<f64>,
}
