use crate::gc::{Marked, StringAlloc, Gc};
use std::{hash::Hash, str};

#[derive(Clone,Debug)]
pub enum LuaString {
    Small{marked:bool,size:u8,array:[u8;24]},
    Big(bool,String)
}

impl LuaString {
    pub fn alloc(self) -> Gc<Self> {
        StringAlloc::alloc(self)
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Small { size, array, .. } => unsafe {
                str::from_utf8_unchecked(&array[0..*size as usize])
            },
            Self::Big(_, string) => &string
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Small{ size,.. } => *size as usize,
            Self::Big(_, str) => str.len()
        }
    }
}

impl Marked for LuaString {
    fn is_marked(&self) -> bool {
        match self {
            Self::Small { marked, .. } => *marked,
            Self::Big(marked,_ ) => *marked
        }
    }

    fn mark(&mut self) {
        match self {
            Self::Small { marked, .. } => *marked = true,
            Self::Big(marked,_ ) => *marked = true
        }
    }

    fn unmark(&mut self) {
        match self {
            Self::Small { marked, .. } => *marked = false,
            Self::Big(marked,_ ) => *marked= false
        }
    }
}

impl PartialEq for LuaString {
    fn eq(&self, other: &Self) -> bool {
        self.to_str() == other.to_str()
    }
}

impl Eq for LuaString {}

impl Hash for LuaString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.to_str().as_bytes());
    }
}

