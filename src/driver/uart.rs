use crate::board;
use super::{ mmio_read, mmio_write, gpio };

fn aux_mu_baud(baud: usize) -> u32 {
    return ((board::AUX_UART_CLOCK / (baud * 8)) - 1) as u32
}

pub fn uart_init() {
    // enable UART1
    mmio_write(board::AUX_ENABLES, 1);
    mmio_write(board::AUX_MU_IER_REG, 0);
    mmio_write(board::AUX_MU_CNTL_REG, 0);
    mmio_write(board::AUX_MU_LCR_REG, 3);
    mmio_write(board::AUX_MU_MCR_REG, 0);
    mmio_write(board::AUX_MU_IER_REG, 0);
    // disable interrupt
    mmio_write(board::AUX_MU_IIR_REG, 0xc6);
    mmio_write(board::AUX_MU_BAUD_REG, aux_mu_baud(115200) as u32);

    gpio::gpio_use_as_alt5(14);
    gpio::gpio_use_as_alt5(15);
    // Enable Rx/Tx
    mmio_write(board::AUX_MU_CNTL_REG, 3);
}

fn uart_write_byte_ready() -> u32 {
    return mmio_read(board::AUX_MU_LSR_REG as u32) & 0x20
}

fn uart_write_char(c: char) {
    loop {
        if uart_write_byte_ready() != 0{
            break
        }
    }
    mmio_write(board::AUX_MU_IO_REG, c as u32);
}

pub fn uart_write_text(buf: &str) {
    for c in buf.chars() {
        uart_write_char(c);
    }
}