use std::os::raw::c_int;

#[link(name = "math", kind = "static")]
extern "C" {
	fn add(a: c_int, b: c_int) -> c_int;
}


fn main() {
	let r = unsafe { add(1, 2) };
	println!("r: {}", r);
}

// rustc -L . main.rs
// msvc 需要math.lib
// gnu 需要libmath.a