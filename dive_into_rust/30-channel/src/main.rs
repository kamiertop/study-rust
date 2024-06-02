use std::sync::mpsc::{channel, sync_channel};
use std::thread;

fn main() {
	println!("异步管道: 发送端和接收端之间存在一个缓冲区");
	println!("同步管道内部有一个固定大小的缓冲区");
}

#[test]
fn test1() {
	let (tx, rx) = channel();
	thread::spawn(move || {
		for i in 0..10 {
			tx.send(i).unwrap();
		}
	});
	while let Ok(r) = rx.recv() {
		println!("r: {}", r);
	}
}

#[test]
fn test2() {
	let (tx, rx) = channel();
	for i in 0..10 {
		let tx = tx.clone();
		thread::spawn(move || {
			tx.send(i).unwrap();
		});
	}
	drop(tx);
	while let Ok(r) = rx.recv() {
		println!("r: {}", r);
	}
}

#[test]
fn test_sync_channel() {
	let (tx, rx) = sync_channel(1);
	tx.send(1).unwrap();
	println!("send first");
	thread::spawn(move || {
		tx.send(2).unwrap();
		println!("send second");
	});
	println!("rx.recv().unwrap(): {}", rx.recv().unwrap());
	println!("rx.recv().unwrap(): {}", rx.recv().unwrap());
}