#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::{entry};
use cortex_m_semihosting::hprintln;
use hal::prelude::*;
use hal::stm32;


#[entry]
fn main() -> ! {
    let peripherals = cortex_m::Peripherals::take().unwrap();
    let device = stm32::Peripherals::take().unwrap();

    hprintln!("Hello semihosting world");

    loop { }
}

