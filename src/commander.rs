//! A command sender for the turret

use std::time::Duration;

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
        let shoot: &[u8] = &[0x72, 0x05, 0x00, 0x00];
        let stop: &[u8] = &[0x72, 0x06, 0x00, 0x00];

        let angle = 90u16;
        let payload = angle.to_be_bytes();

        let angle: &[u8] = &[0x72, 0x04, 0x00, 0x02, payload[0], payload[1]];

        write(self.write_fid, angle);

        loop {
            // write(self.write_fid, shoot);
            // std::thread::sleep(Duration::from_millis(5000));
            // write(self.write_fid, stop);
            // std::thread::sleep(Duration::from_millis(5000));
        }
    }
}
