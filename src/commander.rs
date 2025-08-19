//! A command sender for the turret

pub mod op;

use libc::c_void;
use op::Op;

/// The command sender's state
pub struct Commander {
    /// Write end of the communication pipe
    write_fid: i32,
}

fn write(fid: i32, msg: &[u8]) {
    unsafe { libc::write(fid, msg.as_ptr() as *const c_void, msg.len()) };
}

impl Commander {
    /// Creates a new commander state
    pub fn new(write_fid: i32) -> Self {
        Self { write_fid }
    }

    /// Send a sequence of commands
    pub fn send(&self, cmds: &[Op]) {
        let mut buf = vec![];
        for cmd in cmds {
            cmd.write_to_buf(&mut buf);
        }

        write(self.write_fid, &buf);
    }

    /// Entire commander's process
    pub fn process(&self) {
        loop {}
    }
}
