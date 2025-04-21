use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct Point {
	x: i32,
	y: i32,
}

fn main() {
	let point = Point { x: 1, y: 2 };

	let serialized = serde_json::to_string(&point).unwrap();
	// 输出结果: serialized = {"type":"Point","x":1,"y":2}
	println!("serialized = {}", serialized);
}
