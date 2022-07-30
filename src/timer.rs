use core::arch::asm;
use core::u32;


/// get current counter by inline asm
pub fn get_counter() -> usize {
    let mut counter: usize = 0;
    unsafe{
        asm!("
            mrs {counter}, cntpct_el0
            ",
            counter = out(reg) counter
        )
    }
    counter
}


/// (ref: https://github.com/isometimes/rpi4-osdev/blob/master/part6-breakout/fb.c#L202)
pub fn wait_msec(n : usize) {
    let mut frequency: usize = 0;
    let mut counter: usize = 0;
    let mut cur_counter: usize = 0;
    unsafe{
        asm!(
            "
            mrs {frequency}, cntfrq_el0
            mrs {counter}, cntpct_el0
            ",
            counter = out(reg) counter,
            frequency = out(reg) frequency
        );
        counter += (frequency / 1000) * n;
        
        while cur_counter < counter {
            asm!("
                mrs {cur_counter}, cntpct_el0
                ",
                cur_counter = out(reg) cur_counter
            );
        }
    }

}