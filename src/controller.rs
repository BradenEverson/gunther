//! A control interpreter for the turret

use libc::{c_int, c_void};
use opcode::OpCode;

pub mod opcode;

/// How long the header cmd is
const HEADER_LEN: usize = 4;

const MAX_PAYLOAD: usize = 4;

/// All states a controller may have
#[derive(Debug, Clone, Copy)]
pub enum ControllerState {
    /// Awaiting the 4 byte header
    AwaitingHeader,
    /// Awaiting the specific payload
    AwaitingPayload,
    /// Executing the command received
    Executing,
}

/// The controller state
#[derive(Debug, Clone, Copy)]
pub struct Controller {
    /// The PID
    read_pid: c_int,
    /// Current controller state
    state: ControllerState,
    /// OpCode
    opcode: OpCode,
    /// Payload
    payload: [u8; MAX_PAYLOAD],
}

pub fn read(pid: c_int, buf: &mut [u8]) -> usize {
    unsafe { libc::read(pid, buf.as_mut_ptr() as *mut c_void, buf.len()) as usize }
}

pub fn read_exact(pid: c_int, buf: &mut [u8]) {
    let mut curr = 0;

    while curr < buf.len() {
        curr += read(pid, &mut buf[curr..])
    }
}

impl Controller {
    pub fn new(read_pid: c_int) -> Self {
        Self {
            read_pid,
            state: ControllerState::AwaitingHeader,
            opcode: OpCode::NoOp,
            payload: [0; MAX_PAYLOAD],
        }
    }

    pub fn controller_step(&mut self) {
        match self.state {
            ControllerState::AwaitingHeader => {
                let mut header_buf = [0u8; HEADER_LEN];
                read_exact(self.read_pid, &mut header_buf);

                println!("Header received {header_buf:X?}");
                self.state = ControllerState::AwaitingPayload;
            }
            _ => todo!("Implement {:?} state", self.state),
        }
    }
}
