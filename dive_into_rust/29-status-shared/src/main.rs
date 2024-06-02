use std::sync::{Arc, Barrier, Mutex};
use std::thread;

fn main() {
	println!("免疫一切数据竞争");
	println!("无额外性能损耗");
	println!("无需与编译器紧耦合");
	println!("编译器阶段静态检查实现免疫数据竞争");
}

#[test]
fn arc() {
	let numbers: Vec<i32> = (0..5_i32).collect();
	let shared_numbers = Arc::new(numbers);
	for _ in 0..10 {
		let child_numbers = shared_numbers.clone();
		thread::spawn(move || {
			let _local_numbers = &child_numbers[..];
			println!("{:?}", _local_numbers);
		});
	}
}


#[test]
fn mutex() {
	const COUNT: i32 = 100;
	let global = Arc::new(Mutex::new(0));
	let clone1 = global.clone();
	let thread1 = thread::spawn(move || {
		for _ in 0..COUNT {
			let mut value = clone1.lock().unwrap();
			*value += 1;
		}
	});
	let clone2 = global.clone();
	let thread2 = thread::spawn(move || {
		let mut value = clone2.lock().unwrap();
		*value -= 1;
	});
	thread1.join().ok();
	thread2.join().ok();
	println!("final value: {:?}", global);
}

#[test]
fn barrier() {
	let barrier = Arc::new(Barrier::new(10));
	let mut handlers = vec![];
	for _ in 0..10 {
		let c = barrier.clone();
		let t = thread::spawn(move || {
			println!("before wait");
			c.wait();
			println!("after wait");
		});
		handlers.push(t);
	}
	for h in handlers {
		h.join().ok();
	}
}