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

// impl<K: AddAssign + Copy + SubAssign + MulAssign> Vector<K> {
//     fn add(&mut self, v: &Vector<K>) {
// 		for i in 0..self.vec.len() {
// 			self.vec[i] += v.vec[i];
// 		}
// 	}
//     fn sub(&mut self, v: &Vector<K>) {
// 		for i in 0..self.vec.len() {
// 			self.vec[i] -= v.vec[i];
// 		}
// 	}
//     fn scl(&mut self, a: K) {
// 		for i in 0..self.vec.len() {
// 			self.vec[i] *= a;
// 		}
// 	}
// }

trait MulAdd {
    fn mul_add(self, a: Self, b: Self) -> Self;
}

impl MulAdd for f32 {
    fn mul_add(self, a: f32, b: f32) -> f32 {
        f32::mul_add(self, a, b)
    }
}

fn linear_combination<K: Clone + Default + Copy + MulAdd>(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
    let mut res: Vector<K> = Vector {vec: vec![K::default(); u[0].vec.len()]};
    for idx in 0..u[0].vec.len() {
        for i in 0..coefs.len() {
            res.vec[idx] = coefs[i].mul_add(u[i].vec[idx], res.vec[idx]);
        }
    }
    res
}

fn main() {
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);

    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);

    println!("{}", linear_combination(&[e1, e2, e3], &[10., -2., 0.5]));
    // [10.0]
    // [-2.0]
    // [0.5]

    println!("{}", linear_combination(&[v1, v2], &[10., -2.]));
    // [10.0]
    // [0.0]
    // [230.0]
}