mod array;
mod string;

fn main() {}

/*
* 数组中元素占用的空间大小必须是编译器确定的
* 数组中元素个数也必须是编译器确定的, 执行阶段不可变
* 数组: [T;n]
* 两个数组类型, 元素类型和元素个数都完全相同, 这两个数组才是同类型
* 数组之间不能隐式转换
* 将数组作为参数传给一个函数, 这个数组不会退化成一个指针, 而是完整复制, 函数体内对数组的改动不会影响到外面的数组

- 数组切片本质上还是指针, 不过是在类型系统中丢弃了编译阶段定长数组类型的长度信息, 而将此长度信息存储为运行期的值
- 一个数组[T;n], 它的借用指针的类型就是&[T;n].可以通过编译器内部魔法转换为数组切片类型&[T]

* DST(Dynamic Sized Type): 动态大小类型, 编译阶段无法确定占用空间大小的类型
* 指向DST的指针一般是胖指针
* 不定长数组[T], 胖指针 -> &[T]
* 不定长字符串str类型, 胖指针 -> &str

- 不定长数组 类型[T]在编译阶段是无法判断该类型占用空间的大小的, 我们不能在栈上声明一个不定长大小数组的变量实例, 也就是不能用它作为函数的参数, 返回值
- 指向不定长数组的胖指针的大小是确定的, &[T]类型可以用作变量实例, 函数参数, 返回值
- 数组切片这个胖指针包含两个数据: 指针+长度

* 索引操作也是一个通用的运算符, 可以自行扩展
* 一般情况下, Rust不鼓励大量使用"索引"操作, 正常的"索引"操作都会执行一次"边界检查"
* 很多时候使用数组切片作为被操作对象在函数间传递, 保证效率
* 不能在栈上声明一个不定长大小数组的变量实例
*/

/*
- 字符串索引操作的时间复杂度是O(n)
- str: 字符串类型
- &str: 胖指针, 包含了质量字符串片段头部的指针和一个长度, 是对一块字符串区间的借用, 没有对指向的内存空间的所有权
- String类型在堆上动态申请了一块内存空间, 有权对这块内存空间进行扩容
- 很多时候, &String类型可以被编译器自动转换为&str类型
*/