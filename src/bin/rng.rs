#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use rand::SeedableRng;
use rand::{rngs::SmallRng, Rng};

use stm32f4xx_hal::{
    adc::{
        config::{AdcConfig, SampleTime},
        Adc, Temperature,
    },
    pac,
    prelude::*,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Setup ADC
    let adc_config = AdcConfig::default();
    let mut adc = Adc::adc1(dp.ADC1, true, adc_config);

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(48.MHz()).freeze();
    let mut delay = dp.TIM5.delay_us(&clocks);

    let seed = adc.convert(&Temperature, SampleTime::Cycles_480);
    let mut rng = SmallRng::seed_from_u64(seed as u64);

    loop {
        let number: u32 = rng.gen();
        defmt::println!("RNG: {}", number);
        delay.delay_ms(500_u32);
    }
}
