use std::os::raw::c_int;

#[link(name = "sample_math")]
extern "C" {
	fn add(a: c_int, b: c_int) -> c_int;
}


fn main() {
	let r = unsafe { add(1, 2) };
	println!("r: {}", r);
}