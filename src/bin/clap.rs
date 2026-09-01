use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
enum Backend {
    // value 这个宏定义会把字符串和枚举值映射起来
    #[value(name = "sglang")]
    Sglang,

    #[value(name = "vllm")]
    Vllm,
}

// Parser这个trait 会解析命令行参数 并构造Args这个结构体
#[derive(Parser, Debug)]
struct ArgsStruct {
    // long表示长参数，即 --backend
    #[arg(long)]
    backend: Backend,
}

// cargo run --bin clap -- --backend vllm
fn main() {
    let args = ArgsStruct::parse();
    print!("backend is  {:?}", args.backend);
}
