#[allow(unused)]
fn main1() {
	println!("add tuple: {}",add1((3,4)));
	let a2 = add2;
	println!("result: {}",a2((1,3)));

	// let mut func = add1;
	// func = add2;
	// println!("convert {}",func((1,2)));

	// 使用as进行类型转换
	let mut func = add1 as fn((i32,i32)) -> i32;
	func((2,3));
	func = add2;
	println!("use keyword: `as` to convert {}",func((1,2)));

	let mut func: fn((i32,i32)) -> i32 = add1;
	func((1,2));
	func =  add2;
	println!("displays the claim to convert {}",func((1,2)));
}

#[allow(unused)]
fn add1(t: (i32,i32)) -> i32 {
	t.0 + t.1
}

#[allow(unused)]
fn add2((x,y):(i32,i32)) -> i32 {
	return x+y
}


#[allow(dead_code)]
fn diverges() -> ! {
	panic!("this function never returns")
}

#[test]
fn test1() {
	let x = true;
	let p = if x {
		panic!("此分支返回!类型, 与另一条分支类型相容, 于是编译器可以知道p的类型, 能通过编译器的类型检查")
	} else {
		100
	};
	println!("{}",p);
	let x: i32 = diverges();
}

fn main() {
	for arg in std::env::args() {
		println!("Arg: {}", arg);
	}
	let a = cube(100);
	println!("const fn: {}",a);

	println!("{}", fib(8));
	std::process::exit(0);
}

const fn cube(n: i32) ->i32{
	10 * n
}

fn fib (index:u32) -> u64 {
    if index == 1 || index == 2 {
		1
	} else {
		fib(index-1)+fib(index-2)
	}
}