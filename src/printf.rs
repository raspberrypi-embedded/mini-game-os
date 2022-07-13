use crate::board::driver::UART;
use core::fmt;

pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    // let mut uart = unsafe{ UART };
    // uart.write_fmt(args).unwrap();
    unsafe{ UART.write_fmt(args).unwrap()}
}

pub fn console_ptr(c: u8) {
    // let mut uart = unsafe{ UART.lock() };
    // uart.write_byte(c as char)
    unsafe{ UART.write_byte(c as char) }
}

/// implement print and println! macro
///
/// use [`core::fmt::Write`] trait's [`console::Stdout`]
#[macro_export]
macro_rules! print {
    (fmt:literal$(, $($arg: tt)+)?) => {
        $crate::printf::console_putchar(format_args!($fmt(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt:literal$(, $($arg: tt)+)?) => {
        $crate::printf::_print(format_args!(concat!($fmt, "\n") $(,$($arg)+)?));
    }
}
