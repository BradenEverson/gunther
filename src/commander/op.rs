//! A non-flat Operation representation

use crate::controller::MAGIC_NUMBER;

/// A turret command not represented by binary for ease of use :)
#[derive(Debug, Clone, Copy)]
pub enum Op {
    /// Move left `steps` with a certain `delay` per step
    Left(u16, u8),
    /// Move left `steps` with a certain `delay` per step
    Right(u16, u8),
    /// Set the y-axis stepper's exact angle
    SetStepperAngle(u16),
    /// Start shooting
    StartShoot,
    /// Stop shooting
    StopShoot,
}

impl Op {
    /// Gets an opcode from an op
    pub fn opcode(&self) -> u8 {
        match self {
            Self::Left(_, _) => 0x01,
            Self::Right(_, _) => 0x02,

            Self::SetStepperAngle(_) => 0x04,
            Self::StartShoot => 0x05,
            Self::StopShoot => 0x06,
        }
    }
    /// Write an opcode as binary into a buffer
    pub fn write_to_buf(&self, buf: &mut Vec<u8>) {
        match self {
            Self::Left(steps, delay_ms) | Self::Right(steps, delay_ms) => {
                let steps = steps.to_be_bytes();
                let payload = &[
                    MAGIC_NUMBER,
                    self.opcode(),
                    0x00,
                    0x03,
                    steps[0],
                    steps[1],
                    *delay_ms,
                ];

                buf.extend(payload);
            }

            Self::StartShoot | Self::StopShoot => {
                let payload = &[MAGIC_NUMBER, self.opcode(), 0x00, 0x00];
                buf.extend(payload);
            }

            Self::SetStepperAngle(angle) => {
                let angle = angle.to_be_bytes();
                let payload = &[MAGIC_NUMBER, self.opcode(), 0x00, 0x02, angle[0], angle[1]];
                buf.extend(payload);
            }
        }
    }
}
