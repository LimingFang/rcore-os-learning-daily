use crate::config::MAX_APP_NUM;
use crate::loader::{get_app_num, init_app_ctx};
use crate::sync::UPRefCell;
use lazy_static::*;
mod context;
mod switch;
mod task;

pub use context::TaskContext;
use switch::__switch;
use task::{TaskControlBlk, TaskStatus};

pub struct TaskManager {
    num_app: usize,
    task_mgr_inner: UPRefCell<TaskManagerInner>,
}

pub struct TaskManagerInner {
    tasks: [TaskControlBlk; MAX_APP_NUM],
    current_task: usize,
}

impl TaskManager {
    pub fn run_first_task(&self) {
        let mut inner_mgr = self.task_mgr_inner.exclusive_access();
        let first_task = inner_mgr.tasks[0];
        let mut current_task = TaskContext::init();
        inner_mgr.tasks[0].st = TaskStatus::Running;
        drop(inner_mgr);
        unsafe {
            __switch(
                &mut current_task as *mut TaskContext,
                &first_task.ctx as *const TaskContext,
            );
        }
        panic!("[Kernel]:Shouldn't reach here!");
    }

    // 需要先调用 mark_current_suspend
    // 切换到下一个 Ready task，某个时刻有可能会跳转回来继续运行
    pub fn run_next_task(&self) {
        if let Some(next_task_id) = self.find_next_task() {
            let mut inner_mgr = self.task_mgr_inner.exclusive_access();
            let current_task_id = inner_mgr.current_task;
            inner_mgr.current_task = next_task_id;
            inner_mgr.tasks[next_task_id].st = TaskStatus::Running;
            let current_task = &mut inner_mgr.tasks[current_task_id].ctx as *mut TaskContext;
            let next_task = &inner_mgr.tasks[next_task_id].ctx as *const TaskContext;
            drop(inner_mgr);
            unsafe {
                __switch(current_task, next_task);
            }
        } else {
            panic!("All task finished");
        }
    }

    pub fn mark_current_running(&self) {
        let mut inner_mgr = self.task_mgr_inner.exclusive_access();
        let current_task = inner_mgr.current_task;
        inner_mgr.tasks[current_task].st = TaskStatus::Running;
    }

    pub fn mark_current_suspend(&self) {
        let mut inner_mgr = self.task_mgr_inner.exclusive_access();
        let current_task = inner_mgr.current_task;
        inner_mgr.tasks[current_task].st = TaskStatus::Ready;
    }

    pub fn mark_current_stoped(&self) {
        let mut inner_mgr = self.task_mgr_inner.exclusive_access();
        let current_task = inner_mgr.current_task;
        inner_mgr.tasks[current_task].st = TaskStatus::Exit;
    }

    // 遍历 tasks，返回 Ready 状态的 task
    fn find_next_task(&self) -> Option<usize> {
        let inner_mgr = self.task_mgr_inner.exclusive_access();
        let tasks = &inner_mgr.tasks;
        let current = inner_mgr.current_task;
        (current + 1..current + MAX_APP_NUM)
            .map(|id| id % MAX_APP_NUM)
            .find(|id| tasks[*id].st == TaskStatus::Ready)
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = unsafe {
        TaskManager {
            num_app: get_app_num(),
            task_mgr_inner: UPRefCell::new({
                let mut task_mgr_inner = TaskManagerInner {
                    tasks: [TaskControlBlk::init(); MAX_APP_NUM],
                    current_task: 0,
                };
                for i in 0..MAX_APP_NUM {
                    task_mgr_inner.tasks[i].st = if i < get_app_num() {
                        TaskStatus::Ready
                    } else {
                        TaskStatus::Exit
                    };
                    task_mgr_inner.tasks[i].ctx = TaskContext::restore_task_ctx(init_app_ctx(i));
                }
                task_mgr_inner
            }),
        }
    };
}

// 对 TASK_MANAGER 公开的函数进行封装，避免暴露 TASK_MANAGER
pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

pub fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

pub fn mark_current_suspend() {
    TASK_MANAGER.mark_current_suspend();
}

pub fn mark_current_stop() {
    TASK_MANAGER.mark_current_stoped();
}
