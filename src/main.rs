#![no_std]
#![no_main]

use arduino_hal::prelude::_embedded_hal_serial_Read;
use arduino_hal::Adc;
use arduino_hal::{hal::usart::BaudrateArduinoExt, Delay, Usart};

use panic_halt as _;

use rand::rngs::SmallRng;
use rand::SeedableRng;

use scrolling_game::direction::Direction;
use scrolling_game::takes_ownership;
use scrolling_game::setup_lcd;
use scrolling_game::game::ScrollingGame;

#[arduino_hal::entry]
fn main() -> ! {
    // Main Function
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */
    // Peripherals
    let mut delay = Delay::new();
    let mut serial = Usart::new(
        dp.USART0,
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
