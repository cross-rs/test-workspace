pub use external_lib;
pub use member;

pub static CRATE: &str = env!("CARGO_PKG_NAME");

pub fn print() {
    println!("Hello from {}", CRATE);
}
