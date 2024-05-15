fn main() {}

/*
- 实现机制:
	- unwind: 一层一层退出函数调用栈
	- abort: 直接退出整个程序
- 通过 -C panic=unwind 指定实现方式
- panic safety
- panic并不意味着内存不安全, 它是组织内存不安全的利器. 
*/

#[test]
fn panic() {
	use std::panic;

	let result = panic::catch_unwind(|| {
		println!("hello!");
	});
	assert!(result.is_ok());

	let result = panic::catch_unwind(|| {
		panic!("oh no!");
	});
	assert!(result.is_err());
}