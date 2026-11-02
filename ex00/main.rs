use std::ops::{AddAssign, SubAssign, MulAssign};

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

impl<K: AddAssign + Copy + SubAssign + MulAssign> Vector<K> {
    fn add(&mut self, v: &Vector<K>) {
		for i in 0..self.vec.len() {
			self.vec[i] += v.vec[i];
		}
	}
    fn sub(&mut self, v: &Vector<K>) {
		for i in 0..self.vec.len() {
			self.vec[i] -= v.vec[i];
		}
	}
    fn scl(&mut self, a: K) {
		for i in 0..self.vec.len() {
			self.vec[i] *= a;
		}
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

impl<K: AddAssign + SubAssign + MulAssign + Copy> Matrix<K> {
	fn add(&mut self, m: &Matrix<K>) {
		for i in 0..self.matrix.len() {
			for j in 0..self.matrix[i].len() {
				self.matrix[i][j] += m.matrix[i][j];
			}
		}
	}
	fn sub(&mut self, m: &Matrix<K>) {
		for i in 0..self.matrix.len() {
			for j in 0..self.matrix[i].len() {
				self.matrix[i][j] -= m.matrix[i][j];
			}
		}
	}
	fn scl(&mut self, m: K) {
		for i in 0..self.matrix.len() {
			for j in 0..self.matrix[i].len() {
				self.matrix[i][j] *= m;
			}
		}
	}
}

fn main() {
	let mut u = Vector::from([2., 3.]);
	let v = Vector::from([5., 7.]);
	u.add(&v);
	println!("{}", u);
	// [7.0]
	// [10.0]
	let mut u = Vector::from([2., 3.]);
	let v = Vector::from([5., 7.]);
	u.sub(&v);
	println!("{}", u);
	// [-3.0]
	// [-4.0]
	let mut u = Vector::from([2., 3.]);
	u.scl(2.);
	println!("{}", u);
	// [4.0]
	// [6.0]
	println!("{}", u);
	// [8.0, 6.0]
	// [1.0, 6.0]



	let mut u = Matrix::from([
		[1., 2.],
		[3., 4.]
	]);
	let v = Matrix::from([
		[7., 4.],
		[-2., 2.]
	]);
	u.add(&v);
	let mut u = Matrix::from([
		[1., 2.],
		[3., 4.]
	]);
	let v = Matrix::from([
		[7., 4.],
		[-2., 2.]
	]);
	u.sub(&v);
	println!("{}", u);
	// [-6.0, -2.0]
	// [5.0, 2.0]
	let mut u = Matrix::from([
		[1., 2.],
		[3., 4.]
	]);
	u.scl(2.);
	println!("{}", u);
	// [2.0, 4.0]
	// [6.0, 8.0]
}