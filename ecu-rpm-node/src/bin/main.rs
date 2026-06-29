#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::main;

#[main]
fn main() -> ! {
    let _peripherals = esp_hal::init(esp_hal::Config::default());

    let gpio_enable_reg = 0x3FF44020 as *mut u32; 
    
    let gpio_out_reg = 0x3FF44004 as *mut u32; 

    unsafe {
        core::ptr::write_volatile(gpio_enable_reg, 1 << 2);

        core::ptr::write_volatile(gpio_out_reg, 1 << 2);
    }

    loop {}
}