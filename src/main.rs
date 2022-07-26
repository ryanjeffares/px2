mod compiler;
mod scanner;
mod vm;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => compiler::compile(&args[1], false),
        3 => {
            if args[2] == "--verbose" || args[2] == "-v" {
                compiler::compile(&args[1], true);
            } else {
                usage();
            }
        } 
        _ => usage(),
    };
}

fn usage() {
    println!("px2

Usage:
    px2 <file_path> [--verbose/-v]");
}
