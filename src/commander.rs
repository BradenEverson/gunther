//! A command sender for the turret

use libc::c_void;

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

    /// Entire commander's process
    pub fn process(&self) {
        let msg: &[u8] = &[0x72, 0x02, 0x00, 0x02, 0x03, 0xE8, 1];
        write(self.write_fid, msg);

        loop {}
    }
}
