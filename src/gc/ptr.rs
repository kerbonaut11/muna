use std::{fmt::Debug, ops::{Deref, DerefMut}};

use super::Marked;

pub struct Gc<T> {
    ptr:*mut T
}

impl<T:Marked> Gc<T> {
    pub fn new(ptr:*mut T) -> Self {
        Self { ptr }
    }
}

impl<T> Gc<T>  {
    pub fn addres(&self) -> usize {
        self.ptr as usize
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

impl<T:Clone> Gc<T>  {
    pub fn clone_inner(&self) -> T {
        unsafe {
            (*self.ptr).clone()
        }
    }
}

impl<T> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl<T> Copy for Gc<T> {}

impl<T> Deref for Gc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self.ptr) }
    }
}

impl<T> DerefMut for Gc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self.ptr) }
    }
}

impl<T:Debug> Debug for Gc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner:&T = self.deref();
        f.write_str(&format!("{:?}",inner))?;
        Ok(())
    }
}