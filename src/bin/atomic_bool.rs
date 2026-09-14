use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/**
AtomicBool 是 Rust 标准库 std::sync::atomic 里的原子布尔类型，
用于多线程间无锁地共享和修改一个 bool。
它最典型的用途是跨线程的"开关/标志位"，比如控制循环退出、标记初始化完成等。
普通的 bool 在多线程下不能安全共享修改
*/
fn main() {
    init_once();
    let atomic_bool = AtomicBool::new(false);
    println!("{}", atomic_bool.load(Ordering::Relaxed));
    atomic_bool.store(true, Ordering::Relaxed);

    atomic_bool.swap(false, Ordering::Relaxed);

    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);

    let handle = thread::spawn(move || {
        while r.load(Ordering::Relaxed) {
            println!("工作中...");
            thread::sleep(Duration::from_millis(300));
        }
        println!("exiting...");
    });

    thread::sleep(Duration::from_secs(1));
    running.store(false, Ordering::Relaxed);

    handle.join().unwrap();
}

fn init_once() {
    if !INITIALIZED.swap(true, Ordering::SeqCst) {
        // swap 返回旧值：旧值是 false，说明我是第一个进来的
        println!("首次初始化");
    } else {
        println!("已经初始化过了");
    }
}