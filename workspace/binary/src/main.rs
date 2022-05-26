pub static CRATE: &str = env!("CARGO_PKG_NAME");

pub fn print() {
    println!("Hello from {}, {}", CRATE, file!());
}

pub fn main() {
    println!(
        "Running on {arch}-{os}",
        arch = std::env::consts::ARCH,
        os = std::env::consts::OS
    );
    self::print();
    test_workspace::print();
    test_workspace::external_lib::print();
    test_workspace::member::print();
}
