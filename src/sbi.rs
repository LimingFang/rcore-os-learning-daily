#![allow(unused)]

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

use core::arch::asm;

fn sbi_call(id: usize, args: [usize; 3]) -> usize {
    let mut ret;
    unsafe {
        asm!(
        "ecall", inlateout("x10") args[0] => ret,
        in("x11") args[1],
        in("x12") args[2],
        in("x17") id,
        );
    }
    ret
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, [0, 0, 0]);
    panic!("shutdown");
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, [c, 0, 0]);
}
