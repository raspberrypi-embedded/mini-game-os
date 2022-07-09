use crate::board;
use super::{ mmio_read, mmio_write };

fn gpio_call(
    pin_number: u32, 
    val: u32,
    base: u32, 
    field_size: u32, 
    field_max: u32
) -> Result<(), ()> {
    let field_mask: u32 = (1 << field_size) - 1;
    if pin_number > field_max { return Err(()) }
    if val > field_mask{ return Err(()) }

    let num_fields = 32 / field_size;
    let reg = base + ((pin_number / num_fields) * 4);
    let shift = (pin_number % num_fields) * field_size;

    let mut curval = mmio_read(reg as usize);
    curval &= !(field_mask << shift);
    curval |= val << shift;
    mmio_write(reg as usize, curval);

    return Ok(())
}

fn gpio_set(pin_number: u32, value: u32) -> Result<(), ()>{ 
    return gpio_call(pin_number, value, board::GPSET0 as u32, 1, board::GPIO_MAX_PIN as u32);
}
fn gpio_clear(pin_number: u32, value: u32) -> Result<(), ()> { 
    return gpio_call(pin_number, value, board::GPCLR0 as u32, 1, board::GPIO_MAX_PIN as u32)
}
fn gpio_pull(pin_number: u32, value: u32) -> Result<(), ()> { 
    return gpio_call(pin_number, value, board::GPPUPPDN0 as u32, 2, board::GPIO_MAX_PIN as u32); 
}
fn gpio_function(pin_number: u32, value: u32) -> Result<(), ()> { 
    return gpio_call(pin_number, value, board::GPFSEL0 as u32, 3, board::GPIO_MAX_PIN as u32); 
}

pub fn gpio_use_as_alt3(pin_number: u32) {
    gpio_pull(pin_number, board::PULL_NONE as u32).unwrap();
    gpio_function(pin_number, board::GPIO_FUNCTION_ALT3 as u32).unwrap();
}   

pub fn gpio_use_as_alt5(pin_number: u32) {
    gpio_pull(pin_number, board::PULL_NONE as u32).unwrap();
    gpio_function(pin_number, board::GPIO_FUNCTION_ALT5 as u32).unwrap();
}   