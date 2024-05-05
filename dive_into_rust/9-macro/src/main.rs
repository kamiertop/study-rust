macro_rules! add {
    // 宏没有对两个数执行相加操作，它只是把自己替换为把两个数相加的代码
    ($a:expr,$b:expr)=>{
        {
            $a+$b
        }
    };
    // 第二个分支,可以接收一个参数
    ($a:expr)=>{
        {
            println!("{}",$a);
            $a
        }
    }
}

macro_rules! add_as {
    // 做一个类型转换
    ($a:expr,$b:expr,$typ:ty)=>{
        $a as $typ + $b as $typ
    }
}

// 声明式宏
macro_rules! hashmap {
    // expander   => {transcriber}
    // 宏扩展语法定义  宏扩展的转换机制
    // 语法定义的标识符以$开头,类型支持item, block,  stmt, pat, expr, ty, itent, path, tt
    // 支持重复多个这样的语法元素, 使用+模式和*模式来完成,+代表一个或者多个重复,*代表零个或者多个重复
    // 下面的 " , "是分隔符,用于分割多个参数
    ($($key: expr => $val: expr),+) => {
        {
        let mut map = std::collections::HashMap::new();
        // 在语法扩展的部分也是用*符号, 将输入部分扩展为多条insert语句
        $(
            map.insert($key,$val);
        )+
        map
        }
    };
}
fn main() {
	println!("😲Rust的宏在语法解析之后起作用, 可以获取更多的上下文信息, 而且更加安全");
	println!("😲宏的内部实现和外部调用处于不同名字空间,它的访问范围严格受限");
	println!("😲是一种生成程序的程序, 分为声明式宏和过程宏");
	println!("😮实现编译阶段检查,例如以下代码");
	println!("😮以下代码 打印出当前源代码的文件名,以及当前代码的行数,都是纯编译阶段的信息");
	println!("file: {}, line: {}", file!(), line!());
	println!("😮宏可以实现自动代码生成");
	println!("😮宏可以实现语法扩展");
	let counts = hashmap!("A"=>0,"B"=>0);
	println!("{:?}", counts);
	println!("过程宏初探");
	add!(1,2);
	add!(1);
	println!("{:?}", add_as!(1,2,i32));
	println!("😉感谢这个链接: https://zhuanlan.zhihu.com/p/353421021");
	println!("😉这个小册很厉害: https://zjp-cn.github.io/tlborm/");
	println!("😉这个以后可以参考{:?}", "https://rustwiki.org/zh-CN/reference/macros-by-example.html");
}


/*
- 通过标准库提供的macro_rules!实现
- 通过提供编译器扩展来实现
- + 代表一个或多个重复, *代表零个或者多个重复
*/

/*
item  : 一个项（item），像一个函数，结构体，模块等。
block : 一个块 （block）（即一个语句块或一个表达式，由花括号所包围）
stmt  : 一个语句（statement）
pat   : 一个模式（pattern）
expr  : 一个表达式（expression）
ty    : 一个类型（type）
ident : 一个标识符（indentfier）
path  : 一个路径（path）（例如，foo，::std::mem::replace，transmute::<_, int>，...）
meta  : 一个元数据项；位于#[...]和#![...]属性
tt    : 一个词法树
vis   : 一个可能为空的Visibility限定词
*/
