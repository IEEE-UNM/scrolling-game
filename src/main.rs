#![no_std]
#![no_main]

use panic_halt as _;

use stm32f4xx_hal::pac;
use stm32f4xx_hal::prelude::*;

use rand::rngs::SmallRng;
use rand::SeedableRng;

use scrolling_game::direction::Direction;
use scrolling_game::game::ScrollingGame;
use scrolling_game::setup_lcd;
use scrolling_game::takes_ownership;

#[cortex_m_rt::entry]
fn main() -> ! {
    // Main Function
    // Setting Up Peripherals
    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let pins = (dp.GPIOA.split(), dp.GPIOB.split());
    let clocks = rcc.cfgr.sysclk(16.MHz()).pclk1(8.MHz()).freeze();

    let mut delay = dp.TIM1.delay(&clocks);
    let mut serial = Usart::new(
        dp.USART1,
        pins.d0.into(),
        pins.d1.into_output(),
        57600_u32.into_baudrate(),
    );
    let mut adc = Adc::new(dp.ADC, Default::default());

    // Toggles D13
    let d13 = pins.d13.into_output();
    let d13_2 = d13;
    // Does not work
    // d13.toggle();
    takes_ownership(d13_2);
    // Does not work
    // d13_2.toggle();

    // LCD
    let mut lcd = setup_lcd(
        pins.d8.into_output(),
        pins.d9.into_output(),
        pins.d4.into_output(),
        pins.d5.into_output(),
        pins.d6.into_output(),
        pins.d7.into_output(),
        &mut delay,
    );
    lcd.clear(&mut delay).unwrap();

    // RNG
    // https://stackoverflow.com/questions/67627335/how-do-i-use-the-rand-crate-without-the-standard-library
    let a0 = pins.a0.into_analog_input(&mut adc);
    let seed = a0.analog_read(&mut adc) as u64;
    let mut rng = SmallRng::seed_from_u64(seed);

    // Game
    let mut game = ScrollingGame::new();

    // Prints the intro
    // Absolute path from external crate
    scrolling_game::printer::print_intro(&mut serial);

    loop {
        if game.lost() {
            let input = serial.read().unwrap_or_default();
            if input == 82 || input == 114 {
                game.reset()
            }
        } else {
            game.move_player(Direction::from_serial(&mut serial));
            game.tick(&mut lcd, &mut delay, &mut rng);
        }
    }
    // Main Function End
}
