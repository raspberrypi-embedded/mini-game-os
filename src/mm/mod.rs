use allocator::*;
use core::alloc::{ GlobalAlloc, Layout };
use core::cell::RefCell;
use crate::println;

// Buddy System for memory allocate

// min leaf size for buddy system
pub const LEAF_SIZE:usize = 16;

// max memory size for buddy system
pub const MAX_ALIGNMENT:usize = 4096;

#[global_allocator]
pub static KERNEL_HEAP: KernelHeap = KernelHeap::uninit();

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("alloc error: {:?}", layout);
}

// kernel heap
pub struct KernelHeap(RefCell<BuddySystem>);

unsafe impl GlobalAlloc for KernelHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.borrow_mut().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.borrow_mut().dealloc(ptr, layout)
    }
}

impl KernelHeap {
    const fn uninit() -> Self {
        Self(RefCell::new(BuddySystem::uninit()))
    }

    unsafe fn init(&self, start: usize, end: usize) {
        let res = self.0.borrow_mut().init(start, end, LEAF_SIZE, MAX_ALIGNMENT);
        match res {
            Ok(()) => {
                println!("KernelHeap: success to init.");
            },

            Err(err) => {
                println!("KernelHeap: init error: {}.", err);
            }
        }
    }

    pub fn mm_init(&self) {
        extern "C" {
            fn _end();
        }
        let end = _end as usize;
        #[cfg(feature = "board_qemu")]
        {
            use bcm2837::addr::PERIPHERAL_BASE;
            println!("KernelHeap: available memory: [{:#x}, {:#x})", end, PERIPHERAL_BASE);
            unsafe{ self.init(end, PERIPHERAL_BASE) };
        }
        #[cfg(feature = "board_raspi4")] 
        {
            use bcm2711::addr::PERIPHERAL_BASE;
            println!("KernelHeap: available memory: [{:#x}, {:#x})", end, PERIPHERAL_BASE);
            unsafe{ self.init(end, PERIPHERAL_BASE) };
        }
    }
}

unsafe impl Sync for KernelHeap{}