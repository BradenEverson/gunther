//! OpCode definitions

/// All opcodes
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    NoOp = 0x0,
}
