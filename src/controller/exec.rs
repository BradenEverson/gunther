//! Execution method for all opcodes

use std::time::Duration;

use super::{Controller, opcode::OpCode};

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
            OpCode::Up => {
                // Move servo up
                // [ angle         ]
                // [ 4 bytes (f32) ]
            }
            OpCode::Down => {
                // Move servo down
                // [ angle         ]
                // [ 4 bytes (f32) ]
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
