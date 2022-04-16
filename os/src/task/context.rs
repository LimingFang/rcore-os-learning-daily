#[derive(Copy, Clone)]
#[repr(C)]
// 和 C 一致的布局
pub struct TaskContext {
    ra: usize,
    sp: usize,
    x: [usize; 12], // s[i] = si.
}

impl TaskContext {
    pub fn Init() -> Self {
        TaskContext {
            ra: 0,
            sp: 0,
            x: [0; 12],
        }
    }

    // TaskManager 初始化 task 时使用
    // 生成的 task 运行时会跳转至 __restore
    pub fn restore_task_ctx(kernel_stack_ptr: usize) -> Self {
        extern "C" {
            fn __restore(ctx: usize);
        }
        TaskContext {
            ra: __restore as usize,
            sp: kernel_stack_ptr,
            x: [0; 12],
        }
    }
}
