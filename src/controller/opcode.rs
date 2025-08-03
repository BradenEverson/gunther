//! OpCode definitions

/// Flat opcode
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    /// Do nothing, default OpCode
    NoOp = 0x00,
    /// Move stepper left
    Left = 0x01,
    /// Move stepper right
    Right = 0x02,
    /// Move servo up
    Up = 0x03,
    /// Move servo down
    Down = 0x04,
    /// Start shooting
    StartShoot = 0x05,
    /// Stop shooting
    EndShoot = 0x06,
}

/// Parsing an op failed because the opcode doesn't exist
pub struct UnknownOp;

impl TryFrom<u8> for OpCode {
    type Error = UnknownOp;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::NoOp),
            0x01 => Ok(Self::Left),
            0x02 => Ok(Self::Right),
            0x03 => Ok(Self::Up),
            0x04 => Ok(Self::Down),
            0x05 => Ok(Self::StartShoot),
            0x06 => Ok(Self::EndShoot),
            _ => Err(UnknownOp),
        }
    }
}
