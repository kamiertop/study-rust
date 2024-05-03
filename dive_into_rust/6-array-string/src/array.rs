#[cfg(test)]
mod array {
	#[test]
	fn main() {
		let arr1: [i32; 5] = [1, 2, 3, 4, 5];
		println!("arr1: {:?}", arr1);
		// 初始化所有的元素为0
		let arr2: [i32; 50] = [0; 50];
		println!("{:?}", arr2);
	}

	#[test]
	fn main1() {
		let mut xs = [1, 2, 3, 4, 5];
		println!("{:?}", xs);
		let ys = [6, 7, 8, 9, 10];
		xs = ys;
		println!("{:?}", xs);
	}

	#[test]
	fn main2() {
		let xs = [1, 2, 3, 4, 5];
		modify1(xs);
		println!("{:?}", xs);
	}

	fn modify1(mut arr: [i32; 5]) {
		println!("{:?}", arr);
		arr[0] = 10;
		println!("{:?}", arr);
	}

	#[test]
	fn compare() {
		// 同类型才可以比较
		let arr1 = [1, 2, 3];
		let arr2 = [1, 3, 6];
		println!("{}", arr1 < arr2)    // true
	}

	#[test]
	fn range() {
		let arr = [0; 10];
		for i in arr {
			println!("{}", i);
		}
		let r = std::ops::Range {
			start: 1,
			end: 10,
		};
		for i in r {
			println!("i: {}", i);
		}
		// .. 是上面例子Range的语法糖
		for i in 1..10 {
			println!("i: {}", i);
		}
		// 闭区间
		for i in 1..=10 {
			println!("i: {}", i);
		}
	}

	#[test]
	fn matrix() {
		let v = [[0, 0], [1, 1]];
		for i in v {
			println!("{:?}", i);
		}
	}

	#[test]
	fn array_slice() {
		fn mut_array(a: &mut [i32]) {
			if a.len() >= 3 {
				a[2] = 4;
			}
		}
		println!("size of &[i32;3]: {:?}", std::mem::size_of::<&[i32; 3]>());
		println!("size of &[i32]: {:?}", std::mem::size_of::<&[i32]>());

		let mut v: [i32; 3] = [1, 2, 3];
		{
			let s = &mut v;
			mut_array(s);
		}
		println!("{:?}", v);
	}

	#[test]
	fn fat_pointer() {
		fn raw_slice(arr: &[i32]) {
			let (val1, val2): (usize, usize) = unsafe { std::mem::transmute(arr) };
			println!("value in raw pointer:	");
			println!("val1: {:x}", val1);
			println!("val2: {:x}", val2);
		}

		let arr = [1, 2, 3, 4, 5];
		let address = &arr;
		println!("address of arr: {:p}", address);
		raw_slice(address as &[i32])
	}

	#[test]
	fn get_item() {
		let v = [1, 2, 3, 4];
		let first = v.get(0);
		let tenth = v.get(10);
		println!("{:?}\n{:?}", first, tenth)
	}

	#[test]
	fn iter() {
		let v = [1, 2, 3, 4, 5];
		for (idx, value) in v.iter().enumerate() {
			println!("{} : {}", idx, value)
		}
	}
}