use libc::{_exit, c_void, close, fork, pipe, read, write};
use std::ffi::CString;

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
            let msg = CString::new("suh").unwrap();
            write(
                write_end,
                msg.as_ptr() as *const c_void,
                msg.as_bytes().len(),
            );
            close(write_end);
            _exit(0);
        },
        pid => unsafe {
            println!("I'm the Parent with PID {pid}");
            close(write_end);
            let mut buf = [0u8; 1024];
            let bytes_read = read(read_end, buf.as_mut_ptr() as *mut c_void, buf.len());
            close(read_end);

            if bytes_read > 0 {
                println!(
                    "Parent received: {}",
                    String::from_utf8_lossy(&buf[..bytes_read as usize])
                );
            }
        },
    }
}
