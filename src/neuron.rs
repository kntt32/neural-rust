use crate::matrix::Matrix;
use std::ops::{AddAssign, Mul};

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
        let end_layer = &self.vec[self.len() - 1];
        
        if end_layer.output_len() == layer.input_len() {
            self.vec.push(layer);
        }else {
            panic!("invalid input");
        }
    }
}

#[derive(Clone, Debug)]
pub struct Layer<T> {
    nodes: usize,
    weights: Matrix<T>,
    bias: Matrix<T>,
    activate: fn(&mut Matrix<T>),
    activate_diff: fn(&mut Matrix<T>),
}

impl<T: Clone + Default> Layer<T> {
    pub fn new(input_len: usize, output_len: usize, activate: fn(&mut Matrix<T>), activate_diff: fn(&mut Matrix<T>)) -> Self {
        Layer {
            nodes: output_len,
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

impl<T: Copy> Layer<T> {
    pub fn parm_set(&mut self, f: impl Fn() -> T) {
        self.weights.map(| _, _ | f());
    }
}

impl<T: AddAssign + AddAssign<<T as Mul>::Output> + Mul + Copy + Default> Layer<T> {
    pub fn output(&mut self, input: &Matrix<T>) -> Matrix<T> {
        if input.width() == 1 && input.height() == self.input_len() {
            self.weights.dot(input) + &self.bias
        }else {
            panic!("invalid input")
        }
    }

    pub fn delta() -> Matrix<T> {
        todo!()
    }
}

