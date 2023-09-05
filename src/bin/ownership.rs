#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use stm32f4xx_hal::gpio::Output;
use stm32f4xx_hal::gpio::Pin;
use stm32f4xx_hal::pac;
use stm32f4xx_hal::prelude::*;

/// Takes ownership of D13 and sets it to high.
pub fn takes_ownership(mut d13: Pin<'A', 5, Output>) {
    d13.set_high();
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Setup pin
    let gpioa = dp.GPIOA.split();
    let led = gpioa.pa5.into_push_pull_output();

    // Setup delay
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(48.MHz()).freeze();
    let mut delay = dp.TIM5.delay_us(&clocks);

    defmt::println!("Moving Variable");
    // Moving 1
    let mut led2 = led;
    led2.set_high();
    delay.delay_ms(1000_u32);
    // Does not compile
    // led.set_low();
    led2.set_low();

    delay.delay_ms(1000_u32);
    defmt::println!("Moving Functions");
    // Moving 2
    takes_ownership(led2);
    // Does not compile
    // led2.set_low();

    loop {}
}
