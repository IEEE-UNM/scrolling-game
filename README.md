# Scrolling Game

A scrolling game on an HD44780 16x2 LCD using an STM32 L476VG Discovery board written in Rust.

It makes use of Knurling tools by [Ferrous Systems](http://ferrous-systems.com/) to help with programming the MCU.

## Usage

### Setup

If you don't have them already, install [`flip-link`] and [`probe-run`]:

```bash
cargo install flip-link
cargo install probe-run
```

Clone this repository.

``` bash
git clone https://github.com/IEEE-UNM/scrolling-game.git
cd scrolling-game
```

### Scrolling Game

To run the scrolling game.

```bash
cargo run --bin scrolling-game
```

[`flip-link`]: https://github.com/knurling-rs/flip-link
[`probe-run`]: https://github.com/knurling-rs/probe-run

### Basic MCU Programes

This repository also includes basic peripheral operations on the MCU. They are:

- ADC (adc)
- LED blinking (blinky)
- Digital Input (digital-input)
- Rust Ownership Showcase (ownership)
- PWM (pwm)
- Generating Random Numbers (rng)
- USART (usart)

They can be ran using their respective binary name shown in brackets above. For example for ADC:

``` bash
cargo run --bin adc
```

## License

All code are licensed under GNU General Public License, Version 3.0 or later.

All Fritzing schematics and images are licensed under [Creatie Commons Attribution-ShareAlike 4.0 International](https://creativecommons.org/licenses/by-sa/4.0/) (CC BY-SA 4.0).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the GPL-3.0 or later license, shall
be dual licensed as above, without any additional terms or conditions.
