use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
	let stdin = std::io::stdin();
	let handle = stdin.lock();
	let reader = BufReader::new(handle);
	for line in reader.lines() {
		let line = line.unwrap();
		if line.is_empty() {
			println!("empty");
		}
		println!("read a line: {}", line);
	}
}


fn iter_bytes<T: AsRef<[u8]>>(arg: T) {
	for i in arg.as_ref() {
		println!("{}", i);
	}
}

#[test]
fn as_ref() {
	let s = String::from("this is a string");
	let v = vec![1, 2, 3, 4, 5];
	let c = "hello";
	iter_bytes(s);
	println!("==========");
	iter_bytes(v);
	println!("==========");
	iter_bytes(c);
}

#[test]
fn borrow() {}

#[test]
fn from_info() {
	let s = "hello world";
	let str1: String = s.into();
	println!("str1: {}", str1);
}

#[cfg(test)]
mod test {
	use std::ops::Add;

	#[derive(Copy, Clone, Debug, PartialEq)]
	struct Complex {
		a: i32,
		b: i32,
	}

	impl Add for Complex {
		type Output = Complex;
		fn add(self, rhs: Self) -> Complex {
			Complex {
				a: self.a + rhs.a,
				b: self.b + rhs.b,
			}
		}
	}

	#[test]
	fn main() {
		let c1 = Complex { a: 1, b: 2 };
		let c2 = Complex { a: 1, b: 2 };
		println!("{:?}", c1 + c2);
	}
}

#[test]
fn path() {
	let mut buf = PathBuf::from("/");
	buf.set_file_name("bar");
	if let Some(s) = buf.to_str() {
		println!("s: {}", s);
	} else {
		println!("invalid path");
	}
}

#[cfg(test)]
mod reflect {
	use std::any::Any;
	use std::fmt::Display;

	fn log<T: Any + Display>(value: &T) {
		let value_any = value as &dyn Any;
		if let Some(s) = value_any.downcast_ref::<String>() {
			println!("string: {}", s);
		} else if let Some(i) = value_any.downcast_ref::<i32>() {
			println!("i32: {}", i);
		} else {
			let type_id = value_any.type_id();
			println!("unknown type {:?} {}", type_id, value);
		}
	}

	fn do_work<T: Any + Display>(value: &T) {
		log(value);
	}

	#[test]
	fn main() {
		let my_string = "Hello World".to_string();
		do_work(&my_string);
		let my_i32: i32 = 100;
		do_work(&my_i32);
	}
}