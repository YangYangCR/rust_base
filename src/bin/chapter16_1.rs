use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1000));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    // 通过在闭包之前添加 move 关键字，我们强制闭包获取它正在使用的值的所有权，
    // 而不是允许 Rust 推断它应该借用这些值
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();

}