#[cfg(test)]
#[allow(unused)]
mod test_match {
	#[derive(Debug)]
	#[allow(dead_code)]
	// #[non_exhaustive]
	enum Char {
		A,
		B,
		C,
		D,
		E,
	}

	fn print(x: Char) {
		match x {
			Char::A => println!("A"),
			Char::B => { println!("B"); }
			Char::C => { println!("C"); }
			Char::D => { println!("D"); }
			_ => {}
		}
	}

	fn print1(x: Char) {
		match x {
			Char::A => {}
			//使用下划线表示其他情况
			_ => {
				println!("other");
			}
		}
	}

	// match也是表达式
	#[allow(unused)]
	fn return_value(x: Char) -> i32 {
		match x {
			Char::A => { 10 }
			Char::B => { 20 }
			Char::C => { 30 }
			Char::D => 40,
			Char::E => 50
		}
	}

	fn match_value(x: i32) {
		match x {
			-1 | -2 => println!("negative"),
			0 => println!("zero"),
			1 => println!("positive"),
			_ => { println!("error") }
		}
	}

	fn match_value1(x: i32) {
		match x < 0 {
			true => println!("true"),
			false => println!("false"),
		}
	}

	fn match_range(x: char) {
		match x {
			'a'..='z' => println!("lowercase"),
			'A'..='Z' => println!("uppercase"),
			_ => { println!("something else"); }
		}
	}

	#[test]
	fn main() {
		let x = Char::A;
		print(x);
		print1(Char::B);
		// print1(Char::C);
		// print1(Char::D);
	}

	struct P(i32, i32, i32);

	fn calculate(P(x, _, y): P) -> i32 {
		x + y
	}

	fn calculate1(P(x, .., y): P) -> i32 {
		x + y
	}

	fn calculate2(P(x, ..): P) -> i32 {
		x
	}


	#[test]
	fn guards() {
		enum OptionalInt {
			Value(i32),
			Missing,
		}
		let x = OptionalInt::Value(54);
		match x {
			// 这里使用if作为"匹配看守"(match guards), 匹配成功符合if时才会执行后面的语句
			OptionalInt::Value(i) if i > 5 => println!("match int, and > 5"),
			// OptionalInt::Value(..) => println!("i < 5="),
			_ => println!("i < 5"),
			OptionalInt::Missing => println!("missing")
		}
		// 编译器无法做到数学计算, 即使我们已经匹配完成了
		let i = 10;
		match i {
			i if i > 4 => println!("bigger than 4"),
			i if i <= 4 => println!("small or equal to 4"),
			_ => {}
		}
	}

	#[test]
	fn bind() {
		let x = 1;
		match x {
			e @ 1..=4 => println!("got a range item {} {} ", e, x),
			_ => println!("other"),
		}
		let x = Some(100);
		if let Some(value) = x { println!("{}", value) }


		let x = 2;

		match x {
			mut e @ 1..=5 => {
				e += 10;
				println!("e: {}", e);
			}
			_ => println!("anything"),
		}
		let x = Some(42);

		match x {
			Some(v @ 1..=100) => println!("The value {} is in range 1 to 100", v),
			Some(_) => println!("The value is not in range 1 to 100"),
			None => println!("No value"),
		}
	}

	// #[test]
	fn deep_match(v: Option<Option<i32>>) -> Option<i32> {
		match v {
			Some(r @ Some(1..=4)) => r,
			_ => None
		}
	}

	// #[test]
	// fn ref_mut() {
	// 	let x = 4_i32;
	// 	match x {
	// 		// 使用ref避免出现所有权转移
	// 		ref r => println!("Got a reference to {}", r),    // r的类型是: &i32
	// 	}
	// 	let ref x = 5_i32;
	// 	print_type_name(&x);
	// }
	//
	// #[feature(core_intrinsics)]
	// fn print_type_name<T>(_arg: &T) {
	// 	unsafe {
	// 		println!("{}", std::intrinsics::type_name::<T>());
	// 	}
	// }

	#[test]
	fn ref_mut1() {
		let mut x: Option<String> = Some("hello".into());
		match &mut x {
			None => {}
			Some(i) => {
				i.push_str("world")
			}
		}
		println!("x: {:?}", x);
	}
}