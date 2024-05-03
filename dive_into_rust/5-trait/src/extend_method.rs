#[cfg(test)]
mod tests {
	use crate::Shape;

	#[test]
	fn main() {
		let x: i32 = 10.double();
		println!("x: {}", x);
		let y: f64 = 100.area();
		println!("y: {}", y);
	}

	trait Double {
		fn double(&self) -> Self;
	}

	// 为i32添加方法, 即使这个类型不是自己定义的
	// 有点危险
	impl Double for i32 {
		fn double(&self) -> i32 {
			*self * 2
		}
	}

	impl Shape for i32 {
		fn area(&self) -> f64 {
			100_f64
		}
	}
}