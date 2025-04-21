mod data_type {
	#[test]
	fn test() {
		let h = 0xff;
		let h1 = 0x_ffff;
		let c = b'A';
		println!("h: {}", h);
		println!("h1: {}", h1);
		println!("c: {}", c);
		let some_char = 'z';
		let chinese_char: char = '中';
		println!("some_char: {}", some_char);
		println!("chinese_char: {}", chinese_char);
		let arr = [1, 2, 3];
		let arr_with_type: [i32; 10] = [1; 10];
		let arr1: [i32; 2] = [260, 280];
		let a = [5; 2];
		let first = a[0];
		println!("arr: {:?}", arr);
		println!("arr1: {:?}", arr1);
		println!("a: {:#?}", a);
		println!("first: {}", first);

		let tup = (100, 20, 'Z');
		println!("tup: {:?}", tup);
		println!("tup: {:#?}", tup);

		let tup: (u32, i32, u8) = (100, 20, b'Z');
		println!("{}", tup.2);
		println!("{:?}", tup);
	}
	#[test]
	fn test2() {
		let a: i32 = 10;
		let b: u32 = 20;
		// let c = a + b;
		println!("a: {}, b: {}", a, b);
	}
	#[test]
	fn test3() {
		const THREE: f32 = 3.4028236;
		println!("THREE: {}", THREE);
	}
}