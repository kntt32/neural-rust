use std::fmt::{Debug, Display, Error, Formatter};
use std::iter::Iterator;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Debug)]
pub struct Matrix<T> {
    width: usize,
    height: usize,
    vec: Vec<T>,
}

impl<T: Clone + Default> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut vec = Vec::new();
        vec.resize(width * height, Default::default());
        Matrix {
            width: width,
            height: height,
            vec: vec,
        }
    }
}

impl<T> Matrix<T> {
    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }
}

impl<T: Copy> Matrix<T> {
    pub fn map(&mut self, f: impl Fn((usize, usize), T) -> T) {
        let width = self.width;
        let height = self.height;

        for y in 0..height {
            let row = &mut self[y];
            for x in 0..width {
                row[x] = f((x, y), row[x]);
            }
        }
    }
}

impl<T> Matrix<T> {
    pub fn rows(&self) -> MatrixIter<'_, T> {
        MatrixIter {
            matrix: self,
            index: 0,
        }
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index * self.width..(index + 1) * self.width]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index * self.width..(index + 1) * self.width]
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.vec[index.0 + self.width * index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.vec[index.0 + self.width * index.1]
    }
}

impl<T: AddAssign + Copy> AddAssign<&Self> for Matrix<T> {
    fn add_assign(&mut self, rhs: &Self) {
        if (self.width, self.height) == (rhs.width, rhs.height) {
            let width = self.width;
            let height = self.height;

            for y in 0..height {
                let self_row = &mut self[y];
                let rhs_row = &rhs[y];
                for x in 0..width {
                    self_row[x] += rhs_row[x];
                }
            }
        } else {
            panic!("invalid input")
        }
    }
}

impl<T: AddAssign + Copy> Add<&Self> for Matrix<T> {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: SubAssign + Copy> SubAssign<&Self> for Matrix<T> {
    fn sub_assign(&mut self, rhs: &Self) {
        if (self.width, self.height) == (rhs.width, rhs.height) {
            let width = self.width;
            let height = self.height;

            for y in 0..height {
                let self_row = &mut self[y];
                let rhs_row = &rhs[y];
                for x in 0..width {
                    self_row[x] -= rhs_row[x];
                }
            }
        } else {
            panic!("invalid input")
        }
    }
}

impl<T: SubAssign + Copy> Sub<&Self> for Matrix<T> {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Matrix<T> {
    fn mul_assign(&mut self, rhs: T) {
        let width = self.width;
        let height = self.height;

        for y in 0..height {
            let row = &mut self[y];
            for x in 0..width {
                row[x] *= rhs;
            }
        }
    }
}

impl<T: MulAssign + Copy> Mul<T> for Matrix<T> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: AddAssign<<T as Mul>::Output> + Mul + Copy + Default> Matrix<T> {
    pub fn dot(&mut self, rhs: &Self) -> Self {
        if self.width == rhs.height {
            let width = rhs.width;
            let height = self.height;
            let add_times = self.width;

            let mut matrix = Matrix::new(width, height);

            for x in 0..width {
                for y in 0..height {
                    for i in 0..add_times {
                        matrix[y][x] += self[y][i] * rhs[i][x];
                    }
                }
            }

            matrix
        } else {
            panic!("invalid input")
        }
    }
}

impl<T: Display + Debug> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[\n")?;
        for y in self.rows() {
            write!(f, "  {:?},\n", y)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct MatrixIter<'a, T> {
    matrix: &'a Matrix<T>,
    index: usize,
}

impl<'a, T> Iterator for MatrixIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index != self.matrix.height {
            self.index += 1;
            Some(&self.matrix[self.index - 1])
        } else {
            None
        }
    }
}
