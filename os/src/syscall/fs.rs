use core::{slice, str};

// 检查字节范围
pub fn sys_write(_fd: usize, buf: *const u8, len: usize) -> isize {
    // let buf_lower = buf as usize;
    // let app_manager = TASK_MANAGER.exclusive_access();
    // // let prog_lower = get_app_start(app_manager.get_current_app());
    // // let prog_upper = prog_lower + get_app_len(app_manager.get_current_app());
    // drop(app_manager);
    // if buf_lower < prog_lower || (buf_lower + len) >= prog_upper {
    //     println!(
    //         "buf_lower=0x{:x},prog_lower=0x{:x},len=0x{:x},wrong!",
    //         buf_lower, prog_lower, len
    //     );
    //     return -1;
    // }
    let slice = unsafe { slice::from_raw_parts(buf, len) };
    let str = str::from_utf8(slice).unwrap();
    print!("{}", str);
    str.len() as isize
}
