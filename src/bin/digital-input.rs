#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use stm32f4xx_hal::pac;
use stm32f4xx_hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let gpioc = dp.GPIOC.split();
    let button = gpioc.pc13.into_pull_down_input();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(48.MHz()).freeze();
    let mut delay = dp.TIM5.delay_us(&clocks);

    loop {
        let is_high = button.is_high();
        let is_low = button.is_low();
        defmt::println!("{}, {}", is_high, is_low);
        delay.delay_ms(100_u32);
    }
}
