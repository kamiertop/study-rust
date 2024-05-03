#[cfg(test)]
mod tests {
	trait Cook {
		fn start(&self);
	}

	trait Wash {
		fn start(&self);
	}

	struct Chef;

	// struct Chef {
	//
	// }
	impl Cook for Chef {
		fn start(&self) {
			println!("cook");
		}
	}

	impl Wash for Chef {
		fn start(&self) {
			println!("cook");
		}
	}

	#[test]
	fn main() {
		let me = Chef;
		// me.start();	此时不知道要调用谁
		<dyn Cook>::start(&me);
		<Chef as Wash>::start(&me);
	}

	struct T(usize);

	impl T {
		fn get1(&self) -> usize { self.0 }
		fn get2(&self) -> usize { self.0 }
	}

	fn get3(t: &T) -> usize { t.0 }

	fn check_type(_: fn(&T) -> usize) {}

	#[test]
	fn main1() {
		check_type(T::get1);
		check_type(T::get2);
		check_type(get3);
	}
}