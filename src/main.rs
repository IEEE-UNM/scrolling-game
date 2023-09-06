#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use stm32l4xx_hal::{
    adc::{Adc, AdcCommon},
    delay::Delay,
    hal::timer::CountDown,
    pac,
    prelude::*,
    serial::Serial,
    timer::Timer,
};

use nb::block;

use rand::rngs::SmallRng;
use rand::SeedableRng;

use scrolling_game::direction::Direction;
use scrolling_game::game::ScrollingGame;
use scrolling_game::setup_lcd;

#[cortex_m_rt::entry]
fn main() -> ! {
    // Main Function
    // Setting Up Peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);
    // GPIO
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);

    let mut delay = Delay::new(cp.SYST, clocks);
    let mut counter = Timer::tim2(dp.TIM2, 2.Hz(), clocks, &mut rcc.apb1r1);

    // Serial
    let tx = gpioa
        .pa2
        .into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let rx = gpioa
        .pa3
        .into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let mut serial = Serial::usart2(dp.USART2, (tx, rx), 9_600.bps(), clocks, &mut rcc.apb1r1);

    // Setup ADC
    let adc_common = AdcCommon::new(dp.ADC_COMMON, &mut rcc.ahb2);
    let mut adc = Adc::adc1(dp.ADC1, adc_common.clone(), &mut rcc.ccipr, &mut delay);
    let mut temperature = adc.enable_temperature(&mut delay).unwrap();

    // LCD
    let mut lcd = setup_lcd(
        gpioa
            .pa9
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper),
        gpioc
            .pc0
            .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper),
        gpiob
            .pb5
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper),
        gpiob
            .pb4
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper),
        gpiob
            .pb10
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper),
        gpioa
            .pa8
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper),
        &mut delay,
    );
    lcd.clear(&mut delay).unwrap();

    // RNG
    // https://stackoverflow.com/questions/67627335/how-do-i-use-the-rand-crate-without-the-standard-library
    // let seed = adc.convert(&Temperature, SampleTime::Cycles_480) as u64;
    let seed = adc.read(&mut temperature).unwrap() as u64;
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
            if let Ok(_) = counter.wait() {
                game.tick(&mut lcd, &mut delay, &mut rng, 0);
            }
        }
    }
    // Main Function End
}
