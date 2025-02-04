use std::{alloc::{alloc, dealloc, Layout}, mem::ManuallyDrop, sync::atomic::AtomicPtr};
use super::traits::Marked;

const PAGE_SIZE:usize = 4096;
const GROUP_COUNT:usize = PAGE_SIZE/64;

#[derive(Debug)]
pub(super) struct Page<T> {
    base_ptr: AtomicPtr<ManuallyDrop<T>>,

    free_group_list: [u32;GROUP_COUNT],
    free_group_count: usize,

    groups: [u64;GROUP_COUNT],
    alloc_count: usize,

    was_full:bool,
}

impl<T:Marked> Page<T> {
    pub(super) fn new() -> Self {
        let mem = unsafe {
            alloc(Layout::array::<T>(PAGE_SIZE).unwrap())
        };
        let mut free_group_list:[u32;GROUP_COUNT] = [0;GROUP_COUNT];
        free_group_list.iter_mut().enumerate().for_each(|(i,x)| *x = i as u32 );
        Self {
            base_ptr:AtomicPtr::new(mem as *mut ManuallyDrop<T>),
            free_group_list,
            free_group_count:free_group_list.len(),

            groups: [0;GROUP_COUNT],
            alloc_count:0,

            was_full:false
        }
    }

    pub(super) fn alloc(&mut self) -> *mut T {
        let group_idx = self.free_group_list[self.free_group_count-1] as usize;
        let bit_idx = self.groups[group_idx].trailing_ones();
        self.groups[group_idx] |= 1 << bit_idx;
        if self.groups[group_idx] == u64::MAX {
            self.free_group_count -= 1;
        }
        self.alloc_count += 1;
        return unsafe {
            let ptr = self.base_ptr.get_mut().add(group_idx*64 + bit_idx as usize);
            std::mem::transmute(ptr)
        }
    } 

    pub(super) fn is_full(&self) -> bool {
        self.alloc_count == PAGE_SIZE
    }

    pub(super) fn is_empty(&self) -> bool {
        self.alloc_count == 0
    }

    pub(super) fn should_dealloc(&self) -> bool {
        self.is_empty() && self.was_full
    }

    pub(super) fn sweep(&mut self) {
        for group_idx in 0..GROUP_COUNT {
            let group_prev = self.groups[group_idx];
            for bit_idx in 0..64 {
                if (self.groups[group_idx] >> bit_idx) & 1 == 1 {
                    unsafe {
                        let ptr = self.base_ptr.get_mut().add(group_idx*64 + bit_idx as usize);
                        if !(*ptr).is_marked() {
                            println!("freeing {:?}",ptr);
                            ManuallyDrop::<T>::drop(std::mem::transmute(ptr));
                            self.groups[group_idx] &= !(1 << bit_idx);
                            self.alloc_count -= 1;
                        } else {
                            println!("unmarking {:?}",ptr);
                            (*ptr).unmark();   
                        }
                    }
                }
            }
            if group_prev != 0 && self.groups[group_idx] == 0 {
                self.free_group_list[self.free_group_count] = group_idx as u32;
                self.free_group_count += 1;
            }
        }
        if self.is_full() {
            self.was_full = true;
        }
    }
}

impl<T> Drop for Page<T> {
    fn drop(&mut self) {
        unsafe {
            dealloc(*self.base_ptr.get_mut() as *mut u8, Layout::array::<T>(PAGE_SIZE).unwrap());  
        } 
    }
}