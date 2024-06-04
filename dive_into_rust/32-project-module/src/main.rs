use log::log;

mod config;
mod log;

#[path = "./log.rs"]
mod log2;

fn main() {
	use crate::mod_hello as mod_hello;
	mod_hello::hello();

	println!("使用log::log(), 只需要`mod log`即可");
	log::log();

	println!("直接使用log(), 需要: use log::log");
	log();

	println!("mod_log和log函数在一个文件中, 一个文件就是一个mod, 所以开头只需要导入一个log");
	log::mod_log::log();

	config::config::mod_config();
	config::config::get_config();

	assert_eq!(config::hello(), "config::hello()".to_string());

	log2::log();

	config::pub_use();
	config::hello();
	config::rewrite::rewrite();
}

#[allow(unused)]
fn hello() -> &'static str {
	"main.hello"
}

#[allow(unused)]
fn crate_hello() {
	println!("crate::hello");
}

mod mod_hello {
	pub fn hello() {
		println!("mod.hello.hello");
	}

	#[test]
	fn main() {
		hello();    // mod.hello.hello
		self::hello(); // mod.hello.hello

		assert_eq!(super::hello(), "main.hello");

		crate::crate_hello();    // crate::hello

		assert_eq!(super::hello(), "main.hello");
	}
}
