use std::{error::Error, result::Result as StdResult};

pub use connection::Connection;

#[cfg_attr(target_os = "linux", path = "x11.rs")]
#[cfg_attr(target_os = "windows", path = "win.rs")]
#[cfg_attr(target_os = "macos", path = "mac.rs")]

pub type Result<T> = StdResult<T, Box<dyn Error>>;

pub trait ConnectionTrait: Sized {
	fn new() -> Result<Self>;
	fn windows(&self) -> Result<Vec<String>>;
}
