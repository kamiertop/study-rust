#[cfg(test)]
mod tests {
	pub trait Service {
		type Request;
		type Response;
		type Error;
		fn call(&self, req: Self::Request) -> Self::Response;
	}

	// trait HttpService = Service<Error = (), Request = (), Response = ()>;
}

#[cfg(test)]
mod test_display {
	use std::fmt::{Display, Formatter};

	#[derive(Debug)]
	#[allow(dead_code)]
	struct T {
		a: i32,
		b: i32,
	}

	impl Display for T {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			write!(f, "{{{},{}}}", self.a, self.b)
		}
	}

	#[test]
	fn main() {
		let t = T {
			a: 10,
			b: 32,
		};

		println!("t: {}", t);
		println!("t: {:?}", t);
		println!("t: {:#?}", t);
	}
}


#[cfg(test)]
#[allow(unused)]
mod test_restraint {
	use std::fmt::Debug;

	fn my_print<T: Debug>(x: T) {
		println!("{:?}", x);
	}

	fn my_print_where<T>(x: T) where T: Debug {
		println!("{:?}", x);
	}

	trait Base {}

	// Derived 继承了Base, 满足Derived的也必须满足Base
	trait Derived: Base {}

	#[derive(Debug)]
	struct T {}

	impl Base for T {}

	impl Derived for T {}
}

#[cfg(test)]
mod test_self {
	trait A {
		fn hello(self: Self);
	}

	struct T;

	impl A for T {
		fn hello(self: Self) {
			println!("hello");
		}
	}

	#[test]
	fn main() {
		let t = T;
		t.hello();
	}
}