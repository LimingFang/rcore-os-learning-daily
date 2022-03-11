use crate::batch::run_next_app;

pub fn sys_exit(exit_st: isize) -> isize {
    println!("[kernel] Application exited with code {}", exit_st);
    run_next_app()
}
