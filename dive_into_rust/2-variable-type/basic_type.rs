fn main(){
	let x: bool = false;
	println!("{}", x&true);

	let c1 = '✨';
	println!("{}", c1);

	let x: u8 = 1;
	let y: u8 = b'A';
	let z: u8 = b'1';
	println!("{}",x);
	println!("{}",y);
	println!("{}",z);

	let s:&[u8;5] = b"hello";
	println!("{:?}",s);
	let s:&[u8;5] = b"hello";
	println!("{:?}",s);

	let var1: i32 = 0xFF;
	println!("{}",var1);
	let var2 = 0x_1234_abcd;
	println!("{}",var2);
	let var3 = 123_usize;
	println!("{}",var3);

	println!("{}", 9_i32.pow(3));

	let x = 123.1_f64;
	println!("{}",x);

	let n1: i8 = 14;
	let n2: i16 = n1 as i16;
}