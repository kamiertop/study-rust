#[cfg(test)]
#[allow(unused)]
mod string {
	#[test]
	fn main() {
		let greeting: &str = "Hello";
		let sub_str = &greeting[2..];
		println!("{:?}", sub_str);
	}

	#[test]
	fn string() {
		let mut s = String::new();
		s.push_str("hello");
		println!("s: {}", s);
	}
}