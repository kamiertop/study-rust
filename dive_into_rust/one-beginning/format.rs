fn main() {
    println!("{}",1);
    println!("{:o}",9);     // 8进制
    println!("{:x}",255);   // 16进制, 小写
    println!("{:X}",255);   // 16进制, 大写
    println!("{:p}",&0);    // 指针
    println!("{:b}",15);    // 二进制
    println!("{:e}", 10000f32); // 科学计数(小写)
    println!("{:E}", 10000f32); // 科学计数(大写)
    println!("{:?}", "test");   // debug
    println!("{:#?}",("test1", "test2", "test3")); // 带换行和缩进的debug
    println!("{a} {b}", a=1,b="2");
}