pub static CRATE: &str = env!("CARGO_PKG_NAME");

pub fn print() {
    println!("Hello from {}", CRATE);
}

pub fn main() {
    self::print();
    test_workspace::external_lib::print();
    test_workspace::member::print();
}