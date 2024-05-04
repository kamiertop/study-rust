mod r#match;
mod r#let;

fn main() {}


#[cfg(test)]
#[allow(unused)]
mod tests {
	struct T1(i32, char);

	struct T2 {
		item1: T1,
		item2: bool,
	}

	#[test]
	fn main() {
		let x = T2 {
			item1: T1(0, 'A'),
			item2: false,
		};
		let T2 {
			item1: T1(value1, value2),
			item2: value3,
		} = x;
		println!("{} {} {}", value1, value2, value3);

		let t1 = T1(100, 'a');
		let T1(value1, value2) = t1;
	}
}