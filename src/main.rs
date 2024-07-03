//! # Pi Pico Temperature Exhibit
//!
//! Measures the temperature using an asair dht22 sensor and displays it on a 1602 16x2 LCD Display
//!
//! The display is connected to the I2C bus on GPIO 4 and GPIO 5 (the default i2c pins on pi).
//!
//! The temperature sensor is connected to GPIO 0.
//!
//! Ignore humidity data.
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
    fugit::RateExtU32,
    gpio::{
        bank0::Gpio4, bank0::Gpio5, FunctionI2C, FunctionSioOutput, InOutPin, Pin, Pins, PullUp,
    },
    sio::Sio,
    timer::Timer,
    watchdog::Watchdog,
    I2C,
};

use embedded_hal::blocking::delay::DelayMs;

use dht_sensor::{dht22, DhtReading};

use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};

use core::fmt::Write;
use heapless::String; // v0.7.11 // For the write! macro

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

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Set up the pins
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let sda_pin: Pin<Gpio4, FunctionI2C, PullUp> = pins.gpio4.into_pull_up_input().into_function();
    let scl_pin: Pin<Gpio5, FunctionI2C, PullUp> = pins.gpio5.into_pull_up_input().into_function();

    let mut gpio0: InOutPin<_> = InOutPin::new(pins.gpio0.into_function::<FunctionSioOutput>());

    // Create I2C driver
    let i2c = I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400u32.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    // Initialize LCD with cloned timer
    let mut timer_clone_for_lcd = timer.clone();
    let mut lcd = HD44780::new_i2c(i2c, 0x3F, &mut timer_clone_for_lcd).unwrap();

    // Initialize the LCD
    lcd.reset(&mut timer.clone()).unwrap();

    // Set display mode
    lcd.set_display_mode(
        DisplayMode {
            display: Display::On,
            cursor_visibility: Cursor::Invisible,
            cursor_blink: CursorBlink::Off,
        },
        &mut timer.clone(),
    )
    .unwrap();

    // Clear the display
    lcd.clear(&mut timer.clone()).unwrap();

    let mut last_temp_fahrenheit: Option<f32> = None;

    loop {
        match dht22::Reading::read(&mut timer.clone(), &mut gpio0) {
            Ok(dht22::Reading {
                temperature,
                relative_humidity: _,
            }) => {
                let temp_celsius = temperature as f32;
                let temp_fahrenheit = temp_celsius * 9.0 / 5.0 + 32.0;

                if last_temp_fahrenheit.map_or(true, |last_temp| {
                    let diff = temp_fahrenheit - last_temp;
                    // Manually calculate the absolute value of the difference
                    let abs_diff = if diff < 0.0 { -diff } else { diff };
                    abs_diff > 0.1
                }) {
                    let mut temp_str: String<32> = String::new();
                    write!(temp_str, "Temp: {:.1}F", temp_fahrenheit).unwrap();

                    // Display the temperature on the LCD
                    lcd.clear(&mut timer.clone()).unwrap();
                    lcd.write_str(&temp_str, &mut timer.clone()).unwrap();

                    // Update the last displayed temperature
                    last_temp_fahrenheit = Some(temp_fahrenheit);
                }
            }
            Err(_) => {
                continue;
            }
        }
        timer.clone().delay_ms(2100);
    }
}
