# Temperature Exhibit with Embedded Rust

This embedded Rust application is designed for a Raspberry Pi Pico. It reads data from a DHT22 temperature sensor and displays the current temperature along with the average temperature change over the last 60 readings on an HD44780 16x2 LCD. This project is part of an exhibit demonstrating how different materials can affect temperature readings.

## Features

- Reads temperature data from a DHT22 sensor.
- Displays the current temperature on an HD44780 16x2 LCD.
- Calculates and displays the average temperature change over the last 60 readings in percentage.
- Demonstrates the effect of different materials on temperature.

## Hardware Requirements

- Raspberry Pi Pico
- DHT22 temperature sensor
- HD44780 16x2 LCD
- Breadboard and jumper wires

## Setup

1. **Wiring:**

   - Connect the DHT22 sensor to the Raspberry Pi Pico.
   - Connect the HD44780 LCD to the Raspberry Pi Pico using the appropriate pins.

2. **Install Rust:**

   - Follow the instructions to install Rust from the official [Rust website](https://www.rust-lang.org/).

3. **Clone the Repository:**

   ```sh
   git clone <https://github.com/ljt019/temperature_exhibit.git>
   cd <temperature_exhibit>
   ```

4. **Build and Flash:**
   - Ensure you have the necessary Rust tools and target installed:
   ```sh
   rustup target add thumbv6m-none-eabi
   cargo install cargo-hf2
   ```
   - Build and flash the application:
   ```sh
   cargo build --release
   cargo hf2 --release --example <your-example>
   ```

## Usage

1. **Power on the Raspberry Pi Pico:**

   - The application will start automatically.

2. **Observe the LCD:**

   - The current temperature will be displayed on the first line.
   - The average temperature change over the last 60 readings will be displayed on the second line.

3. **Experiment with Materials:**
   - Place different insulating or conductive materials over the temperature sensor and observe how the readings change over time.

## Acknowledgements

- [Rust Embedded Working Group](https://github.com/rust-embedded/wg)

## Contact

For any questions or further information, please contact [Lucien](mailto:lthomas@sciport.org).
