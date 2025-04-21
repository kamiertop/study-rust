use crate::vector::Vector;

mod vector;


fn main() {
	let v1 = Vector { value: vec![1, 2, 3] };
	let v2 = Vector { value: vec![4, 5, 6] };

	// 使用 + 运算符来相加两个 Vector
	let v3 = v1 + v2;

	// 输出结果
	println!("{:?}", v3);  // 输出: Vector { value: [5, 7, 9] }
}

