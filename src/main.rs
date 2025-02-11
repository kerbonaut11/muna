#![allow(dead_code,unused_variables)]

mod value;
mod table;
mod err;
mod gc;
mod vm;
mod string;
mod bytecode;
mod exec;
mod program;
mod stack;
mod ops;
mod upval;
mod compiler;
mod user_data;
mod function;
mod test;

pub use crate::err::{Error,Result};

fn main() {
    
}


