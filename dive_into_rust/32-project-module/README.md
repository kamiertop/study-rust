# 项目和模块

> crate之间不能出现循环引用
> rustc编译器不是基于单个的.rs或者mod来执行编译的

## crate

- 简单理解就是一个项目. 是Rust中的独立编译单元
- 每个crate对应生成一个库或者可执行文件

## mod

- 命名空间, 可以嵌套, 还可以控制内部元素的可见性
- super 表示从父模块开始的引用方式

## cargo

> cargo是一个包管理工具, 不是编译器
> https://doc.rust-lang.org/cargo/

- `cargo new {NAME} --bin`
- `cargo new {NAME} --lib`
- `cargo build --release`: 生成release版的可执行文件, 比debug版本优化得更好
- `cargo check`: 只检查编译错误, 不做代码优化以及生成可执行程序, 适合在开发过程中快速检查语法, 类型错误
- `cargo clean`: 清理以前的编译结果
- `cargo doc`: 生成该项目的文档
- `cargo test`: 执行单元测试
- `cargo bench`: 执行benchmark性能测试
- `cargo install`
- `cargo uninstall`
- `cargo update`
- 引用本地库:

```toml
[dependencies]
{NAME } = { path = "../{NAME}" }
```

- crates.io网站, 可以使用 `subcommand`搜索子命令

## 项目依赖

> 版本号: https://semver.org/

```toml
[dependencies]
lazy_static = "1.0.0"
rand = { git = https://github.com/rust-lang-nursery/rand, branch = "master" }
rand1 = { git = https://github.com/rust-lang-nursery/rand, tag = "0.3.15" }
rand2 = { git = https://github.com/rust-lang-nursery/rand, rev = "31f2663" }
my_own_project = { path = "/my/local/path", version = "0.1.0" }
```

- ^1.2.3: 1.2.3 <= version < 2.0.0
- ~1.2.3: 1.2.3 <= version < 1.3.0
- 1.* : 1.0.0 <= version < 2.0.0
- > 1.2.3
- > =1.2.3
- <=1.2.3

### `cargo.lock`: 记录当前项目所有依赖项目的具体版本

- 项目是库: 不要把Cargo.lock文件纳入到vcs中
- 项目是可执行程序: 把Cargo.lock纳入到vcs中

### workspace

- Cargo.toml

```toml
members = [
    "project1", "lib1"
]
```

```
|--Cargo.lock
|--Cargo.toml
|--project1
|  |--Cargo.toml
|  └──src
|     └──main.rs
|--lib1
|  |--Cargo.toml
|  └──src
|     └──main.rs
└──target 
```

### build.rs

- cargo运行用户在正式编译开始前执行一些自定义的逻辑, 方法是在Cargo.toml中配置一个build的属性

```toml
[package]
# ...
build = "build.rs"
```

> build.rs一般用于下面这些情况

1. 提前调用外部编译工具, 比如调用gcc编译一个C库
2. 在操作系统中查找C库的位置
3. 根据某些配置, 自动生成源码
4. 执行某些平台相关的配置

- `CARGO_MANIFEST_DIR`
- `CARGO_PKG_NAME`: 当前crate的名字
- `OUT_DIR`: build.rs的输出路径, 如果要在build.rs中生成代码, 那么生成的代码就要存在这个文件夹下
- `HOST`
- `OPT_LEVEL`: 优化级别
- `PEOFILE`: 判断是release还是debug版本
