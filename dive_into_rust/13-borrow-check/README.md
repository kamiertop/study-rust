# 共享不可变, 可变不共享

> 解决内存安全, 也是解决线程安全的基础

- 线程安全, 实质上是内存安全在多线程情况下的自然延伸
- 可以看做传统的线程安全机制Read Write Locker的编译阶段执行的版本
-
- **alias+mutation**
- &mut型借用也经常被称为独占指针, &型借用也经常被称为共享指针
- 在迭代器的生命周期内, 任何对容器的修改都是无法编译通过的
- 一个动态数组, p是arr[0]的指针, 如果扩容, 那么p可能就会改变
- 函数式编程强调无副作用的函数以及不可变类型