use derive_more::derive::From;

use crate::{function::CallErr, ops::OpErr};

#[derive(From,Debug)]
pub enum Error {
    OpErr(OpErr),
    CallError(CallErr),
    Halt
}

pub type Result<T> = core::result::Result<T,Error>;