//! Execution method for all opcodes

use std::time::Duration;

use pca9685_rppal::Pca9685;

use super::{Controller, opcode::OpCode};

fn move_servo(pca: &mut Pca9685, idx: u8, angle: u16) -> rppal::i2c::Result<()> {
    let pulse_len_in_ticks = angle_to_ticks(angle);
    pca.set_pwm(idx, 0, pulse_len_in_ticks)?;

    Ok(())
}

const PCA9685_FREQUENCY_HZ: f32 = 50.0;

const TICK_PERIOD_US: f32 = 1_000_000.0 / (PCA9685_FREQUENCY_HZ * 4096.0);

fn us_to_ticks(us: f32) -> u16 {
    (us / TICK_PERIOD_US) as u16
}

fn angle_to_ticks(angle: u16) -> u16 {
    const MIN_PULSE_US: f32 = 1000.0;
    const MAX_PULSE_US: f32 = 2000.0;

    let pulse_us = MIN_PULSE_US + (angle as f32 / 180.0) * (MAX_PULSE_US - MIN_PULSE_US);

    us_to_ticks(pulse_us)
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

            OpCode::YAxis => {
                // Move Y-Axis
                // [ angle         ]
                // [ 2 bytes (u16) ]
                let angle = u16::from_be_bytes([self.payload[0], self.payload[1]]);

                move_servo(&mut self.servos, 3, angle).expect("Move servo");
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
