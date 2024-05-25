fn main() {
	println!("编译器生成匿名struct类型, 通过自动分析closure的内部逻辑, 来决定该结构体包括哪些数据, 以及这些数据如何初始化");
	println!("如果一个外部变量在闭包内, 只通过借用指针&使用, 那么这个变量就可以通过&的方式捕获");
	println!("如果一个外部变量在闭包内, 通过&mut指针使用过, 那么这个变量就需要通过&mut的方式捕获");
	println!("如果一个外部变量在闭包中, 通过所有权转移的方式使用过, 那么这个变量就需要使用by value self的方式捕获");
	println!("编译器自动选择对外部影响最小的类型存储, 尽可能先选择&T, 其次&mut T, 最后选择T");
	println!("闭包依靠trait实现");
}

#[test]
fn test1() {
	let add_func = |a: i32, b: i32| -> i32 {
		a + b
	};
	let x = add_func(1, 2);
	println!("x: {}", x);
	let f1 = |a: i32,
			  b: i32| -> i32{ a + b };

	let f2 = |a: i32, b: i32| a + b;
	f1(1, 2);
	f2(2, 3);
}

#[test]
fn test2() {
	let x = 1_i32;
	let inner_add = || x + 1;
	let result = inner_add();
	println!("result: {}", result);
}

#[test]
fn test3() {
	let option = Some(2);
	let new: Option<i32> = option.map(multiple2);
	println!("new: {:?}", new);
	fn multiple2(val: i32) -> i32 { val * 2 }
	let new1 = option.map(|val| val * 2);
	println!("new1: {:?}", new1);
}

#[test]
fn test4() {
	let x = 10_i32;
	let add_x_fn = |a: i32| x + a;
	let result = add_x_fn(100);
	println!("result: {}", result);
}

#[test]
fn test5() {
	fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
		Box::new(move |y| x + y)
	}
	let f = make_adder(100);
	println!("f(1): {}", f(1));
}
