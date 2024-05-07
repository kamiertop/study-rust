fn main() {}

#[derive(Debug)]
#[allow(unused)]
struct Foo {
	data: i32,
}

#[derive(Debug, Clone)]
#[allow(unused)]
struct Weather(i32);

impl Clone for Foo {
	fn clone(&self) -> Foo { *self }
}

impl Copy for Foo {}

#[test]
fn impl_copy() {
	let v1 = Foo {
		data: 100,
	};
	let v2 = v1;
	println!("{:?}\n{:?}", v2, v1);
}

impl Drop for Weather {
	fn drop(&mut self) {
		println!("destruct Weather, value: {}", self.0)
	}
}

#[test]
#[allow(unused)]
fn drop() {
	let x = Weather(1);
	println!("construct x");
	std::mem::drop(x);
	// x.drop(); 禁止显示调用
	{
		let y = Weather(54);
		println!("construct y");
	}
}