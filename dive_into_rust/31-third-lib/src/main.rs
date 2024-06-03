use std::{thread, time};
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::Duration;

use crossbeam::channel::bounded;
use go_spawn::go;
use parking_lot::Mutex;
use scoped_threadpool::Pool;
use threadpool::ThreadPool;

fn main() {}

#[test]
fn thread_pool() {
	let n_workers = 4;
	let n_jobs = 8;
	let pool = ThreadPool::new(n_workers);

	let (tx, rx) = channel();
	for _ in 0..n_jobs {
		let tx = tx.clone();
		pool.execute(move || {
			tx.send(1).expect("channel will be there waiting for the pool");
		});
	}

	assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
}

#[test]
fn scoped_thread_pool() {
	let mut pool = Pool::new(4);
	let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
	pool.scoped(|scope| {
		for e in &mut vec {
			scope.execute(move || {
				*e += 1;
			})
		}
	});
	println!("vec: {:?}", vec);
}

#[test]
fn parking_lot() {
	const N: usize = 10;
	let data = Arc::new(Mutex::new(0));

	let (tx, rx) = channel();
	for _ in 0..10 {
		let (data, tx) = (Arc::clone(&data), tx.clone());
		thread::spawn(move || {
			// The shared state can only be accessed once the lock is held.
			// Our non-atomic increment is safe because we're the only thread
			// which can access the shared state when the lock is held.
			let mut data = data.lock();
			*data += 1;
			if *data == N {
				tx.send(()).unwrap();
			}
			// the lock is unlocked here when `data` goes out of scope.
		});
	}

	rx.recv().unwrap();
}

#[test]
fn crossbeam() {
	let (s, r) = bounded(1);
	s.send("foo").unwrap();
	assert_eq!(r.recv(), Ok("foo"));
}

#[test]
fn go() {
	let data = 10;
	for _ in 0..10 {
		go!(println!("{:?}", time::SystemTime::now()));
		go!(println!("{}", data))
	};
	thread::sleep(Duration::from_secs(2))
}

