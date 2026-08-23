fn main() {
    let s = String::from("hello world");
    let t = s;
    // println!("{}", s); 编译报错 所有权以及转移到t 不能再使用s了 string不会copy，只是一份数据， 一份数据只能一个所有者
    // let c = t.clone(); 编译不报错 正常使用 clone相当于copy 类型 两份数据

    let s0 = String::from("hello world");
    take(s0);
    // println!("{}", s0); // 进入函数，所有权转移，推出函数，t 被 drop ,所以不能编译
    let s1 = String::from("hello world");
    borrow(&s1);
    println!("{}", s1); // 进入函数，& 不会转移所有权

    let mut s2 = String::from("hello world");
    borrow_mut(&mut s2);


    let a: i32 = 10;
    let b: i32 = a;
    println!("{}", b); // 编译不报错 i32类型是copy类型 相当于两份数据

    let y = create_string();
}

fn take(s: String) {
    println!("{}", s);
}

// 不可变借用 s 的值不能修改
fn borrow(s: &String) {
    println!("{}", s);
    //s.push('xx');  // 编译报错 不可变借用 所以不能修改
}

fn borrow_mut(s: &mut String) {
    println!("{}", s);
    s.push('x');  // 编译 通过  可变借用 所以可以修改
}

fn create_string() -> String {
    let s = String::from("hello");
    s
}
