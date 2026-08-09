use crate::garden::vegetables::Asparagus;
pub mod config;

// `pub mod garden;` 行告诉编译器包含在 src/garden.rs 中找到的代码，即编程
pub mod garden;
fn main() {
    let plant = Asparagus {};
    println!("{:?}", plant);
    let config = config::Config {
        host: String::from("Tom"),
        port: 80,
    };
}
