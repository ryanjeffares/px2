mod compiler;
mod scanner;
mod vm;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => compiler::compile(&args[1]),
        _ => usage(),
    };
}

fn usage() {
    println!("px2

Usage:
    px2 <file_path>");
}
