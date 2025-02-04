pub trait Marked {
    fn is_marked(&self) -> bool;
    fn mark(&mut self);
    fn unmark(&mut self);
}

use crate::vm::Vm;

pub trait MarkDown {
    fn mark_down(&mut self,vm:&mut Vm);
}