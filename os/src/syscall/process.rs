use crate::batch::{run_next_app, APP_MANAGER};

pub fn sys_exit(exit_st: isize) -> isize {
    println!("[kernel] application exited with code {}", exit_st);
    {
        let mut app_manager = APP_MANAGER.exclusive_access();
        app_manager.move_to_next_app();
    }
    run_next_app()
}
#[allow(unused)]
pub fn sys_get_taskinfo() -> isize {
    let app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    println!("[kernel]:app_{} is currently running", current_app);
    0
}
