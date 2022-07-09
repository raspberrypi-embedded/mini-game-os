use core::ptr::{read, write};


/// Read something from mmio address
pub fn mmio_read(addr: u32) -> u32 {
    unsafe{
        let val = read(addr as *const u32);
    }
}

/// Write something to mmio address
pub fn mmio_write(addr: u32, val: u32) {
    unsafe{
        write(addr as *mut u32, val);
    }
}