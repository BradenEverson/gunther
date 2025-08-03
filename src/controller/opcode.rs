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
            _ => Err(UnknownOp),
        }
    }
}

/// Opcode with contextualized Paylod
pub enum Op {
    /// Do nothing, default OpCode
    NoOp,
    /// Move stepper left
    /// [steps, delay ms]
    Left(u8, u8),
    /// Move stepper right
    /// [steps, delay ms]
    Right(u8, u8),
    /// Move servo up
    /// [ angle ]
    Up(f32),
    /// Move servo down
    /// [ angle ]
    Down(f32),
    /// Start shooting
    StartShoot,
    /// Stop shooting
    EndShoot,
}

impl TryFrom<(OpCode, &[u8])> for Op {
    type Error = UnknownOp;

    fn try_from(value: (OpCode, &[u8])) -> Result<Self, Self::Error> {
        let (op, _payload) = value;

        match op {
            OpCode::NoOp => Ok(Self::NoOp),
            _ => todo!("Parse op"),
        }
    }
}
