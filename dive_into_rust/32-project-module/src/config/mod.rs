pub use config::config::pub_use;

pub fn hello() -> String {
	"config::hello()".to_string()
}

pub mod config;
pub mod rewrite;

