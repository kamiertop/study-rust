#[test]
fn move_a_box() {}

fn m(b: Box<i32>) {}


#[test]
fn borrow() {
	let mut x: Box<i32> = Box::new(100);
	let a: i32 = *x;
	*x += 1;
	println!("x: {}", x);

	let abx1 = x.abs();
	let abs2 = i32::abs(*x);
}

fn incr(n: &mut i32) {
	*n += 1;
}

#[test]
fn test1() {
	let mut n = 1;
	incr(&mut n);
	println!("n: {}", n);
}