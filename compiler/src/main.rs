#![allow(dead_code,unused_variables,unused_imports)]

mod tokenizer;
mod expr;
mod ast_gen;
mod utils;
mod err;
mod compiler;

pub type LocalId = u32;
pub use crate::err::{Error,Result};

fn main() {
    
}


