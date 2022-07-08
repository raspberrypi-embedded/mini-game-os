
pub const PERIPHERAL_BASE: usize = 0xFE000000;

// Mini Uart MMIO Address
/// Auxiliary Interrupt status
pub const AUX_IRQ: usize = PERIPHERAL_BASE + 0x215000;
/// Auxiliary enables
pub const AUX_ENABLES: usize = AUX_IRQ + 0x4;
/// Mini UART I/O Data
pub const AUX_MU_IO_REG: usize = AUX_IRQ + 0x40;
/// Mini UART Interrupt Enable
pub const AUX_MU_IER_REG: usize = AUX_IRQ + 0x44;