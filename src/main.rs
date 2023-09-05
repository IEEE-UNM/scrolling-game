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
    timer::{Channel1, Channel2},
    block
};

use rand::SeedableRng;
use rand::{rngs::SmallRng, Rng};

#[cortex_m_rt::entry]
fn main() -> ! {
    // Setting Up Peripherals
    let dp = pac::Peripherals::take().unwrap();

    loop {
    }
}
