//! A control interpreter for the turret

use libc::{c_int, c_void};
use opcode::OpCode;

pub mod opcode;

/// How long the header cmd is
const HEADER_LEN: usize = 3;

/// How large the largest opcode payload could possibly be
const MAX_PAYLOAD: usize = 4;

/// Magic number that starts all payloads
const MAGIC_NUMBER: u8 = 0x72;

/// All states a controller may have
#[derive(Debug, Clone, Copy)]
pub enum ControllerState {
    /// Awaiting the 1 byte magic number
    AwaitingMagic,
    /// Awaiting the 3 byte header
    AwaitingHeader,
    /// Awaiting the specific payload
    AwaitingPayload,
    /// Executing the command received
    Executing,
}

/// The controller state
pub struct Controller {
    /// The PID
    read_pid: c_int,
    /// Current controller state
    state: ControllerState,
    /// OpCode
    opcode: OpCode,
    /// Payload len
    len: u16,
    /// Payload
    payload: [u8; MAX_PAYLOAD],
}

/// Read into a buffer
pub fn read(pid: c_int, buf: &mut [u8]) -> usize {
    unsafe { libc::read(pid, buf.as_mut_ptr() as *mut c_void, buf.len()) as usize }
}

/// Block and wait until we read an exact number of bytes
pub fn read_exact(pid: c_int, buf: &mut [u8], len: usize) {
    let mut curr = 0;

    while curr < len {
        curr += read(pid, &mut buf[curr..])
    }
}

impl Controller {
    /// Creates a new controller
    pub fn new(read_pid: c_int) -> Self {
        Self {
            read_pid,
            state: ControllerState::AwaitingMagic,
            opcode: OpCode::NoOp,
            len: 0,
            payload: [0; MAX_PAYLOAD],
        }
    }

    /// Steps once through the controller state
    pub fn controller_step(&mut self) {
        match self.state {
            ControllerState::AwaitingMagic => {
                let mut header_buf = [0u8];
                read_exact(self.read_pid, &mut header_buf, 1);

                if header_buf[0] == MAGIC_NUMBER {
                    self.state = ControllerState::AwaitingHeader
                }
            }
            ControllerState::AwaitingHeader => {
                let mut header_buf = [0u8; HEADER_LEN];
                read_exact(self.read_pid, &mut header_buf, HEADER_LEN);

                if let Ok(op) = OpCode::try_from(header_buf[0]) {
                    self.opcode = op;
                } else {
                    self.state = ControllerState::AwaitingMagic;
                    return;
                }

                self.len = u16::from_be_bytes([header_buf[1], header_buf[2]]);

                println!("Header received {header_buf:X?}");
                self.state = ControllerState::AwaitingPayload;
            }
            ControllerState::AwaitingPayload => {
                read_exact(self.read_pid, &mut self.payload, self.len as usize);
                self.state = ControllerState::Executing;
            }
            ControllerState::Executing => {
                self.exec();
                self.reset();
            }
        }
    }

    /// Execute once a payload and opcode are parsed
    pub fn exec(&mut self) {
        println!("Executing {:?}", self.opcode);
        match self.opcode {
            OpCode::NoOp => {}
            op => todo!("Implement {op:?}"),
        }
    }

    /// Reset all state
    pub fn reset(&mut self) {
        self.len = 0;
        self.opcode = OpCode::NoOp;
        self.state = ControllerState::AwaitingMagic;
    }
}
