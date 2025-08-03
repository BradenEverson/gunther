//! Execution method for all opcodes

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
                // [  steps ] [ delay ms ]
                // [ 1 byte ] [  1 byte  ]
            }
            OpCode::Right => {
                // Move stepper right
                // [  steps ] [ delay ms ]
                // [ 1 byte ] [  1 byte  ]
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
            }
            OpCode::EndShoot => {
                // Stop shooting
                // [ no payload ]
            }
        }
    }
}
