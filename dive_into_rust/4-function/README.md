- 每一个函数都有自己独特的类型, 但是这个类型可以进行转换
- 发散函数, 返回类型是感叹号: `!`
- 发散函数可以被转换为任意类型, 下面的例子中由于发散函数可以转换为任意函数, 所以可以和其他分支相容, 编译器能正确判断p的类型, 可以通过类型检查
```rust
fn main(){
	let b = true;
	let p = if b {
		panic!("panic");
	} else {
		100
	};
}
```
- 以下情况永远不会返回, 它们的类型就是 `!`
    1. `panic!` 以及基于它实现的各种函数/宏, 比如`unimplemented!` `unreachable!`
    2. 死循环`loop{}`
    3. 进程退出函数 `std::process:exit`以及类似的 `libc`中的exec一类函数
- `main`函数的传递参数和返回状态码都由单独的api来完成
```rust
fn main() {
	for arg in std::env::args() {
		println!("Arg: {}", arg);
	}

	std::process::exit(0);
}
// cargo run a b c d
```
- main函数的函数签名为一个泛型函数, 这个函数的返回类型可以是任何一个满足 `Terminationtrait`约束的类型, 其中(), bool, Result都满足, 都可以作为main函数的返回类型
- `const fn`: 函数可以用const关键字修饰, 这样的函数可以在编译阶段被执行, 返回值也被视为编译器常量