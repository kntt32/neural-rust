use crate::matrix::Matrix;
use std::ops::{AddAssign, Mul};

pub mod activate_functions;

#[derive(Clone, Debug)]
pub struct Model<T> {
    vec: Vec<Layer<T>>,
}

impl<T> Model<T> {
    pub fn new() -> Self {
        Model { vec: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn push(&mut self, layer: Layer<T>) {
        if 0 < self.len() {
            let end_layer = &self.vec[self.len() - 1];

            if end_layer.output_len() == layer.input_len() {
                self.vec.push(layer);
            } else {
                panic!("invalid input");
            }
        } else {
            self.vec.push(layer);
        }
    }
}

impl<T: AddAssign + AddAssign<<T as Mul>::Output> + Mul + Copy + Default> Model<T> {
    pub fn run(&mut self, input: Matrix<T>) -> Matrix<T> {
        let mut matrix = input;

        for layer in &mut self.vec {
            matrix = layer.output(&matrix);
        }

        matrix
    }
}

#[derive(Clone, Debug)]
pub struct Layer<T> {
    weights: Matrix<T>,
    bias: Matrix<T>,
    activate: fn(&mut Matrix<T>),
    activate_diff: fn(&mut Matrix<T>),
}

impl<T: Clone + Default> Layer<T> {
    pub fn new(
        input_len: usize,
        output_len: usize,
        activate: fn(&mut Matrix<T>),
        activate_diff: fn(&mut Matrix<T>),
    ) -> Self {
        Layer {
            weights: Matrix::new(input_len, output_len),
            bias: Matrix::new(1, output_len),
            activate: activate,
            activate_diff: activate_diff,
        }
    }
}

impl<T> Layer<T> {
    pub fn input_len(&self) -> usize {
        self.weights.width()
    }

    pub fn output_len(&self) -> usize {
        self.bias.height()
    }
}

impl<T> Layer<T> {
    pub fn set_weight(&mut self, value: Matrix<T>) {
        if (self.weights.width(), self.weights.height()) == (value.width(), value.height()) {
            self.weights = value;
        } else {
            panic!("invalid input");
        }
    }

    pub fn set_bias(&mut self, value: Matrix<T>) {
        if (1, self.bias.height()) == (value.width(), value.height()) {
            self.bias = value;
        } else {
            panic!("invalid input");
        }
    }
}

impl<T: AddAssign + AddAssign<<T as Mul>::Output> + Mul + Copy + Default> Layer<T> {
    pub fn output(&mut self, input: &Matrix<T>) -> Matrix<T> {
        if input.width() == 1 && input.height() == self.input_len() {
            let mut u = self.weights.dot(input) + &self.bias;

            let width = u.width();
            let height = u.height();

            (self.activate)(&mut u);

            if (u.width(), u.height()) == (width, height) {
                u
            } else {
                panic!("invalid activation function");
            }
        } else {
            panic!("invalid input")
        }
    }

    pub fn delta() -> Matrix<T> {
        todo!()
    }
}
