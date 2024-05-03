mod static_method;
mod extend_method;
mod complete_call;
mod method_test;
mod derive;
mod trait_alias;

fn main() {
	let c = Circle { val: 0.1_f64 };
	println!("area {}", c.area());
	println!("method: {}", c.method());
}

#[allow(unused)]
trait T {
	// self1是一个变量, method0和method1是一样的
	fn method0(self1: Self);
	fn method1(self: Self);
	fn method2(self: &Self);
	fn method3(self: &mut Self);
}

#[allow(unused)]
trait T1 {
	fn method1(self);
	fn method2(&self);
	fn method3(&mut self);
}

trait Shape {
	fn area(&self) -> f64;
}

struct Circle {
	val: f64,
}

impl Shape for Circle {
	fn area(&self) -> f64 {
		return self.val * self.val;
	}
}

impl Circle {
	fn method(&self) -> f64 {
		self.val
	}
}

#[cfg(test)]
mod test1 {
	// self具有特殊意义, trait中的声明和实现必须一样, 没特殊需求, 保持一致
	trait Shape {
		fn area(self: Box<Self>) -> f64;
	}

	struct Circle {
		radius: f64,
	}

	impl Shape for Circle {
		fn area(self: Box<Self>) -> f64 {
			std::f64::consts::PI * self.radius * self.radius
		}
	}

	impl Circle {
		fn area(self: Box<Self>) -> f64 {
			return 100_f64;
		}
	}

	#[test]
	fn main() {
		let b = Box::new(Circle { radius: 4_f64 });
		println!("area: {}", b.area());
	}
}

#[cfg(test)]
mod test2 {
	trait Shape1 { fn area1(&self) -> f64; }

	trait Round {
		fn get_radius(&self) -> f64;
	}

	struct Circle {
		radius: f64,
	}

	impl Round for Circle { fn get_radius(&self) -> f64 { self.radius } }

	// 注意这里是 impl Trait for Trait
	impl Shape1 for dyn Round {
		fn area1(&self) -> f64 {
			std::f64::consts::PI * self.get_radius() * self.get_radius()
		}
	}

	#[test]
	fn main() {
		let b = Circle {
			radius: 10.0,
		};
		// 编译错误, 因为Circle只实现了Round
		// b.area1();	// 虽然没有报错, 但是写的时候IDE没有弹出, 侧面说明是错误的.
		println!("get radius {}", b.get_radius());

		// let a = Circle { radius: 100_f64 } as Round; 编译器不知道Circle的内存大小, 也不知道Round的内存大小, 因为实现Round的struct的大小是不确定的, 所以需要使用Box包裹, 同时需要使用dyn关键字

		let c = Box::new(Circle { radius: 4f64 }) as Box<dyn Round>;
		println!("use as to convert: {}", c.area1());
	}
}

#[cfg(test)]
mod test3 {
	trait Shape {
		fn area(&self) {
			println!("area");
		}
	}

	struct Circle;

	impl Shape for Circle {}

	#[test]
	fn main() {
		let b = Circle;
		// area有接收者, 才可以使用类型调用
		b.area()
	}
}