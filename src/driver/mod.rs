use core::ptr::{read, write};



pub fn mmio_read(addr: usize) -> usize {
    unsafe{
        let val = read(addr as *const usize);
    }
}

pub fn mmio_write(addr: usize, val: usize) {
    unsafe{
        write(addr as *mut usize, val);
    }
}