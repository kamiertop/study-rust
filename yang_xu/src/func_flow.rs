mod func_flow {
    use std::ops::{Sub, SubAssign};

    fn another_function(x: i32) {
		println!("x: {}", x);
		println!("{:?}", x);
	}

	#[test]
	fn test1() -> () {
		let y = {
			let x = 3;
			x + 1
		};
		println!("{:?}", y);
		()
	}
	#[test]
	fn control_flow() {
		let number = 3;
		if number < 3 {
			println!("condition was true");
		} else if number == 3 {
			println!("condition was false")
		} else {}
		let number = if number == 4 { 5 } else { 100 };
		println!("number: {}", number);
	}
	#[test]
	fn test2() {
		let x = 1;
		let y = if x == 1 { 0 } else { 1 };
		println!("{:?}", y);
	}
	#[test]
	fn test3() {
		let mut x = 100;
		loop {
			x = x.sub(1);
			if x == 50 {
				break;
			}
		}
		println!("x: {}", x);
		let y = loop {
			x += 10;
			if x > 1 {
				break 1;
			} else {
				break 10;
			}
		};
		println!("result: {y}");
	}
	#[test]
	fn test4() {
		let mut count = 10;
		'a: loop {
			count.sub_assign(1);
			println!("count: {}", count);
			if count == 0 {
				break 'a;
			}
		}
		while count == 0 {
			break;
		}
	}
	#[test]
	fn test_for() {
		let a = [1, 2, 3, 4, 5];
		// 使用索引会检查越界, for不会
		for v in a {
			println!("v: {}", v);
		}
		for v in (1..4) {
			println!("v: {}", v);
		}
		for v in (1..=20).rev() {
			println!("{v}");
		}
	}
	#[test]
	fn test5() {
		let a = [5; 10];
		let mut sum: i32 = 0;
		for x in a {
			sum += x;
		}
		println!("{sum}");
	}
}