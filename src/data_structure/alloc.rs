use std::alloc::{GlobalAlloc, Layout, System};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let data = System.alloc(layout);
        eprintln!("Alloc: {:p}, size {}", data, layout.size());
        data
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        eprintln!("Free: {:p}, size {}", ptr, layout.size());
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

#[allow(dead_code)]
struct Matrix {
    data: [u8; 505],
}

impl Default for Matrix {
    fn default() -> Self {
        Self { data: [0; 505] }
    }
}

#[cfg(test)]
mod test{
    use crate::data_structure::alloc::Matrix;

    #[test]
    fn test_alloc(){
        // 传入Box::new的参数会首先出现在栈上，然后移动到堆上，如果用release模式进行build `cargo build --release`
        // 会进行inline优化。
        let data = Box::new(Matrix::default());
        println!("allocated memory {:p}, len: {}", &*data, std::mem::size_of::<Matrix>());
    }
}