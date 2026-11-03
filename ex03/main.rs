// use std::ops::{AddAssign, SubAssign, MulAssign};
trait MulAdd {
    fn mul_add(self, a: Self, b: Self) -> Self;
}

impl MulAdd for f32 {
    fn mul_add(self, a: f32, b: f32) -> f32 {
        f32::mul_add(self, a, b)
    }
}

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


impl<K: Clone + Copy + MulAdd + Default> Vector<K> {
	fn dot(&self, v: &Vector<K>) -> K {
		let mut acc = K::default();
		for i in 0..self.vec.len() {
			acc = self.vec[i].mul_add(v.vec[i], acc);
		}
		acc
	}

}


fn main() {
    let u = Vector::from([0., 0.]);
    let v = Vector::from([1., 1.]);
    println!("{}", u.dot(&v));  // 0

    let u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    println!("{}", u.dot(&v));  // 2

    let u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    println!("{}", u.dot(&v));  // 9
}