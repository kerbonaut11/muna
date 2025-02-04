use std::{alloc::{alloc, dealloc, Layout}, fmt::Debug, hash::Hash, slice};



#[repr(packed)]
pub struct RawStrSlice {
    ptr: *const u8,
    len: u32,
    is_temp:bool,
}

impl RawStrSlice {
    pub fn new_temp(str:&str) -> Self {
        Self {
            is_temp:true,
            ptr:str.as_ptr(),
            len:str.len() as u32
        }
    }

    pub fn new_clone(str:&str) -> Self {
        
        let mem = unsafe {
            alloc(Layout::array::<u8>(str.len()).unwrap())
        };
        let slice = unsafe {
            slice::from_raw_parts_mut(mem, str.len())
        };

        slice.copy_from_slice(str.as_bytes());

        Self {
            is_temp:false,
            ptr:slice.as_ptr(),
            len:slice.len() as u32,
        }
    }

    pub fn slice(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len as usize).into()
        }
    }
}

impl PartialEq for RawStrSlice {
    fn eq(&self, other: &Self) -> bool {
        self.slice() == other.slice()
    }
}

impl Eq for RawStrSlice {}

impl Hash for RawStrSlice {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.slice());
    }
}

impl Debug for RawStrSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(unsafe {
            std::mem::transmute(self.slice())
        })?;
        Ok(())
    }
}

impl Clone for RawStrSlice {
    fn clone(&self) -> Self {
        let str:&str = unsafe {
            std::mem::transmute(self.slice())
        };

        Self::new_clone(str)
    }
}

impl Drop for RawStrSlice {
    fn drop(&mut self) {
        if !self.is_temp {unsafe{
            dealloc(self.ptr as *mut u8, Layout::array::<u8>(self.len as usize).unwrap());
        }}
    }
}