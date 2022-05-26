pub static CRATE: &str = env!("CARGO_PKG_NAME");

pub fn print() {
    println!("Hello from {}, {}", CRATE, file!());
}