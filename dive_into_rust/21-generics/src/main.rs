fn main() {}

#[allow(unused)]
enum OptionGenerics<T> {
	Some(T),
	None,
}


#[test]
fn test1() {
	let _: Option<i32> = Some(21);
	let _: Option<&str> = None;
}

// The default type of T is i32.
#[allow(unused)]
struct Data<T = i32> {
	data: T,
}

#[test]
fn test2() {
	let v1 = Data { data: 0 };
	let v2 = Data::<bool> { data: false };
	println!("v1.data: {}", v1.data);
	println!("v2.data: {}", v2.data);
}

#[test]
fn test_function() {
	fn compare_option<T>(first: Option<T>, second: Option<T>) -> bool {
		match (first, second) {
			(Some(..), Some(..)) => true,
			(None, None) => true,
			_ => false
		}
	}

	println!("compare_option(Some(1_i32),Some(2_i32)): {}", compare_option::<i32>(Some(1_i32), Some(2_i32)));
}

#[test]
fn test3() {
	#[allow(unused)]
	fn max<T: PartialOrd>(a: T, b: T) -> T {
		if a < b {
			b
		} else {
			a
		}
	}
}

trait ConvertTo<T> {
	fn convert(&self) -> T;
}

impl ConvertTo<f32> for i32 {
	fn convert(&self) -> f32 {
		0.3_f32
	}
}

impl ConvertTo<f64> for i32 {
	fn convert(&self) -> f64 {
		0.3_f64
	}
}

impl ConvertTo1 for i32 {
	type Dest = f32;
	fn convert(&self) -> Self::Dest {
		100_f32
	}
}

trait ConvertTo1 {
	// 关联类型
	type Dest;
	fn convert(&self) -> Self::Dest;
}

#[cfg(test)]
mod tests {
	trait Example { fn call(&self); }

	impl<T> Example for T {
		fn call(&self) {
			println!("most generic");
		}
	}

	impl Example for str {
		fn call(&self) {
			println!("specialized for str, {}", self);
		}
	}

	fn main() {
		let v1 = vec![1i32, 2, 3];
		let v2 = 1_i32;
		let v3 = "hello";
		v1.call();
		v2.call();
		v3.call();
	}
}