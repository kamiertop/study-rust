# 如何组织, 就如何解构!

## `let`

```rust
fn main() {
	struct T1(i32, char);

	let t1 = T1(100, 'a');
	// 这里需要有T1, 告诉编译器如何进行解构
	let T1(value1, value2) = t1;
}
```

- 一个稍微复杂点的例子

```rust
struct T1(i32, char);

struct T2 {
	item1: T1,
	item2: bool,
}

fn main() {
	let x = T2 {
		item1: T1(0, 'A'),
		item2: false,
	};
	// 要在等号左边复现变量x的结构
	let T2 {
		item1: T1(value1, value2),
		item2: value3,
	} = x;

	println!("{} {} {}", value1, value2, value3);

	let t1 = T1(100, 'a');
	let T1(value1, value2) = t1;
}
```

- 函数参数本身就具备 "模式结构"功能, 可以在参数在进行解构, 或者使用`_`忽略某个值
- 使用`..`忽略多个值

```rust
struct P(i32, i32, i32);

fn calculate(P(x, _, y): P) -> i32 {
	return x + y;
}

fn calculate1(P(x, .., y): P) -> i32 {
	return x + y;
}

fn calculate2(P(x, ..): P) -> i32 {
	return x;
}
```

## match

- `non_exhaustive`在当前crate内不起作用
- match也是表达式
- 模式之间使用逗号分隔

```rust
fn value(x: Char) -> i32 {
	return match x {
		Char::A => { 10 }
		Char::B => { 20 }
		Char::C => { 30 }
		Char::D => 40,
		Char::E => 50
	};
}
```

- match可以匹配值

```rust
fn match_value(x: i32) {
	match x {
		-1 | -2 => println!("negative"),
		0 => println!("zero"),
		1 => println!("positive"),
		_ => { println!("error") }
	}
}
```

```rust
fn match_value1(x: i32) {
	match x < 0 {
		true => println!("true"),
		false => println!("false"),
	}
}
```

- match 还可以匹配范围

```rust
fn match_range(x: char) {
	match x {
		'a'..='z' => println!("lowercase"),
		'A'..='Z' => println!("uppercase"),
		_ => { println!("something else"); }
	}
}
```

- guards

```rust
fn guards() {
	enum OptionalInt {
		Value(i32),
		Missing,
	}
	let x = OptionalInt::Value(54);
	match x {
		// 这里使用if作为"匹配看守"(match guards), 匹配成功符合if时才会执行后面的语句
		OptionalInt::Value(i) if i > 5 => println!("match int, and > 5"),
		// 两种方式都可以
		// OptionalInt::Value(..) => println!("i < 5="),
		_ => println!("i < 5"),
		OptionalInt::Missing => println!("missing")
	}
	let i = 10;
	match i {
		i if i > 4 => println!("bigger than 4"),
		i if i <= 4 => println!("small or equal to 4"),
		_ => {}
	}
}
```

- 变量绑定

```rust
fn main() {
	let x = Some(42);

	match x {
		Some(v @ 1..=100) => println!("The value {} is in range 1 to 100", v),
		Some(_) => println!("The value is not in range 1 to 100"),
		None => println!("No value"),
	}
}
```

- 进行匹配的值同时符合好几条分支, 总会执行第一条匹配成功的分支
- Rust中, 所有的变量绑定默认都是"不可更改的", 使用mut修饰的变量绑定才有修改数据的能力