use neural_rust::matrix::Matrix;

fn main() {
    let mut matrix = Matrix::new(2, 2);
    matrix[0][0] = 3;
    matrix[1][0] = 1;
    matrix[0][1] = 5;
    matrix[(1, 1)] = 2;

    let mut matrix2 = Matrix::new(2, 2);
    matrix2[0][0] = 1;
    matrix2[0][1] = 2;
    matrix2[1][0] = 3;
    matrix2[1][1] = 4;
    println!("{}", matrix);
    println!("{}", matrix2);
    println!("{}", matrix.dot(&matrix2));
}
