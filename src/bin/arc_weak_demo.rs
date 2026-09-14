use std::sync::{Arc, Weak};

fn main() {
    let strong: Arc<String> = Arc::new("hello".to_string());
    // 降级为弱引用
    let weak: Weak<String> = Arc::downgrade(&strong);
    // 此时强引用还在，upgrade 成功
    match weak.upgrade() {
        None => {
            println!("data is not exists")
        }
        Some(_) => {
            println!("data is exists");
        }
    }

    // 丢弃唯一的强引用
    drop(strong);
    // 数据已被释放，upgrade 失败
    match weak.upgrade() {
        None => {
            println!("data is not exists")
        }
        Some(_) => {
            println!("data is exists");
        }
    }

    

}
