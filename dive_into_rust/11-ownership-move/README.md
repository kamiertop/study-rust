# 所有权

- 每个值在Rust中都有一个变量来管理它, 这个变量就是这个值, 这块内存的所有者
- 每个值在一个时间点上只有一个管理者
- 当变量所在的作用域结束的时候, 变量以及它代表的值将会被销毁
- 变量从出生到死亡的整个阶段, 叫做一个变量的**生命周期**

# 移动语义

- 一个变量可以把它拥有的值转移给另外一个变量, 称为**所有权转移**
- Rust中的变量绑定操作, 默认是**move语义**, 执行了新的变量绑定后, 原来的变量就不能再被使用
- `let v1: Vec<i32> = v2;`: 移动语义, 而不是复制语义
- **语义**不代表最终的执行效率, 只是规定了什么样的代码是编译器可以接受的, 以及它执行后的效果是可以用怎样的思维模型去理解.
- 编译器有权在不改变语义的情况下做任何优化, 语义和优化是两个阶段的事情
- Rust默认move语义, 一些简单的类型是复制语义

# 复制语义

- **copy语义**: 凡是实现了 `std::marker::Copy`trait的类型, 都会执行copy语义.
- 一个类型的所有成员都是具有 `Clone` trait, 就可以使用derive自动实现

# Box类型

- 代表"拥有所有权的指针"
- Box类型永远执行的是move语义, 不能是copy语义

# Clone VS Copy

- Copy, Send, Sized, Sync这四个特殊trait对编译器的行为有影响, 唯一任务是给类型打一个标记, 表明符合某种约定,
  这些约定会影响编译器的静态检查以及代码生成
- 一旦实现了Copy trait, 那么在变量绑定, 函数参数传递, 函数返回值传递等场景下, 都是Copy语义
- 对于自定义类型, 所有成员都实现了Copy trait, 这个类型才有资格实现Copy trait
- 对于实现了Copy的类型, 它的clone方法应该跟Copy语义相容, 等同于按字节复制
- clone方法一般用于 "基于语义的复制"操作
- 区别联系
	- Copy内部没有方法, Clone内部有两个方法
	- Copy trait是给编译器用的, 告诉编译器这个类型默认采用Copy语义
	- Clone trait是给程序员用的, 手动调用clone方法

# 析构函数

- 析构函数: 对象被销毁的时候调用的函数
- 利用变量生命周期绑定资源的使用周期: RAII(Resource Acquisition Is Initialization)
- 变量生命周期开始时申请, 结束时利用析构函数释放资源
- Drop: 顺序是先进后出
- 编译器不允许手动调用析构函数drop, 需要使用标准库中的 `std::mem::drop`
- 变量遮蔽不会导致变量生命周期结束
- 如果一个类型可以使用memcpy的方式执行复制操作, 且没有内存安全问题, 那么它才能被允许实现Copy trait
- 带有析构函数的类型是不能满足Copy语义的. 不能保证, 对于带析构函数的类型, 使用memcpy复制一个副本一定不会有内存安全类型
- 不能同时实现Copy和Drop

# 析构标记

- 在析构函数被调用的时候, 就把标记设置一个状态, 在各个可能调用析构函数的地方都先判断一下状态再调用析构函数.