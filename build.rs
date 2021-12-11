#[cfg(windows)]
fn main() {
	panic!("this crate does not yet support windows!")
}

#[cfg(not(windows))]
fn main() {}
