fn main() {
    println!("hello world!");
    another_function(5, 'h');
    let x= five();
    println!("The value of x is: {x}");
    let y = plus_one(10);
}

fn another_function(x: i32, unit_label: char) {
    println!("Another function.");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
