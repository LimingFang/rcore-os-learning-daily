#![no_std]
#![no_main]

extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    unsafe {
        (0x0 as *mut u8).write_volatile(0);
        // asm!("sret");
    }
    0
}
