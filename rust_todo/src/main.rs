use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    quotes: Vec<String>,
}

fn main() {
    let args: Args = Args::parse();
    for v in &args.quotes {
        println!("print arg -> {}", v);
    }

    if Path::new("~/.todo/todo.md").is_file() {
        print!("ある!")
    } else {
        print!("ない!")
    }
}
