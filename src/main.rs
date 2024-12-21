use neural_rust::matrix::Matrix;
use neural_rust::neuron::{activate_functions::relu_f32::*, Layer, Model};

fn main() {
    let mut nnet: Model<f32> = Model::new();
    let mut layer: Layer<f32> = Layer::new(2, 1, relu, relu_diff);

    let weight = Matrix::from([[1.0, 2.0]]);
    let bias = Matrix::from([[3.0]]);

    layer.set_weight(weight);
    layer.set_bias(bias);
    nnet.push(layer);

    let input = Matrix::from([[4.0], [5.0]]);

    println!("{}", nnet.run(input));
}
