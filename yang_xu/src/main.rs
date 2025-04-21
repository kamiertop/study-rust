use std::env::VarError;
use std::ffi::OsStr;

fn main() {
	println!("{:?}", std::env::var("UV_TOOL_BIN_DIR"));
}


pub fn var(key: Box<dyn AsRef<OsStr>>) -> Result<String, VarError>{

}