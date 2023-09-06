#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use rand::SeedableRng;
use rand::{rngs::SmallRng, Rng};

#[cortex_m_rt::entry]
fn main() -> ! {
    // Setting Up Peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f4xx_hal::Peripherals::take().unwrap();

    loop {}
}
