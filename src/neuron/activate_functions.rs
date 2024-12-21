use super::*;

pub mod relu_f32 {
    use super::*;

    pub fn relu(u: &mut Matrix<f32>) {
        u.map(|_, v| if 0.0 <= v { v } else { 0.0 });
    }

    pub fn relu_diff(u: &mut Matrix<f32>) {
        u.map(|_, v| if 0.0 <= v { 1.0 } else { 0.0 });
    }
}
