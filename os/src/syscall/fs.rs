use core::str;
#[allow(unused)]
pub fn sys_write(_fd: usize, buf: &[u8]) -> isize {
    let chars = str::from_utf8(buf).unwrap();
    println!("{}", chars);
    0
}
