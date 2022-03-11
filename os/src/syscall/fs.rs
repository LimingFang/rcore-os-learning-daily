use core::{slice, str};
#[allow(unused)]
pub fn sys_write(_fd: usize, buf: *const u8, len: usize) -> isize {
    let slice = unsafe { slice::from_raw_parts(buf, len) };
    let str = str::from_utf8(slice).unwrap();
    println!("{}", str);
    0
}
