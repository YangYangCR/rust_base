fn main() {
    let guess : u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);
    let x = 2.0; // 64位浮点
    let y : f32 = 3.0; // 32位浮点
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let tup: (i32, f64, u8) = (500, 6.4, 1); // 元组

    let a = [1, 2, 3, 4, 5]; // 数组

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // 5个元素的i32类型数组

    // 数组取值
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];


}