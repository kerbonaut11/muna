use super::page::Page;
use super::Gc;
use super::Marked;

pub struct GcAlloc<T> {
    pages:Vec<Option<Box<Page<T>>>>
}

impl<T:Marked> GcAlloc<T> {
    pub const fn new() -> Self {
        Self { pages: vec![] }
    }

    pub fn alloc(&mut self,val:T) -> Gc<T> {
        for page in &mut self.pages {
            if let Some(page) = page {
                if !page.is_full() {
                    let ptr = page.alloc();
                    unsafe{*ptr = val;}
                    return Gc::new(ptr);
                }
            }
        }

        let mut new_page = Box::new(Page::<T>::new());
        let ptr = new_page.alloc();
        unsafe{*ptr = val;}
        self.add_page(new_page);
        return Gc::new(ptr);
    }

    fn add_page(&mut self,p:Box<Page<T>>) {
        let p = Some(p);
        for page in &mut self.pages {
            if page.is_none() {
                *page = p;
                return;
            }
        }
        self.pages.push(p);
    }

    pub fn sweep(&mut self) {
        self.pages.iter_mut().for_each(|page| {
            if let Some(p) = page {
                p.sweep();
                if p.should_dealloc() {
                    *page = None;
                }
            }
        });
    }
}

use std::sync::Mutex;
use std::cell::UnsafeCell;

macro_rules! global_alloc {
    ($type:ty,$static:ident,$alloc:ident) => {
        static $static:Mutex<UnsafeCell<GcAlloc<$type>>> = Mutex::new(UnsafeCell::new(GcAlloc::new()));
        pub struct $alloc;
        impl $alloc {
            pub fn alloc(val:$type) -> Gc<$type> {
                unsafe {
                    (*$static
                        .lock()
                        .unwrap()
                        .get())
                        .alloc(val)
                }
            }

            pub fn sweep() {
                unsafe {
                    (*$static
                        .lock()
                        .unwrap()
                        .get())
                        .sweep()
                }
            }
        }
    };
}

use crate::table::Table;
use crate::string::LuaString;
use crate::upval::UpVal;
global_alloc!{LuaString,STRING_ALLOC,StringAlloc}
global_alloc!{Table,TABLE_ALLOC,TableAlloc}
global_alloc!{UpVal,UP_VAL_ALLOC,UpValAlloc}