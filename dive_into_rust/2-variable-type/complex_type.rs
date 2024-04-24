#[allow(unused_parens,unused)]
fn main(){
	let tuple = (1_i32, false);
	println!("{:?}", tuple);
	println!("{},{}", tuple.0,tuple.1);

	let (a,b) = tuple;
	println!("{},{}", a,b);

	let single_tuple = ("single term", );
	let single_expr = ("single expr");	// 这是一个括号表达式
	println!("{:?}{}",single_tuple,single_expr);

	// let empty: () = ();	// unit 单元类型
	println!("size of i8 {}", std::mem::size_of::<i8>());
	println!("size of char {}", std::mem::size_of::<char>());
	println!("size of () {}", std::mem::size_of::<()>());

	let p1 = Point{
		x:1,
		y:10,
	};
	println!("{:#?}",p1);

	let x = 10;
	let y = 290;
	let mut p2 = Point{x,y};
	println!("{:?}",p2);
	p2.x = 1000;
	p2.y = 1230;
	let Point{x,y} = p2;
	println!("{},{}",x,y);

	let simple = Point{x:10, ..default()};
	println!("{:?}",simple);

	let x = Message::ChangeColor(1,1,1.23);
	let y = Message::Move {x:12,y:1.1};
	println!("{:?}",x);
	println!("{:?}",y);
}
#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
}
#[allow(unused)]
struct 	Foo1;
#[allow(unused)]
struct	Foo2();
#[allow(unused)]
struct	Foo3{}

fn default() -> Point {
	Point{x:33,y:55}
}
#[allow(unused)]
struct Color(i32,i32,i32);
#[allow(unused)]
enum Number{
	Int(i32),
	Float(f32),
}
#[allow(unused)]
fn read_num(num: Number) {
	match num {
		Number::Int(value) => println!("{}", value),
		Number::Float(value) => println!("{}", value)
	}
}

#[derive(Debug)]
#[allow(dead_code)]
enum Message {
	ChangeColor(i32, i32, f64),
	Move{x: i32, y: f32},
}
#[allow(unused)]
enum Option<T> {
	None,
	Some(T),
}

#[allow(unused)]
struct Recursive {
	data: i32,
	rec: Box<Recursive>
}