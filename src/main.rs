#![no_main]
#![no_std]

use cortex_m_rt::{entry};
use cortex_m_semihosting::hprintln;
use hal::prelude::*;
use hal::stm32;

#[allow(unused_imports)]
use panic_semihosting;

#[entry]
fn main() -> ! {
    let _peripherals = cortex_m::Peripherals::take().unwrap();
    let _device = stm32::Peripherals::take().unwrap();

    hprintln!("Hello semihosting world");

    loop { }
}

