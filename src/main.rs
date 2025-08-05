//! Main system that creates two controller and commander processes that communicate via a UNIX
//! pipe

use gunther::{commander::Commander, controller::Controller};
use libc::{close, fork, pipe};

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
        0 => {
            unsafe { close(read_end) };
            let commander = Commander::new(write_end);
            commander.process();
        }
        _ => {
            unsafe { close(write_end) };
            let mut controller = Controller::new(read_end);

            loop {
                controller.step();
            }
        }
    }
}
