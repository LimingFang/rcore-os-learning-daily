mod trapctx;
use core::arch::global_asm;
use riscv::register::{mtvec::TrapMode, stvec};
use riscv::register::{
    scause::{self, Exception, Trap},
    stval,
};
pub use trapctx::TrapCtx;

use crate::syscall::syscall;

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

global_asm!(include_str!("alltrap.asm"));

#[no_mangle]
pub fn trap_handler(cx: &mut TrapCtx) -> &mut TrapCtx {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        _ => {
            panic!("Unsupport trap {:?},stval = {:?}", scause.cause(), stval)
        }
    }
    cx
}
