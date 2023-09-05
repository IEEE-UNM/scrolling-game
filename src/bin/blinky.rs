#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use stm32f4xx_hal::pac;
use stm32f4xx_hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(48.MHz()).freeze();
    let mut delay = dp.TIM5.delay_us(&clocks);

    loop {
        led.set_high();
        delay.delay_ms(1000_u32);
        led.set_low();
        delay.delay_us(1_000_000_u32);
    }
}
