mod trapctx;
use core::arch::global_asm;
use riscv::register::{mtvec::TrapMode, stvec};
use riscv::register::{
    scause::{self, Exception, Trap},
    sstatus, stval,
};
pub use trapctx::TrapCtx;

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
            // 调用 syscall
        }
        _ => {
            panic!("Unsupport trap {:?},stval = {:?}", scause.cause(), stval)
        }
    }
    cx
}
