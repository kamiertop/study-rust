- trait中可以包含: 函数, 常量, 类型等

```rust
trait Shape {
	fn area(&self) -> f64;
}
```

- 所有的trait中都有一个隐藏的类型 `Self`, 代表当前这个实现了此trait的具体类型, trait中定义的函数, 也可以称作关联函数
	- 函数的第一个参数如果是Self相关的类型, 且命名为self, 这个参数可以被称为receiver, 具有receiver参数的函数,
	  我们称之为方法(method), 可以通过变量实例使用小数点来调用
	- 没有 `receiver`参数的函数, 我们称为"静态函数", 可以通过类型加双冒号"::"的方式来调用
	- 函数和方法没有本质区别

```rust
// T和T1中的写法是一样的
trait T {
	fn method1(self: Self);
	fn method2(self: &Self);
	fn method3(self: &mut Self);
}

trait T1 {
	fn method1(self);
	fn method2(&self);
	fn method3(&mut self);
}
```

- `impl trait for strut` 类比Go中的实现`interface`

```rust
struct T {}

impl T {
	// 内在方法, 类比Go中的method
	fn method(&self) {}
}
```

- trait中可以包含方法的默认实现
- 如果一个类型既有内在方法, 又有成员方法, 使用`.`调用, 默认会调用内在方法
- 没有receiver参数的方法, 称为静态方法, 通过`Type::FunctionName()`调用, 避免trait的成员方法和T的成员方法重名
- 扩展方法
	- 规则: Coherence Rule(一致性规则)/Orphan Rule(孤儿规则)
	- impl块要么与trait的声明在同一个crate中, 要么与类型的声明在同一个crate中
	- rustc --explain E0117
	- rustc --explain E0210
- 完整调用语法提供无歧义的调用方法: `<T as TraitName>::item`
- 成员说法跟普通的静态方法的唯一区别是: 第一个参数是self
- 类型会优先调用类型自身的方法, 而不是trait中的方法(Rust优先选择类型)
- 实现了Display trait的类型, 才能用{}格式控制打印出来
- 实现了Debug trait的类型, 才能用{:?} {:#?}格式控制打印出来
- Display假定了这个类型可以用utf-8格式的字符串表示
- 所有实现了Display trait的类型, 都自动实现了std::string::ToString
- 但凡编译阶段能确定大小的类型, 都满足Sized约束.