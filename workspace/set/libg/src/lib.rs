pub static CRATE: &str = env!("CARGO_PKG_NAME");
compile_error!("crate should not compile");

pub fn print() {
    println!("Hello from {}, {}", CRATE, file!());
}
