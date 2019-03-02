use libc;
use uuid;
use std;
use std::ffi::CString;

fn main() {
    let container_id = uuid::Uuid::new_v4();
    let pid;
    unsafe {
        pid = libc::fork();
    }

    if pid == 0 {
        let command = CString::new("/bin/cat").unwrap();
        let arg = CString::new("/etc/os-release").unwrap();
        let root_path = CString::new("/home/mozamimy/var/ziproc_images/ubuntu-18.04").unwrap();
        run_in_container(&command, &arg, &root_path, &container_id);
    } else {
        let status = 0 as *mut i32;
        unsafe {
            libc::waitpid(pid, status, 0);
        }
        println!("status: {:?}", status);
    }
}

fn run_in_container(command: &CString, arg: &CString, root_path: &CString, _container_id: &uuid::Uuid) -> i32 {
    unsafe {
        libc::chroot(root_path.as_ptr());
        libc::chdir(CString::new("/").unwrap().as_ptr());
        libc::execvp(
            command.as_ptr(),
            vec![command.as_ptr(), arg.as_ptr(), std::ptr::null()].as_ptr(),
        )
    }
}
