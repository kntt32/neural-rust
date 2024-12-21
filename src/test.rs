use crate::matrix::Matrix;

#[test]
fn matrix_dot_test() {
    let mut mat1 = Matrix::new(2, 2);
    mat1[0][0] = 1;
    mat1[0][1] = 2;
    mat1[1][0] = 3;
    mat1[1][1] = 4;

    let mut mat2 = Matrix::new(2, 2);
    mat2[0][0] = 5;
    mat2[0][1] = 6;
    mat2[1][0] = 7;
    mat2[1][1] = 8;

    let mut dot = Matrix::new(2,2);
    dot[0][0] = 19;
    dot[0][1] = 22;
    dot[1][0] = 43;
    dot[1][1] = 50;

    assert_eq!(mat1.dot(&mat2), dot);
}

