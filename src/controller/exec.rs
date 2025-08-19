//! Execution method for all opcodes

use std::time::Duration;

use pca9685_rppal::Pca9685;

use super::{Controller, opcode::OpCode};

const MIN_PULSE_US: f32 = 500.0;
const MAX_PULSE_US: f32 = 2500.0;
const ACTUATION_RANGE: f32 = 180.0;

fn angle_to_ticks(angle: f32) -> u16 {
    let pulse_us = MIN_PULSE_US + (angle / ACTUATION_RANGE) * (MAX_PULSE_US - MIN_PULSE_US);

    let ticks = (pulse_us * 4096.0) / 20_000.0;

    ticks.round() as u16
}

fn set_servo_angle(pca: &mut Pca9685, channel: u8, angle: f32) {
    let off_tick = angle_to_ticks(angle);
    pca.set_pwm(channel, 0, off_tick)
        .expect("Failed to set PWM");
}

impl Controller {
    /// Execute once a payload and opcode are parsed
    pub fn exec(&mut self) {
        match self.opcode {
            OpCode::NoOp => {
                // Do nothing, default OpCode
                // [ no payload ]
            }
            OpCode::Left => {
                // Move stepper left
                // [  steps  ] [ delay ms ]
                // [ 2 bytes ] [  1 byte  ]
                self.dir.set_low();

                let steps = u16::from_be_bytes([self.payload[0], self.payload[1]]);
                let delay_ms = Duration::from_millis(self.payload[2] as u64);

                for _ in 0..steps {
                    self.step.set_high();
                    std::thread::sleep(delay_ms);

                    self.step.set_low();
                    std::thread::sleep(delay_ms);
                }
            }
            OpCode::Right => {
                // Move stepper right
                // [  steps  ] [ delay ms ]
                // [ 2 bytes ] [  1 byte  ]
                self.dir.set_high();

                let steps = u16::from_be_bytes([self.payload[0], self.payload[1]]);
                let delay_ms = Duration::from_millis(self.payload[2] as u64);

                for _ in 0..steps {
                    self.step.set_high();
                    std::thread::sleep(delay_ms);

                    self.step.set_low();
                    std::thread::sleep(delay_ms);
                }
            }

            OpCode::SetStepperAngle => {
                // Move Y-Axis
                // [ angle         ]
                // [ 2 bytes (u16) ]
                let angle = u16::from_be_bytes([self.payload[0], self.payload[1]]);

                set_servo_angle(&mut self.servos, 3, angle as f32);
            }
            OpCode::StartShoot => {
                // Start shooting
                // [ no payload ]
                self.rev.set_high();
                std::thread::sleep(Duration::from_millis(500));
                self.trigger.set_high();
            }
            OpCode::EndShoot => {
                // Stop shooting
                // [ no payload ]
                self.rev.set_low();
                self.trigger.set_low();
            }
        }
    }
}
