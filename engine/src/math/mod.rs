use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::Index;
use crate::err::{Error, ErrorKind};

pub mod parse;
pub mod solve;

// Rows x Columns
#[derive(PartialEq, Clone, Debug)]
pub struct Matrix<const M: usize, const N: usize> {
    pub content: [[f64; N]; M],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn empty() -> Self {
        Matrix {
            content: [[0.0; N]; M],
        }
    }

    pub fn new(content: [[f64; N]; M]) -> Self {
        Matrix { content }
    }

    pub fn plus(&self, other: &Matrix<M, N>) -> Matrix<M, N> {
        let mut new = Matrix::<M, N>::empty();

        for i in 0..M {
            for j in 0..N {
                new.content[i][j] = self.content[i][j] + other.content[i][j];
            }
        }

        new
    }

    pub fn scale(&self, scalar: f64) -> Matrix<M, N> {
        let mut new = Matrix::<M, N>::empty();

        for i in 0..M {
            for j in 0..N {
                new.content[i][j] = scalar * self.content[i][j];
            }
        }

        new
    }

    pub fn multiply<const P: usize>(&self, other: &Matrix<N, P>) -> Matrix<M, P> {
        let mut new = Matrix::<M, P>::empty();

        for m in 0..M {
            for p in 0..P {
                let mut val = 0.0;

                for n in 0..N {
                    val += self.content[m][n] * other.content[n][p]
                }

                new.content[m][p] = val;
            }
        }

        new
    }
}

impl Matrix<2, 2> {
    pub fn det(&self) -> f64 {
        self.content[0][0]*self.content[1][1]-self.content[0][1]*self.content[1][0]
    }
}

impl<const M: usize> Matrix<M, 1> {
    pub fn vector(content: [f64; M]) -> Self {
        let mut vec = Matrix::<M, 1>::empty();

        for (i, x) in content.into_iter().enumerate() {
            vec.content[i][0] = x
        }

        vec
    }
}

impl<const M: usize, const N: usize> Display for Matrix<M, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Columns
        for m in 0..M {
            // Rows

            write!(
                f,
                "{}",
                match m {
                    0 => "⎡",
                    i if i + 1 == M => "⎣",
                    _ => "⎢",
                }
            )?;
            for n in 0..N {
                write!(f, "{}", self.content[m][n])?;
                if n + 1 < N {
                    write!(f, " ")?;
                }
            }

            write!(
                f,
                "{}",
                match m {
                    0 => "⎤",
                    i if i + 1 == M => "⎦",
                    _ => "⎢",
                }
            )?;
            writeln!(f)?;
        }

        Ok(())
    }
}

pub type Column<const M: usize> = Matrix<M, 1>;

pub trait Vector: Clone + Display {
    fn dof() -> usize;
    fn new(value: Vec<f64>) -> Result<Self, Error>;
    fn empty() -> Self;
    fn get(&self, i: usize) -> &f64;
    fn magnitude(&self) -> f64;
    fn unit(&self) -> Self;
    fn plus(&self, other: &Self) -> Self;
    fn scale(&self, scalar: f64) -> Self;
    fn dot(&self, other: &Self) -> f64;
}

impl<const N: usize> Index<usize> for Column<N> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl<const M: usize> Vector for Column<M> {
    fn dof() -> usize {
        M
    }

    fn new(value: Vec<f64>) -> Result<Self, Error> {
        let content: Result<[f64; M], _> = value.try_into();
        if let Ok(content) = content {
            Ok(Matrix::vector(content))
        } else {
            Err(Error::new(ErrorKind::InvalidDimensions))
        }
    }

    fn empty() -> Self {
        Matrix::empty()
    }

    fn get(&self, i: usize) -> &f64 {
        &self.content[i][0]
    }

    fn magnitude(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..M {
            sum += self.get(i).powf(2.0);
        }

        sum.sqrt()
    }

    fn unit(&self) -> Self {
        let x = self.magnitude();

        if x == 0.0 {
            return Self::empty();
        }

        self.scale(1.0 / x)
    }

    fn plus(&self, other: &Self) -> Self {
        self.plus(other)
    }

    fn scale(&self, scalar: f64) -> Self {
        self.scale(scalar)
    }

    fn dot(&self, other: &Self) -> f64 {
        let mut sum = 0.0;
        for i in 0..M {
            sum += self.get(i) * other.get(i);
        }
        sum
    }
}

impl<const D: usize> TryFrom<Vec<f64>> for Matrix<D, 1> {
    type Error = Error;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        let content: Result<[f64; D], _> = value.try_into();
        if let Ok(content) = content {
            Ok(Matrix::vector(content))
        } else {
            Err(Error::new(ErrorKind::InvalidDimensions))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Equation {
    id: u8,
    node: parse::Node,
    // Dependencies on other variables
    dependencies: HashSet<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_display() {
        let a = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);

        println!("{}", a);
    }

    #[test]
    fn test_matrix_scaling() {
        let a = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);

        let res = a.scale(2.0);
        println!("{}", res);
    }

    #[test]
    fn test_matrix_addition() {
        let a = Matrix::new([[1.1, 2.0], [3.0, 4.0]]);
        let b = Matrix::new([[1.0, 1.0], [1.0, 1.0]]);

        let res = a.plus(&b);
        println!("{}", res);
    }

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
        let b = Matrix::new([[5.0, 6.0], [7.0, 8.0]]);

        // Expected Result:
        // [5+14, 6+16  ]    =    [19, 22]
        // [15+28, 18+32]    =    [43, 50]

        let res = a.multiply(&b);
        println!("{}", res);
    }

    #[test]
    fn test_vector_multiplication() {
        let a = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
        let b = Matrix::vector([5.0, 6.0]);

        // Expected Result:
        // [17, 39]

        let res = a.multiply(&b);
        println!("{}", res);
    }
}

pub mod integration {
    // Displacement, velocity

    pub fn leapfrog_displacement(
        delta: f64,
        displacement: f64,
        velocity: f64,
        acceleration: f64,
    ) -> f64 {
        let velocity = velocity * delta;
        let accel = 1.0 / 2.0 * acceleration * delta.powf(2.0);

        displacement + velocity + accel
    }

    pub fn leapfrog_velocity(
        delta: f64,
        velocity: f64,
        acceleration: f64,
        next_acceleration: f64,
    ) -> f64 {
        let accel = 1.0 / 2.0 * (acceleration + next_acceleration) * delta;

        velocity + accel
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_leapfrog() {
            let acceleration = 2.0;
            let mut velocity = 0.0;
            let mut displacement = 0.0;

            for _ in 0..100 {
                displacement = leapfrog_displacement(0.1, displacement, velocity, acceleration);
                velocity = leapfrog_velocity(0.1, velocity, acceleration, acceleration);

                println!("{}, {}", displacement, velocity);
            }
        }
    }
}
