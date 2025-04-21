#[test]
fn test1() {
	let s = String::from("abc");
	let h = &s[0..1];
	println!("{:?}", h);
}

fn first_world(s: &String) -> &str {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[..i];
		}
	}
	&s[..]
}