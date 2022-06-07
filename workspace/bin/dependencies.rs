pub use external_lib;
pub use member;
pub use folder_lib1;
pub use folder_lib2;
pub use question_lib1;
pub use recursive_include1;
pub use recursive_include2;
pub use set_lib1;
pub use set_libf;
pub use wildcard_include;

pub static CRATE: &str = env!("CARGO_PKG_NAME");

pub fn main() {
    println!("Hello from {}, {}", CRATE, file!());
}
