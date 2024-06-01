use std::thread;
use std::time::Duration;

fn main() {}

#[test]
fn test1() {
	let child = thread::spawn(move || {
		println!("Hello, Rust thread!");
	});
	let _ = child.join();
}

#[test]
fn test2() {
	let t = thread::Builder::new()
		.name("child1".to_string())
		.spawn(move || {
			println!("enter child thread");
			thread::park();
			println!("resume child thread");
		}).unwrap();
	println!("span a thread");
	thread::sleep(Duration::new(2, 0));
	t.thread().unpark();
	let _ = t.join();
	println!("child thread finished")
}