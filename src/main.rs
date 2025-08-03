//! Main system that creates two controller and commander processes that communicate via a UNIX
//! pipe

use gunther::controller::Controller;
use libc::{_exit, c_void, close, fork, pipe, write};

fn main() {
    let mut fds = [0; 2];

    unsafe {
        if pipe(fds.as_mut_ptr()) == -1 {
            panic!("Failed to create pipe");
        }
    }

    let (read_end, write_end) = (fds[0], fds[1]);

    let pid = unsafe { fork() };
    match pid {
        -1 => panic!("Fork failed"),
        0 => unsafe {
            close(read_end);
            let msg = [0x72, 0x02, 0x00, 0x02, 10, 1];
            write(write_end, msg.as_ptr() as *const c_void, msg.len());
            close(write_end);
            _exit(0);
        },
        _ => {
            let mut controller = Controller::new(read_end);

            loop {
                controller.controller_step();
            }
        }
    }
}
