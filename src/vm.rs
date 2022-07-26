use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

pub struct VM {
    op_list: Vec<Op>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum DataType {
    Bool,
    Int,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            DataType::Bool => "Bool",
            DataType::Int => "Int",
        })
    } 
}

pub enum Op {
    Add,
    Divide,
    Drop,
    Dup,
    Multiply,
    Over,
    Push(Value),
    PrintLn,
    Rot,
    Subtract,
    Swap,
}

#[derive(Clone, Copy)]
union Data {
    int_value: i64,
    bool_value: bool,
}

#[derive(Clone, Copy)]
pub struct Value {
    pub data_type: DataType,
    data: Data,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            match self.data_type {
                DataType::Int => write!(f, "{}", self.data.int_value),
                DataType::Bool => write!(f, "{}", self.data.bool_value),
            }
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Self) -> Self {
        // type checked in compiler
        unsafe {
            Value {
                data_type: DataType::Int,
                data: Data {
                    int_value: self.data.int_value + other.data.int_value,
                }
            }
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Self) -> Self {
        unsafe {
            Value {
                data_type: DataType::Int,
                data: Data {
                    int_value: self.data.int_value - other.data.int_value,
                }
            }
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Self) -> Self {
        unsafe {
            Value {
                data_type: DataType::Int,
                data: Data {
                    int_value: self.data.int_value * other.data.int_value,
                }
            }
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Self) -> Self {
        unsafe {
            Value {
                data_type: DataType::Int,
                data: Data {
                    int_value: self.data.int_value / other.data.int_value,
                }
            }
        }
    }
}

impl Value {
    pub fn from_int(value: i64) -> Self {
        Value {
            data_type: DataType::Int,
            data: Data { int_value: value },
        }
    }

    pub fn from_bool(value: bool) -> Self {
        Value {
            data_type: DataType::Bool,
            data: Data { bool_value: value },
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "add"),
            Op::Subtract => write!(f, "sub"),
            Op::Divide => write!(f, "div"),
            Op::Multiply => write!(f, "mul"),
            Op::Push(value) => write!(f, "push {}", value),
            Op::Dup => write!(f, "dup"),
            Op::Drop => write!(f, "drop"),
            Op::Swap => write!(f, "swap"),
            Op::Over => write!(f, "over"),
            Op::Rot => write!(f, "rot"),
            Op::PrintLn => write!(f, "println"),
        }
    }
}

impl VM {
    pub fn new() -> Self {
        VM {
            op_list: Vec::<Op>::new(),
        }
    }

    #[allow(dead_code)]
    pub fn print_ops(&self) {
        for op in self.op_list.iter() {
            println!("{}", op);
        }
    }

    pub fn push_op(&mut self, op: Op) {
        self.op_list.push(op);
    }

    pub fn run(&self) {
        let mut stack = Vec::<Value>::new();
        for op in self.op_list.iter() {
            // unwrap calls here are ok since it is checked in the compiler
            match op {
                Op::Add => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    stack.push(v2 + v1);
                }
                Op::Subtract => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    stack.push(v2 - v1);
                }
                Op::Divide => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    stack.push(v2 / v1);
                }
                Op::Multiply => {
                    let v1 = stack.pop().unwrap();
                    let v2 = stack.pop().unwrap();
                    stack.push(v2 * v1);
                }
                Op::Push(value) => stack.push(*value),
                Op::Dup => stack.push(*stack.last().unwrap()),
                Op::Drop => { stack.pop(); },
                Op::Over => {
                    // a b => a b a
                    let v = stack[stack.len() - 2];
                    stack.push(v);
                }
                Op::Rot => {
                    // a b c => b c a
                    let v = stack.remove(stack.len() - 3);
                    stack.push(v);
                }
                Op::Swap => {
                    // a b => b a
                    let v = stack.remove(stack.len() - 2);
                    stack.push(v);
                }
                Op::PrintLn => {
                    let v = stack.pop().unwrap();
                    println!("{}", v);
                }
            }
        }
    }
}
