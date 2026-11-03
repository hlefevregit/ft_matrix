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




impl Vector<f32> {
	fn norm_1(&self) -> f32 {
		let mut res = 0.;
		for i in 0..self.vec.len() {
			let x = self.vec[i];
			res += if x < 0. {-x} else {x};
		}
		res
	}
	fn norm(&self) -> f32 {
		let mut res = 0.;
		for i in 0..self.vec.len() {
			res += self.vec[i].powf(2.);
		}
		res.powf(0.5)
	}

	fn norm_inf(&self) -> f32 {
		let mut res = 0.;
		for i in 0..self.vec.len() {
			let x = self.vec[i];
			let a = if x < 0. { -x } else { x };
			if a > res {res = a;}
		}
		res
	}

}


fn main() {
    let u = Vector::from([1., 2., 3.]);
	println!("NORM 1 => \n{}", u.norm_1());   // 6
	let u = Vector::from([-1., -2.]);
	println!("{}\n", u.norm_1());   // 3

	let u = Vector::from([1., 2., 3.]);
	println!("NORM EUCL =>\n{}", u.norm());   // 3.7416575
	let u = Vector::from([-1., -2.]);
	println!("{}", u.norm());   // 2.236068
	let u = Vector::from([3., 4.]);
	println!("{}\n", u.norm());   // 5

	let u = Vector::from([1., 2., 3.]);
	println!("NORM INF =>\n{}", u.norm_inf());   // 3
	let u = Vector::from([-1., -2.]);
	println!("{}", u.norm_inf());   // 2
	let u = Vector::from([0., 0., 0.]);
	println!("{}\n", u.norm_inf());   // 0
}