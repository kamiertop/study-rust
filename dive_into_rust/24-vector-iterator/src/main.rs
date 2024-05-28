fn main() {}


#[cfg(test)]
mod vector {
	use std::collections::{BTreeMap, HashMap, VecDeque};

	#[test]
	fn vec() {
		let v1: Vec<f64> = Vec::new();
		println!("v1: {:?}", v1);
		let v2: Vec<String> = Vec::with_capacity(100);
		println!("v2: {:?}", v2);
		let v3 = vec![1, 2, 3];
		println!("v3: {:?}", v3);
		let mut v4 = Vec::<i32>::new();
		v4.push(1);
		v4.extend_from_slice(&[10, 20]);
		v4.insert(2, 100);
		println!("capacity: {} length: {}", v4.capacity(), v4.len())
	}

	#[test]
	fn vec_deque() {
		let mut queue = VecDeque::with_capacity(64);
		for i in 0..=64 {
			queue.push_back(i);
		}
		while let Some(i) = queue.pop_front() {
			println!("i: {}", i);
		}
	}


	#[test]
	fn hashmap() {
		#[derive(Hash, Eq, PartialEq, Debug)]
		struct Person {
			first: i32,
			last: i32,
		}

		let mut book: HashMap<Person, &str> = HashMap::new();
		book.insert(Person { first: 1, last: 1 }, "521-8976");
		book.insert(Person { first: 0, last: 0 }, "521-8906");
		let p = Person { first: 0, last: 0 };

		if let Some(v) = book.get(&p) {
			println!("found: {:?}", v);
		} else {
			println!("not found")
		}
		book.remove(&p);
		println!("book.contains_key(&p): {}", book.contains_key(&p));
	}

	#[test]
	fn btree_map() {
		let mut bmap = BTreeMap::<i64, &str>::new();
		bmap.insert(3, "a");
		bmap.insert(4, "b");
		bmap.insert(8, "c");
		for (k, v) in bmap.range(2..5) {
			println!("{}: {}", k, v)
		}
	}
}


#[cfg(test)]
mod iterator {
	struct Seq(i32);

	impl Seq {
		fn new() -> Self {
			Seq(0)
		}
	}

	impl Iterator for Seq {
		type Item = i32;
		fn next(&mut self) -> Option<Self::Item> {
			if self.0 < 100 {
				self.0 += 1;
				Some(self.0)
			} else {
				None
			}
		}
	}

	#[test]
	fn tst1() {
		let mut seq = Seq::new();
		while let Some(i) = seq.next() {
			println!("i: {}", i);
		}

		let seq = Seq::new();
		for i in seq {
			println!("i: {}", i);
		}
	}

	#[test]
	fn test2() {
		let v = vec![1, 2, 3, 4, 5];
		let iter = v.iter();
		for i in iter {
			println!("i: {}", i);
		}
	}

	#[test]
	fn test3() {
		let v = [1, 2, 3, 4, 5, 6];
		let iter = v.iter()
			.take(5)
			.filter(|&x| x % 2 == 0)
			.map(|&x| x * x)
			.enumerate();
		for (i, v) in iter {
			println!("i: {} v: {}", i, v);
		}
		println!("v: {:?}", v);
	}
}