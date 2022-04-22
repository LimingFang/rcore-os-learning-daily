use core::arch::asm;

const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_YIELD: usize = 124;

fn sys_call(syscall_id: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    // println!(
    //     "syscall_id = {},arg0 = {},arg1 = {},arg2 = {}",
    //     syscall_id, arg0, arg1, arg2
    // );
    let mut ret;
    unsafe {
        asm!("ecall", inlateout("x10") arg0 => ret,in("x11") arg1,in("x12") arg2,in("x17") syscall_id);
    }
    ret
}

pub fn exit(status: isize) {
    sys_call(SYSCALL_EXIT, status as usize, 0, 0);
}

pub fn write(fd: usize, buffer: &[u8]) -> isize {
    sys_call(SYSCALL_WRITE, fd, buffer.as_ptr() as usize, buffer.len())
}

pub fn yield_() {
    sys_call(SYSCALL_YIELD, 0, 0, 0);
}
