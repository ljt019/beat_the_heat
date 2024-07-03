//! # Pi Pico Temperature Exhibit
//!
//! Measures the temperature using an asair dht22 sensor and displays it on a 1602 16x2 LCD Display
//!
//! The display is connected to the I2C bus on GPIO 4 and GPIO 5 (the default i2c pins on pi).
//! 
//! The temperature sensor is not yet connected.
//! Testing out printing "Hello, World!" to the LCD display first.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// The macro for our start-up function
use rp_pico::entry;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Peripheral Access Crate, which provides low-level register access
use rp_pico::hal::pac;

// Raspberry Pi Pico crate imports for hardware access
use rp_pico::hal::{
    clocks::init_clocks_and_plls,
    sio::Sio,
    watchdog::Watchdog,
    I2C,
    gpio::{FunctionI2C, Pin, PullUp, bank0::Gpio4, bank0::Gpio5},
    timer::Timer,
};
use fugit::RateExtU32;
use embedded_hal::timer::CountDown;
use nb::block;

// LCD driver
use hd44780_driver::{HD44780, DisplayMode, Cursor, CursorBlink, Display};

use fugit::ExtU32;

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // Set up the clocks
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Set up the pins
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure pins for I2C with PullUp
    let sda_pin: Pin<Gpio4, FunctionI2C, PullUp> = pins.gpio4.into_pull_up_input().into_function();
    let scl_pin: Pin<Gpio5, FunctionI2C, PullUp> = pins.gpio5.into_pull_up_input().into_function();

    // Create I2C driver
    let i2c = I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400u32.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    let mut lcd = HD44780::new_i2c(i2c, 0x3F, &mut timer).unwrap();

    // Initialize the LCD
    lcd.reset(&mut timer).unwrap();

    // Set display mode
    lcd.set_display_mode(DisplayMode {
        display: Display::On,
        cursor_visibility: Cursor::Invisible,
        cursor_blink: CursorBlink::Off,
    }, &mut timer).unwrap();

    // Clear the display
    lcd.clear(&mut timer).unwrap();

    let delay: u32 = 1000_u32;

    loop {
        lcd.clear(&mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.write_str("How many booms", &mut timer).unwrap();
        lcd.set_cursor_pos(0x40, &mut timer).unwrap();
        lcd.write_str("on the meter?", &mut timer).unwrap();
                lcd.set_cursor_pos(0, &mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.clear(&mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.write_str("Boom!", &mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.clear(&mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.write_str("Boom!", &mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.clear(&mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.write_str("Boom!", &mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.clear(&mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.write_str("Boom!", &mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.clear(&mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.write_str("Booooooom!", &mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }
    }
}
#![no_std]
#![no_main]

// The macro for our start-up function
use rp_pico::entry;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Peripheral Access Crate, which provides low-level register access
use rp_pico::hal::pac;

// Raspberry Pi Pico crate imports for hardware access
use rp_pico::hal::{
    clocks::init_clocks_and_plls,
    sio::Sio,
    watchdog::Watchdog,
    I2C,
    gpio::{FunctionI2C, Pin, PullUp, bank0::Gpio4, bank0::Gpio5},
    timer::Timer,
};
use fugit::RateExtU32;
use embedded_hal::timer::CountDown;
use nb::block;

// LCD driver
use hd44780_driver::{HD44780, DisplayMode, Cursor, CursorBlink, Display};

use fugit::ExtU32;

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // Set up the clocks
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Set up the pins
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure pins for I2C with PullUp
    let sda_pin: Pin<Gpio4, FunctionI2C, PullUp> = pins.gpio4.into_pull_up_input().into_function();
    let scl_pin: Pin<Gpio5, FunctionI2C, PullUp> = pins.gpio5.into_pull_up_input().into_function();

    // Create I2C driver
    let i2c = I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400u32.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    let mut lcd = HD44780::new_i2c(i2c, 0x3F, &mut timer).unwrap();

    // Initialize the LCD
    lcd.reset(&mut timer).unwrap();

    // Set display mode
    lcd.set_display_mode(DisplayMode {
        display: Display::On,
        cursor_visibility: Cursor::Invisible,
        cursor_blink: CursorBlink::Off,
    }, &mut timer).unwrap();

    // Clear the display
    lcd.clear(&mut timer).unwrap();

    let delay: u32 = 1000_u32;

    loop {
        lcd.clear(&mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.write_str("How many booms", &mut timer).unwrap();
        lcd.set_cursor_pos(0x40, &mut timer).unwrap();
        lcd.write_str("on the meter?", &mut timer).unwrap();
                lcd.set_cursor_pos(0, &mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.clear(&mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.write_str("Boom!", &mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.clear(&mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.write_str("Boom!", &mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.clear(&mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.write_str("Boom!", &mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.clear(&mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.write_str("Boom!", &mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.clear(&mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }

        lcd.write_str("Booooooom!", &mut timer).unwrap();

        {
            let mut count_down = timer.count_down();
            count_down.start(delay.millis());
            let _ = block!(count_down.wait());
        }
    }
}
