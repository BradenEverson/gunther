//! OpCode definitions

/// All opcodes
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    NoOp = 0x00,
}

/// Parsing an op failed because the opcode doesn't exist
pub struct UnknownOp;

impl TryFrom<u8> for OpCode {
    type Error = UnknownOp;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::NoOp),
            _ => Err(UnknownOp),
        }
    }
}
