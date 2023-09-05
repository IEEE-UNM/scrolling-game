#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use stm32f4xx_hal::adc::{
    config::{AdcConfig, SampleTime},
    Adc, Temperature,
};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::{block, pac};

use rand::rngs::SmallRng;
use rand::SeedableRng;

use scrolling_game::direction::Direction;
use scrolling_game::game::ScrollingGame;
use scrolling_game::setup_lcd;

#[cortex_m_rt::entry]
fn main() -> ! {
    // Main Function
    // Setting Up Peripherals
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    // GPIO
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();

    let mut delay = dp.TIM5.delay::<8_000_000>(&clocks);
    let mut counter = dp.TIM1.counter_ms(&clocks);
    counter.start(1000.millis()).unwrap();
    let pins = (gpioa.pa2, gpioa.pa3);
    let mut serial = dp.USART2.serial(pins, 9600.bps(), &clocks).unwrap();
    // Setup ADC
    let adc_config = AdcConfig::default();
    let mut adc = Adc::adc1(dp.ADC1, true, adc_config);

    // LCD
    let mut lcd = setup_lcd(
        gpioa.pa9.into_push_pull_output(),
        gpioc.pc0.into_push_pull_output(),
        gpiob.pb5.into_push_pull_output(),
        gpiob.pb4.into_push_pull_output(),
        gpiob.pb10.into_push_pull_output(),
        gpioa.pa8.into_push_pull_output(),
        &mut delay,
    );
    lcd.clear(&mut delay).unwrap();

    // RNG
    // https://stackoverflow.com/questions/67627335/how-do-i-use-the-rand-crate-without-the-standard-library
    let seed = adc.convert(&Temperature, SampleTime::Cycles_480) as u64;
    let mut rng = SmallRng::seed_from_u64(seed);

    // Game
    let mut game = ScrollingGame::new();

    // Prints the intro
    // Absolute path from external crate
    scrolling_game::printer::print_intro(&mut serial);

    loop {
        if game.lost() {
            game.print(&mut lcd, &mut delay);
            let input = block!(serial.read()).unwrap_or_default();
            if input == 82 || input == 114 {
                game.reset()
            }
        } else {
            game.move_player(Direction::from_serial(&mut serial));
            game.tick(&mut lcd, &mut delay, &mut rng, counter.now().ticks());
        }
    }
    // Main Function End
}
