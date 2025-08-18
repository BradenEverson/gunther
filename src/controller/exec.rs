//! Execution method for all opcodes

use std::time::Duration;

use pca9685_rppal::Pca9685;

use super::{Controller, opcode::OpCode};

/// Minimum pulse length
const SERVO_MIN: f32 = 500.0;
/// Maximum pulse length
const SERVO_MAX: f32 = 2500.0;

fn map_angle_to_pulse(angle: f32) -> f32 {
    let input_min = 0.0;
    let input_max = 180.0;
    SERVO_MIN + (angle - input_min) * (SERVO_MAX - SERVO_MIN) / (input_max - input_min)
}

fn move_servo(pca: &mut Pca9685, idx: u8, angle: f32) -> rppal::i2c::Result<()> {
    let len = map_angle_to_pulse(angle);
    pca.set_pwm(idx, 0, len as u16)?;

    Ok(())
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
                // [ 4 bytes (f32) ]
                let steps = f32::from_be_bytes([
                    self.payload[0],
                    self.payload[1],
                    self.payload[2],
                    self.payload[3],
                ]);

                move_servo(&mut self.servos, 3, steps).expect("Move servo");
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
