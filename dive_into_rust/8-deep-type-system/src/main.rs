fn main() {
	call_fn(std::process::exit, 0);
}

#[allow(unused)]
fn call_fn<T, F: Fn(i32) -> T>(f: F, arg: i32) -> T {
	f(arg)
}

#[test]
fn option() {
	use std::mem::size_of;
	println!("size of isize            : {}", size_of::<isize>());
	println!("size of Option<isize>    : {}", size_of::<Option<isize>>());
	println!("size of &isize           : {}", size_of::<&isize>());
	println!("size of Box<isize>       : {}", size_of::<Box<isize>>());
	println!("size of Option<&isize>     : {}", size_of::<Option<&isize>>());
	println!("size of Option<Box<isize>> : {}", size_of::<Option<Box<isize>>>());
	println!("size of *const isize     : {}", size_of::<*const isize>());
	println!("size of Option<*const isize> : {}", size_of::<Option<*const isize>>());
}