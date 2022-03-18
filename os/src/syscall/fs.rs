use crate::config::APP_BASE_ADDRESS;
use crate::loader::get_app_len;
use crate::task::APP_MANAGER;
use core::{slice, str};

// 检查字节范围
pub fn sys_write(_fd: usize, buf: *const u8, len: usize) -> isize {
    let buf_lower = buf as usize;
    let app_manager = APP_MANAGER.exclusive_access();
    let prog_lower = APP_BASE_ADDRESS;
    let prog_upper = APP_BASE_ADDRESS + get_app_len(app_manager.get_current_app());
    drop(app_manager);
    if buf_lower < prog_lower || (buf_lower + len) >= prog_upper {
        return -1;
    }
    let slice = unsafe { slice::from_raw_parts(buf, len) };
    let str = str::from_utf8(slice).unwrap();
    print!("{}", str);
    str.len() as isize
}
