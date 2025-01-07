from machine import I2C, Pin
import time

# MCP9808 I2C address
MCP9808_ADDR = 0x18

# LCD 1602 I2C address
LCD_ADDR = 0x3F

# MCP9808 register addresses
MCP9808_REG_AMBIENT_TEMP = 0x05

# Initialize I2C on Raspberry Pi Pico
# GP14 = SDA, GP15 = SCL
i2c = I2C(1, scl=Pin(15), sda=Pin(14), freq=100000)  # 100 kHz frequency

# Minimal LCD 1602 I2C driver with backlight control
class LCD:
    def __init__(self, i2c, address):
        self.i2c = i2c
        self.address = address
        self.backlight_state = True  # Backlight is on by default
        self.init_lcd()
    
    def init_lcd(self):
        self.write_command(0x33)  # Initialize
        self.write_command(0x32)  # Set to 4-bit mode
        self.write_command(0x28)  # 2 lines, 5x8 matrix
        self.write_command(0x0C)  # Display on, cursor off
        self.write_command(0x06)  # Increment cursor
        self.write_command(0x01)  # Clear display
        time.sleep_ms(2)
    
    def write_command(self, cmd):
        # Send command to LCD
        high_nibble = cmd & 0xF0
        low_nibble = (cmd << 4) & 0xF0
        self.i2c.writeto(self.address, bytearray([
            high_nibble | 0x04 | (0x08 if self.backlight_state else 0x00),
            high_nibble | (0x08 if self.backlight_state else 0x00)
        ]))
        self.i2c.writeto(self.address, bytearray([
            low_nibble | 0x04 | (0x08 if self.backlight_state else 0x00),
            low_nibble | (0x08 if self.backlight_state else 0x00)
        ]))
        time.sleep_ms(2)
    
    def write_data(self, data):
        # Send data to LCD
        high_nibble = data & 0xF0
        low_nibble = (data << 4) & 0xF0
        self.i2c.writeto(self.address, bytearray([
            high_nibble | 0x05 | (0x08 if self.backlight_state else 0x00),
            high_nibble | 0x01 | (0x08 if self.backlight_state else 0x00)
        ]))
        self.i2c.writeto(self.address, bytearray([
            low_nibble | 0x05 | (0x08 if self.backlight_state else 0x00),
            low_nibble | 0x01 | (0x08 if self.backlight_state else 0x00)
        ]))
        time.sleep_ms(2)
    
    def clear(self):
        self.write_command(0x01)  # Clear display
    
    def set_cursor(self, row, col):
        # Set cursor position
        addr = 0x80 + (row * 0x40) + col
        self.write_command(addr)
    
    def write(self, text):
        # Write text to LCD
        for char in text:
            self.write_data(ord(char))
    
    def backlight_on(self):
        # Turn on backlight
        self.backlight_state = True
        self.i2c.writeto(self.address, bytearray([0x08]))
    
    def backlight_off(self):
        # Turn off backlight
        self.backlight_state = False
        self.i2c.writeto(self.address, bytearray([0x00]))


# ----------------------------------------------------------------------
# Helper function: Update a single LCD line with minimal flicker
# Only overwrite differences, and if new text is shorter than old text,
# overwrite leftover characters with spaces.
# ----------------------------------------------------------------------
def update_line(lcd, row, new_text, old_text, line_length=16):
    """
    Update a single line on the LCD without clearing the entire row first.
    1. If new_text == old_text, do nothing (no flicker).
    2. Otherwise, set cursor, write new_text.
    3. If new_text is shorter than old_text, fill leftover with spaces.
    4. Return new_text to store for next iteration.
    """
    if new_text == old_text:
        return old_text
    
    lcd.set_cursor(row, 0)
    lcd.write(new_text)
    
    # If new text is shorter, overwrite leftover characters with spaces
    if len(new_text) < len(old_text):
        spaces_needed = len(old_text) - len(new_text)
        lcd.write(" " * spaces_needed)
    
    return new_text


# ----------------------------------------------------------------------
# Main Code
# ----------------------------------------------------------------------

# Initialize LCD
lcd = LCD(i2c, LCD_ADDR)
lcd.backlight_on()
lcd.clear()

def read_temperature():
    # Read 2 bytes from the temperature register
    data = i2c.readfrom_mem(MCP9808_ADDR, MCP9808_REG_AMBIENT_TEMP, 2)
    
    # Convert the data to temperature in Celsius
    upper_byte = data[0]
    lower_byte = data[1]
    
    # Extract temperature value (first 12 bits)
    temp_raw = (upper_byte << 8) | lower_byte
    temp_celsius = (temp_raw & 0x0FFF) / 16.0
    
    # Check if negative
    if upper_byte & 0x10:
        temp_celsius -= 256
    
    # Convert to Fahrenheit
    return (temp_celsius * 9 / 5) + 32

# 1) Smaller rolling average window for faster reaction
NUM_READINGS = 5

# 2) Small stability filter
# - We keep these thresholds low so it remains fairly responsive
TEMP_THRESHOLD   = 0.01   # ±0.05 °F must change to update displayed temperature
CHANGE_THRESHOLD = 0.025   # ±0.10% must change to update displayed % change
PERCENT_CLAMP    = 0.01  # If abs(% change) < 0.10%, display 0.00%

# 3) Faster but still smoothing factor
ALPHA = 0.6  # 0 < ALPHA < 1; higher means more weight on newest data

readings = []
smoothed_temp = None

last_displayed_temp = None
last_displayed_change = None

# Track what was written on each of the 2 LCD lines
old_lines = ["", ""]

while True:
    current_temp = read_temperature()
    
    # Rolling list for average
    readings.append(current_temp)
    if len(readings) > NUM_READINGS:
        readings.pop(0)
    
    # Rolling average
    avg_temp = sum(readings) / len(readings)
    
    # Exponential smoothing of the rolling average
    if smoothed_temp is None:
        smoothed_temp = avg_temp
    else:
        smoothed_temp = smoothed_temp * (1 - ALPHA) + avg_temp * ALPHA
    
    # Compute percent difference from the oldest reading in the window
    if len(readings) == NUM_READINGS:
        oldest_temp = readings[0]
        if oldest_temp != 0:
            temp_change_percent = ((smoothed_temp - oldest_temp) / oldest_temp) * 100
        else:
            temp_change_percent = 0
    else:
        temp_change_percent = 0  # Not enough readings yet
    
    # Clamp small % changes to zero
    if abs(temp_change_percent) < PERCENT_CLAMP:
        temp_change_percent = 0
    
    # Threshold-based update logic
    # Temperature
    if (last_displayed_temp is None or
        abs(smoothed_temp - last_displayed_temp) > TEMP_THRESHOLD):
        last_displayed_temp = smoothed_temp
    
    # Percentage
    if (last_displayed_change is None or
        abs(temp_change_percent - last_displayed_change) > CHANGE_THRESHOLD):
        last_displayed_change = temp_change_percent
    
    # Format lines
    line0_text = f"{last_displayed_temp:.2f} F"
    line1_text = f"{last_displayed_change:+.2f}%"
    
    # Update LCD line 0 & line 1 with minimal flicker
    old_lines[0] = update_line(lcd, 0, line0_text, old_lines[0], line_length=16)
    old_lines[1] = update_line(lcd, 1, line1_text, old_lines[1], line_length=16)
    
    # Print the same text to the console
    print(f"LCD Display: {line0_text}, {line1_text}")
    
    # Adjust update rate as needed (every 0.25 s = 4 updates/s)
    time.sleep(0.25)
