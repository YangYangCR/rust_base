use std::sync::mpsc;
use std::thread;

fn main() {
    // 标准 mpsc 是无界队列
    let (sender, receiver) = mpsc::channel::<u32>();
    let sender2 = sender.clone();
    thread::spawn(move || {
        for i in 1..=3 {
            sender.send(i).unwrap();
        }
    });
    thread::spawn(move || {
        for i in 4..=6 {
            sender2.send(i).unwrap();
        }
    });
    for _ in 0..6 {
        println!("{}", receiver.recv().unwrap());
    }
}

