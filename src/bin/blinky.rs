#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use stm32l4xx_hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32l4xx_hal::pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut led = gpioa
        .pa5
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze(&mut flash.acr, &mut pwr);
    let mut delay = stm32l4xx_hal::delay::Delay::new(cp.SYST, clocks);

    loop {
        led.set_high();
        delay.delay_ms(1000_u32);
        led.set_low();
        delay.delay_us(1_000_000_u32);
    }
}
