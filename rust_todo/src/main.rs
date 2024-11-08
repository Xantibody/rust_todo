use std::env;

struct Cli {
    str: String,
}

fn main() {
    let str: String = env::args().nth(1).expect("no string giving");
    let args = Cli { str };
    println!("print arg -> {}", args.str);
}
