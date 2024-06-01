fn main() {
	println!("如果一个类型可以安全地从一个线程move进入另一个线程, 那它就是Send类型");
	println!("内部不包含引用的类型, 都是Send");
	println!("大部分具有泛型参数的类型是否满足Sync/Send, 很多都是取决于参数类型是否满足Sync/Send");
	println!("标准库把所有基本类型, 以及标准库中定义的类型, 都做了合适的Send/Sync标记");
	println!("编译器会自动推理类型是否满足Send/Sync");
}