fn main() {
    let args: Vec<String> = std::env::args().collect();
    print!("arg is {}", args.join(" "));
}

