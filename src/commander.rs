//! A command sender for the turret

pub mod cv;
pub mod op;

use cv::pose::KeyPoint;
use libc::c_void;
use op::Op;

/// The command sender's state
pub struct Commander {
    /// Write end of the communication pipe
    write_fid: i32,
    /// Current known position of person
    position: KeyPoint,
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
            // Start by looking in middle position and to the left
            position: KeyPoint { x: 1.0, y: 0.5 },
            frames_without_seen: 0,
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
