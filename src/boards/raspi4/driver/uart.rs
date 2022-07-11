use bcm2711::{ addr, gpio, mmio_write };

pub const AUX_UART_CLOCK: usize = 500000000;
pub const UART_MAX_QUEUE: usize  = 16 * 1024;

fn aux_mu_baud(baud: usize) -> u32 {
    return ((AUX_UART_CLOCK / (baud * 8)) - 1) as u32
}

pub fn uart_init() {
    // enable UART1
    mmio_write(addr::AUX_ENABLES, 1);
    mmio_write(addr::AUX_MU_IER_REG, 0);
    mmio_write(addr::AUX_MU_CNTL_REG, 0);
    mmio_write(addr::AUX_MU_LCR_REG, 3);
    mmio_write(addr::AUX_MU_MCR_REG, 0);
    mmio_write(addr::AUX_MU_IER_REG, 0);
    // disable interrupt
    mmio_write(addr::AUX_MU_IIR_REG, 0xc6);
    mmio_write(addr::AUX_MU_BAUD_REG, aux_mu_baud(115200) as u32);

    gpio::gpio_use_as_alt5(14);
    gpio::gpio_use_as_alt5(15);
    // Enable Rx/Tx
    mmio_write(addr::AUX_MU_CNTL_REG, 3);
}