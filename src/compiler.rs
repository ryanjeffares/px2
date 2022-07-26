use std::fs;
use std::path::Path;
use std::num::IntErrorKind;

use colored::*;

use crate::scanner::{Scanner, TokenType, Token};
use crate::vm::{DataType, Op, Value, VM};

struct CompilerContext<'a> {
    file_path: String,
    code_string: &'a String,
    stack: Vec<DataType>,
    vm: VM,
    had_error: bool,
}

pub fn compile(file_path: &String, verbose: bool) {
    let path = Path::new(file_path.trim());

    let extension = path.extension();
    if extension.is_none() || extension.unwrap() != "px2" {
        eprintln!("Given file {:?} was not a '.px2' file", path);
        return;
    }

    if !path.exists() {
        eprintln!("Given file {:?} does not exist", path);
        return;
    }

    let code_string = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            return;
        }
    };

    let mut scanner = Scanner::new(&code_string);
    let mut compiler = CompilerContext {
        file_path: file_path.to_string(),
        code_string: &code_string,
        stack: Vec::<DataType>::new(),
        vm: VM::new(),
        had_error: false,
    };

    let start = std::time::Instant::now();

    loop {
        let token = scanner.scan_token();

        #[cfg(debug_assertions)]
        if verbose {
            println!("{}", token);
        }

        match token.token_type {
            TokenType::Dup => {
                if compiler.stack.is_empty() {
                    error(&token, &mut compiler, "no data on the stack to dup".to_string());
                } else {
                    compiler.push_op(Op::Dup);
                }
            }
            TokenType::Drop => {
                if compiler.stack.is_empty() {
                    error(&token, &mut compiler, "no data on the stack to drop".to_string());
                } else {
                    compiler.push_op(Op::Drop);
                }
            }
            TokenType::EndOfFile => break,
            TokenType::Error => error(&token, &mut compiler, "invalid token".to_string()),
            TokenType::False => compiler.push_op(Op::Push(Value::from_bool(false))),
            TokenType::Int => int(&token, &mut compiler),
            TokenType::Minus => subtract(&token, &mut compiler),
            TokenType::Over => {
                let len = compiler.stack.len();
                if len < 2 {
                    error(&token, &mut compiler, format!("need 2 elements on the stack to perform over but found {}", len));
                } else {
                    compiler.push_op(Op::Over);
                }
            }
            TokenType::Plus => add(&token, &mut compiler),
            TokenType::PrintLn => println(&token, &mut compiler),
            TokenType::Slash => divide(&token, &mut compiler),
            TokenType::Rot => {
                let len = compiler.stack.len();
                if len < 3 {
                    error(&token, &mut compiler, format!("need 3 elements on the stack to perform rot but found {}", len));
                } else {
                    compiler.push_op(Op::Rot);
                }
            }
            TokenType::Star => multiply(&token, &mut compiler),
            TokenType::Swap => {
                let len = compiler.stack.len();
                if len < 2 {
                    error(&token, &mut compiler, format!("need 2 elements on the stack to perform swap but found {}", len));
                } else {
                    compiler.push_op(Op::Swap);
                }
            } 
            TokenType::True => compiler.push_op(Op::Push(Value::from_bool(true))),
            TokenType::Identifier => error(&token, &mut compiler, "identifiers are not implemented yet".to_string())
        }

        if compiler.had_error {
            eprintln!("Stopping execution due to compilation errors");
            return;
        }
    }

    if !compiler.stack.is_empty() {
        eprintln!("Unhandled data on the stack");
        return;
    }

    if verbose {
        println!("Compilation succeeded in {:?}", start.elapsed());
    }

    #[cfg(debug_assertions)]
    compiler.vm.print_ops();

    compiler.vm.run();
}

impl<'a> CompilerContext<'a> {
    fn push_op(&mut self, op: Op) {
        match op {
            Op::Add|Op::Divide|Op::Subtract|Op::Multiply|Op::Drop|Op::PrintLn => { self.stack.pop(); },
            Op::Dup => self.stack.push(*self.stack.last().unwrap()),
            Op::Over => {
                // a b => a b a
                self.stack.push(self.stack[self.stack.len() - 2]);
            }
            Op::Push(value) => self.stack.push(value.data_type),
            Op::Rot => {
                // a b c => b c a
                let v = self.stack.remove(self.stack.len() - 3);
                self.stack.push(v);
            }
            Op::Swap => {
                // a b => b a
                let v = self.stack.remove(self.stack.len() - 2);
                self.stack.push(v);
            }
        };

        self.vm.push_op(op);
    }
}

fn get_code_at_line(line: usize, code_string: &String) -> String {
    let mut curr_line = 1usize;
    let mut str_index = 0usize;
    let code_bytes = code_string.as_bytes();

    while curr_line < line {
        if code_bytes[str_index] as char == '\n' {
            curr_line += 1;
        }
        str_index += 1;
    }

    let code_substr = &code_string.as_str()[str_index..];
    code_substr[0..code_substr.find('\n').unwrap()].to_string()
}

fn int(token: &Token, compiler: &mut CompilerContext) {
    let parse_result = token.text.parse::<i64>();
    if parse_result.is_err() {
        error(token, compiler, match parse_result.err().unwrap().kind() {
            IntErrorKind::Empty => "tried to parse int from empty string",
            IntErrorKind::InvalidDigit => "invalid digit found in string",
            IntErrorKind::PosOverflow => "positive integer out of range",
            IntErrorKind::NegOverflow => "negative integer out of range",
            IntErrorKind::Zero => "value cannot be zero",
            _ => "unexpected error"
        }.to_string());
        return;
    }

    compiler.push_op(Op::Push(Value::from_int(parse_result.unwrap())));
}

fn add(token: &Token, compiler: &mut CompilerContext) {
    let len = compiler.stack.len();
    if len < 2 {
        error(token, compiler, format!("expected 2 values on the stack to perform addition, found {}", len));
        return;
    }
    if compiler.stack[len - 1] != DataType::Int {
        error(token, compiler, format!("expected integer on top of the stack to perform addition, found {}", compiler.stack[len - 1]));
        return;
    }
    if compiler.stack[len - 2] != DataType::Int {
        error(token, compiler, format!("expected integer one down from the top of the stack to perform addition, found {}", compiler.stack[len - 2]));
        return;
    }

    compiler.push_op(Op::Add);
}

fn subtract(token: &Token, compiler: &mut CompilerContext) {
    let len = compiler.stack.len();
    if len < 2 {
        error(token, compiler, format!("expected 2 values on the stack to perform subtraction, found {}", len));
        return;
    }
    if compiler.stack[len - 1] != DataType::Int {
        error(token, compiler, format!("expected integer on top of the stack to perform subtraction, found {}", compiler.stack[len - 1]));
        return;
    }
    if compiler.stack[len - 2] != DataType::Int {
        error(token, compiler, format!("expected integer one down from the top of the stack to perform subtraction, found {}", compiler.stack[len - 2]));
        return;
    }
    compiler.push_op(Op::Subtract);
}

fn multiply(token: &Token, compiler: &mut CompilerContext) {
    let len = compiler.stack.len();
    if len < 2 {
        error(token, compiler, format!("expected 2 values on the stack to perform multiplication, found {}", len));
        return;
    }
    if compiler.stack[len - 1] != DataType::Int {
        error(token, compiler, format!("expected integer on top of the stack to perform multiplication, found {}", compiler.stack[len - 1]));
        return;
    }
    if compiler.stack[len - 2] != DataType::Int {
        error(token, compiler, format!("expected integer one down from the top of the stack to perform multiplication, found {}", compiler.stack[len - 2]));
        return;
    }
    compiler.push_op(Op::Multiply);
}

fn divide(token: &Token, compiler: &mut CompilerContext) {
    let len = compiler.stack.len();
    if len < 2 {
        error(token, compiler, format!("expected 2 values on the stack to perform division, found {}", len));
        return;
    }
    if compiler.stack[len - 1] != DataType::Int {
        error(token, compiler, format!("expected integer on top of the stack to perform division, found {}", compiler.stack[len - 1]));
        return;
    }
    if compiler.stack[len - 2] != DataType::Int {
        error(token, compiler, format!("expected integer one down from the top of the stack to perform division, found {}", compiler.stack[len - 2]));
        return;
    }
    compiler.push_op(Op::Divide);
}

fn println(token: &Token, compiler: &mut CompilerContext) {
    if compiler.stack.is_empty() {
        error(token, compiler, "nothing on stack to print".to_string());
        return;
    }
    compiler.push_op(Op::PrintLn);
}

fn error(token: &Token, compiler: &mut CompilerContext, message: String) {
    compiler.had_error = true;
    eprintln!("{} at '{}': {}", "Compiler Error".red(), token.text, message);
    eprintln!("       --> {}:{}:{}", compiler.file_path, token.line, token.column);
    eprintln!("        |");
    eprintln!("{:7} | {}", token.line, get_code_at_line(token.line, compiler.code_string));
    eprint!("        | ");

    for _ in 0..token.column - 1{
        eprint!(" ");
    }

    for _ in 0..token.length {
        eprint!("{}", "^".red());
    }

    eprintln!();
}
