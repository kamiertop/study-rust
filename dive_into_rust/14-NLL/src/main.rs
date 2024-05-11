fn main() {
	let mut v = vec![1, 2, 3, 4, 5];
	v.push(v.len());
	println!("{:?}", v);
}