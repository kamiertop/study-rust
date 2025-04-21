use std::rc::Rc;

#[test]
fn test1() {
	let value = return_a_string();
	println!("value: {}", value);

	let value1 = return_a_string1();
	println!("value1: {:?}", value1);
}

fn return_a_string() -> String {
	let s = String::from("Hello world");
	s
}
fn return_a_string1() -> &'static str {
	"hello, world"
}

fn return_a_string2() -> Rc<String> {
	let s = Rc::new(String::from("Hello, world"));
	Rc::clone(&s)
}
#[test]
fn test2() {
	let mut name = (String::from("a"), String::from("b"));
	let first = &name.0;
	name.1.push_str("c");
	println!("{} {}", first, name.1);
}