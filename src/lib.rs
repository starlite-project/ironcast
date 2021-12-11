mod util;

pub use self::util::*;

#[cfg(windows)]
compile_error!("this crate is not designed for windows yet.");

pub struct IronCast {}
