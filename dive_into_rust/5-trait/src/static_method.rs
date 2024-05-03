#[cfg(test)]
mod tests {
	struct T(i64);    // tuple struct
	impl T {
		// 参数名字决定方法类别
		fn print(this: &Self) {
			println!("value:{}", this.0);
		}
		fn print1(self: &Self) {}
	}

	#[test]
	fn main() {
		let x = T(42);
		// 没有receiver关键字:self, 所以x.print() 使用小数点调用静态方法不合法
		T::print(&x);
		//
		x.print1();
	}

	trait Default {
		fn default() -> Self;
	}

	impl Default for T {
		fn default() -> Self {
			return T(100_i64);
		}
	}
}
