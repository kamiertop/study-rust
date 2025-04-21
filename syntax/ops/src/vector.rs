use std::ops::Add;

#[derive(Debug)]
pub struct Vector {
	pub(crate) value: Vec<i32>,
}

// 实现 Add trait
impl Add for Vector {
	type Output = Vector;

	// fn add(self, rhs: Self) -> Self::Output {
	// 	// 创建一个新的 Vector，用来存储相加的结果
	// 	let mut res = Vector {
	// 		value: Vec::<i32>::with_capacity(self.value.len().min(rhs.value.len())),
	// 	};
	//
	// 	// 使用 zip 遍历两个向量的元素，并将它们相加
	// 	for (a, b) in self.value.into_iter().zip(rhs.value.into_iter()) {
	// 		res.value.push(a + b);
	// 	}
	//
	// 	// 返回相加后的新向量
	// 	res
	// }
	fn add(self, mut rhs: Self) -> Self::Output {
		for i in 0..rhs.value.len() {
			rhs.value[i] += self.value[i]
		}

		rhs
	}
}

