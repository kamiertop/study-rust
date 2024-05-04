#[cfg(test)]
mod if_let {
	#[test]
	fn if_let() {
		let v = Option::Some(3);
		match v {
			None => {}
			Some(_x) => {}
		}
	}

	#[allow(unused)]
	fn value(v: Option<i32>) {
		match v {
			Some(_x) => { println!("some") }
			_ => {
				println!()
			}
		}
		if let Some(_x) = v {
			println!("some")
		} else {
			print!("");
		}
	}

	struct T {
		item1: char,
		item2: bool,
	}

	#[allow(unused)]
	fn test(T { item1: arg1, item2: arg2 }: T) {
		println!("{} {}", arg1, arg2)
	}

	#[test]
	fn main() {
		let x = T {
			item1: 'A',
			item2: false,
		};
		test(x);
	}
}