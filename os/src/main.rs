#![no_main]
#![no_std]
#![feature(panic_info_message)]
use core::arch::global_asm;
use core::panic::PanicInfo;

#[macro_use]
mod console;
mod config;
mod loader;
mod sbi;
mod sync;
mod syscall;
mod task;
mod trap;

use sbi::shutdown;

use crate::task::run_first_task;

#[panic_handler]
fn my_panic_handler(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown();
}

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    trap::init();
    println!("trap mod init");
    loader::load_all_apps();
    println!("load_all_apps");
    run_first_task();
    panic!("Shutdown!!!")
}

fn clear_bss() {
    // 根据 linker script
    // [sbss,ebss] 内的数据需要清零，stack的不需要
    extern "C" {
        fn sbss();
        fn ebss();
    }
    // 需要对内存直接操作，unsafe
    (sbss as usize..ebss as usize).for_each(|a| unsafe {
        (a as *mut u8).write_volatile(0);
    });
}
