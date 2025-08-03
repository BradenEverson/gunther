//! Execution method for all opcodes

use super::{Controller, opcode::Op};

impl Controller {
    /// Execute once a payload and opcode are parsed
    pub fn exec(&mut self) {
        if let Ok(op) = Op::try_from((self.opcode, self.payload.as_slice())) {
            match op {
                _ => todo!("Handle op"),
            }
        }
    }
}
