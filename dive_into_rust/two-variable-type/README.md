## 变量声明
> `let varibale: i32 = 100;`
- 语法分析更容易, 语法歧义更少, 语法分析器更容易编写
- 方便引入类型推导, 变量声明中, 最重要的变量本身
- 模式结构
- 变量必须为合理初始化才能使用
- 一个变量的类型必须在编译器确定
- 只允许**局部变量/全局变量**实现类型推导, 全局变量必须当场初始化而函数签名具有全局性影响
  - 函数签名等场景不允许, 如果使用自动类型推导, 可能导致某个调用的地方使用方式发生变化, 它的参数, 返回值类型就发生了变化
- 静态变量: 生命周期为整个程序, 生命周期永远是 `'static`, 占用的内存空间也不会在执行过程中回收
  - 带有mut修饰的全局变量, 在使用的时候必须使用unsafe关键字
  - 不能在声明时调用普通函数
  - Rust不允许用户在main函数之前或者之后执行自己的代码, 所以比较复杂的static变量的初始化一般需要使用lazy方式, 在第一次使用的时候初始化
  ```rust
  fn main(){
    static mut G2 : i32 = 4;
    unsafe {
        G2 = 5;
        println!("{}",G2);
    }
  }
  // 全局变量的内存不是分配在当前函数栈上, 函数退出的时候, 并不会销毁全局变量占用的内存空间, 程序退出才会回收
  ```
- `const`声明常量, 不是变量.
  - 常量的初始化表达式一定要是一个编译器常量, 不能是运行期的值
  - 编译器并不一定会给const常量分配内存空间, 编译期间, 可能会被内联优化