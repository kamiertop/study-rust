#[cfg(test)]
mod tests {
	trait A {
		fn print(&self);
		fn print11(&self) {
			println!("default implement");
		}
	}

	struct T;

	#[allow(unused)]
	impl T {
		fn print() {
			println!("impl T");
		}
		fn print1(&self) {
			println!("impl T: print1");
		}
		fn print111(&self) {
			println!("使用内在方法");
		}
	}

	impl A for T {
		fn print(&self) {}
	}

	#[test]
	fn main() {
		// 类型可以直接调用静态方法
		T::print();

		let t = T;
		T::print1(&t);
		// 此时的print11会使用内在方法
		t.print11();
		T::print111(&t);
		// 两种方式都可以
		A::print11(&t);
		<dyn A>::print11(&t);
	}
}

#[cfg(test)]
mod test1 {
	// 定义一个 trait
	trait Greet {
		// 默认方法
		fn greet(&self) {
			println!("Hello, there!");
		}

		// 非默认方法
		fn greet_custom(&self, message: &str) {
			println!("{}", message);
		}
		fn greet1(this: Self);
	}

	// 实现 trait
	struct Person;

	// 实现 Greet trait，但没有提供额外的实现，因为它们都有默认实现
	impl Greet for Person {
		fn greet1(_: Self) {}
	}

	#[test]
	fn main() {
		let person = Person;

		// 使用默认实现的 greet 方法
		person.greet();
		// 使用默认实现的 greet_custom 方法
		person.greet_custom("Hi, there!");
	}
}

#[cfg(test)]
mod test4 {
	trait Hello {
		fn hello() {}
	}

	struct A;

	impl Hello for A {}

	#[test]
	fn main() {
		<A as Hello>::hello();
	}
}