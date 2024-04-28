- 表达式: expression
- 语句: statement
- 语句也可以是表达式的一部分
  - 一个表达式总是产生一个值
  - 语句不产生值, 它的类型永远是 `()`
- 一个表达式加上分号, 就变成了语句
- 如果把语句放到一个语句块中包起来, 就可以把它当成一个表达式使用
- Rust是一个表达式语言
- Rust表达式可以分为'左值和右值'两类
  - 左值: 这个表达式可以表示一个内存地址, 可以放到赋值运算符左边使用
  - ```rust
    fn main(){
        // 左值表达式, 赋值运算符(=), 右值表达式
        let mut x: i32 = 1;
        x = 2;
    }
    ```
  - 其他都是右值
- 赋值表达式执行的时候, 会把右边表达式的值"复制或者移动"(copy or move)到左边的表达式中
- 赋值表达式的类型永远是 `()`
- 语句块也可以是表达式的一部分
  - 带了分号`;`, 意味着这是一条语句, 类型是 `()`
  - 不带分号, 它的类型就是表达式的类型, 取决于这个语句块的最后一行或者一部分 
```rust
fn main(){
	// x的类型是()
	let x:() = {
		println!();
	};
    // y的类型是i32
	let y = {
		println!();
		5
	};
}
```
```rust
fn func(i: i32) -> bool {
  return if i <= 0 {
    false
  } else {
    true
  }
}
fn func1(i:i32) -> bool {
  if i <= 0 {
  	return false
  } else  {
  	return true
  }
}

fn func2(i:i32) -> bool {
  return i > 0
}
```
```rust
fn main(){
  // v的类型是i32
  let v = loop {
    break 10;
  };
}
```
- `loop`和`while`运行时没什么区别
  - 编译器会觉得while语句的执行跟表达式在运行阶段的值有关, 可能未被初始化
  ```rust
  fn main(){
     let y;
	 while true {
        // if this condition isn't met and the `while` loop runs 0 times, `y` is not initialized
		y = 1;
		break;
	 }
     // 运行时推断
     // it is possibly-uninitialized  
  }
  ```
