## `prelude`
- 编译器为用户写的每个create自动插入 `use std::prelude::*;`

## `crate` and `mode`
- 每个 `crate`是一个完整的编译单元, 可以生成为一个 lib或exe可执行文件
- `crate`内部, 由`mod`管理, 可以理解为`namespace`

## `println!`
- 输出宏, 完成编译器静态格式检查, 更加安全