fn main() {}

// #[test]
// fn test1() {
// 	let x = 1_i32;
// 	let mut y: u32 = 1;
// 	// safe
// 	let raw_mut = &mut y as *mut u32 as *mut i32 as *mut i64;
// 	unsafe {
// 		// 不安全
// 		*raw_mut = -1;
// 	}
// 	println!("x: {:X}", x);
// 	println!("y: {:X}", y);
// }

#[test]
fn test2() {
	let x = vec![1, 2, 3, 4];
	unsafe {
		let t: (usize, usize, usize) = std::mem::transmute_copy(&x);
		println!("{},{},{}", t.0, t.1, t.2);
	}
	println!("x: {:?}", x);
}

#[test]
fn test3() {
	let mut x = [1, 2, 3];
	let (first, rest): (&mut [i32], &mut [i32]) = x.split_at_mut(1);
	let (second, third): (&mut [i32], &mut [i32]) = rest.split_at_mut(1);
	first[0] += 2;
	second[0] += 4;
	third[0] += 8;
	println!("{:?},{:?},{:?}", first, second, third);
	println!("{:?}", &x)
}

#[test]
fn test4() {
	fn print_str(s: StrRef) {
		println!("{}", s);
	}
	type StrRef<'a> = &'a str;
	let s: StrRef<'static> = "hello";
	print_str(s);
}