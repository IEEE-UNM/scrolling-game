#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

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

    // Setup Analog Pin
    let gpioa = dp.GPIOA.split();
    let pin = gpioa.pa0.into_analog();

    // Setup ADC
    let adc_config = AdcConfig::default();
    let mut adc = Adc::adc1(dp.ADC1, true, adc_config);

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(48.MHz()).freeze();
    let mut delay = dp.TIM5.delay_us(&clocks);

    loop {
        let value = adc.convert(&pin, SampleTime::Cycles_480);
        let voltage = adc.sample_to_millivolts(value);
        defmt::println!("Value: {}, {}mV", value, voltage);
        let temp = adc.convert(&Temperature, SampleTime::Cycles_480);
        defmt::println!("Temperature: {}", temp);
        delay.delay_ms(500_u32);
    }
}
