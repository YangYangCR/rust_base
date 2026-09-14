use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let a = Arc::new(String::from("hello"));
    let b = Arc::clone(&a); // 不复制底层数据 只是把计数 +1，返回一个新的 Arc 句柄
    let c = Arc::clone(&a); // 不复制底层数据 只是把计数 +1，返回一个新的 Arc 句柄
    println!("{}", a); // hello
    println!("{}", b); // hello
    println!("{}", c); // hello
    let count = Arc::strong_count(&a);
    println!("count is {}", count);


    let data_a = String::from("hello");
    let data_b = data_a.clone(); // 复制底层数据,并使用新的句柄
    let data_c  = data_a.clone(); //  复制底层数据,并使用新的句柄
    println!("{}", data_a); // hello
    println!("{}", data_b); // hello
    println!("{}", data_c); // hello


    let data = Arc::new(vec![1, 2, 3]);
    let data2 = Arc::clone(&data);   // 为子线程准备一份
    let handle = thread::spawn(move || {     // move 把 data2 搬进线程
        println!("子线程看到: {:?}", data2);
    });

    println!("主线程看到: {:?}", data);       // 主线程仍持有 data
    handle.join().unwrap();

    let data = Arc::new(5);
    // *data += 1;   // ❌ 不能改，Arc 只给 &T

    // 要在多线程间修改共享数据，得在 Arc 里再套一个提供内部可变性 + 线程安全的类型：
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = c.lock().unwrap();   // 加锁
            *num += 1;
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("{}", *counter.lock().unwrap());   // 10

}
