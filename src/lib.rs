#![no_std]
//! A scrolling game on a 16x2 LCD screen.

use embedded_hal::{
    blocking::delay::{DelayMs, DelayUs},
    digital::v2::OutputPin,
    serial::Write,
};
use hd44780_driver::{
    bus::{DataBus, FourBitBus},
    error::Error,
    HD44780,
};

pub mod direction;
pub mod game;

/// Setups the LCD and returns the created struct.
///
/// This funtion will clear the screen and print "Hello, world!" on the LCD
/// screen intially. Cursor visibility and blinking would also be turned off.
///
/// The LCD would be in 4-bit mode with no RW pin (only write) data.
pub fn setup_lcd<
    D: DelayUs<u16> + DelayMs<u8>,
    RS: OutputPin,
    EN: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
>(
    rs: RS,
    en: EN,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
    delay: &mut D,
) -> HD44780<FourBitBus<RS, EN, D4, D5, D6, D7>> {
    // Creating instance
    let mut lcd = HD44780::new_4bit(rs, en, d4, d5, d6, d7, delay).unwrap();

    // Unshift display and set cursor to 0
    lcd.reset(delay).unwrap();

    // Clear existing characters
    lcd.clear(delay).unwrap();

    // Display the following string
    lcd.write_str("Hello, world!", delay).unwrap();

    // Making cursor invisible and disable blinking
    lcd.set_cursor_visibility(hd44780_driver::Cursor::Invisible, delay)
        .unwrap();
    lcd.set_cursor_blink(hd44780_driver::CursorBlink::Off, delay)
        .unwrap();

    lcd
}

/// Prints "Hello, world!" onto the given LCD.
///
/// This function will clear the LCD before printing.
///
/// Any errors when clearing and printing would be returned.
/// If the printing was successful, an Ok(69) would be returned.
pub fn print_hello<T: DataBus, D: DelayUs<u16> + DelayMs<u8>>(
    lcd: &mut HD44780<T>,
    delay: &mut D,
) -> Result<u8, Error> {
    lcd.clear(delay)?;
    lcd.write_str("Hello, world!", delay)?;
    Ok(69_u8)
}

/// Prints "Hello, world!" into the given serial port.
pub fn print_hello_serial<T: Write<u8>>(serial: &mut T) {
    let str = "Hello, world!";
    for c in str.as_bytes() {
        match serial.write(*c) {
            Ok(_) => (),
            Err(_) => (),
        };
    }
}

/// Takes ownership of D13 and sets it to high.
pub fn takes_ownership<P: OutputPin>(mut d13: P) {
    match d13.set_high() {
        Ok(_) => (),
        Err(_) => panic!("Failed to set pin to high."),
    }
}
