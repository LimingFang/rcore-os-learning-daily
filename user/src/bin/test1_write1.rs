#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::write;
const DATA_STRING: &str = "string from data section\n";

/// 正确输出：
/// string from data section
/// strinstring from stack section
/// strin
/// Test write1 OK!

#[no_mangle]
pub fn main() -> i32 {
    //assert_eq!(write(1, DATA_STRING.as_bytes()), DATA_STRING.len() as isize);
    // assert_eq!(write(1, &DATA_STRING.as_bytes()[..5]), 5);
    let stack_string = "string from stack section\n";
    assert_eq!(
        write(1, stack_string.as_bytes()),
        stack_string.len() as isize
    );
    // assert_eq!(write(1, &stack_string.as_bytes()[..5]), 5);
    println!("\nTest write1 OK!");
    0
}
