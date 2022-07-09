use crate::board;
use super::{ mmio_read, mmio_write };

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

    // Enable Rx/Tx
    mmio_write(board::AUX_MU_CNTL_REG, 3);
}