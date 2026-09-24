use std::thread;

// 闭包 = 匿名函数 + 捕获环境。 它是一个可以像值一样传递的"函数"，能记住定义时作用域里的变量。
fn main() {
    let add = |a, b| a + b;
    println!("{}", add(2, 3));

    let base = 100;
    let add_base = |x| x + base;
    println!("{}", add_base(5));

    let mut count = 0;
    let mut inct = || count = count + 1;
    inct();
    inct();
    println!("{}", count);

    let msg = String::from("Hello, world!");
    let a = 10;
    // move 强制闭包按值捕获用到的变量。对非 Copy 类型是所有权转移，对 Copy 类型是复制。
    let handle = std::thread::spawn(move || {
        println!("{}", msg); // 将msg的所有权转移到闭包
        println!("{}", a); //  复制一份数据到闭包，对原数据没有影响
    });
    handle.join().unwrap();
    //println!("{}", msg); // 所有权已经被转移到闭包内，闭包外不可以再使用
    println!("{}", a); // 数据拷贝了一份到闭包内，这里可以继续使用

    let nums: Vec<i32> = vec![1, 2, 3];
    let event: Vec<i32> = nums.iter().filter(|&&x| x & 2 == 0).copied().collect();
    println!("{:?}", event);



    let mut s:String = String::from("hello");
    let h1 = thread::spawn(move || { println!("{}", s); });
    // let h2 = thread::spawn(move || { println!("{}", s); });  // 所有权已经进入闭包h1中，不能再使用
    h1.join().unwrap();
    //h2.join().unwrap();

    let mut x = 0;
    let h1 = thread::spawn(move || { x += 1; }); // 拷贝一份到闭包内
    let h2 = thread::spawn(move || { x += 1; });  // 拷贝一份到闭包内 两个线程不会出现竟态，修改的是两个变量
    h1.join().unwrap();
    h2.join().unwrap();

    // Arc 只负责共享所有权，不负责同步。要改数据，还得配 Mutex / RwLock / 原子类型

}
