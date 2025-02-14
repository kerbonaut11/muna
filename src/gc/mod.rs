mod alloc;
mod page;
mod ptr;
mod traits;

pub use ptr::Gc;
pub use traits::*;
pub use alloc::{StringAlloc,TableAlloc, UpValAlloc, FunctionAlloc};


mod test {
    use crate::vm::Vm;

    use super::traits::Marked;

    #[derive(Debug)]
    struct Test {
        marked:bool,
    }
    
    impl Marked for Test {
        fn is_marked(&self) -> bool {
            self.marked
        }

        fn mark(&mut self) {
            self.marked = true
        }

        fn unmark(&mut self) {
            self.marked = false
        }
    }

    #[test]
    fn test1() {
        use super::page::Page;
        assert_eq!(0b1101_i32.trailing_ones(),1);
        let mut page:Page<Test> = Page::new();
        let _ = page.alloc();
        let b = page.alloc();
        unsafe {
            (*b).marked = true;
            println!("{:?}",page);
            page.sweep();
        }
        println!("{:?}",page);
    }

    #[test]
    fn test2() {
        use crate::value::Value;
        use crate::table::Table;
        use crate::gc::MarkDown;
        use crate::gc::TableAlloc;

        let mut vm = Vm::new(vec![], Box::new([]));
        let mut t1 = Table::new().alloc();
        let t2 = Table::new().alloc();
        let t3 = Table::new().alloc();

        t1.push(Value::Table(t2));
        t1.mark_down(&mut vm);

        assert!(t1.is_marked());
        assert!(t2.is_marked());
        assert!(!t3.is_marked());

        TableAlloc::sweep();

        assert!(!t1.is_marked());
        assert!(!t2.is_marked());

        for _ in 0..4094 {
            let _ = Table::new().alloc();
        }
    }
}