#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod syscall;
use core::panic::PanicInfo;
pub use syscall::*;

#[panic_handler]
fn my_panic_handler(info: &PanicInfo) -> ! {
    if let Some(loc) = info.location() {
        println!(
            "panicked at {}:{} {}",
            loc.file(),
            loc.line(),
            info.message().unwrap(),
        );
    } else {
        println!("panicked: {}", info.message().unwrap());
    }
    loop {}
}

#[no_mangle]
#[link_section = ".text.start"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("shouldn't go here!");
}

#[no_mangle]
#[linkage = "weak"]
fn main() -> isize {
    panic!("Can't find main!!!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|b| unsafe {
        (b as *mut u8).write_volatile(0);
    })
}
