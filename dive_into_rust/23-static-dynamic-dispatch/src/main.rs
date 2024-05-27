fn main() {
	println!("静态分派: 具体调用哪个函数, 在编译阶段就确定下来, Rust中依靠泛型以及impl trait来完成");
	println!("动态分派: 具体调用哪个函数, 在执行阶段才确定, 依靠Trait Object来完成, 本质上是指针, 可以指向不同的类型");
	println!("指向trait的指针就是trait object, 一个胖指针, 包含一个指向所谓的虚函数表的指针");
	println!("如果一个类型实现了多个trait, 那么不同的trait object指向的虚函数表也不一样");
	println!("哪些条件trait object无法够搞出来");
	println!("1. 当trait有Self:Sized约束时, trait当做类型来看, 不满足Sized条件, 因为trait只描述了公共的部分, 并不描述内部的具体实现");
	println!("2. 当函数中有Self类型作为参数或者返回类型时");
	println!("3. 当函数第一个参数不是self时");
	println!("4. 当函数有泛型参数时");

	println!("函数无法直接返回一个闭包, 因为闭包的类型是编译器自动生成的一个匿名类型, 没办法在函数的返回类型中手工指定");
}

#[cfg(test)]
mod test1 {
	use std::mem;

	trait Bird {
		fn fly(&self);
	}

	struct Duck;

	struct Swan;

	impl Bird for Duck {
		fn fly(&self) {
			println!("duck fly");
		}
	}

	impl Bird for Swan {
		fn fly(&self) {
			println!("swan fly")
		}
	}

	// 通过泛型函数实现的多态, 在编译阶段就已经确定了调用哪个版本的函数, 因此被成为静态分派
	#[allow(unused)]
	fn test1<T: Bird>(arg: T) {
		arg.fly();
	}

	// 通过指针实现, 动态分配, 只知道可以调用Bird trait方法
	#[allow(unused)]
	fn test2(arg: Box<dyn Bird>) {
		arg.fly();
	}

	#[allow(unused)]
	fn print_trait_object(p: &dyn Bird) {
		let (data, vtable): (usize, *const usize) = unsafe { mem::transmute(p) };
		println!("trait object [data:{}, vtable: {:p}]", data, vtable);
		unsafe {
			println!("data in vtable [{}, {}, {}, {}]", *vtable, *vtable.offset(1), *vtable.offset(2), *vtable.offset(3));
		}
	}

	#[test]
	fn main() {
		let duck = Duck;
		let p_duck = &duck;
		let p_bird = p_duck as &dyn Bird;
		println!("size of p_duck {}, size of p_bird {}", mem::size_of_val(&p_duck), mem::size_of_val(&p_bird));
		let duck_fly: usize = Duck::fly as usize;
		let swan_flg: usize = Swan::fly as usize;
		println!("duck_fly: {}", duck_fly);
		println!("swan_flg: {}", swan_flg);

		print_trait_object(p_bird);
		let swan = Swan;
		print_trait_object(&swan as &dyn Bird);
	}
}

#[cfg(test)]
mod test2 {
	#[allow(unused)]
	fn foo(n: u32) -> impl Iterator<Item = u32> {
		(0..n).map(|x| x * 10)
	}

	#[allow(unused)]
	fn consume_iter_static<I: Iterator<Item = u8>>(_iter: I) {}

	#[allow(unused)]
	fn consumer_iter_dynamic(iter: Box<dyn Iterator<Item = u8>>) {}

	// fn produce_iter_dynamic() -> Box<dyn Iterator<Item=u8>>
	// fn produce_iter_static()->Iterator<Item=u8>{} 编译时没有已知常量大小

	// multiply 返回一个闭包
	// 泛型函数的类型参数是函数的调用者指定的, impl trait的具体类型是函数的函数体指定的
	fn multiply(m: i32) -> impl Fn(i32) -> i32 {
		move |x| x * m
	}

	#[test]
	fn main() {
		let f = multiply(10);
		println!("f(2): {}", f(2));
	}
}