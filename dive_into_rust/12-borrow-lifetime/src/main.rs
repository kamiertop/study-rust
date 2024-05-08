fn main() {}

#[test]
fn test1() {
	let mut v = vec![];
	fn foo(v: &mut Vec<i32>) {
		v.push(5);
	}
	foo(&mut v);
	println!("{:?}", v);
}

#[cfg(test)]
mod life_time {
	#[allow(unused)]
	struct T {
		data: i32,
	}

	// 'a 和 'b没有联系
	// fn test<'a, 'b>(arg: &'a T) -> &'b i32 {
	// 	&arg.data
	// }
	#[allow(unused)]
	fn test<'a, 'b>(arg: &'a T) -> &'b i32 where 'a: 'b {
		&arg.data
	}

	fn select<'a>(arg1: &'a i32, arg2: &'a i32) -> &'a i32 {
		if *arg1 > *arg2 {
			arg1
		} else {
			arg2
		}
	}

	#[test]
	fn main() {
		// x的生命周期大于y的生命周期
		// 在select中却使用相同的生命周期参数, 是因为生命周期的协变特性. 可以把两个生命周期缩小到某个生命周期内
		let x = 1;
		let y = 2;
		let s = select(&x, &y);
		println!("{}", s);
	}

	#[allow(unused)]
	struct Flag<'a> {
		data: &'a str,
	}

	impl<'t> Flag<'t> {
		// test2是省略版本
		fn test1<'a>(&self, _s: &'a str) {}
		fn test2(&self, _s: &str) {}
	}
}