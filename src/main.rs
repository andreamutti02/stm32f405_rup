//! CDC-ACM serial port example using polling in a busy loop.
//! Target board: any STM32F4 with a OTG FS peripheral and a 25MHz HSE crystal
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;

use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    

    loop {
        
    }
}