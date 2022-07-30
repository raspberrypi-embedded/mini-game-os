// use bcm2711::{ addr, gpio, mmio_write, mini_uart };
use raspiberry_peripherals::{ addr, gpio, mmio_write, mini_uart };
use core::fmt::{ Write, Error };

pub const AUX_UART_CLOCK: usize = 500000000;
pub const UART_MAX_QUEUE: usize  = 16 * 1024;

/// Uart Output Queue which wrapped by mutex
pub static mut UART: Uart = Uart{
    read_index: 0,
    write_index: 0,
    queue: [0u8; UART_MAX_QUEUE]
};

pub struct Uart {
    read_index: usize,
    write_index: usize,
    queue: [u8; UART_MAX_QUEUE]
}

impl Uart {
    pub fn init(&self) {
        // enable UART1
        mmio_write(addr::AUX_ENABLES, 1);
        mmio_write(addr::AUX_MU_IER_REG, 0);
        mmio_write(addr::AUX_MU_CNTL_REG, 0);
        mmio_write(addr::AUX_MU_LCR_REG, 3);
        mmio_write(addr::AUX_MU_MCR_REG, 0);
        mmio_write(addr::AUX_MU_IER_REG, 0);
        // disable interrupt
        mmio_write(addr::AUX_MU_IIR_REG, 0xc6);
        mmio_write(addr::AUX_MU_BAUD_REG, Self::aux_mu_baud(115200) as u32);

        gpio::gpio_use_as_alt5(14);
        gpio::gpio_use_as_alt5(15);
        // Enable Rx/Tx
        mmio_write(addr::AUX_MU_CNTL_REG, 3);
    }


    fn aux_mu_baud(baud: usize) -> u32 {
        return ((AUX_UART_CLOCK / (baud * 8)) - 1) as u32
    }

    /// Check if queue is empty
    fn is_queue_empty(&self) -> bool {
        return self.read_index == self.write_index
    }

    fn readable(&self) -> bool {
        mini_uart::uart_read_byte_ready()
    }

    fn writeable(&self) -> bool {
        mini_uart::uart_write_byte_ready()
    }

    /// Read a char from mini uart
    pub fn read_byte(&self) -> char {
        mini_uart::uart_read_char()
    }

    pub fn write_byte(&self, c: char) {
        mini_uart::uart_write_char(c)
    }

    /// Write a serials of chars into mini uart
    pub fn write(&self, buf: &str) {
        mini_uart::uart_write_text(buf);
    }


    fn update_fifo(&mut self) {
        while !self.is_queue_empty() && self.writeable() {
            self.write_byte(self.queue[self.read_index] as char);
            self.read_index = (self.read_index + 1) % UART_MAX_QUEUE;
        }
    }

    fn update_write_index(&mut self, c: char) {
        let next = (self.write_index + 1) % UART_MAX_QUEUE;
        self.update_fifo();

        self.queue[self.write_index] = c as u8;
        self.write_index = next;
    }
    
    /// Check mini uart is readable and update
    /// uart status
    pub fn non_block_wait_read(&mut self) {
        self.update_fifo();

        if self.readable() {
            let c = self.read_byte();
            if c == '\r' { self.write_byte('\n') }
            else{ self.update_write_index(c) }
        }
    }

    pub fn uart_read(&mut self) -> Option<char> {
        self.update_fifo();

        if self.readable() {
            let c = self.read_byte();
            return Some(c)
        }
        None
    }
}

pub fn uart_init() {
    unsafe{ UART.init() };
}

pub fn uart_write_text(buf: &str) {
    unsafe{ UART.write(buf) }
}

pub fn uart_wait_read() {
    unsafe{ UART.non_block_wait_read() }
}

pub fn uart_read() -> Option<char> {
    unsafe{ UART.uart_read() }
}

impl Write for Uart {
    fn write_str(&mut self, out: &str) -> Result<(), Error> {
        for c in out.chars() {
            self.write_byte(c);
        }
        Ok(())
    }
}
