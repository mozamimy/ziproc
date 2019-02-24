use libc;
use std;
use std::ffi::CString;

fn main() {
    let pid;
    unsafe {
        pid = libc::fork();
    }

    if pid == 0 {
        let command = CString::new("/usr/bin/echo").unwrap();
        let arg = CString::new("Hello world.").unwrap();
        let status;
        unsafe {
            status = libc::execv(command.as_ptr(), vec![command.as_ptr(), arg.as_ptr(), std::ptr::null()].as_ptr())
        }
        println!("s: {:?}", status)
    } else {
        let status = 0 as *mut i32;
        unsafe {
            libc::waitpid(pid, status, 0);
        }
        println!("status: {:?}", status);
    }
}
