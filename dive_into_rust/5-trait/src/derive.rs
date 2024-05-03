#[cfg(test)]
mod tests {
	#[derive(Copy, Clone, Default, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
	struct Foo {
		data: i32,
	}

	#[test]
	fn main() {
		let v1 = Foo { data: 0 };
		let v2 = v1;
		println!("{:?}", v2);
	}
}