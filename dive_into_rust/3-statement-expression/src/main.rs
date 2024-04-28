#[allow(unused)]
fn main() {
	let x = 1;
	let mut y = 2;
	let z = (y=x);

	println!("{:?}",z);
	println!("{:?}",z);
	// loop_while();
	func_for()
}
#[allow(unused)]
fn test1() -> (i32,i32){
	// x的类型是()
	let x:() = {
		println!();
	};
	// y的类型是i32
	let y = {
		println!();
		5
	};

	(4,4)
}
#[allow(unused)]
fn func(i: i32) -> bool {
	return if i < 0 {
		false
	} else {
		true
	}
	// if i < 0 {
	// 	return false
	// } else  {
	// 	return true
	// }
}
#[allow(unused)]
fn func_loop() {
	let mut count = 1;
	loop {
		count += 1;
		if count == 3 {
			println!("3");
			continue;
		}
		if count == 10 {
			println!("10, break");
			break;
		}
	}
}
#[allow(unused)]
fn loop1() {
	let mut m = 1;
	let n = 1;
	'a1: loop {
		if m < 100 {
			m += 1;
		} else {
			'b1: loop {
				if m + n > 50 {
					break 'a1;
				} else {
					continue 'a1;
				}
			}
		}
	}

	let v = loop {
		break 10;
	};
}

#[allow(unused)]
fn loop_while(){
	let x;
	loop {
		x = 1;
		break;
	}
	// let y;
	// while true {
	// 	y = 1;
	// 	break;
	// }
	// println!("{}",y);
}

#[allow(unused)]
fn func_for() {
	let array = [1,2,3,4,5];
	for i in array {
		println!("{}",i);
	}
}