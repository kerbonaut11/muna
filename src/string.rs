use crate::gc::{Marked, StringAlloc, Gc};
use std::{hash::Hash, str};

#[derive(Clone,Debug)]
pub enum LuaString {
    Small{marked:bool,size:u8,array:[u8;16]},
    Big{marked:bool,str:Box<str>},
}

impl LuaString {
    pub fn new(str:&str) -> Self {
        if str.len() <= 16 {
            Self::Small { marked: false, size: str.len() as u8, array: {
                let mut array = [0u8;16];
                for i in 0..str.len() {
                    array[i] = str.as_bytes()[i];
                }
                array
            }}
        } else {
            Self::Big{marked:false,str:str.into()}
        }
    }

    pub fn alloc(self) -> Gc<Self> {
        StringAlloc::alloc(self)
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Small { size, array, .. } => unsafe {
                str::from_utf8_unchecked(&array[0..*size as usize])
            },
            Self::Big{str,..} => &str
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Small{ size,.. } => *size as usize,
            Self::Big{str,..}=> str.len()
        }
    }
}

impl Marked for LuaString {
    fn is_marked(&self) -> bool {
        match self {
            Self::Small{marked,..} => *marked,
            Self::Big{marked,.. } => *marked
        }
    }

    fn mark(&mut self) {
        match self {
            Self::Small{marked,..} => *marked = true,
            Self::Big{marked,.. } => *marked = true
        }
    }

    fn unmark(&mut self) {
        match self {
            Self::Small{marked,..} => *marked = false,
            Self::Big{marked,.. } => *marked= false
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

