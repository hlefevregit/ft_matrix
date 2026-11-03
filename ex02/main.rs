// use std::ops::{AddAssign, SubAssign, MulAssign};

struct Vector<K> {
	vec: Vec<K>,
}

impl<K: Clone, const N: usize> From<[K; N]> for Vector<K> {
	fn from(arr: [K; N]) -> Self {
		Vector {vec: arr.to_vec()}
	}
}

impl<K: std::fmt::Display> std::fmt::Display for Vector<K> { 
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for i in self.vec.iter() {
			write!(f, "[{}]\n", i)?;
		}
		Ok(())
	}
}

struct Matrix<K> {
	matrix: Vec<Vec<K>>,
}

impl<K: Clone, const N: usize, const M: usize> From<[[K; N]; M]> for Matrix<K> {
	fn from(arr: [[K; N]; M]) -> Self {
		let mut rows: Vec<Vec<K>> = Vec::new();
		for i in 0..M {
			rows.push(arr[i].to_vec());
		}
		Matrix { matrix: rows }
	}
}

impl<K: std::fmt::Display> std::fmt::Display for Matrix<K> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for i in 0..self.matrix.len() {
			write!(f, "[")?;
			for j in 0..self.matrix[i].len() {
				if j > 0 {
					write!(f, ", ")?;
				}
				write!(f, "{}", self.matrix[i][j])?;
			}
			write!(f, "]\n")?;
		}
		Ok(())
	}
}

trait Lerpable {
	fn scaled(self, t: f32) -> Self;
	fn plus(self, other: Self) -> Self;
}

impl Lerpable for Matrix<f32> {
	fn scaled(mut self, t: f32) -> Self {
		for i in 0..self.matrix.len() {
			for j in 0..self.matrix[i].len() {
				self.matrix[i][j] *= t;
			}
		}
		self
	}
	fn plus(mut self, other: Self) -> Self {
		for i in 0..self.matrix.len() {
			for j in 0..self.matrix[i].len() {
				self.matrix[i][j] += other.matrix[i][j];
			}
		}
		self
	}
}

impl Lerpable for Vector<f32> {
	fn scaled(mut self, t: f32) -> Self {
		for i in 0..self.vec.len() {
			self.vec[i] *= t;
		}
		self
	}
	fn plus(mut self, other: Self) -> Self {
		for i in 0..self.vec.len() {
			self.vec[i] += other.vec[i];
		}
		self
	}
}

impl Lerpable for f32 {
	fn scaled(self, t: f32) -> f32 {
		self * t
	}
	fn plus(self, other: f32) -> f32 {
		self + other
	}
}

fn lerp<V: Lerpable>(u: V, v: V, t: f32) -> V {
	u.scaled(1. - t).plus(v.scaled(t))
}


fn main() {
    println!("{}", lerp(0., 1., 0.));     // 0
    println!("{}", lerp(0., 1., 1.));     // 1
    println!("{}", lerp(0., 1., 0.5));    // 0.5
    println!("{}", lerp(21., 42., 0.3));  // 27.3
    println!("{}", lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3));
    // [2.6]
    // [1.3]
	println!("{}", lerp(
		Matrix::from([[2., 1.], [3., 4.]]),
		Matrix::from([[20., 10.], [30., 40.]]),
		0.5
	));
	// [11, 5.5]
	// [16.5, 22]
}