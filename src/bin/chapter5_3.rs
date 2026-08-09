#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 结构体中定义的叫方法，结构体之外的叫函数
// 在 impl 块中定义的所有函数都称为关联函数，因为它们与 impl 后命名的类型关联
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let react1 = Rectangle { width: 30, height: 50 };
    println!("{}", react1.area());
    if react1.width() {
        println!("The rectangle has a nonzero width; it is {}", react1.width);
    }
}