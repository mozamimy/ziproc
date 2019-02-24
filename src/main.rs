use libc;
use std;
use std::ffi::CString;

fn main() {
    let pid;
    unsafe {
        pid = libc::fork();
    }

    if pid == 0 {
        let command = CString::new("/usr/bin/env").unwrap();
        let env1 = CString::new("ZIPROC1=foo").unwrap();
        let env2 = CString::new("ZIPROC2=bar").unwrap();
        let status;
        unsafe {
            status = libc::execve(
                command.as_ptr(),
                vec![command.as_ptr(), std::ptr::null()].as_ptr(),
                vec![env1.as_ptr(), env2.as_ptr(), std::ptr::null()].as_ptr(),
            )
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
