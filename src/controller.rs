//! A control interpreter for the turret

use libc::{c_int, c_void};

/// How long the header cmd is
const HEADER_LEN: usize = 4;

/// The controller state
pub struct Controller {
    /// The PID
    read_pid: c_int,
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
        Self { read_pid }
    }

    pub fn controller_process(&self) {
        loop {
            let mut header_buf = [0u8; HEADER_LEN];
            read_exact(self.read_pid, &mut header_buf);

            println!("Header received {header_buf:X?}");
        }
    }
}
