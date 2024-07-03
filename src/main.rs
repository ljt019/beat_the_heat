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
    sio::Sio,
    watchdog::Watchdog,
    gpio::{ Pins, Pin, bank0::Gpio5, bank0::Gpio4, PullUp, InOutPin, FunctionSioOutput, FunctionI2C},
    fugit::RateExtU32,
    I2C,
    timer::Timer,
};

use embedded_hal::delay::DelayNs;

use dht_embedded::{Dht22, DhtSensor, NoopInterruptControl};

use hd44780_driver::{HD44780, DisplayMode, Cursor, CursorBlink, Display};

use heapless::String; // v0.7.11
use core::fmt::Write; // For the write! macro

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

    let mut shared_timer = SharedTimer::new(timer);

    // Set up the pins
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let sda_pin: Pin<Gpio4, FunctionI2C, PullUp> = pins.gpio4.into_pull_up_input().into_function();
    let scl_pin: Pin<Gpio5, FunctionI2C, PullUp> = pins.gpio5.into_pull_up_input().into_function();

    let gpio0: InOutPin<_> = InOutPin::new(pins.gpio0.into_function::<FunctionSioOutput>());

    // Create I2C driver
    let i2c = I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400u32.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    let mut sensor = Dht22::new(NoopInterruptControl, &mut shared_timer, gpio0);

    let mut lcd = HD44780::new_i2c(i2c, 0x3F, &mut shared_timer).unwrap();

    // Initialize the LCD
    lcd.reset(&mut shared_timer).unwrap();

    // Set display mode
    lcd.set_display_mode(DisplayMode {
        display: Display::On,
        cursor_visibility: Cursor::Invisible,
        cursor_blink: CursorBlink::Off,
    }, &mut shared_timer).unwrap();

    // Clear the display
    lcd.clear(&mut shared_timer).unwrap();

    loop {
       match sensor.read() {
        Ok(reading) => {
            let temp = reading.temperature();
            let mut temp_str: String<32> = String::new();
            write!(temp_str, "Temp: {:.1}C", temp).unwrap();
            lcd.clear(&mut shared_timer).unwrap();
            lcd.write_str(&temp_str, &mut shared_timer).unwrap();
        }
        Err(_) => {
            continue;
        }
       }
        shared_timer.delay_ms(2100);
    }
} 

struct SharedTimer {
    timer: Timer,
}

impl SharedTimer {
    fn new(timer: Timer) -> Self {
        SharedTimer { timer }
    }
}

impl DelayNs for SharedTimer {
    fn delay_ns(&mut self, ns: u32) {
        self.timer.delay_ns(ns)
    }
}
