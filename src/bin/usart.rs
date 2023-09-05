#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use stm32f4xx_hal::{block, pac, prelude::*};

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    let pins = (gpioa.pa2, gpioa.pa3);
    let mut serial = dp.USART2.serial(pins, 9600.bps(), &clocks).unwrap();

    loop {
        if let Ok(c) = block!(serial.read()) {
            if c != 10 {
                for i in "You Typed: ".as_bytes() {
                    block!(serial.write(*i)).unwrap_or_default();
                }
                // Printing character
                block!(serial.write(c)).unwrap_or_default();
                // Printing new line
                block!(serial.write(10)).unwrap_or_default();
            };
        };
    }
}
