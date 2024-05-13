fn main() {}


// 编译器找不到成员方法时, 会自动尝试使用deref方法后再找该方法
#[test]
fn auto_deref() {
	let s = "hello";
	println!("s.le(): {:?}", s.len());
	println!("(&s).len(): {}", (&s).len());
	println!("length: {}", str::len(&s));
}


#[test]
fn test1() {
	use std::rc::Rc;
	// Rc类型本身没有bytes方法, 编译器会尝试自动deref, 试试s.deref().bytes()
	// String()类型其实也没有bytes()方法, 但是String可以继续deref,, 于是再试试s.deref().deref().bytes()
	// 在str类型中找到了bytes()方法
	let s = Rc::new(String::from("hello"));
	println!("{:?}", s.bytes());
}

#[test]
fn cow() {
	fn remove_spaces(input: &str) -> Cow<str> {
		if input.contains(' ') {
			let mut buf = String::with_capacity(input.len());
			for c in input.chars() {
				if c != ' ' {
					buf.push(c);
				}
			}
			return Cow::Owned(buf);
		}

		return Cow::Borrowed(input);
	}

	use std::borrow::Cow;
	let s1 = "no_spaces_in_string";
	let res1 = remove_spaces(s1);
	let s2 = "spaces in string";
	let res2 = remove_spaces(s2);

	println!("res1: {}", res1);
	println!("res2: {}", res2);
}