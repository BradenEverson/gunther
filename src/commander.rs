//! A command sender for the turret

pub mod cv;
pub mod op;

use std::time::Duration;

use libc::c_void;
use op::Op;

/// The command sender's state
pub struct Commander {
    /// Write end of the communication pipe
    write_fid: i32,
    /// Current known position of person
    shooting: bool,
    /// If we don't see the person, how many frames has it been like this?
    frames_without_seen: u32,
}

fn write(fid: i32, msg: &[u8]) {
    unsafe { libc::write(fid, msg.as_ptr() as *const c_void, msg.len()) };
}

impl Commander {
    /// Creates a new commander state
    pub fn new(write_fid: i32) -> Self {
        Self {
            write_fid,
            shooting: false,
            frames_without_seen: 0,
        }
    }

    /// Start shooting if not already
    pub fn stop_shoot(&mut self) {
        if self.shooting {
            self.shooting = false;
            self.send(&[Op::StopShoot]);
        }
    }

    /// Start shooting if not already
    pub fn shoot(&mut self) {
        if !self.shooting {
            self.shooting = true;
            self.send(&[Op::StartShoot]);
            std::thread::sleep(Duration::from_millis(500));
        }
    }

    /// Send a sequence of commands
    pub fn send(&self, cmds: &[Op]) {
        let mut buf = vec![];
        for cmd in cmds {
            cmd.write_to_buf(&mut buf);
        }

        write(self.write_fid, &buf);
    }
}
