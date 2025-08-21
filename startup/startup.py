import time
import potentiometer
from adafruit_servokit import ServoKit

kit = ServoKit(channels=16)

kit.servo[3].set_pulse_width_range(500, 2500)

voltage = Potentiometer_Turret_Code.read_potentiometer_voltage()
print(voltage)

if .2 < voltage < .610:
    angle = (170)
    kit.servo[3].angle = angle
    time.sleep(.1)
elif .610 < voltage < .857:
    angle = (160)
    kit.servo[3].angle = angle
    time.sleep(.1)
elif .857 < voltage < 1.085:
    angle = (150)
    kit.servo[3].angle = angle
    time.sleep(.1)
elif 1.085 < voltage < 1.317:
    angle = (140)
    kit.servo[3].angle = angle
    time.sleep(.1)
elif 1.317 < voltage < 1.570:
    angle = (130)
    kit.servo[3].angle = angle
    time.sleep(.1)
elif 1.570 < voltage < 2:
    angle = (120)
    kit.servo[3].angle = angle
    time.sleep(.1)
else:
    angle = (170)
    kit.servo[3].angle = angle
    time.sleep(.1)

initilization_angle = 140

while angle != initilization_angle:
    if angle < initilization_angle:
        angle += 1
    else:
        angle -= 1
    kit.servo[3].angle = angle
    time.sleep(.3)
