use crate::task::{mark_current_stop, run_next_task};

pub fn sys_exit(exit_st: isize) -> ! {
    println!("[kernel] application exited with code {}", exit_st);
    mark_current_stop();
    run_next_task();
    panic!("[kernel]:Can't reach here");
}

pub fn sys_yield() {
    mark_current_stop();
    run_next_task();
}
