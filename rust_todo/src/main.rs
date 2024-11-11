use clap::Parser;
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
}
