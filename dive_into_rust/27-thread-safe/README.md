- 无法在多线程中直接读写普通的共享变量, 除非使用Rust提供的线程安全相关的设施
- `std::marker::Sync`: 类型T实现了Sync, 在不同的线程中使用&T访问同一个变量是安全的
- `std::marker::Send`: 如果类型T实现了Send, 说明这个类型的变量在不同的线程中传递所有权是安全的

```rust
#[stable(feature = "rust1", since = "1.0.0")]
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
	where
		F: FnOnce() -> T,
		F: Send + 'static,
		T: Send + 'static,
{
	Builder::new().spawn(f).expect("failed to spawn thread")
}
```