import board
import time
import busio
import adafruit_ads1x15.ads1115 as ADS
from adafruit_ads1x15.analog_in import AnalogIn

# Initialize I2C and ADC
i2c = busio.I2C(board.SCL, board.SDA)
ads = ADS.ADS1115(i2c, address=0x48)

# Use channel A0
channel = AnalogIn(ads, ADS.P0)

Pot_Angle = 180 - (0.0178 * channel.voltage)

def read_potentiometer_voltage():
    return channel.voltage

