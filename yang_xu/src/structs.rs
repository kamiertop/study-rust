use std::fmt::Debug;

struct User {
	name: bool,
	username: String,
	email: String,
}

#[test]
fn test1() {
	let mut u = User {
		name: false,
		username: "".to_string(),
		email: "".to_string(),
	};
	u.email = String::from("123@qq.com");
	let u2 = User {
		name: true,
		..u
	};
}
struct Color();

fn build_user(email: String, username: String) -> User {
	User {
		name: true,
		email,
		username,
	}
}

#[test]
fn test2() {
	struct Point {
		x: i32,
		y: i32,
	}
	let mut p = Point { x: 10, y: 20 };
	let x = &mut p.x;
	// println!("p.x: {}", p.x);
	*x += 1;
	println!("{:?}", p.y);
}

#[derive(Debug)]
struct Point {
	x: u32,
	y: u32,
}
#[allow(unused)]
impl Point {
	fn gen0(size: u32) -> Self {
		Self {
			x: size,
			y: size,
		}
	}
	fn gen1(size: u32) -> Self {
		Point {
			x: size,
			y: size,
		}
	}
	fn gen2(size: u32) -> Point {
		Point {
			x: size,
			y: size,
		}
	}
	fn gen3(size: u32) -> Point {
		Self {
			x: size,
			y: size,
		}
	}
	fn area(&self) -> u32 { // &self == self: &Self
		self.x * self.y
	}

	fn area1(self: &Self) -> u32 {
		self.x * self.y
	}

	fn area2(self) -> u32 {
		self.x * self.y
	}

	fn can_hold(&self, other: &Point) -> bool {
		self.x > other.x && self.y > other.y
	}

	fn p() {
		println!("function p");
	}
	fn incr_x(&mut self) {
		self.x += 1;
	}
}


enum K {
	Age,
	Name,
}

enum Ip {
	V4,
	K(K), // 将 K 作为 Ip 的一个变体
}
#[test]
fn test6() {}
#[test]
fn k() {
	let config_max = Some(3);
	if let Some(m) = config_max {
		println!("m: {}", m);
	}
}

